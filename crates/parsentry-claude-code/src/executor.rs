//! Claude Code CLI executor with parallel execution support.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

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
}

impl ClaudeCodeExecutor {
    /// Create a new executor with the given configuration.
    pub fn new(config: ClaudeCodeConfig) -> Result<Self> {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        Ok(Self {
            claude_path: config.claude_path,
            timeout_secs: config.timeout_secs,
            semaphore,
            working_dir: config.working_dir,
            enable_poc: config.enable_poc,
        })
    }

    /// Execute a prompt and return the parsed output.
    pub async fn execute(&self, prompt: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        // Acquire semaphore permit
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ClaudeCodeError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Claude Code");

        // Execute with timeout
        let result = timeout(
            Duration::from_secs(self.timeout_secs),
            self.spawn_claude_process(prompt),
        )
        .await
        .map_err(|_| ClaudeCodeError::Timeout {
            timeout_secs: self.timeout_secs,
        })?;

        result
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
            .arg("json")
            .arg("--dangerously-skip-permissions")
            .current_dir(&self.working_dir)
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

        // Write prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(prompt.as_bytes())
                .await
                .map_err(ClaudeCodeError::SpawnError)?;
            stdin.flush().await.map_err(ClaudeCodeError::SpawnError)?;
        }

        // Wait for output
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
        debug!("Claude Code raw output length: {} bytes", raw_output.len());

        if !stderr_output.is_empty() {
            warn!("Claude Code stderr: {}", stderr_output);
        }

        if raw_output.is_empty() {
            debug!("Claude Code returned empty output");
            return Err(ClaudeCodeError::ParseError("Empty output from Claude Code".to_string()));
        }

        debug!("Claude Code raw output length: {} bytes", raw_output.len());

        // Parse the output
        self.parse_output(&raw_output)
    }

    /// Parse the JSON output from Claude Code.
    fn parse_output(&self, raw_output: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        // Claude Code output format with --output-format json
        // Try to extract the result from the JSON structure
        let parsed: serde_json::Value = serde_json::from_str(raw_output)
            .map_err(|e| ClaudeCodeError::ParseError(format!("JSON parse error: {}", e)))?;

        // Extract metadata if available
        let cost_usd = parsed.get("total_cost_usd").and_then(|v| v.as_f64());
        let duration_ms = parsed.get("duration_ms").and_then(|v| v.as_u64());
        let session_id = parsed
            .get("session_id")
            .and_then(|v| v.as_str())
            .map(String::from);

        // Get the result content
        let result_str = if let Some(result) = parsed.get("result") {
            if let Some(s) = result.as_str() {
                s.to_string()
            } else {
                result.to_string()
            }
        } else {
            // If no "result" field, treat the whole output as the result
            raw_output.to_string()
        };

        // Claude Code may return markdown with embedded JSON
        // Try to extract JSON from code blocks first
        let json_str = Self::extract_json_from_markdown(&result_str)
            .unwrap_or_else(|| result_str.clone());

        // Parse the response
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
        // Look for ```json ... ``` blocks
        let json_start = text.find("```json")?;
        let content_start = json_start + 7; // length of "```json"
        let remaining = &text[content_start..];

        // Skip whitespace/newline after ```json
        let remaining = remaining.trim_start();

        // Find the closing ```
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
            "cost_usd": 0.01,
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
