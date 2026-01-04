//! Security analysis prompt.

use parsentry_i18n::Language;

use crate::templates;
use crate::traits::Prompt;
use crate::types::{sanitize_for_prompt, OutputFormat, PatternContext, PocMode, TargetFile};

/// Declarative security analysis prompt.
///
/// # Example
///
/// ```rust
/// use parsentry_prompt::{Prompt, SecurityAnalysisPrompt, TargetFile, PocMode, OutputFormat};
/// use parsentry_i18n::Language;
///
/// let prompt = SecurityAnalysisPrompt {
///     target: TargetFile::new("src/main.rs", "fn main() {}"),
///     pattern: None,
///     language: Language::Japanese,
///     poc_mode: PocMode::GenerateOnly,
///     deep_context: true,
///     output_format: OutputFormat::Json,
/// };
///
/// let rendered = prompt.render();
/// ```
#[derive(Debug, Clone)]
pub struct SecurityAnalysisPrompt {
    /// Target file to analyze.
    pub target: TargetFile,
    /// Optional pattern context for focused analysis.
    pub pattern: Option<PatternContext>,
    /// Language for the response.
    pub language: Language,
    /// PoC generation mode.
    pub poc_mode: PocMode,
    /// Enable deep context analysis (file operations).
    pub deep_context: bool,
    /// Output format for the analysis result.
    pub output_format: OutputFormat,
}

impl Default for SecurityAnalysisPrompt {
    fn default() -> Self {
        Self {
            target: TargetFile::new("", ""),
            pattern: None,
            language: Language::Japanese,
            poc_mode: PocMode::default(),
            deep_context: true,
            output_format: OutputFormat::default(),
        }
    }
}

impl Prompt for SecurityAnalysisPrompt {
    fn render(&self) -> String {
        let safe_path = sanitize_for_prompt(&self.target.path.display().to_string());
        let safe_content = sanitize_for_prompt(&self.target.content);

        let pattern_section = self
            .pattern
            .as_ref()
            .map(templates::render_pattern)
            .unwrap_or_default();

        let deep_context_section = if self.deep_context {
            templates::render_deep_context()
        } else {
            ""
        };

        let poc_section = templates::render_poc_instructions(self.poc_mode);
        let output_section = templates::render_output_format(&self.output_format, &self.language);
        let notes_section = templates::render_important_notes();

        format!(
            r#"{role}

{target}
{pattern}
{instructions}
{deep_context}
{poc}

{output}

{notes}"#,
            role = templates::render_role(),
            target = templates::render_target(&safe_path, &safe_content),
            pattern = pattern_section,
            instructions = templates::render_par_instructions(),
            deep_context = deep_context_section,
            poc = poc_section,
            output = output_section,
            notes = notes_section,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_security_analysis_prompt_render() {
        let prompt = SecurityAnalysisPrompt {
            target: TargetFile::new("src/main.rs", "fn main() { println!(\"Hello\"); }"),
            pattern: None,
            language: Language::Japanese,
            poc_mode: PocMode::GenerateOnly,
            deep_context: true,
            output_format: OutputFormat::Json,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("src/main.rs"));
        assert!(rendered.contains("PAR"));
        assert!(rendered.contains("confidence_score"));
        assert!(rendered.contains("日本語"));
    }

    #[test]
    fn test_security_analysis_prompt_with_pattern() {
        let prompt = SecurityAnalysisPrompt {
            target: TargetFile::new(
                "src/db.py",
                "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
            ),
            pattern: Some(PatternContext::sql_injection(
                "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
            )),
            language: Language::Japanese,
            poc_mode: PocMode::GenerateOnly,
            deep_context: true,
            output_format: OutputFormat::Json,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("sql_injection"));
        assert!(rendered.contains("UNION-based injection"));
    }

    #[test]
    fn test_security_analysis_prompt_with_sarif() {
        let prompt = SecurityAnalysisPrompt {
            target: TargetFile::new("test.py", "os.system(user_input)"),
            pattern: None,
            language: Language::English,
            poc_mode: PocMode::ExecuteIfSafe,
            deep_context: true,
            output_format: OutputFormat::Sarif {
                output_path: PathBuf::from("/tmp/output.sarif"),
            },
        };

        let rendered = prompt.render();

        assert!(rendered.contains("SARIF"));
        assert!(rendered.contains("/tmp/output.sarif"));
        assert!(rendered.contains("Execute the PoC"));
    }
}
