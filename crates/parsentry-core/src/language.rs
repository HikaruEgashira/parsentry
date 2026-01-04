//! Programming language definitions.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Supported programming languages for analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    Rust,
    TypeScript,
    Java,
    Go,
    Ruby,
    C,
    Cpp,
    Terraform,
    CloudFormation,
    Kubernetes,
    Yaml,
    Bash,
    Shell,
    Php,
    Other,
}

impl Language {
    /// Create a Language from a file extension.
    #[must_use]
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "py" => Language::Python,
            "js" => Language::JavaScript,
            "rs" => Language::Rust,
            "ts" | "tsx" => Language::TypeScript,
            "java" => Language::Java,
            "go" => Language::Go,
            "rb" => Language::Ruby,
            "c" | "h" => Language::C,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Language::Cpp,
            "tf" | "hcl" => Language::Terraform,
            "yml" | "yaml" => Language::Yaml,
            "sh" | "bash" => Language::Bash,
            "php" | "php3" | "php4" | "php5" | "phtml" => Language::Php,
            _ => Language::Other,
        }
    }

    /// Create a Language from a filename.
    #[must_use]
    pub fn from_filename(filename: &str) -> Self {
        if let Some(ext) = std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
        {
            Self::from_extension(ext)
        } else {
            Language::Other
        }
    }

    /// Check if this language is an Infrastructure as Code language.
    #[must_use]
    pub fn is_iac(&self) -> bool {
        matches!(
            self,
            Language::Terraform | Language::CloudFormation | Language::Kubernetes | Language::Yaml
        )
    }

    /// Get the display name for this language.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Go => "Go",
            Language::Ruby => "Ruby",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Terraform => "Terraform",
            Language::CloudFormation => "CloudFormation",
            Language::Kubernetes => "Kubernetes",
            Language::Yaml => "YAML",
            Language::Bash => "Bash",
            Language::Shell => "Shell",
            Language::Php => "PHP",
            Language::Other => "Other",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "python" | "py" => Ok(Language::Python),
            "javascript" | "js" => Ok(Language::JavaScript),
            "rust" | "rs" => Ok(Language::Rust),
            "typescript" | "ts" | "tsx" => Ok(Language::TypeScript),
            "java" => Ok(Language::Java),
            "go" => Ok(Language::Go),
            "ruby" | "rb" => Ok(Language::Ruby),
            "c" => Ok(Language::C),
            "cpp" | "c++" | "cxx" => Ok(Language::Cpp),
            "terraform" | "tf" => Ok(Language::Terraform),
            "cloudformation" | "cfn" => Ok(Language::CloudFormation),
            "kubernetes" | "k8s" => Ok(Language::Kubernetes),
            "yaml" | "yml" => Ok(Language::Yaml),
            "bash" => Ok(Language::Bash),
            "shell" | "sh" => Ok(Language::Shell),
            "php" => Ok(Language::Php),
            "other" => Ok(Language::Other),
            _ => Err(format!(
                "Unknown language: '{}'. Supported languages: python, javascript, rust, typescript, java, go, ruby, c, cpp, terraform, cloudformation, kubernetes, yaml, bash, shell, php",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_extension() {
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("unknown"), Language::Other);
    }

    #[test]
    fn test_from_filename() {
        assert_eq!(Language::from_filename("test.py"), Language::Python);
        assert_eq!(Language::from_filename("app.tsx"), Language::TypeScript);
        assert_eq!(Language::from_filename("noext"), Language::Other);
    }

    #[test]
    fn test_is_iac() {
        assert!(Language::Terraform.is_iac());
        assert!(!Language::Python.is_iac());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Language::Python), "Python");
        assert_eq!(format!("{}", Language::Cpp), "C++");
    }

    #[test]
    fn test_from_str() {
        // Basic language names
        assert_eq!(Language::from_str("python").unwrap(), Language::Python);
        assert_eq!(Language::from_str("rust").unwrap(), Language::Rust);
        assert_eq!(Language::from_str("javascript").unwrap(), Language::JavaScript);

        // Case insensitivity
        assert_eq!(Language::from_str("PYTHON").unwrap(), Language::Python);
        assert_eq!(Language::from_str("Rust").unwrap(), Language::Rust);
        assert_eq!(Language::from_str("JavaScript").unwrap(), Language::JavaScript);

        // Aliases
        assert_eq!(Language::from_str("py").unwrap(), Language::Python);
        assert_eq!(Language::from_str("js").unwrap(), Language::JavaScript);
        assert_eq!(Language::from_str("ts").unwrap(), Language::TypeScript);
        assert_eq!(Language::from_str("rs").unwrap(), Language::Rust);
        assert_eq!(Language::from_str("c++").unwrap(), Language::Cpp);
        assert_eq!(Language::from_str("tf").unwrap(), Language::Terraform);
        assert_eq!(Language::from_str("k8s").unwrap(), Language::Kubernetes);

        // Error case
        assert!(Language::from_str("unknown_lang").is_err());
        let err = Language::from_str("invalid").unwrap_err();
        assert!(err.contains("Unknown language"));
        assert!(err.contains("Supported languages"));
    }
}
