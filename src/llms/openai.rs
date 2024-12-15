use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, Function, LLM};

pub struct OpenAI {
    pub model: String,
    pub base_url: String,
    pub client: Client,
    pub system_prompt: String,
}

impl OpenAI {
    pub fn new(model: String, base_url: String, system_prompt: String) -> Self {
        Self {
            model,
            base_url,
            client: Client::new(),
            system_prompt,
        }
    }

    async fn make_request(
        &self,
        messages: Vec<ChatMessage>,
        functions: Option<Vec<Function>>,
        function_call: Option<String>,
    ) -> Result<ChatMessage> {
        #[derive(Serialize)]
        struct Request {
            model: String,
            messages: Vec<ChatMessage>,
            #[serde(skip_serializing_if = "Option::is_none")]
            functions: Option<Vec<Function>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            function_call: Option<String>,
        }

        #[derive(Deserialize)]
        struct Response {
            choices: Vec<Choice>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: ChatMessage,
        }

        let request = Request {
            model: self.model.clone(),
            messages,
            functions,
            function_call,
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", std::env::var("OPENAI_API_KEY")?),
            )
            .json(&request)
            .send()
            .await?;

        let response_text = response.text().await?;

        match serde_json::from_str::<Response>(&response_text) {
            Ok(response) => {
                if response.choices.is_empty() {
                    return Err(anyhow::anyhow!("No choices in response"));
                }
                Ok(response.choices[0].message.clone())
            }
            Err(e) => {
                println!("JSON parsing error: {}", e);
                println!("Failed to parse response: {}", response_text);
                Err(e.into())
            }
        }
    }
}

#[async_trait]
impl LLM for OpenAI {
    async fn chat(&self, messages: &[ChatMessage]) -> Result<ChatMessage> {
        let mut all_messages = vec![ChatMessage {
            role: "system".to_string(),
            content: self.system_prompt.clone(),
            function_call: None,
        }];
        all_messages.extend_from_slice(messages);

        self.make_request(all_messages, None, None).await
    }

    async fn chat_with_functions(
        &self,
        messages: &[ChatMessage],
        functions: &[Function],
        function_call: Option<String>,
    ) -> Result<ChatMessage> {
        let mut all_messages = vec![ChatMessage {
            role: "system".to_string(),
            content: self.system_prompt.clone(),
            function_call: None,
        }];
        all_messages.extend_from_slice(messages);

        self.make_request(
            all_messages,
            Some(functions.to_vec()),
            function_call,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use serde_json::json;
    use std::env;
    use tokio;

    const TEST_MODEL: &str = "gpt-4";
    const TEST_SYSTEM_PROMPT: &str = "You are a helpful AI assistant.";
    const BASE_URL: &str = "https://api.openai.com/v1/chat/completions";

    fn setup_openai() -> OpenAI {
        dotenv().ok();
        OpenAI::new(
            TEST_MODEL.to_string(),
            BASE_URL.to_string(),
            TEST_SYSTEM_PROMPT.to_string(),
        )
    }

    #[tokio::test]
    async fn test_chat_success() {
        dotenv().ok();
        let openai = setup_openai();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
            function_call: None,
        }];

        let result = openai.chat(&messages).await;
        assert!(result.is_ok(), "Chat should succeed with valid API key");

        let response = result.unwrap();
        assert!(!response.content.is_empty(), "Response should not be empty");
    }

    #[tokio::test]
    async fn test_chat_with_functions() {
        dotenv().ok();
        let openai = setup_openai();
        
        let get_weather = Function {
            name: "get_weather".to_string(),
            description: "Get the weather in a given location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    }
                },
                "required": ["location"]
            }),
        };

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What's the weather like in San Francisco?".to_string(),
            function_call: None,
        }];

        let result = openai
            .chat_with_functions(&messages, &[get_weather], None)
            .await;
        
        assert!(result.is_ok(), "Chat with functions should succeed");
        
        let response = result.unwrap();
        assert!(
            response.function_call.is_some(),
            "Response should include a function call"
        );
        
        if let Some(function_call) = response.function_call {
            assert_eq!(function_call.name, "get_weather");
            // Verify the arguments are valid JSON
            let args: serde_json::Value = serde_json::from_str(&function_call.arguments)
                .expect("Function arguments should be valid JSON");
            assert!(args.get("location").is_some(), "Arguments should include location");
        }
    }

    #[tokio::test]
    async fn test_chat_invalid_api_key() {
        // Temporarily set invalid API key
        env::set_var("OPENAI_API_KEY", "invalid_key");

        let openai = setup_openai();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
            function_call: None,
        }];

        let result = openai.chat(&messages).await;
        assert!(result.is_err(), "Chat should fail with invalid API key");
    }
}
