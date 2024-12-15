use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, LLM};

#[derive(Serialize)]
pub struct Tool {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
struct Request {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
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
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Debug)]
struct ToolCall {
    id: String,
    name: String,
    arguments: serde_json::Value,
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
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {

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
            tools: None,
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("x-api-key", std::env::var("ANTHROPIC_API_KEY")?)
            .header("anthropic-version", "2023-01-01")
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
                Ok(response.content[0].text.clone())
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use serde_json;

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

    #[test]
    fn test_claude_initialization() {
        let claude = setup_claude();
        assert_eq!(claude.model, TEST_MODEL);
        assert_eq!(claude.base_url, BASE_URL);
        assert_eq!(claude.system_prompt, TEST_SYSTEM_PROMPT);
    }

    #[test]
    fn test_chat_empty_message() {
        let claude = setup_claude();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "".to_string(),
        }];

        let result = tokio_test::block_on(claude.chat(&messages));
        assert!(result.is_err(), "Chat should fail with empty message");
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No valid messages provided"));
    }

    #[test]
    fn test_request_serialization() {
        let tool = Tool {
            name: "calculator".to_string(),
            description: "A basic calculator".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"]
                    },
                    "operands": {
                        "type": "array",
                        "items": {"type": "number"}
                    }
                },
                "required": ["operation", "operands"]
            }),
        };

        let request = Request {
            model: TEST_MODEL.to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: "What is 2+2?".to_string(),
            }],
            tools: Some(vec![tool]),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("tools"));
        assert!(serialized.contains("calculator"));
        assert!(serialized.contains("add"));
        assert!(serialized.contains("operands"));
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = r#"{
            "content": [{"type": "text", "text": "4"}],
            "role": "assistant",
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "end_turn",
            "stop_sequence": null,
            "usage": {"input_tokens": 10, "output_tokens": 1},
            "tool_calls": [{"id": "1", "name": "calculator", "arguments": {"operation": "add", "operands": [2, 2]}}]
        }"#;

        let response: Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.content[0].text, "4");
        assert_eq!(response.role, "assistant");
        assert_eq!(response.model, "claude-3-5-sonnet-20241022");
        assert_eq!(response.stop_reason, Some("end_turn".to_string()));
        assert_eq!(response.stop_sequence, None);
        assert_eq!(response.usage.input_tokens, 10);
        assert_eq!(response.usage.output_tokens, 1);
        assert!(response.tool_calls.is_some());
        let tool_calls = response.tool_calls.unwrap();
        assert_eq!(tool_calls[0].id, "1");
        assert_eq!(tool_calls[0].name, "calculator");
        assert_eq!(
            tool_calls[0].arguments,
            serde_json::json!({
                "operation": "add",
                "operands": [2, 2]
            })
        );
    }
}
