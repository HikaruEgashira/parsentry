use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

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
            "tf" | "hcl" => Language::Terraform,
            _ => Language::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    Principal,  // Who: user input (Programming) | AWS account/role (IaC)
    Action,     // What: operations/methods (Programming) | API actions (IaC)
    Resource,   // Where: files/databases (Programming) | AWS resources (IaC)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatternConfig {
    pub pattern: String,
    pub description: String,
    pub attack_vector: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguagePatterns {
    pub principals: Option<Vec<PatternConfig>>,  // Who (sources of authority/input)
    pub actions: Option<Vec<PatternConfig>>,     // What (operations/permissions)
    pub resources: Option<Vec<PatternConfig>>,   // Where (targets/sinks)
}

pub struct SecurityRiskPatterns {
    principal_patterns: Vec<Regex>,   // Who patterns
    action_patterns: Vec<Regex>,      // What patterns
    resource_patterns: Vec<Regex>,    // Where patterns
    pattern_type_map: HashMap<String, PatternType>,
    attack_vector_map: HashMap<String, Vec<String>>,
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
        let mut attack_vector_map = HashMap::new();

        if let Some(principals) = &lang_patterns.principals {
            for config in principals {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Principal);
                if !config.attack_vector.is_empty() {
                    attack_vector_map.insert(config.pattern.clone(), config.attack_vector.clone());
                }
                principal_patterns.push(regex);
            }
        }

        if let Some(actions) = &lang_patterns.actions {
            for config in actions {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Action);
                if !config.attack_vector.is_empty() {
                    attack_vector_map.insert(config.pattern.clone(), config.attack_vector.clone());
                }
                action_patterns.push(regex);
            }
        }

        if let Some(resources) = &lang_patterns.resources {
            for config in resources {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Resource);
                if !config.attack_vector.is_empty() {
                    attack_vector_map.insert(config.pattern.clone(), config.attack_vector.clone());
                }
                resource_patterns.push(regex);
            }
        }

        Self {
            principal_patterns,
            action_patterns,
            resource_patterns,
            pattern_type_map,
            attack_vector_map,
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

    pub fn get_pattern_type(&self, content: &str) -> Option<PatternType> {
        for (pattern_str, pattern_type) in &self.pattern_type_map {
            let regex = Regex::new(pattern_str).ok()?;
            if regex.is_match(content) {
                return Some(pattern_type.clone());
            }
        }
        None
    }

    pub fn get_attack_vectors(&self, content: &str) -> Vec<String> {
        for (pattern_str, attack_vectors) in &self.attack_vector_map {
            if let Ok(regex) = Regex::new(pattern_str) {
                if regex.is_match(content) {
                    return attack_vectors.clone();
                }
            }
        }
        Vec::new()
    }

    fn load_patterns() -> HashMap<Language, LanguagePatterns> {
        use Language::*;

        let mut map = HashMap::new();

        // Load patterns from individual language files
        let languages = [
            (Python, include_str!("patterns/python.yml")),
            (JavaScript, include_str!("patterns/javascript.yml")),
            (Rust, include_str!("patterns/rust.yml")),
            (TypeScript, include_str!("patterns/typescript.yml")),
            (Java, include_str!("patterns/java.yml")),
            (Go, include_str!("patterns/go.yml")),
            (Ruby, include_str!("patterns/ruby.yml")),
            (C, include_str!("patterns/c.yml")),
            (Cpp, include_str!("patterns/cpp.yml")),
            (Terraform, include_str!("patterns/terraform.yml")),
            (Kubernetes, include_str!("patterns/kubernetes.yml")),
        ];

        for (lang, content) in languages {
            match serde_yaml::from_str::<LanguagePatterns>(content) {
                Ok(patterns) => {
                    map.insert(lang, patterns);
                }
                Err(e) => {
                    eprintln!("Failed to parse patterns for {:?}: {}", lang, e);
                }
            }
        }

        map
    }
}
