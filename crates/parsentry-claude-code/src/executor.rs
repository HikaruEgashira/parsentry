//! Claude Code CLI executor with parallel execution support.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::stream::{
    ContentBlock, ResultMessage, StreamCallback, StreamEvent, StreamMessage,
};

use crate::parser::ClaudeCodeResponse;
use crate::ClaudeCodeConfig;

/// Errors that can occur during Claude Code execution.
#[derive(Error, Debug)]
pub enum ClaudeCodeError {
    #[error("Failed to spawn Claude Code process: {0}")]
    SpawnError(#[from] std::io::Error),

    #[error("Claude Code timed out after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    #[error("Claude Code exited with code {code}: {stderr}")]
    NonZeroExit { code: i32, stderr: String },

    #[error("Failed to parse Claude Code output: {0}")]
    ParseError(String),

    #[error("Claude Code binary not found at: {0}")]
    BinaryNotFound(PathBuf),

    #[error("Semaphore acquisition failed: max concurrent limit reached")]
    ConcurrencyLimit,
}

/// Output from a Claude Code execution.
#[derive(Debug, Clone)]
pub struct ClaudeCodeOutput {
    /// The parsed response from Claude Code.
    pub response: ClaudeCodeResponse,
    /// Cost in USD (if available).
    pub cost_usd: Option<f64>,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Session ID (if available).
    pub session_id: Option<String>,
    /// Raw output for debugging.
    pub raw_output: String,
}

/// Executor for Claude Code CLI with semaphore-based concurrency control.
pub struct ClaudeCodeExecutor {
    claude_path: PathBuf,
    timeout_secs: u64,
    semaphore: Arc<Semaphore>,
    working_dir: PathBuf,
    #[allow(dead_code)]
    enable_poc: bool,
    log_dir: Option<PathBuf>,
    model: Option<String>,
}

impl ClaudeCodeExecutor {
    /// Create a new executor with the given configuration.
    pub fn new(config: ClaudeCodeConfig) -> Result<Self> {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        // Create log directory if specified
        if let Some(ref log_dir) = config.log_dir {
            std::fs::create_dir_all(log_dir).ok();
        }

        Ok(Self {
            claude_path: config.claude_path,
            timeout_secs: config.timeout_secs,
            semaphore,
            working_dir: config.working_dir,
            enable_poc: config.enable_poc,
            log_dir: config.log_dir,
            model: config.model,
        })
    }

    /// Execute a prompt and return the parsed output.
    pub async fn execute(&self, prompt: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ClaudeCodeError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Claude Code");

        timeout(
            Duration::from_secs(self.timeout_secs),
            self.spawn_claude_process(prompt),
        )
        .await
        .map_err(|_| ClaudeCodeError::Timeout {
            timeout_secs: self.timeout_secs,
        })?
    }

    /// Execute with retry logic using exponential backoff.
    pub async fn execute_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(1000 * (1 << attempt.min(5)));
                warn!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.execute(prompt).await {
                Ok(output) => {
                    info!("Claude Code execution succeeded on attempt {}", attempt + 1);
                    return Ok(output);
                }
                Err(e) => {
                    error!("Claude Code execution failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Spawn the Claude Code process and capture output.
    async fn spawn_claude_process(&self, prompt: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError>
    {
        let mut cmd = Command::new(&self.claude_path);

        cmd.arg("--print")
            .arg("--output-format")
            .arg("json");

        // Add model argument if specified
        if let Some(ref model) = self.model {
            cmd.arg("--model").arg(model);
        }

        cmd.current_dir(&self.working_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        debug!("Spawning Claude Code process: {:?}", cmd);

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ClaudeCodeError::BinaryNotFound(self.claude_path.clone())
            } else {
                ClaudeCodeError::SpawnError(e)
            }
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(prompt.as_bytes())
                .await
                .map_err(ClaudeCodeError::SpawnError)?;
            stdin.flush().await.map_err(ClaudeCodeError::SpawnError)?;
        }

        let output = child.wait_with_output().await.map_err(ClaudeCodeError::SpawnError)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ClaudeCodeError::NonZeroExit {
                code: output.status.code().unwrap_or(-1),
                stderr,
            });
        }

        let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();

        // Save log if log_dir is configured
        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("claude_code_{}.log", timestamp));
            let log_content = format!(
                "=== Claude Code Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 \n\
                 === STDOUT ===\n\
                 {}\n\
                 \n\
                 === STDERR ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                raw_output,
                stderr_output
            );
            if let Err(e) = std::fs::write(&log_file, &log_content) {
                warn!("Failed to write Claude Code log: {}", e);
            } else {
                info!("Claude Code log saved: {}", log_file.display());
            }
        }

