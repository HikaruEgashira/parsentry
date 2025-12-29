//! Codex CLI executor with parallel execution support.

use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::parser::CodexResponse;
use crate::stream::{ResultMessage, StreamCallback, StreamEvent};
use crate::CodexConfig;

/// Errors that can occur during Codex execution.
#[derive(Error, Debug)]
pub enum CodexError {
    #[error("Failed to spawn Codex process: {0}")]
    SpawnError(#[from] std::io::Error),

    #[error("Codex request timed out after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    #[error("Codex process failed with exit code {code}: {message}")]
    ProcessError { code: i32, message: String },

    #[error("Failed to parse Codex output: {0}")]
    ParseError(String),

    #[error("Semaphore acquisition failed: max concurrent limit reached")]
    ConcurrencyLimit,
}

/// Output from a Codex execution.
#[derive(Debug, Clone)]
pub struct CodexOutput {
    /// The parsed response from Codex.
    pub response: CodexResponse,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Raw output for debugging.
    pub raw_output: String,
}

/// Executor for Codex CLI with semaphore-based concurrency control.
pub struct CodexExecutor {
    codex_path: PathBuf,
    model: String,
    timeout_secs: u64,
    semaphore: Arc<Semaphore>,
    working_dir: PathBuf,
    #[allow(dead_code)]
    enable_poc: bool,
    log_dir: Option<PathBuf>,
}

impl CodexExecutor {
    /// Create a new executor with the given configuration.
    pub fn new(config: CodexConfig) -> Result<Self, CodexError> {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        // Create log directory if specified
        if let Some(ref log_dir) = config.log_dir {
            std::fs::create_dir_all(log_dir).ok();
        }

        // Use codex from PATH or config
        let codex_path = config.codex_path.unwrap_or_else(|| PathBuf::from("codex"));

        Ok(Self {
            codex_path,
            model: config.model,
            timeout_secs: config.timeout_secs,
            semaphore,
            working_dir: config.working_dir,
            enable_poc: config.enable_poc,
            log_dir: config.log_dir,
        })
    }

    /// Execute a prompt and return the parsed output.
    pub async fn execute(&self, prompt: &str) -> Result<CodexOutput, CodexError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| CodexError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Codex request");

        let start = std::time::Instant::now();

        let result = timeout(
            Duration::from_secs(self.timeout_secs),
            self.run_codex(prompt),
        )
        .await
        .map_err(|_| CodexError::Timeout {
            timeout_secs: self.timeout_secs,
        })?;

