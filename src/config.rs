use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    /// OpenAI API key for LLM analysis
    pub openai_api_key: Option<String>,
    /// GitHub personal access token for private repository access
    pub github_token: Option<String>,
    /// Anthropic API key (alternative to OpenAI)
    pub anthropic_api_key: Option<String>,
    /// OpenRouter API key (alternative to OpenAI)
    pub openrouter_api_key: Option<String>,
    /// Custom API endpoint base URL
    pub api_base_url: Option<String>,
    /// Disable automatic /v1/chat/completions path appending
    pub disable_v1_path: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            openai_api_key: None,
            github_token: None,
            anthropic_api_key: None,
            openrouter_api_key: None,
            api_base_url: None,
            disable_v1_path: false,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            github_token: env::var("GITHUB_TOKEN").ok(),
            anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
            openrouter_api_key: env::var("OPENROUTER_API_KEY").ok(),
            api_base_url: env::var("API_BASE_URL").ok(),
            disable_v1_path: env::var("PARSENTRY_DISABLE_V1_PATH").is_ok(),
        })
    }

    /// Validate configuration for basic requirements
    pub fn validate(&self) -> Result<()> {
        // Check if at least one LLM provider API key is available
        if self.openai_api_key.is_none() 
            && self.anthropic_api_key.is_none() 
            && self.openrouter_api_key.is_none() {
            return Err(anyhow::anyhow!(
                "At least one LLM provider API key is required. Set one of:\n\
                 - OPENAI_API_KEY for OpenAI\n\
                 - ANTHROPIC_API_KEY for Anthropic Claude\n\
                 - OPENROUTER_API_KEY for OpenRouter\n\
                 See docs/configuration.md for setup instructions."
            ));
        }

        // Validate API key formats
        if let Some(ref key) = self.openai_api_key {
            if !key.starts_with("sk-") {
                return Err(anyhow::anyhow!(
                    "Invalid OPENAI_API_KEY format. OpenAI API keys should start with 'sk-'.\n\
                     See https://platform.openai.com/api-keys for setup instructions."
                ));
            }
        }

        if let Some(ref key) = self.anthropic_api_key {
            if !key.starts_with("sk-ant-") {
                return Err(anyhow::anyhow!(
                    "Invalid ANTHROPIC_API_KEY format. Anthropic API keys should start with 'sk-ant-'.\n\
                     See https://console.anthropic.com/ for setup instructions."
                ));
            }
        }

        if let Some(ref key) = self.openrouter_api_key {
            if !key.starts_with("sk-or-") {
                return Err(anyhow::anyhow!(
                    "Invalid OPENROUTER_API_KEY format. OpenRouter API keys should start with 'sk-or-'.\n\
                     See https://openrouter.ai/keys for setup instructions."
                ));
            }
        }

        // Validate GitHub token format if provided
        if let Some(ref token) = self.github_token {
            if !token.starts_with("ghp_") && !token.starts_with("github_pat_") {
                return Err(anyhow::anyhow!(
                    "Invalid GITHUB_TOKEN format. GitHub tokens should start with 'ghp_' (classic) or 'github_pat_' (fine-grained).\n\
                     See https://github.com/settings/tokens for setup instructions."
                ));
            }
        }

        // Validate custom API endpoint configuration
        if self.api_base_url.is_some() && !self.disable_v1_path {
            eprintln!(
                "⚠️  Warning: API_BASE_URL is set but PARSENTRY_DISABLE_V1_PATH is not.\n\
                 This may cause issues with non-OpenAI compatible endpoints.\n\
                 Set PARSENTRY_DISABLE_V1_PATH=1 if using custom endpoints like Groq.\n\
                 See docs/configuration.md for details."
            );
        }

        Ok(())
    }

    /// Validate configuration for specific operations
    pub fn validate_for_operation(&self, operation: Operation) -> Result<()> {
        self.validate()?;
        
        match operation {
            Operation::PrivateRepoAccess => {
                if self.github_token.is_none() {
                    return Err(anyhow::anyhow!(
                        "Private repository access requires GITHUB_TOKEN.\n\
                         Set GITHUB_TOKEN environment variable with appropriate permissions.\n\
                         See docs/configuration.md#github-token for setup instructions."
                    ));
                }
            }
            Operation::CustomEndpoint => {
                if self.api_base_url.is_none() {
                    return Err(anyhow::anyhow!(
                        "Custom endpoint operation requires API_BASE_URL.\n\
                         Set API_BASE_URL environment variable with your endpoint URL.\n\
                         See docs/configuration.md#api-configuration for setup instructions."
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get the primary LLM provider API key
    pub fn get_primary_api_key(&self) -> Option<&String> {
        self.openai_api_key.as_ref()
            .or(self.anthropic_api_key.as_ref())
            .or(self.openrouter_api_key.as_ref())
    }

    /// Get the API base URL, if configured
    pub fn get_api_base_url(&self) -> Option<&String> {
        self.api_base_url.as_ref()
    }

    /// Check if v1 path should be disabled
    pub fn should_disable_v1_path(&self) -> bool {
        self.disable_v1_path
    }

    /// Get GitHub token for repository access
    pub fn get_github_token(&self) -> Option<&String> {
        self.github_token.as_ref()
    }

    /// Check if configuration supports private repository access
    pub fn supports_private_repos(&self) -> bool {
        self.github_token.is_some()
    }

    /// Get configuration summary for debugging
    pub fn debug_summary(&self) -> String {
        format!(
            "Configuration Summary:\n\
             - OpenAI API Key: {}\n\
             - Anthropic API Key: {}\n\
             - OpenRouter API Key: {}\n\
             - GitHub Token: {}\n\
             - API Base URL: {}\n\
             - Disable V1 Path: {}",
            if self.openai_api_key.is_some() { "✓ Set" } else { "✗ Not set" },
            if self.anthropic_api_key.is_some() { "✓ Set" } else { "✗ Not set" },
            if self.openrouter_api_key.is_some() { "✓ Set" } else { "✗ Not set" },
            if self.github_token.is_some() { "✓ Set" } else { "✗ Not set" },
            self.api_base_url.as_deref().unwrap_or("Not set"),
            if self.disable_v1_path { "✓ Enabled" } else { "✗ Disabled" }
        )
    }
}

/// Specific operations that require certain configuration
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    /// Accessing private repositories
    PrivateRepoAccess,
    /// Using custom API endpoints
    CustomEndpoint,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_validation_no_api_keys() {
        let config = Config::default();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid_openai_key() {
        let config = Config {
            openai_api_key: Some("sk-test-key".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_openai_key() {
        let config = Config {
            openai_api_key: Some("invalid-key".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid_anthropic_key() {
        let config = Config {
            anthropic_api_key: Some("sk-ant-test-key".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_github_token() {
        let config = Config {
            openai_api_key: Some("sk-test-key".to_string()),
            github_token: Some("invalid-token".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid_github_token() {
        let config = Config {
            openai_api_key: Some("sk-test-key".to_string()),
            github_token: Some("ghp_test-token".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_private_repo_operation_validation() {
        let config = Config {
            openai_api_key: Some("sk-test-key".to_string()),
            ..Default::default()
        };
        assert!(config.validate_for_operation(Operation::PrivateRepoAccess).is_err());

        let config_with_token = Config {
            openai_api_key: Some("sk-test-key".to_string()),
            github_token: Some("ghp_test-token".to_string()),
            ..Default::default()
        };
        assert!(config_with_token.validate_for_operation(Operation::PrivateRepoAccess).is_ok());
    }

    #[test]
    fn test_get_primary_api_key() {
        let config = Config {
            openai_api_key: Some("sk-openai-key".to_string()),
            anthropic_api_key: Some("sk-ant-anthropic-key".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_primary_api_key(), Some(&"sk-openai-key".to_string()));

        let config_anthropic_only = Config {
            anthropic_api_key: Some("sk-ant-anthropic-key".to_string()),
            ..Default::default()
        };
        assert_eq!(config_anthropic_only.get_primary_api_key(), Some(&"sk-ant-anthropic-key".to_string()));
    }
}