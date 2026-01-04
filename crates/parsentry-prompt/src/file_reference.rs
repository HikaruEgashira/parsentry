//! File reference prompt for analyzing files with related context.

use std::path::PathBuf;

use parsentry_i18n::Language;

use crate::templates;
use crate::traits::Prompt;
use crate::types::{sanitize_for_prompt, OutputFormat, PatternContext, RelatedFunction};

/// Declarative file reference prompt.
///
/// Used for analyzing files with references to related functions
/// and contextual information from the codebase.
#[derive(Debug, Clone)]
pub struct FileReferencePrompt {
    /// Path to the target file.
    pub file_path: PathBuf,
    /// Optional pattern context for focused analysis.
    pub pattern: Option<PatternContext>,
    /// Related functions from the codebase.
    pub related_functions: Vec<RelatedFunction>,
    /// Output format for the analysis result.
    pub output_format: OutputFormat,
    /// Language for the response.
    pub language: Language,
}

impl Default for FileReferencePrompt {
    fn default() -> Self {
        Self {
            file_path: PathBuf::new(),
            pattern: None,
            related_functions: Vec::new(),
            output_format: OutputFormat::default(),
            language: Language::Japanese,
        }
    }
}

impl Prompt for FileReferencePrompt {
    fn render(&self) -> String {
        let safe_path = sanitize_for_prompt(&self.file_path.display().to_string());

        let pattern_section = self
            .pattern
            .as_ref()
            .map(templates::render_pattern)
            .unwrap_or_default();

        let related_section = templates::render_related_functions(&self.related_functions);
        let output_section = templates::render_output_format(&self.output_format, &self.language);
        let notes_section = templates::render_important_notes();

        format!(
            r#"You are a security vulnerability analyzer.

## Analysis Target
{file_path}
{pattern}{related}
{instructions}

{output}

{notes}"#,
            file_path = safe_path,
            pattern = pattern_section,
            related = related_section,
            instructions = templates::render_par_instructions(),
            output = output_section,
            notes = notes_section,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reference_prompt_render() {
        let prompt = FileReferencePrompt {
            file_path: PathBuf::from("src/handler.py"),
            pattern: None,
            related_functions: vec![
                RelatedFunction::new("validate_input", "src/utils.py", 42),
                RelatedFunction::new("execute_query", "src/db.py", 100),
            ],
            output_format: OutputFormat::Json,
            language: Language::Japanese,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("src/handler.py"));
        assert!(rendered.contains("src/utils.py:42 validate_input"));
        assert!(rendered.contains("src/db.py:100 execute_query"));
    }

    #[test]
    fn test_file_reference_prompt_with_pattern() {
        let prompt = FileReferencePrompt {
            file_path: PathBuf::from("src/api.py"),
            pattern: Some(PatternContext::command_injection("os.system(cmd)")),
            related_functions: vec![],
            output_format: OutputFormat::Json,
            language: Language::English,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("command_injection"));
        assert!(rendered.contains("Shell metacharacters"));
    }

    #[test]
    fn test_file_reference_prompt_with_sarif() {
        let prompt = FileReferencePrompt {
            file_path: PathBuf::from("src/main.rs"),
            pattern: None,
            related_functions: vec![],
            output_format: OutputFormat::Sarif {
                output_path: PathBuf::from("/tmp/result.sarif"),
            },
            language: Language::Japanese,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("SARIF"));
        assert!(rendered.contains("/tmp/result.sarif"));
    }
}
