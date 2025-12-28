//! Codex API executor with parallel execution support.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::parser::CodexResponse;
use crate::CodexConfig;

/// Errors that can occur during Codex execution.
#[derive(Error, Debug)]
pub enum CodexError {
    #[error("Failed to make HTTP request: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Codex request timed out after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to parse Codex output: {0}")]
    ParseError(String),

    #[error("API key not provided. Set OPENAI_API_KEY environment variable or pass via config")]
    MissingApiKey,

    #[error("Semaphore acquisition failed: max concurrent limit reached")]
    ConcurrencyLimit,
}

/// Output from a Codex execution.
#[derive(Debug, Clone)]
pub struct CodexOutput {
    /// The parsed response from Codex.
    pub response: CodexResponse,
    /// Cost in USD (if available).
    pub cost_usd: Option<f64>,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Request ID (if available).
    pub request_id: Option<String>,
    /// Raw output for debugging.
    pub raw_output: String,
}

/// Request body for OpenAI chat completions API.
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

/// Response from OpenAI chat completions API.
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    choices: Vec<Choice>,
    #[allow(dead_code)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Usage {
    total_tokens: u32,
}

/// Executor for Codex API with semaphore-based concurrency control.
pub struct CodexExecutor {
    api_key: String,
    api_base_url: String,
    model: String,
    timeout_secs: u64,
    semaphore: Arc<Semaphore>,
    working_dir: PathBuf,
    #[allow(dead_code)]
    enable_poc: bool,
    log_dir: Option<PathBuf>,
    http_client: HttpClient,
}

impl CodexExecutor {
    /// Create a new executor with the given configuration.
    pub fn new(config: CodexConfig) -> Result<Self, CodexError> {
        let api_key = config
            .api_key
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .ok_or(CodexError::MissingApiKey)?;

        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        // Create log directory if specified
        if let Some(ref log_dir) = config.log_dir {
            std::fs::create_dir_all(log_dir).ok();
        }

        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()?;

        Ok(Self {
            api_key,
            api_base_url: config.api_base_url,
            model: config.model,
            timeout_secs: config.timeout_secs,
            semaphore,
            working_dir: config.working_dir,
            enable_poc: config.enable_poc,
            log_dir: config.log_dir,
            http_client,
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

        timeout(
            Duration::from_secs(self.timeout_secs),
            self.call_api(prompt),
        )
        .await
        .map_err(|_| CodexError::Timeout {
            timeout_secs: self.timeout_secs,
        })?
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
                    error!("Codex execution failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Call the OpenAI API and parse the response.
    async fn call_api(&self, prompt: &str) -> Result<CodexOutput, CodexError> {
        let start_time = std::time::Instant::now();

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            response_format: Some(ResponseFormat {
                format_type: "json_object".to_string(),
            }),
            temperature: Some(0.0),
        };

        debug!("Sending request to OpenAI API: {}", self.api_base_url);

        let response = self
            .http_client
            .post(format!("{}/chat/completions", self.api_base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CodexError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let response_text = response.text().await?;
        let api_response: ChatCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| CodexError::ParseError(format!("Failed to parse API response: {}", e)))?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // Save log if log_dir is configured
        if let Some(ref log_dir) = self.log_dir {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
            let log_file = log_dir.join(format!("codex_{}.log", timestamp));
            let log_content = format!(
                "=== Codex Execution Log ===\n\
                 Timestamp: {}\n\
                 Working Dir: {}\n\
                 Model: {}\n\
                 Duration: {}ms\n\
                 \n\
                 === RESPONSE ===\n\
                 {}\n",
                chrono::Utc::now().to_rfc3339(),
                self.working_dir.display(),
                self.model,
                duration_ms,
                response_text
            );
            if let Err(e) = std::fs::write(&log_file, &log_content) {
                warn!("Failed to write Codex log: {}", e);
            } else {
                info!("Codex log saved: {}", log_file.display());
            }
        }

        if api_response.choices.is_empty() {
            return Err(CodexError::ParseError("No choices in API response".to_string()));
        }

        let content = &api_response.choices[0].message.content;

        // Extract JSON from markdown if present
        let json_str = Self::extract_json_from_markdown(content)
            .unwrap_or_else(|| content.clone());

        let response: CodexResponse = serde_json::from_str(&json_str)
            .map_err(|e| CodexError::ParseError(format!("Response parse error: {} - Content: {}", e, &json_str.chars().take(200).collect::<String>())))?;

        Ok(CodexOutput {
            response,
            cost_usd: None, // OpenAI doesn't provide cost in response
            duration_ms: Some(duration_ms),
            request_id: Some(api_response.id),
            raw_output: response_text,
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

    /// Execute a prompt and return raw result string without parsing as CodexResponse.
    /// Useful for pattern generation that uses custom JSON structures.
    pub async fn execute_raw(&self, prompt: &str) -> Result<String, CodexError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| CodexError::ConcurrencyLimit)?;

        debug!("Acquired semaphore permit, executing Codex request (raw mode)");

        timeout(
            Duration::from_secs(self.timeout_secs),
            self.call_api_raw(prompt),
        )
        .await
        .map_err(|_| CodexError::Timeout {
            timeout_secs: self.timeout_secs,
        })?
    }

    /// Execute raw with retry logic.
    pub async fn execute_raw_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<String, CodexError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(1000 * (1 << attempt.min(5)));
                warn!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.execute_raw(prompt).await {
                Ok(output) => {
                    info!("Codex raw execution succeeded on attempt {}", attempt + 1);
                    return Ok(output);
                }
                Err(e) => {
                    error!("Codex raw execution failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Call API and return raw content string.
    async fn call_api_raw(&self, prompt: &str) -> Result<String, CodexError> {
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            response_format: Some(ResponseFormat {
                format_type: "json_object".to_string(),
            }),
            temperature: Some(0.0),
        };

        debug!("Sending raw request to OpenAI API: {}", self.api_base_url);

        let response = self
            .http_client
            .post(format!("{}/chat/completions", self.api_base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CodexError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let response_text = response.text().await?;
        let api_response: ChatCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| CodexError::ParseError(format!("Failed to parse API response: {}", e)))?;

        if api_response.choices.is_empty() {
            return Err(CodexError::ParseError("No choices in API response".to_string()));
        }

        Ok(api_response.choices[0].message.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_markdown() {
        let text = r#"Here is the JSON:
```json
{"key": "value"}
```
End of JSON"#;
        let result = CodexExecutor::extract_json_from_markdown(text);
        assert_eq!(result, Some(r#"{"key": "value"}"#.to_string()));
    }
}