        if !stderr_output.is_empty() {
            warn!("Claude Code stderr: {}", stderr_output);
        }

        if raw_output.is_empty() {
            debug!("Claude Code returned empty output");
            return Err(ClaudeCodeError::ParseError("Empty output from Claude Code".to_string()));
        }

        self.parse_output(&raw_output)
    }

    /// Parse the JSON output from Claude Code.
    fn parse_output(&self, raw_output: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let parsed: serde_json::Value = serde_json::from_str(raw_output)
            .map_err(|e| ClaudeCodeError::ParseError(format!("JSON parse error: {}", e)))?;

        let cost_usd = parsed.get("total_cost_usd").and_then(|v| v.as_f64());
        let duration_ms = parsed.get("duration_ms").and_then(|v| v.as_u64());
        let session_id = parsed
            .get("session_id")
            .and_then(|v| v.as_str())
            .map(String::from);

        let result_str = if let Some(result) = parsed.get("result") {
            if let Some(s) = result.as_str() {
                s.to_string()
            } else {
                result.to_string()
            }
        } else {
            raw_output.to_string()
        };

        let json_str = Self::extract_json_from_markdown(&result_str)
            .unwrap_or_else(|| result_str.clone());

        let response: ClaudeCodeResponse = serde_json::from_str(&json_str)
            .map_err(|e| ClaudeCodeError::ParseError(format!("Response parse error: {} - Content: {}", e, &json_str.chars().take(200).collect::<String>())))?;

        Ok(ClaudeCodeOutput {
            response,
            cost_usd,
            duration_ms,
            session_id,
            raw_output: raw_output.to_string(),
        })
    }

    /// Extract JSON from markdown code blocks.
    fn extract_json_from_markdown(text: &str) -> Option<String> {
        let json_start = text.find("```json")?;
        let content_start = json_start + 7;
        let remaining = &text[content_start..];

        let remaining = remaining.trim_start();

        let json_end = remaining.find("```")?;
        let json_content = &remaining[..json_end].trim();

        if json_content.is_empty() {
            return None;
        }

        Some(json_content.to_string())
    }

    /// Get the number of available permits.
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Execute a prompt and return raw result string without parsing as ClaudeCodeResponse.
    /// Useful for pattern generation that uses custom JSON structures.
    pub async fn execute_raw(&self, prompt: &str) -> Result<String, ClaudeCodeError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ClaudeCodeError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Claude Code (raw mode)");

