//! Minimal analysis prompt: origin + scope + output.

use crate::traits::Prompt;
use crate::types::{Origin, OutputConfig, Scope};

/// Analysis prompt with only origin, scope, and output config.
///
/// # Example
///
/// ```rust
/// use parsentry_prompt::{AnalysisPrompt, Origin, Scope, OutputConfig, Prompt};
/// use std::path::PathBuf;
///
/// let prompt = AnalysisPrompt {
///     origin: Origin::pattern("src/db.py", "sql_injection", "query = f\"SELECT * FROM users WHERE id = {user_id}\""),
///     scope: Scope::SecurityAudit,
///     output: OutputConfig::Sarif(PathBuf::from("/tmp/output.sarif")),
/// };
///
/// let rendered = prompt.render();
/// assert!(rendered.contains("src/db.py"));
/// assert!(rendered.contains("sql_injection"));
/// ```
#[derive(Debug, Clone)]
pub struct AnalysisPrompt {
    /// Where to start the analysis.
    pub origin: Origin,
    /// What to look for.
    pub scope: Scope,
    /// How to output results.
    pub output: OutputConfig,
}

impl Default for AnalysisPrompt {
    fn default() -> Self {
        Self {
            origin: Origin::File(std::path::PathBuf::new()),
            scope: Scope::default(),
            output: OutputConfig::default(),
        }
    }
}

impl Prompt for AnalysisPrompt {
    fn render(&self) -> String {
        let mut parts = Vec::new();

        // Origin
        match &self.origin {
            Origin::File(path) => {
                parts.push(format!(
                    "Analyze {} for security vulnerabilities.",
                    path.display()
                ));
            }
            Origin::Pattern {
                file,
                pattern_type,
                matched_code,
            } => {
                parts.push(format!(
                    "Analyze {} for security vulnerabilities.\n{} pattern detected: `{}`",
                    file.display(),
                    pattern_type,
                    matched_code
                ));
            }
        }

        // Scope
        match &self.scope {
            Scope::SecurityAudit => {}
        }

        // Output
        match &self.output {
            OutputConfig::Sarif(path) => {
                parts.push(format!("Write SARIF v2.1.0 results to {}.", path.display()));
            }
            OutputConfig::Text => {}
        }

        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_minimal_file_prompt() {
        let prompt = AnalysisPrompt {
            origin: Origin::file("src/main.rs"),
            scope: Scope::SecurityAudit,
            output: OutputConfig::Text,
        };

        let rendered = prompt.render();
        assert_eq!(
            rendered,
            "Analyze src/main.rs for security vulnerabilities."
        );
    }

    #[test]
    fn test_minimal_pattern_prompt() {
        let prompt = AnalysisPrompt {
            origin: Origin::pattern("src/db.py", "sql_injection", "query = f\"SELECT...\""),
            scope: Scope::SecurityAudit,
            output: OutputConfig::Text,
        };

        let rendered = prompt.render();
        assert!(rendered.contains("src/db.py"));
        assert!(rendered.contains("sql_injection"));
        assert!(rendered.contains("SELECT"));
    }

    #[test]
    fn test_minimal_sarif_prompt() {
        let prompt = AnalysisPrompt {
            origin: Origin::file("src/handler.py"),
            scope: Scope::SecurityAudit,
            output: OutputConfig::Sarif(PathBuf::from("/tmp/output.sarif")),
        };

        let rendered = prompt.render();
        assert!(rendered.contains("SARIF"));
        assert!(rendered.contains("/tmp/output.sarif"));
    }

    #[test]
    fn test_prompt_is_minimal() {
        let prompt = AnalysisPrompt {
            origin: Origin::pattern("src/db.py", "sql_injection", "execute(query)"),
            scope: Scope::SecurityAudit,
            output: OutputConfig::Sarif(PathBuf::from("/tmp/out.sarif")),
        };

        let rendered = prompt.render();

        // Must NOT contain verbose instructions
        assert!(!rendered.contains("PAR"));
        assert!(!rendered.contains("Principal"));
        assert!(!rendered.contains("confidence_score"));
        assert!(!rendered.contains("Instructions"));

        // Should be short
        assert!(
            rendered.len() < 300,
            "Prompt too long: {} chars",
            rendered.len()
        );
    }
}
