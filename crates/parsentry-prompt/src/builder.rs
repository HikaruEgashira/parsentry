//! PromptBuilder wrapper for backward compatibility.

use std::path::Path;

use crate::analysis::AnalysisPrompt;
use crate::traits::Prompt;
use crate::types::{Origin, OutputConfig, Scope};

/// Builder for constructing analysis prompts.
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    _language: String,
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self {
            _language: "ja".to_string(),
        }
    }
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Kept for API compatibility; no-op (agentic mode doesn't need PoC config).
    pub fn with_poc(self, _enable: bool) -> Self {
        self
    }

    /// Kept for API compatibility; stored but not injected into prompt.
    pub fn with_language(mut self, lang: &str) -> Self {
        self._language = lang.to_string();
        self
    }

    /// Build a minimal prompt from file reference + optional pattern.
    pub fn build_file_reference_prompt(
        &self,
        file_path: &Path,
        pattern_type: Option<&str>,
        matched_code: Option<&str>,
        sarif_output_path: Option<&Path>,
    ) -> String {
        let origin = match (pattern_type, matched_code) {
            (Some(pt), Some(mc)) => Origin::pattern(file_path, pt, mc),
            _ => Origin::file(file_path),
        };

        let output = sarif_output_path
            .map(|p| OutputConfig::Sarif(p.to_path_buf()))
            .unwrap_or(OutputConfig::Text);

        let prompt = AnalysisPrompt {
            origin,
            scope: Scope::SecurityAudit,
            output,
        };

        prompt.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_builder_minimal() {
        let builder = PromptBuilder::new().with_poc(false).with_language("ja");

        let prompt = builder.build_file_reference_prompt(
            &PathBuf::from("src/main.rs"),
            None,
            None,
            None,
        );

        assert!(prompt.contains("src/main.rs"));
        assert!(!prompt.contains("PAR"));
    }

    #[test]
    fn test_builder_with_pattern() {
        let builder = PromptBuilder::new();

        let prompt = builder.build_file_reference_prompt(
            &PathBuf::from("src/handler.py"),
            Some("sql_injection"),
            Some("execute(query)"),
            None,
        );

        assert!(prompt.contains("src/handler.py"));
        assert!(prompt.contains("sql_injection"));
    }

    #[test]
    fn test_builder_with_sarif() {
        let builder = PromptBuilder::new();

        let prompt = builder.build_file_reference_prompt(
            &PathBuf::from("src/handler.py"),
            Some("xss"),
            Some("innerHTML = data"),
            Some(&PathBuf::from("/tmp/out.sarif")),
        );

        assert!(prompt.contains("SARIF"));
        assert!(prompt.contains("/tmp/out.sarif"));
    }
}
