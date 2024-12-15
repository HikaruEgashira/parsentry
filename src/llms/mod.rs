mod claude;
mod ollama;
mod openai;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use self::{claude::Claude, ollama::Ollama, openai::OpenAI};

#[async_trait]
pub trait LLM {
    async fn chat(&self, messages: &[ChatMessage]) -> Result<String>;
}

// OpenAI Message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub fn initialize_llm(llm_arg: &str, system_prompt: &str) -> Result<Box<dyn LLM>> {
    match llm_arg.to_lowercase().as_str() {
        "claude" => {
            let model = std::env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());
            let base_url = std::env::var("ANTHROPIC_BASE_URL")
                .unwrap_or_else(|_| "https://api.anthropic.com/v1/messages".to_string());
            Ok(Box::new(Claude::new(
                model,
                base_url,
                system_prompt.to_string(),
            )))
        }
        "gpt" => {
            let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());
            let base_url = std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
            Ok(Box::new(OpenAI::new(
                model,
                base_url,
                system_prompt.to_string(),
            )))
        }
        "ollama" => {
            let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama2".to_string());
            let base_url = std::env::var("OLLAMA_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());
            Ok(Box::new(Ollama::new(
                model,
                base_url,
                system_prompt.to_string(),
            )))
        }
        _ => anyhow::bail!(
            "Invalid LLM argument: {}. Valid options are: claude, gpt, ollama",
            llm_arg
        ),
    }
}

#[cfg(feature = "integration-tests")]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_claude_initialization() -> Result<()> {
        dotenv().ok();

        let claude = initialize_llm("claude", "You are a helpful assistant.")?;
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Say 'test successful' in exactly those words.".to_string(),
        }];
        let response = claude.chat(&messages[..]).await?;
        assert!(
            response.len() > 0,
            "Claude response has a length greater than 0"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_openai_initialization() -> Result<()> {
        dotenv().ok();

        let openai = initialize_llm("gpt", "You are a helpful assistant.")?;
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Say 'test successful' in exactly those words.".to_string(),
        }];
        let response = openai.chat(&messages[..]).await?;
        assert!(
            response.len() > 0,
            "OpenAI response has a length greater than 0"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_ollama_initialization() -> Result<()> {
        dotenv().ok();

        let ollama = initialize_llm("ollama", "You are a helpful assistant.")?;
        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Say 'test successful' in exactly those words.".to_string(),
        }];
        let response = ollama.chat(&messages[..]).await?;
        assert!(
            response.len() > 0,
            "Ollama response has a length greater than 0"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_llm_initialization() {
        dotenv().ok();

        let result = initialize_llm("invalid", "You are a helpful assistant.");
        assert!(
            result.is_err(),
            "Initialization with invalid LLM should fail"
        );
    }
}
