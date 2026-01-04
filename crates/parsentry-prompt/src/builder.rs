//! Backward-compatible PromptBuilder wrapper.
//!
//! This module provides a builder-pattern API for backward compatibility
//! with existing code that uses PromptBuilder.

use std::path::Path;

use parsentry_i18n::Language;

use crate::types::{PatternContext, PocMode, RelatedFunction};
use crate::FileReferencePrompt;
use crate::OutputFormat;
use crate::Prompt;
use crate::SecurityAnalysisPrompt;
use crate::TargetFile;

/// Builder for constructing security analysis prompts.
///
/// This is a backward-compatible wrapper that internally uses
/// the new declarative prompt types.
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    poc_mode: PocMode,
    deep_context: bool,
    language: Language,
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self {
            poc_mode: PocMode::GenerateOnly,
            deep_context: true,
            language: Language::Japanese,
        }
    }
}

impl PromptBuilder {
    /// Create a new prompt builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable PoC execution.
    pub fn with_poc(mut self, enable: bool) -> Self {
        self.poc_mode = if enable {
            PocMode::ExecuteIfSafe
        } else {
            PocMode::GenerateOnly
        };
        self
    }

    /// Enable file operations.
    pub fn with_file_ops(mut self, enable: bool) -> Self {
        self.deep_context = enable;
        self
    }

    /// Set the response language.
    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = Language::from_string(lang);
        self
    }

    /// Build a security analysis prompt for a file.
    pub fn build_security_analysis_prompt(
        &self,
        file_path: &Path,
        content: &str,
        pattern_context: Option<&PatternContext>,
    ) -> String {
        let prompt = SecurityAnalysisPrompt {
            target: TargetFile::new(file_path, content),
            pattern: pattern_context.cloned(),
            language: self.language,
            poc_mode: self.poc_mode,
            deep_context: self.deep_context,
            output_format: OutputFormat::Json,
        };
        prompt.render()
    }

    /// Build a prompt for vulnerability verification.
    pub fn build_verification_prompt(
        &self,
        vuln_type: &str,
        file_path: &Path,
        analysis: &str,
    ) -> String {
        format!(
            r#"## Vulnerability Verification Task

Verify the following potential vulnerability:

- **Type**: {vuln_type}
- **Location**: {file_path}
- **Initial Analysis**: {analysis}

## Verification Instructions

1. Read the source code at the specified location
2. Trace the data flow from source to sink
3. Identify any security controls in the path
4. Determine if the vulnerability is exploitable
5. Create a minimal PoC if confirmed

## Output Format

Respond with the same JSON format as the initial analysis, but with:
- Updated confidence_score based on verification
- More detailed par_analysis
- Verified or refined poc code"#,
            vuln_type = vuln_type,
            file_path = file_path.display(),
            analysis = analysis,
        )
    }

    /// Build a prompt for direct SARIF output mode.
    pub fn build_sarif_output_prompt(
        &self,
        file_path: &Path,
        content: &str,
        sarif_output_path: &Path,
        pattern_context: Option<&PatternContext>,
    ) -> String {
        let prompt = SecurityAnalysisPrompt {
            target: TargetFile::new(file_path, content),
            pattern: pattern_context.cloned(),
            language: self.language,
            poc_mode: self.poc_mode,
            deep_context: self.deep_context,
            output_format: OutputFormat::Sarif {
                output_path: sarif_output_path.to_path_buf(),
            },
        };
        prompt.render()
    }

    /// Build a prompt using file references.
    pub fn build_file_reference_prompt(
        &self,
        file_path: &Path,
        pattern_context: Option<&PatternContext>,
        related_functions: Option<&[(&str, &str, usize)]>,
        sarif_output_path: Option<&Path>,
    ) -> String {
        let related = related_functions
            .map(|funcs| {
                funcs
                    .iter()
                    .map(|(name, path, line)| RelatedFunction::new(*name, *path, *line))
                    .collect()
            })
            .unwrap_or_default();

        let output_format = sarif_output_path
            .map(|p| OutputFormat::Sarif {
                output_path: p.to_path_buf(),
            })
            .unwrap_or(OutputFormat::Json);

        let prompt = FileReferencePrompt {
            file_path: file_path.to_path_buf(),
            pattern: pattern_context.cloned(),
            related_functions: related,
            output_format,
            language: self.language,
        };
        prompt.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_prompt_builder_backward_compat() {
        let builder = PromptBuilder::new()
            .with_poc(false)
            .with_language("ja");

        let prompt = builder.build_security_analysis_prompt(
            &PathBuf::from("src/main.rs"),
            "fn main() {}",
            None,
        );

        assert!(prompt.contains("src/main.rs"));
        assert!(prompt.contains("PAR"));
    }

    #[test]
    fn test_file_reference_prompt() {
        let builder = PromptBuilder::new();

        let prompt = builder.build_file_reference_prompt(
            &PathBuf::from("src/handler.py"),
            None,
            Some(&[("validate", "src/utils.py", 10)]),
            None,
        );

        assert!(prompt.contains("src/handler.py"));
        assert!(prompt.contains("validate"));
    }
}