        timeout(
            Duration::from_secs(self.timeout_secs),
            self.spawn_claude_process_raw(prompt),
        )
        .await
        .map_err(|_| ClaudeCodeError::Timeout {
            timeout_secs: self.timeout_secs,
        })?
    }

    /// Execute raw with retry logic.
    pub async fn execute_raw_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<String, ClaudeCodeError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(1000 * (1 << attempt.min(5)));
                warn!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.execute_raw(prompt).await {
                Ok(output) => {
                    info!("Claude Code raw execution succeeded on attempt {}", attempt + 1);
                    return Ok(output);
                }
                Err(e) => {
                    error!("Claude Code raw execution failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Spawn the Claude Code process and return raw result string.
    async fn spawn_claude_process_raw(&self, prompt: &str) -> Result<String, ClaudeCodeError> {
        let mut cmd = Command::new(&self.claude_path);

        cmd.arg("--print")
            .arg("--output-format")
            .arg("json");

        // Add model argument if specified
        if let Some(ref model) = self.model {
            cmd.arg("--model").arg(model);
        }

        cmd.current_dir(&self.working_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        debug!("Spawning Claude Code process (raw): {:?}", cmd);

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ClaudeCodeError::BinaryNotFound(self.claude_path.clone())
            } else {
                ClaudeCodeError::SpawnError(e)
            }
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(prompt.as_bytes())
                .await
                .map_err(ClaudeCodeError::SpawnError)?;
            stdin.flush().await.map_err(ClaudeCodeError::SpawnError)?;
        }

        let output = child.wait_with_output().await.map_err(ClaudeCodeError::SpawnError)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ClaudeCodeError::NonZeroExit {
                code: output.status.code().unwrap_or(-1),
                stderr,
            });
        }

        let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();

        // Save log if log_dir is configured
        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("claude_code_raw_{}.log", timestamp));
            let log_content = format!(
                "=== Claude Code Raw Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 \n\
                 === STDOUT ===\n\
                 {}\n\
                 \n\
                 === STDERR ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                raw_output,
                stderr_output
            );
            if let Err(e) = std::fs::write(&log_file, &log_content) {
                warn!("Failed to write Claude Code log: {}", e);
            } else {
                info!("Claude Code log saved: {}", log_file.display());
            }
        }

        if !stderr_output.is_empty() {
            warn!("Claude Code stderr: {}", stderr_output);
        }

        if raw_output.is_empty() {
            debug!("Claude Code returned empty output");
            return Err(ClaudeCodeError::ParseError("Empty output from Claude Code".to_string()));
        }

        // Parse JSON to extract just the result field
        let parsed: serde_json::Value = serde_json::from_str(&raw_output)
            .map_err(|e| ClaudeCodeError::ParseError(format!("JSON parse error: {}", e)))?;

        let result_str = if let Some(result) = parsed.get("result") {
            if let Some(s) = result.as_str() {
                s.to_string()
            } else {
                result.to_string()
            }
        } else {
            raw_output
        };

        Ok(result_str)
    }

    /// Execute with streaming output and callbacks
    pub async fn execute_streaming<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ClaudeCodeError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Claude Code with streaming");

        timeout(
            Duration::from_secs(self.timeout_secs),
            self.spawn_claude_process_streaming(prompt, callback),
        )
        .await
        .map_err(|_| ClaudeCodeError::Timeout {
            timeout_secs: self.timeout_secs,
        })?
    }

    /// Execute with streaming and retry logic
    pub async fn execute_streaming_with_retry<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
        max_retries: u32,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(1000 * (1 << attempt.min(5)));
                warn!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.execute_streaming(prompt, callback).await {
                Ok(output) => {
                    info!(
                        "Claude Code streaming execution succeeded on attempt {}",
                        attempt + 1
                    );
                    return Ok(output);
                }
                Err(e) => {
                    error!("Claude Code streaming execution failed: {}", e);
                    callback.on_event(StreamEvent::Error(format!("Attempt {}: {}", attempt + 1, e)));
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Spawn process with streaming output handling
    async fn spawn_claude_process_streaming<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let mut cmd = Command::new(&self.claude_path);

        cmd.arg("--print")
            .arg("--verbose")
            .arg("--output-format")
            .arg("stream-json");

        if let Some(ref model) = self.model {
            cmd.arg("--model").arg(model);
        }

        cmd.current_dir(&self.working_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        debug!("Spawning Claude Code process with streaming: {:?}", cmd);

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ClaudeCodeError::BinaryNotFound(self.claude_path.clone())
            } else {
                ClaudeCodeError::SpawnError(e)
            }
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(prompt.as_bytes())
                .await
                .map_err(ClaudeCodeError::SpawnError)?;
            stdin.flush().await.map_err(ClaudeCodeError::SpawnError)?;
            drop(stdin);
        }

        let stdout = child.stdout.take().ok_or_else(|| {
            ClaudeCodeError::SpawnError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture stdout",
            ))
        })?;

        let stderr = child.stderr.take().ok_or_else(|| {
            ClaudeCodeError::SpawnError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture stderr",
            ))
        })?;

        let (stdout_result, stderr_output) = tokio::join!(
            self.process_stdout_stream(stdout, callback),
            self.process_stderr_stream(stderr, callback),
        );

        let status = child.wait().await.map_err(ClaudeCodeError::SpawnError)?;

        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("claude_code_streaming_{}.log", timestamp));
            let log_content = format!(
                "=== Claude Code Streaming Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 Status: {:?}\n\
                 \n\
                 === STDERR ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                status,
                stderr_output
            );
            if let Err(e) = std::fs::write(&log_file, &log_content) {
                warn!("Failed to write Claude Code streaming log: {}", e);
            } else {
                info!("Claude Code streaming log saved: {}", log_file.display());
            }
        }

        if !status.success() {
            return Err(ClaudeCodeError::NonZeroExit {
                code: status.code().unwrap_or(-1),
                stderr: stderr_output,
            });
        }

        stdout_result
    }

    /// Process NDJSON stdout stream
    async fn process_stdout_stream<C: StreamCallback>(
        &self,
        stdout: tokio::process::ChildStdout,
        callback: &C,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        let mut accumulated_text = String::new();
        let mut final_result: Option<ResultMessage> = None;

        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<StreamMessage>(&line) {
                Ok(msg) => match msg {
                    StreamMessage::System(system) => {
                        if let Some(ref message) = system.message {
                            callback.on_event(StreamEvent::Progress(message.clone()));
                        }
                        if let Some(ref session_id) = system.session_id {
                            debug!("Stream init: session_id={}", session_id);
                        }
                    }
                    StreamMessage::Assistant(assistant) => {
                        if let Some(ref message) = assistant.message {
                            for block in &message.content {
                                match block {
                                    ContentBlock::Text { text } => {
                                        accumulated_text.push_str(text);
                                        callback.on_event(StreamEvent::Text(text.clone()));
                                    }
                                    ContentBlock::ToolUse { name, input, .. } => {
                                        callback.on_event(StreamEvent::ToolUse {
                                            name: name.clone(),
                                            input: input.clone(),
                                        });
                                    }
                                    ContentBlock::ToolResult {
                                        tool_use_id,
                                        is_error,
                                        ..
                                    } => {
                                        let name = tool_use_id
                                            .clone()
                                            .unwrap_or_else(|| "unknown".to_string());
                                        let success = !is_error.unwrap_or(false);
                                        callback.on_event(StreamEvent::ToolComplete {
                                            name,
                                            success,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    StreamMessage::Result(result) => {
                        callback.on_event(StreamEvent::Complete(result.clone()));
                        final_result = Some(result);
                    }
                },
                Err(e) => {
                    warn!(
                        "Failed to parse stream message: {} - line: {}",
                        e,
                        &line[..line.len().min(100)]
                    );
                }
            }
        }

        // Parse final result
        if let Some(result) = final_result {
            self.parse_stream_result(&result, &accumulated_text)
        } else if !accumulated_text.is_empty() {
            // Fallback: try to parse accumulated text as JSON
            self.parse_accumulated_text(&accumulated_text)
        } else {
            Err(ClaudeCodeError::ParseError(
                "No final result in stream".to_string(),
            ))
        }
    }

    /// Process stderr for progress messages
    async fn process_stderr_stream<C: StreamCallback>(
        &self,
        stderr: tokio::process::ChildStderr,
        callback: &C,
    ) -> String {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        let mut stderr_output = String::new();

        while let Ok(Some(line)) = lines.next_line().await {
            stderr_output.push_str(&line);
            stderr_output.push('\n');

            // Emit progress events for stderr lines
            callback.on_event(StreamEvent::Progress(line));
        }

        stderr_output
    }

    /// Parse the final stream result into ClaudeCodeOutput
    fn parse_stream_result(
        &self,
        result: &ResultMessage,
        _accumulated_text: &str,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let result_str = result
            .result
            .clone()
            .unwrap_or_else(|| "{}".to_string());

        let json_str =
            Self::extract_json_from_markdown(&result_str).unwrap_or_else(|| result_str.clone());

        let response: ClaudeCodeResponse = serde_json::from_str(&json_str).map_err(|e| {
            ClaudeCodeError::ParseError(format!(
                "Response parse error: {} - Content: {}",
                e,
                &json_str.chars().take(200).collect::<String>()
            ))
        })?;

        Ok(ClaudeCodeOutput {
            response,
            cost_usd: result.cost_usd.or(result.total_cost_usd),
            duration_ms: result.duration_ms,
            session_id: result.session_id.clone(),
            raw_output: result_str,
        })
    }

    /// Fallback: parse accumulated text when no result message received
    fn parse_accumulated_text(
        &self,
        accumulated_text: &str,
    ) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let json_str = Self::extract_json_from_markdown(accumulated_text)
            .unwrap_or_else(|| accumulated_text.to_string());

        let response: ClaudeCodeResponse = serde_json::from_str(&json_str).map_err(|e| {
            ClaudeCodeError::ParseError(format!(
                "Accumulated text parse error: {} - Content: {}",
                e,
                &json_str.chars().take(200).collect::<String>()
            ))
        })?;

        Ok(ClaudeCodeOutput {
            response,
            cost_usd: None,
            duration_ms: None,
            session_id: None,
            raw_output: accumulated_text.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let config = ClaudeCodeConfig::default();
        let executor = ClaudeCodeExecutor::new(config).unwrap();
        assert_eq!(executor.available_permits(), 10);
    }

    #[test]
    fn test_parse_output_structure() {
        let executor = ClaudeCodeExecutor::new(ClaudeCodeConfig::default()).unwrap();

        let json_output = r#"{
            "result": "{\"analysis\": \"Test analysis\", \"confidence_score\": 80, \"vulnerability_types\": [\"XSS\"], \"par_analysis\": {\"principals\": [], \"actions\": [], \"resources\": [], \"policy_violations\": []}}",
            "total_cost_usd": 0.01,
            "duration_ms": 1500,
            "session_id": "test-session"
        }"#;

        let result = executor.parse_output(json_output);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output.cost_usd, Some(0.01));
        assert_eq!(output.duration_ms, Some(1500));
        assert_eq!(output.session_id, Some("test-session".to_string()));
    }
}
