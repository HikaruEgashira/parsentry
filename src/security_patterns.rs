use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    // IaC Languages
    Terraform,
    CloudFormation,
    Kubernetes,
    Other,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "py" => Language::Python,
            "js" => Language::JavaScript,
            "rs" => Language::Rust,
            "ts" => Language::TypeScript,
            "java" => Language::Java,
            "go" => Language::Go,
            "rb" => Language::Ruby,
            "c" | "h" => Language::C,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Language::Cpp,
            // IaC extensions
            "tf" | "hcl" => Language::Terraform,
            "json" => Language::CloudFormation, // Context-dependent
            "yaml" | "yml" => Language::Kubernetes, // Context-dependent
            _ => Language::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    // PAR Model - Unified for Programming & IaC
    Principal,  // Who: user input (Programming) | AWS account/role (IaC)
    Action,     // What: operations/methods (Programming) | API actions (IaC)  
    Resource,   // Where: files/databases (Programming) | AWS resources (IaC)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatternConfig {
    pub pattern: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguagePatterns {
    // PAR Model patterns
    pub principals: Option<Vec<PatternConfig>>,  // Who (sources of authority/input)
    pub actions: Option<Vec<PatternConfig>>,     // What (operations/permissions)
    pub resources: Option<Vec<PatternConfig>>,   // Where (targets/sinks)
}

pub struct SecurityRiskPatterns {
    // PAR Model patterns  
    principal_patterns: Vec<Regex>,   // Who patterns
    action_patterns: Vec<Regex>,      // What patterns  
    resource_patterns: Vec<Regex>,    // Where patterns
    pattern_type_map: HashMap<String, PatternType>,
}

impl SecurityRiskPatterns {
    pub fn new(language: Language) -> Self {
        let pattern_map = Self::load_patterns();
        let lang_patterns = pattern_map
            .get(&language)
            .or_else(|| pattern_map.get(&Language::Other))
            .unwrap_or(&LanguagePatterns {
                principals: None,
                actions: None,
                resources: None,
            });

        let mut principal_patterns = Vec::new();
        let mut action_patterns = Vec::new();
        let mut resource_patterns = Vec::new();
        let mut pattern_type_map = HashMap::new();

        if let Some(principals) = &lang_patterns.principals {
            for config in principals {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Principal);
                principal_patterns.push(regex);
            }
        }

        if let Some(actions) = &lang_patterns.actions {
            for config in actions {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Action);
                action_patterns.push(regex);
            }
        }

        if let Some(resources) = &lang_patterns.resources {
            for config in resources {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Resource);
                resource_patterns.push(regex);
            }
        }

        Self {
            principal_patterns,
            action_patterns,
            resource_patterns,
            pattern_type_map,
        }
    }

    pub fn matches(&self, content: &str) -> bool {
        self.principal_patterns
            .iter()
            .any(|pattern| pattern.is_match(content))
            || self
                .action_patterns
                .iter()
                .any(|pattern| pattern.is_match(content))
            || self
                .resource_patterns
                .iter()
                .any(|pattern| pattern.is_match(content))
    }

    /// パターンの種類を取得する。
    pub fn get_pattern_type(&self, content: &str) -> Option<PatternType> {
        for (pattern_str, pattern_type) in &self.pattern_type_map {
            let regex = Regex::new(pattern_str).ok()?;
            if regex.is_match(content) {
                return Some(pattern_type.clone());
            }
        }
        None
    }

    /// 言語ごとのパターン定義を読み込む
    fn load_patterns() -> HashMap<Language, LanguagePatterns> {
        use Language::*;

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let yaml_path = manifest_dir.join("patterns.yml");
        let content = fs::read_to_string(&yaml_path)
            .unwrap_or_else(|_| panic!("failed to read {}", yaml_path.display()));
        let raw_map: HashMap<String, LanguagePatterns> =
            serde_yaml::from_str(&content).expect("failed to parse patterns.yml");

        let mut map = HashMap::new();
        for (lang, patterns) in raw_map {
            let key = match lang.as_str() {
                "Python" => Python,
                "JavaScript" => JavaScript,
                "Rust" => Rust,
                "TypeScript" => TypeScript,
                "Java" => Java,
                "Go" => Go,
                "Ruby" => Ruby,
                "C" => C,
                "Cpp" => Cpp,
                // IaC Languages
                "Terraform" => Terraform,
                "CloudFormation" => CloudFormation,
                "Kubernetes" => Kubernetes,
                "Other" => Other,
                _ => continue,
            };
            map.insert(key, patterns);
        }

        map
    }
}