        let duration_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(mut output) => {
                output.duration_ms = Some(duration_ms);
                Ok(output)
            }
            Err(e) => Err(e),
        }
    }

    /// Run the Codex CLI process.
    async fn run_codex(&self, prompt: &str) -> Result<CodexOutput, CodexError> {
        let mut cmd = Command::new(&self.codex_path);

        cmd.arg("exec");

        // Only pass model if explicitly configured (non-empty)
        if !self.model.is_empty() {
            cmd.arg("-m").arg(&self.model);
        }

        cmd.arg("-C")
            .arg(&self.working_dir)
            .arg("--full-auto")
            .arg("--json")
            .arg("--skip-git-repo-check")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        debug!("Spawning Codex process: {:?}", cmd);

        let mut child = cmd.spawn()?;

        // Write prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(prompt.as_bytes()).await?;
            stdin.flush().await?;
            drop(stdin);
        }

        let output = child.wait_with_output().await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Log output if log_dir is configured
        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("codex_{}.log", timestamp));
            let log_content = format!(
                "=== Codex Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 Model: {}\n\
                 Exit Code: {:?}\n\
                 \n\
                 === STDOUT ===\n\
                 {}\n\
                 \n\
                 === STDERR ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                self.model,
                output.status.code(),
                stdout,
                stderr
            );
            if let Err(e) = std::fs::write(&log_file, log_content) {
                warn!("Failed to write log file: {}", e);
            }
        }

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            error!("Codex process failed with code {}: {}", code, stderr);
            return Err(CodexError::ProcessError {
                code,
                message: stderr,
            });
        }

        // Parse JSONL output - look for the last message with content
        let response = self.parse_jsonl_output(&stdout)?;

        Ok(CodexOutput {
            response,
            duration_ms: None,
            raw_output: stdout,
        })
    }

    /// Parse JSONL output from Codex CLI.
    fn parse_jsonl_output(&self, output: &str) -> Result<CodexResponse, CodexError> {
        // Look for the last assistant message in JSONL output
        let mut last_content: Option<String> = None;

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                // Look for message events with assistant content
                if let Some(msg_type) = json.get("type").and_then(|t| t.as_str()) {
                    if msg_type == "message" || msg_type == "assistant" {
                        if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                            last_content = Some(content.to_string());
                        } else if let Some(text) = json.get("text").and_then(|t| t.as_str()) {
                            last_content = Some(text.to_string());
                        }
                    }
                }

                // Also check for direct content field
                if let Some(content) = json.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                    last_content = Some(content.to_string());
                }
            }
        }

        // If no structured output found, try to extract JSON from the raw output
        let content = last_content.unwrap_or_else(|| output.to_string());

        // Try to extract JSON from markdown code blocks
        let json_str = extract_json_from_markdown(&content);

        // Parse as CodexResponse
        match serde_json::from_str::<CodexResponse>(&json_str) {
            Ok(response) => Ok(response),
            Err(e) => {
                debug!("Failed to parse as CodexResponse: {}", e);
                // Create a minimal response with the raw content
                Ok(CodexResponse {
                    analysis: content,
                    ..Default::default()
                })
            }
        }
    }

    /// Execute with retry logic using exponential backoff.
    pub async fn execute_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<CodexOutput, CodexError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(1000 * (1 << attempt.min(5)));
                warn!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.execute(prompt).await {
                Ok(output) => {
                    info!("Codex execution succeeded on attempt {}", attempt + 1);
                    return Ok(output);
                }
                Err(e) => {
                    error!("Codex execution failed on attempt {}: {}", attempt + 1, e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or(CodexError::ParseError(
            "Unknown error after retries".to_string(),
        )))
    }

    /// Execute a prompt and return the raw output string.
    pub async fn execute_raw(&self, prompt: &str) -> Result<String, CodexError> {
        let output = self.execute(prompt).await?;
        Ok(output.raw_output)
    }

    /// Execute raw with retry logic using exponential backoff.
    pub async fn execute_raw_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<String, CodexError> {
        let output = self.execute_with_retry(prompt, max_retries).await?;
        Ok(output.raw_output)
    }

    /// Execute with streaming output and callbacks
    pub async fn execute_streaming<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
    ) -> Result<CodexOutput, CodexError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| CodexError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Codex with streaming");

        let start = std::time::Instant::now();

        let result = timeout(
            Duration::from_secs(self.timeout_secs),
            self.run_codex_streaming(prompt, callback),
        )
        .await
        .map_err(|_| CodexError::Timeout {
            timeout_secs: self.timeout_secs,
        })?;

        let duration_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(mut output) => {
                output.duration_ms = Some(duration_ms);
                Ok(output)
            }
            Err(e) => Err(e),
        }
    }

    /// Execute with streaming and retry logic
    pub async fn execute_streaming_with_retry<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
        max_retries: u32,
    ) -> Result<CodexOutput, CodexError> {
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
                        "Codex streaming execution succeeded on attempt {}",
                        attempt + 1
                    );
                    return Ok(output);
                }
                Err(e) => {
                    error!("Codex streaming execution failed: {}", e);
                    callback.on_event(StreamEvent::Error(format!("Attempt {}: {}", attempt + 1, e)));
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or(CodexError::ParseError(
            "Unknown error after retries".to_string(),
        )))
    }

    /// Run the Codex CLI process with streaming output handling.
    async fn run_codex_streaming<C: StreamCallback>(
        &self,
        prompt: &str,
        callback: &C,
    ) -> Result<CodexOutput, CodexError> {
        let mut cmd = Command::new(&self.codex_path);

        cmd.arg("exec");

        // Only pass model if explicitly configured (non-empty)
        if !self.model.is_empty() {
            cmd.arg("-m").arg(&self.model);
        }

        cmd.arg("-C")
            .arg(&self.working_dir)
            .arg("--full-auto")
            .arg("--json")
            .arg("--skip-git-repo-check")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        debug!("Spawning Codex process with streaming: {:?}", cmd);

        let mut child = cmd.spawn()?;

        // Write prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(prompt.as_bytes()).await?;
            stdin.flush().await?;
            drop(stdin);
        }

        let stdout = child.stdout.take().ok_or_else(|| {
            CodexError::SpawnError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture stdout",
            ))
        })?;

        let stderr = child.stderr.take().ok_or_else(|| {
            CodexError::SpawnError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture stderr",
            ))
        })?;

        // Process stdout and stderr concurrently
        let (stdout_result, stderr_output) = tokio::join!(
            self.process_stdout_stream(stdout, callback),
            self.process_stderr_stream(stderr, callback),
        );

        let status = child.wait().await?;

        // Log output if log_dir is configured
        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("codex_streaming_{}.log", timestamp));
            let log_content = format!(
                "=== Codex Streaming Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 Model: {}\n\
                 Status: {:?}\n\
                 \n\
                 === STDERR ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                self.model,
                status,
                stderr_output
            );
            if let Err(e) = std::fs::write(&log_file, log_content) {
                warn!("Failed to write log file: {}", e);
            }
        }

        if !status.success() {
            let code = status.code().unwrap_or(-1);
            error!("Codex process failed with code {}: {}", code, stderr_output);
            return Err(CodexError::ProcessError {
                code,
                message: stderr_output,
            });
        }

        stdout_result
    }

    /// Process JSONL stdout stream
    async fn process_stdout_stream<C: StreamCallback>(
        &self,
        stdout: tokio::process::ChildStdout,
        callback: &C,
    ) -> Result<CodexOutput, CodexError> {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        let mut accumulated_output = String::new();
        let mut last_content: Option<String> = None;

        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().is_empty() {
                continue;
            }

            accumulated_output.push_str(&line);
            accumulated_output.push('\n');

            // Parse the JSONL line
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                // Look for message events with assistant content
                if let Some(msg_type) = json.get("type").and_then(|t| t.as_str()) {
                    match msg_type {
                        "message" | "assistant" => {
                            if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                                last_content = Some(content.to_string());
                                callback.on_event(StreamEvent::Text(content.to_string()));
                            } else if let Some(text) = json.get("text").and_then(|t| t.as_str()) {
                                last_content = Some(text.to_string());
                                callback.on_event(StreamEvent::Text(text.to_string()));
                            }
                        }
                        "tool_use" => {
                            let name = json.get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let input = json.get("input").cloned().unwrap_or(serde_json::Value::Null);
                            callback.on_event(StreamEvent::ToolUse { name, input });
                        }
                        "tool_result" => {
                            let name = json.get("tool_use_id")
                                .and_then(|n| n.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let success = !json.get("is_error")
                                .and_then(|e| e.as_bool())
                                .unwrap_or(false);
                            callback.on_event(StreamEvent::ToolComplete { name, success });
                        }
                        _ => {}
                    }
                }

                // Also check for direct content field
                if let Some(content) = json.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                    last_content = Some(content.to_string());
                }
            }
        }

        // Parse the final response
        let content = last_content.unwrap_or_else(|| accumulated_output.clone());
        let json_str = extract_json_from_markdown(&content);

        let response = match serde_json::from_str::<CodexResponse>(&json_str) {
            Ok(resp) => resp,
            Err(_) => CodexResponse {
                analysis: content.clone(),
                ..Default::default()
            },
        };

        // Emit completion event
        callback.on_event(StreamEvent::Complete(ResultMessage {
            result: Some(content),
            duration_ms: None,
            is_error: Some(false),
        }));

        Ok(CodexOutput {
            response,
            duration_ms: None,
            raw_output: accumulated_output,
        })
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
}

