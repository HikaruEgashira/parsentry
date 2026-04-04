//! Core types for prompt construction.

use std::path::{Path, PathBuf};

/// Origin: where to start the analysis.
#[derive(Debug, Clone)]
pub enum Origin {
    /// Analyze a specific file.
    File(PathBuf),
    /// Analyze a file with a detected pattern as the starting point.
    Pattern {
        file: PathBuf,
        pattern_type: String,
        matched_code: String,
    },
}

impl Origin {
    /// Create a file-only origin.
    pub fn file(path: impl AsRef<Path>) -> Self {
        Self::File(path.as_ref().to_path_buf())
    }

    /// Create a pattern-based origin.
    pub fn pattern(
        file: impl AsRef<Path>,
        pattern_type: impl Into<String>,
        matched_code: impl Into<String>,
    ) -> Self {
        Self::Pattern {
            file: file.as_ref().to_path_buf(),
            pattern_type: pattern_type.into(),
            matched_code: matched_code.into(),
        }
    }

    /// Get the file path from the origin.
    pub fn file_path(&self) -> &Path {
        match self {
            Self::File(path) => path,
            Self::Pattern { file, .. } => file,
        }
    }
}

/// Scope: what to look for.
#[derive(Debug, Clone, Default)]
pub enum Scope {
    /// General security audit.
    #[default]
    SecurityAudit,
}

/// Output configuration.
#[derive(Debug, Clone)]
pub enum OutputConfig {
    /// SARIF format written to the specified path.
    Sarif(PathBuf),
    /// Free-form text/JSON response.
    Text,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self::Text
    }
}
