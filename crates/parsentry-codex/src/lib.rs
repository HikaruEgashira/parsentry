//! OpenAI Codex integration for Parsentry security scanner.
//!
//! This crate provides functionality to execute OpenAI Codex API for
//! security vulnerability analysis with parallel execution support.

mod executor;
mod parser;
mod prompt;

pub use executor::{CodexExecutor, CodexOutput};
pub use parser::{CodexResponse, ParAnalysis, VulnerabilityInfo};
pub use prompt::{PromptBuilder, PatternContext};

use std::path::PathBuf;

/// Configuration for Codex executor.
#[derive(Debug, Clone)]
pub struct CodexConfig {
    /// OpenAI API key (can be set via environment variable OPENAI_API_KEY).
    pub api_key: Option<String>,
    /// API base URL (defaults to OpenAI's endpoint).
    pub api_base_url: String,
    /// Maximum number of concurrent requests.
    pub max_concurrent: usize,
    /// Timeout in seconds for each request.
    pub timeout_secs: u64,
    /// Enable PoC (Proof of Concept) execution.
    pub enable_poc: bool,
    /// Working directory for Codex execution.
    pub working_dir: PathBuf,
    /// Directory to save Codex execution logs (optional).
    pub log_dir: Option<PathBuf>,
    /// Model to use for Codex (e.g., "gpt-4", "gpt-3.5-turbo", "gpt-4-turbo").
    pub model: String,
}

impl Default for CodexConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            api_base_url: "https://api.openai.com/v1".to_string(),
            max_concurrent: 10,
            timeout_secs: 300,
            enable_poc: false,
            working_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            log_dir: None,
            model: "gpt-4-turbo".to_string(),
        }
    }
}

impl CodexConfig {
    /// Create a new configuration with the specified API key.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
            ..Default::default()
        }
    }

    /// Set the API base URL.
    pub fn with_api_base_url(mut self, url: String) -> Self {
        self.api_base_url = url;
        self
    }

    /// Set the maximum number of concurrent requests.
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max.min(50); // Cap at 50
        self
    }

    /// Set the timeout in seconds.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Enable or disable PoC execution.
    pub fn with_poc(mut self, enable: bool) -> Self {
        self.enable_poc = enable;
        self
    }

    /// Set the working directory.
    pub fn with_working_dir(mut self, dir: PathBuf) -> Self {
        self.working_dir = dir;
        self
    }

    /// Set the log directory for saving execution logs.
    pub fn with_log_dir(mut self, dir: PathBuf) -> Self {
        self.log_dir = Some(dir);
        self
    }

    /// Set the model to use for Codex.
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CodexConfig::default();
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timeout_secs, 300);
        assert!(!config.enable_poc);
        assert_eq!(config.model, "gpt-4-turbo");
    }

    #[test]
    fn test_config_max_concurrent_capped() {
        let config = CodexConfig::default().with_max_concurrent(100);
        assert_eq!(config.max_concurrent, 50);
    }
}
