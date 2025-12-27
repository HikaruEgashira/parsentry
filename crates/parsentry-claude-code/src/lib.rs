//! Claude Code CLI integration for Parsentry security scanner.
//!
//! This crate provides functionality to execute Claude Code CLI for
//! security vulnerability analysis with parallel execution support.

mod executor;
mod parser;
mod prompt;

pub use executor::{ClaudeCodeExecutor, ClaudeCodeOutput};
pub use parser::{ClaudeCodeResponse, ParAnalysis, VulnerabilityInfo};
pub use prompt::{PromptBuilder, PatternContext};

use std::path::PathBuf;

/// Configuration for Claude Code executor.
#[derive(Debug, Clone)]
pub struct ClaudeCodeConfig {
    /// Path to the claude CLI binary.
    pub claude_path: PathBuf,
    /// Maximum number of concurrent processes.
    pub max_concurrent: usize,
    /// Timeout in seconds for each execution.
    pub timeout_secs: u64,
    /// Enable PoC (Proof of Concept) execution.
    pub enable_poc: bool,
    /// Working directory for Claude Code execution.
    pub working_dir: PathBuf,
    /// Directory to save Claude Code execution logs (optional).
    pub log_dir: Option<PathBuf>,
}

impl Default for ClaudeCodeConfig {
    fn default() -> Self {
        Self {
            claude_path: PathBuf::from("claude"),
            max_concurrent: 10,
            timeout_secs: 300,
            enable_poc: false,
            working_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            log_dir: None,
        }
    }
}

impl ClaudeCodeConfig {
    /// Create a new configuration with the specified claude path.
    pub fn new(claude_path: PathBuf) -> Self {
        Self {
            claude_path,
            ..Default::default()
        }
    }

    /// Set the maximum number of concurrent processes.
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max.min(10); // Cap at 10
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClaudeCodeConfig::default();
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timeout_secs, 300);
        assert!(!config.enable_poc);
    }

    #[test]
    fn test_config_max_concurrent_capped() {
        let config = ClaudeCodeConfig::default().with_max_concurrent(100);
        assert_eq!(config.max_concurrent, 10);
    }
}
