//! Core types for prompt construction.

use std::path::{Path, PathBuf};

/// Target file for security analysis.
#[derive(Debug, Clone)]
pub struct TargetFile {
    /// Path to the file.
    pub path: PathBuf,
    /// Content of the file.
    pub content: String,
}

impl TargetFile {
    /// Create a new target file.
    pub fn new(path: impl AsRef<Path>, content: impl Into<String>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            content: content.into(),
        }
    }
}

/// Context information about a security pattern match.
#[derive(Debug, Clone)]
pub struct PatternContext {
    /// Type of pattern (e.g., "sql_injection", "command_injection").
    pub pattern_type: String,
    /// Description of the pattern.
    pub description: String,
    /// The matched source code.
    pub matched_code: String,
    /// Known attack vectors for this pattern.
    pub attack_vectors: Vec<String>,
}

impl PatternContext {
    /// Create a new pattern context.
    pub fn new(
        pattern_type: impl Into<String>,
        description: impl Into<String>,
        matched_code: impl Into<String>,
    ) -> Self {
        Self {
            pattern_type: pattern_type.into(),
            description: description.into(),
            matched_code: matched_code.into(),
            attack_vectors: Vec::new(),
        }
    }

    /// Create a SQL injection pattern context.
    pub fn sql_injection(matched_code: impl Into<String>) -> Self {
        Self::new(
            "sql_injection",
            "Direct SQL query construction with user input",
            matched_code,
        )
        .with_attack_vectors(vec![
            "UNION-based injection".to_string(),
            "Boolean-based blind injection".to_string(),
            "Time-based blind injection".to_string(),
        ])
    }

    /// Create a command injection pattern context.
    pub fn command_injection(matched_code: impl Into<String>) -> Self {
        Self::new(
            "command_injection",
            "Command execution with user-controlled input",
            matched_code,
        )
        .with_attack_vectors(vec![
            "Shell metacharacters".to_string(),
            "Command chaining".to_string(),
            "Argument injection".to_string(),
        ])
    }

    /// Create a path traversal pattern context.
    pub fn path_traversal(matched_code: impl Into<String>) -> Self {
        Self::new(
            "path_traversal",
            "File path construction with user input",
            matched_code,
        )
        .with_attack_vectors(vec![
            "Path traversal sequences (../)".to_string(),
            "URL encoding".to_string(),
            "Null byte injection".to_string(),
        ])
    }

    /// Add attack vectors.
    pub fn with_attack_vectors(mut self, vectors: Vec<String>) -> Self {
        self.attack_vectors = vectors;
        self
    }
}

/// PoC (Proof of Concept) generation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PocMode {
    /// Do not generate PoC.
    Disabled,
    /// Generate PoC code but do not execute.
    #[default]
    GenerateOnly,
    /// Execute PoC if it can be done safely.
    ExecuteIfSafe,
}

/// Output format for the analysis result.
#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// JSON format with PAR analysis structure.
    Json,
    /// SARIF format written to the specified path.
    Sarif { output_path: PathBuf },
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// Related function information for file reference prompts.
#[derive(Debug, Clone)]
pub struct RelatedFunction {
    /// Function name.
    pub name: String,
    /// File path containing the function.
    pub path: String,
    /// Line number where the function is defined.
    pub line: usize,
}

impl RelatedFunction {
    /// Create a new related function.
    pub fn new(name: impl Into<String>, path: impl Into<String>, line: usize) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            line,
        }
    }
}

/// IaC (Infrastructure as Code) type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IacType {
    /// Terraform configuration.
    Terraform,
    /// CloudFormation template.
    CloudFormation,
    /// Kubernetes manifest.
    Kubernetes,
    /// Docker configuration.
    Docker,
}

/// Compliance framework for IaC analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceFramework {
    /// CIS Benchmark.
    CIS,
    /// NIST Cybersecurity Framework.
    NIST,
    /// SOC 2 Type II.
    SOC2,
    /// PCI DSS.
    PCIDSS,
    /// HIPAA.
    HIPAA,
}

/// Focus area for IaC analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IacFocusArea {
    /// Network security configuration.
    NetworkSecurity,
    /// Access control and IAM.
    AccessControl,
    /// Data protection and encryption.
    DataProtection,
    /// Monitoring and logging.
    MonitoringLogging,
    /// Resource management.
    ResourceManagement,
}

/// Sanitize input to prevent prompt injection attacks.
pub fn sanitize_for_prompt(text: &str) -> String {
    text.replace("```", "\\`\\`\\`")
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect()
}
