use regex::Regex;
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
            _ => Language::Other,
        }
    }
}

pub struct SecurityRiskPatterns {
    patterns: Vec<Regex>,
}

impl SecurityRiskPatterns {
    pub fn new(language: Language) -> Self {
        let pattern_map = Self::pattern_map();
        let patterns = pattern_map
            .get(&language)
            .or_else(|| pattern_map.get(&Language::Other))
            .unwrap()
            .iter()
            .map(|p| Regex::new(p).unwrap())
            .collect();

        Self { patterns }
    }

    pub fn matches(&self, content: &str) -> bool {
        self.patterns
            .iter()
            .any(|pattern| pattern.is_match(content))
    }

    fn pattern_map() -> HashMap<Language, Vec<String>> {
        use Language::*;

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let yaml_path = manifest_dir
            .join("security_patterns")
            .join("patterns.yml");
        let content = fs::read_to_string(&yaml_path)
            .expect(&format!("failed to read {}", yaml_path.display()));
        let raw_map: HashMap<String, Vec<String>> =
            serde_yaml::from_str(&content).expect("failed to parse patterns.yml");

        let mut map = HashMap::new();
        for (lang, pats) in raw_map {
            let key = match lang.as_str() {
                "Python" => Python,
                "JavaScript" => JavaScript,
                "Rust" => Rust,
                "TypeScript" => TypeScript,
                "Java" => Java,
                "Go" => Go,
                "Ruby" => Ruby,
                "Other" => Other,
                _ => continue,
            };
            map.insert(key, pats);
        }

        map
    }
}
