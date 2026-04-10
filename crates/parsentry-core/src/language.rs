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
    Html,
    Css,
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
            "html" | "htm" => Language::Html,
            "css" => Language::Css,
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
            Language::Html => "HTML",
            Language::Css => "CSS",
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
            "html" | "htm" => Ok(Language::Html),
            "css" => Ok(Language::Css),
            "other" => Ok(Language::Other),
            _ => Err(format!(
                "Unknown language: '{}'. Supported languages: python, javascript, rust, typescript, java, go, ruby, c, cpp, terraform, cloudformation, kubernetes, yaml, bash, shell, php, html, css",
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
        assert_eq!(Language::from_str("python").unwrap(), Language::Python);
        assert_eq!(Language::from_str("rust").unwrap(), Language::Rust);
        assert_eq!(
            Language::from_str("javascript").unwrap(),
            Language::JavaScript
        );

        assert_eq!(Language::from_str("PYTHON").unwrap(), Language::Python);
        assert_eq!(Language::from_str("Rust").unwrap(), Language::Rust);
        assert_eq!(
            Language::from_str("JavaScript").unwrap(),
            Language::JavaScript
        );

        assert_eq!(Language::from_str("py").unwrap(), Language::Python);
        assert_eq!(Language::from_str("js").unwrap(), Language::JavaScript);
        assert_eq!(Language::from_str("ts").unwrap(), Language::TypeScript);
        assert_eq!(Language::from_str("rs").unwrap(), Language::Rust);
        assert_eq!(Language::from_str("c++").unwrap(), Language::Cpp);
        assert_eq!(Language::from_str("tf").unwrap(), Language::Terraform);
        assert_eq!(Language::from_str("k8s").unwrap(), Language::Kubernetes);

        assert!(Language::from_str("unknown_lang").is_err());
        let err = Language::from_str("invalid").unwrap_err();
        assert!(err.contains("Unknown language"));
        assert!(err.contains("Supported languages"));
    }

    // --- Mutant-killing: every FromStr arm ---

    #[test]
    fn test_from_str_all_arms() {
        // Ensure every match arm is hit and returns the correct variant
        assert_eq!(Language::from_str("java").unwrap(), Language::Java);
        assert_eq!(Language::from_str("go").unwrap(), Language::Go);
        assert_eq!(Language::from_str("ruby").unwrap(), Language::Ruby);
        assert_eq!(Language::from_str("rb").unwrap(), Language::Ruby);
        assert_eq!(Language::from_str("c").unwrap(), Language::C);
        assert_eq!(Language::from_str("cpp").unwrap(), Language::Cpp);
        assert_eq!(Language::from_str("cxx").unwrap(), Language::Cpp);
        assert_eq!(
            Language::from_str("terraform").unwrap(),
            Language::Terraform
        );
        assert_eq!(
            Language::from_str("cloudformation").unwrap(),
            Language::CloudFormation
        );
        assert_eq!(Language::from_str("cfn").unwrap(), Language::CloudFormation);
        assert_eq!(
            Language::from_str("kubernetes").unwrap(),
            Language::Kubernetes
        );
        assert_eq!(Language::from_str("yaml").unwrap(), Language::Yaml);
        assert_eq!(Language::from_str("yml").unwrap(), Language::Yaml);
        assert_eq!(Language::from_str("bash").unwrap(), Language::Bash);
        assert_eq!(Language::from_str("shell").unwrap(), Language::Shell);
        assert_eq!(Language::from_str("sh").unwrap(), Language::Shell);
        assert_eq!(Language::from_str("php").unwrap(), Language::Php);
        assert_eq!(Language::from_str("html").unwrap(), Language::Html);
        assert_eq!(Language::from_str("htm").unwrap(), Language::Html);
        assert_eq!(Language::from_str("css").unwrap(), Language::Css);
        assert_eq!(Language::from_str("other").unwrap(), Language::Other);
        assert_eq!(Language::from_str("tsx").unwrap(), Language::TypeScript);
        assert_eq!(
            Language::from_str("typescript").unwrap(),
            Language::TypeScript
        );
    }

    // --- Mutant-killing: every from_extension arm ---

    #[test]
    fn test_from_extension_all_arms() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("ts"), Language::TypeScript);
        assert_eq!(Language::from_extension("tsx"), Language::TypeScript);
        assert_eq!(Language::from_extension("java"), Language::Java);
        assert_eq!(Language::from_extension("go"), Language::Go);
        assert_eq!(Language::from_extension("rb"), Language::Ruby);
        assert_eq!(Language::from_extension("c"), Language::C);
        assert_eq!(Language::from_extension("h"), Language::C);
        assert_eq!(Language::from_extension("cpp"), Language::Cpp);
        assert_eq!(Language::from_extension("cxx"), Language::Cpp);
        assert_eq!(Language::from_extension("cc"), Language::Cpp);
        assert_eq!(Language::from_extension("hpp"), Language::Cpp);
        assert_eq!(Language::from_extension("hxx"), Language::Cpp);
        assert_eq!(Language::from_extension("tf"), Language::Terraform);
        assert_eq!(Language::from_extension("hcl"), Language::Terraform);
        assert_eq!(Language::from_extension("yml"), Language::Yaml);
        assert_eq!(Language::from_extension("yaml"), Language::Yaml);
        assert_eq!(Language::from_extension("sh"), Language::Bash);
        assert_eq!(Language::from_extension("bash"), Language::Bash);
        assert_eq!(Language::from_extension("php"), Language::Php);
        assert_eq!(Language::from_extension("php3"), Language::Php);
        assert_eq!(Language::from_extension("php4"), Language::Php);
        assert_eq!(Language::from_extension("php5"), Language::Php);
        assert_eq!(Language::from_extension("phtml"), Language::Php);
        assert_eq!(Language::from_extension("html"), Language::Html);
        assert_eq!(Language::from_extension("htm"), Language::Html);
        assert_eq!(Language::from_extension("css"), Language::Css);
    }

    // --- Mutant-killing: is_iac all variants ---

    #[test]
    fn test_is_iac_all_true_variants() {
        assert!(Language::Terraform.is_iac());
        assert!(Language::CloudFormation.is_iac());
        assert!(Language::Kubernetes.is_iac());
        assert!(Language::Yaml.is_iac());
    }

    #[test]
    fn test_is_iac_false_variants() {
        assert!(!Language::Python.is_iac());
        assert!(!Language::Rust.is_iac());
        assert!(!Language::JavaScript.is_iac());
        assert!(!Language::Php.is_iac());
        assert!(!Language::Other.is_iac());
    }

    // --- Mutant-killing: display_name all variants ---

    #[test]
    fn test_display_name_all_variants() {
        assert_eq!(Language::Python.display_name(), "Python");
        assert_eq!(Language::JavaScript.display_name(), "JavaScript");
        assert_eq!(Language::Rust.display_name(), "Rust");
        assert_eq!(Language::TypeScript.display_name(), "TypeScript");
        assert_eq!(Language::Java.display_name(), "Java");
        assert_eq!(Language::Go.display_name(), "Go");
        assert_eq!(Language::Ruby.display_name(), "Ruby");
        assert_eq!(Language::C.display_name(), "C");
        assert_eq!(Language::Cpp.display_name(), "C++");
        assert_eq!(Language::Terraform.display_name(), "Terraform");
        assert_eq!(Language::CloudFormation.display_name(), "CloudFormation");
        assert_eq!(Language::Kubernetes.display_name(), "Kubernetes");
        assert_eq!(Language::Yaml.display_name(), "YAML");
        assert_eq!(Language::Bash.display_name(), "Bash");
        assert_eq!(Language::Shell.display_name(), "Shell");
        assert_eq!(Language::Php.display_name(), "PHP");
        assert_eq!(Language::Html.display_name(), "HTML");
        assert_eq!(Language::Css.display_name(), "CSS");
        assert_eq!(Language::Other.display_name(), "Other");
    }
}
