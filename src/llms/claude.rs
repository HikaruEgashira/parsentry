use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, LLM};

pub struct Claude {
    pub model: String,
    pub base_url: String,
    pub client: Client,
    pub system_prompt: String,
}

impl Claude {
    pub fn new(model: String, base_url: String, system_prompt: String) -> Self {
        Self {
            model,
            base_url,
            client: Client::new(),
            system_prompt,
        }
    }
}

#[async_trait]
impl LLM for Claude {
    async fn chat(&self, messages: &[ChatMessage]) -> Result<ChatMessage> {
#[derive(Serialize)]
        struct Request {
            model: String,
            max_tokens: u32,
            messages: Vec<Message>,
        }

        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize, Debug)]
        struct Response {
            content: Vec<Content>,
            #[serde(default)]
            role: String,
            #[serde(default)]
            model: String,
            #[serde(default)]
            stop_reason: Option<String>,
            #[serde(default)]
            stop_sequence: Option<String>,
            #[serde(default)]
            usage: Usage,
        }

        #[derive(Deserialize, Debug, Default)]
        struct Content {
            #[serde(rename = "type", default)]
            content_type: String,
            #[serde(default)]
            text: String,
        }

        #[derive(Deserialize, Debug, Default)]
        struct Usage {
            #[serde(default)]
            input_tokens: u32,
            #[serde(default)]
            output_tokens: u32,
        }

        #[derive(Deserialize, Debug)]
        struct ErrorResponse {
            error: ErrorDetail,
        }

        #[derive(Deserialize, Debug)]
        struct ErrorDetail {
            message: String,
            #[serde(rename = "type")]
            error_type: String,
        }

        let filtered_messages: Vec<Message> = messages
            .iter()
            .filter(|msg| !msg.content.trim().is_empty())
            .map(|msg| Message {
                role: msg.role.clone(),
                content: msg.content.clone(),
            })
            .collect();

        if filtered_messages.is_empty() {
            return Err(anyhow::anyhow!("No valid messages provided"));
        }

        // role: "system" is deprecated, use "assistant" instead
        let system_message = Message {
            role: "assistant".to_string(),
            content: self.system_prompt.clone(),
        };

        let mut all_messages = vec![system_message];
        all_messages.extend(filtered_messages);

        let request = Request {
            model: self.model.clone(),
            max_tokens: 1024,
            messages: all_messages,
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("x-api-key", std::env::var("ANTHROPIC_API_KEY")?)
            .header("anthropic-version", "2023-06-01")
            .header("accept", "application/json")
            .json(&request)
            .send()
            .await?;

        let response_text = response.text().await?;
        log::debug!("Raw API response: {}", response_text);

        // First, check for API error response
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
            return Err(anyhow::anyhow!(
                "API Error: {} ({})",
                error_response.error.message,
                error_response.error.error_type
            ));
        }

        // Then try parsing the successful response
        match serde_json::from_str::<Response>(&response_text) {
            Ok(response) => {
                if response.content.is_empty() {
                    log::error!("Empty response content: {}", response_text);
                    return Err(anyhow::anyhow!("Empty response content"));
                }
                Ok(ChatMessage {
                    role: "assistant".to_string(),
                    content: response.content[0].text.clone(),
                    function_call: None,
                })
            }
            Err(parse_error) => {
                log::error!(
                    "JSON Parsing error: {} | Raw response: {}",
                    parse_error,
                    response_text
                );
                Err(anyhow::anyhow!(
                    "Parsing error: {} | Raw response: {}",
                    parse_error,
                    response_text
                ))
            }
        }
    }
}

#[cfg(feature = "integration-tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use tokio;

    const TEST_MODEL: &str = "claude-3-5-sonnet-20241022";
    const TEST_SYSTEM_PROMPT: &str = "You are a helpful AI assistant.";
    const BASE_URL: &str = "https://api.anthropic.com/v1/messages";

    fn setup_claude() -> Claude {
        dotenv().ok();
        Claude::new(
            TEST_MODEL.to_string(),
            BASE_URL.to_string(),
            TEST_SYSTEM_PROMPT.to_string(),
        )
    }

    #[tokio::test]
    async fn test_chat_success() {
        let claude = setup_claude();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
            function_call: None,
        }];

        let result = claude.chat(&messages).await;
        assert!(result.is_ok(), "Chat should succeed with valid API key");

        let response = result.unwrap();
        assert!(!response.content.is_empty(), "Response should not be empty");
    }

    #[tokio::test]
    async fn test_chat_invalid_api_key() {
        env::set_var("ANTHROPIC_API_KEY", "invalid_key");

        let claude = setup_claude();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
            function_call: None,
        }];

        let result = claude.chat(&messages).await;
        assert!(result.is_err(), "Chat should fail with invalid API key");
    }

    #[test]
    fn test_claude_initialization() {
        let claude = setup_claude();
        assert_eq!(claude.model, TEST_MODEL);
        assert_eq!(claude.base_url, BASE_URL);
        assert_eq!(claude.system_prompt, TEST_SYSTEM_PROMPT);
    }

    #[tokio::test]
    async fn test_chat_empty_message() {
        let claude = setup_claude();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "".to_string(),
            function_call: None,
        }];

        let result = claude.chat(&messages).await;
        assert!(result.is_err(), "Chat should fail with empty message");
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No valid messages provided"));
    }
}
