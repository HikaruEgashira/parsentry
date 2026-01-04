//! Vulnerability verification prompt.

use std::path::{Path, PathBuf};

use crate::traits::Prompt;

/// Declarative vulnerability verification prompt.
///
/// Used to verify and confirm potential vulnerabilities identified
/// in the initial analysis.
#[derive(Debug, Clone)]
pub struct VerificationPrompt {
    /// Type of vulnerability to verify.
    pub vuln_type: String,
    /// Path to the file containing the vulnerability.
    pub file_path: PathBuf,
    /// Initial analysis result to verify.
    pub initial_analysis: String,
}

impl VerificationPrompt {
    /// Create a new verification prompt.
    pub fn new(
        vuln_type: impl Into<String>,
        file_path: impl AsRef<Path>,
        initial_analysis: impl Into<String>,
    ) -> Self {
        Self {
            vuln_type: vuln_type.into(),
            file_path: file_path.as_ref().to_path_buf(),
            initial_analysis: initial_analysis.into(),
        }
    }
}

impl Prompt for VerificationPrompt {
    fn render(&self) -> String {
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
            vuln_type = self.vuln_type,
            file_path = self.file_path.display(),
            analysis = self.initial_analysis,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_prompt_render() {
        let prompt = VerificationPrompt::new(
            "SQLI",
            "src/db.py",
            "Potential SQL injection in query construction",
        );

        let rendered = prompt.render();

        assert!(rendered.contains("SQLI"));
        assert!(rendered.contains("src/db.py"));
        assert!(rendered.contains("Verification Instructions"));
    }
}
