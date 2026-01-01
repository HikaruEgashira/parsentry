//! Claude Code executor with ACP (Agent Client Protocol) support.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::LocalSet;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::acp::AcpConnection;
use crate::parser::ClaudeCodeResponse;
use crate::ClaudeCodeConfig;

/// Errors that can occur during Claude Code execution.
#[derive(Error, Debug)]
pub enum ClaudeCodeError {
    #[error("Failed to spawn Claude Code process: {0}")]
    SpawnError(#[from] std::io::Error),

    #[error("Claude Code timed out after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    #[error("ACP connection error: {0}")]
    AcpError(String),

    #[error("Failed to parse Claude Code output: {0}")]
    ParseError(String),

    #[error("Claude Code binary not found at: {0}")]
    BinaryNotFound(PathBuf),

    #[error("Semaphore acquisition failed: max concurrent limit reached")]
    ConcurrencyLimit,

    #[error("No active session")]
    NoSession,
}

impl From<anyhow::Error> for ClaudeCodeError {
    fn from(e: anyhow::Error) -> Self {
        ClaudeCodeError::AcpError(e.to_string())
    }
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
}

/// Executor for Claude Code with ACP protocol and semaphore-based concurrency control.
pub struct ClaudeCodeExecutor {
    claude_path: PathBuf,
    timeout_secs: u64,
    semaphore: Arc<Semaphore>,
    working_dir: PathBuf,
    model: Option<String>,
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
            model: config.model,
        })
    }

    /// Execute a prompt and return the parsed output.
    pub async fn execute(&self, prompt: &str) -> Result<ClaudeCodeOutput, ClaudeCodeError> {
        let start = std::time::Instant::now();
        let result_text = self.execute_raw(prompt).await?;
        let duration_ms = start.elapsed().as_millis() as u64;

        // Parse the response as ClaudeCodeResponse
        let parsed: ClaudeCodeResponse = serde_json::from_str(&result_text).or_else(|_| {
            // Try to extract JSON from markdown
            Self::extract_json_from_markdown(&result_text)
                .ok_or_else(|| ClaudeCodeError::ParseError("No JSON found".to_string()))
                .and_then(|json| {
                    serde_json::from_str(&json)
                        .map_err(|e| ClaudeCodeError::ParseError(e.to_string()))
                })
        })?;

        Ok(ClaudeCodeOutput {
            response: parsed,
            cost_usd: None,
            duration_ms: Some(duration_ms),
            session_id: None,
        })
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

    /// Execute a prompt and return raw result string without parsing as ClaudeCodeResponse.
    pub async fn execute_raw(&self, prompt: &str) -> Result<String, ClaudeCodeError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ClaudeCodeError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing raw via ACP");

        let timeout_secs = self.timeout_secs;
        let claude_path = self.claude_path.clone();
        let working_dir = self.working_dir.clone();
        let model = self.model.clone();
        let prompt = prompt.to_string();

        // Run ACP operations in LocalSet to support spawn_local
        let local = LocalSet::new();
        let result = local
            .run_until(async move {
                timeout(Duration::from_secs(timeout_secs), async {
                    // Create fresh connection within LocalSet context
                    let mut conn = AcpConnection::spawn(&claude_path, &working_dir, model.as_deref())
                        .await
                        .map_err(|e| ClaudeCodeError::AcpError(e.to_string()))?;

                    conn.initialize()
                        .await
                        .map_err(|e| ClaudeCodeError::AcpError(e.to_string()))?;
                    conn.new_session()
                        .await
                        .map_err(|e| ClaudeCodeError::AcpError(e.to_string()))?;

                    let result = conn
                        .prompt(&prompt)
                        .await
                        .map_err(|e| ClaudeCodeError::AcpError(e.to_string()))?;

                    conn.close()
                        .await
                        .map_err(|e| ClaudeCodeError::AcpError(e.to_string()))?;

                    Ok::<String, ClaudeCodeError>(result)
                })
                .await
            })
            .await;

        result.map_err(|_| ClaudeCodeError::Timeout { timeout_secs })?
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
                    info!(
                        "Claude Code raw execution succeeded on attempt {}",
                        attempt + 1
                    );
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
    fn test_extract_json_from_markdown() {
        let text = r#"Here is the result:

```json
{"key": "value"}
```

Done."#;

        let result = ClaudeCodeExecutor::extract_json_from_markdown(text);
        assert_eq!(result, Some(r#"{"key": "value"}"#.to_string()));
    }

    #[test]
    fn test_extract_json_from_markdown_no_json() {
        let text = "No JSON here";
        let result = ClaudeCodeExecutor::extract_json_from_markdown(text);
        assert_eq!(result, None);
    }
}