/// Extract JSON content from markdown code blocks.
fn extract_json_from_markdown(text: &str) -> String {
    // Look for ```json blocks
    if let Some(start) = text.find("```json") {
        let content_start = start + 7;
        if let Some(end) = text[content_start..].find("```") {
            return text[content_start..content_start + end].trim().to_string();
        }
    }

    // Look for ``` blocks (generic code blocks)
    if let Some(start) = text.find("```") {
        let content_start = start + 3;
        // Skip language identifier if present
        let content_start = text[content_start..]
            .find('\n')
            .map(|i| content_start + i + 1)
            .unwrap_or(content_start);
        if let Some(end) = text[content_start..].find("```") {
            let extracted = text[content_start..content_start + end].trim();
            if extracted.starts_with('{') || extracted.starts_with('[') {
                return extracted.to_string();
            }
        }
    }

    // Try to find raw JSON object
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return text[start..=end].to_string();
            }
        }
    }

    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_markdown() {
        let input = r#"Here is the analysis:

```json
{"analysis": "test", "confidence_score": 80}
```

That's it."#;

        let result = extract_json_from_markdown(input);
        assert!(result.contains("analysis"));
        assert!(result.contains("confidence_score"));
    }

    #[test]
    fn test_extract_raw_json() {
        let input = r#"Result: {"analysis": "test"}"#;
        let result = extract_json_from_markdown(input);
        assert_eq!(result, r#"{"analysis": "test"}"#);
    }
}
