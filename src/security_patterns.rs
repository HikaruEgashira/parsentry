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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    Source,
    Sink,
    Validate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatternConfig {
    pub pattern: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguagePatterns {
    pub sources: Option<Vec<PatternConfig>>,
    pub sinks: Option<Vec<PatternConfig>>,
    pub validate: Option<Vec<PatternConfig>>,
}

pub struct SecurityRiskPatterns {
    source_patterns: Vec<Regex>,
    sink_patterns: Vec<Regex>,
    validate_patterns: Vec<Regex>,
    pattern_type_map: HashMap<String, PatternType>,
}

impl SecurityRiskPatterns {
    pub fn new(language: Language) -> Self {
        let pattern_map = Self::load_patterns();
        let lang_patterns = pattern_map
            .get(&language)
            .or_else(|| pattern_map.get(&Language::Other))
            .unwrap();

        let mut source_patterns = Vec::new();
        let mut sink_patterns = Vec::new(); 
        let mut validate_patterns = Vec::new();
        let mut pattern_type_map = HashMap::new();

        if let Some(sources) = &lang_patterns.sources {
            for config in sources {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Source);
                source_patterns.push(regex);
            }
        }

        if let Some(sinks) = &lang_patterns.sinks {
            for config in sinks {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Sink);
                sink_patterns.push(regex);
            }
        }

        if let Some(validates) = &lang_patterns.validate {
            for config in validates {
                let regex = Regex::new(&config.pattern).unwrap();
                pattern_type_map.insert(config.pattern.clone(), PatternType::Validate);
                validate_patterns.push(regex);
            }
        }

        Self { 
            source_patterns,
            sink_patterns, 
            validate_patterns,
            pattern_type_map,
        }
    }

    pub fn matches(&self, content: &str) -> bool {
        self.source_patterns.iter().any(|pattern| pattern.is_match(content))
            || self.sink_patterns.iter().any(|pattern| pattern.is_match(content))
            || self.validate_patterns.iter().any(|pattern| pattern.is_match(content))
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
        let yaml_path = manifest_dir
            .join("security_patterns")
            .join("patterns.yml");
        let content = fs::read_to_string(&yaml_path)
            .expect(&format!("failed to read {}", yaml_path.display()));
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
                "Other" => Other,
                _ => continue,
            };
            map.insert(key, patterns);
        }

        map
    }
}
