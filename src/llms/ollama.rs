use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, LLM};

pub struct Ollama {
    pub model: String,
    pub base_url: String,
    pub client: Client,
    pub system_prompt: String,
}

impl Ollama {
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
impl LLM for Ollama {
    async fn chat(&self, messages: &[ChatMessage]) -> Result<ChatMessage> {
        #[derive(Serialize)]
        struct Request {
            model: String,
            prompt: String,
            system: String,
        }

        #[derive(Deserialize)]
        struct Response {
            response: String,
        }

        let prompt = messages
            .iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let request = Request {
            model: self.model.clone(),
            prompt,
            system: self.system_prompt.clone(),
        };

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await?;

        // Print the raw response for debugging
        let response_text = response.text().await?;
        println!("Raw response: {}", response_text);

        match serde_json::from_str::<Response>(&response_text) {
            Ok(response) => {
                if response.response.is_empty() {
                    return Err(anyhow::anyhow!("Empty response from Ollama"));
                }
                Ok(response.response)
            }
            Err(e) => {
                println!("JSON parsing error: {}", e);
                println!("Failed to parse response: {}", response_text);
                Err(e.into())
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

    const TEST_MODEL: &str = "llama2";
    const TEST_SYSTEM_PROMPT: &str = "You are a helpful AI assistant.";
    const BASE_URL: &str = "http://localhost:11434/api/generate";

    fn setup_ollama() -> Ollama {
        dotenv().ok();
        Ollama::new(
            TEST_MODEL.to_string(),
            BASE_URL.to_string(),
            TEST_SYSTEM_PROMPT.to_string(),
        )
    }

    #[tokio::test]
    async fn test_chat_success() {
        let ollama = setup_ollama();
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
        }];

        let result = ollama.chat(&messages).await;
        assert!(
            result.is_ok(),
            "Chat should succeed with valid Ollama server"
        );

        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
    }

    #[tokio::test]
    async fn test_chat_server_unavailable() {
        let mut ollama = setup_ollama();
        // Point to non-existent server
        ollama.base_url = "http://localhost:99999/api/generate".to_string();

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "What is 2+2?".to_string(),
        }];

        let result = ollama.chat(&messages).await;
        assert!(result.is_err(), "Chat should fail with invalid server");
    }

    #[test]
    fn test_ollama_initialization() {
        let ollama = setup_ollama();
        assert_eq!(ollama.model, TEST_MODEL);
        assert_eq!(ollama.base_url, BASE_URL);
        assert_eq!(ollama.system_prompt, TEST_SYSTEM_PROMPT);
    }
}
