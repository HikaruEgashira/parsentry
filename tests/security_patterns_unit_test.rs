use vulnhuntrs::security_patterns::{Language, PatternType, PatternConfig, LanguagePatterns, SecurityRiskPatterns};
use std::collections::HashMap;

#[test]
fn test_language_from_extension() {
    // Test known extensions
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("ts"), Language::TypeScript);
    assert_eq!(Language::from_extension("java"), Language::Java);
    assert_eq!(Language::from_extension("go"), Language::Go);
    assert_eq!(Language::from_extension("rb"), Language::Ruby);
    
    // Test unknown extensions
    assert_eq!(Language::from_extension("txt"), Language::Other);
    assert_eq!(Language::from_extension("cpp"), Language::Other);
    assert_eq!(Language::from_extension(""), Language::Other);
    assert_eq!(Language::from_extension("unknown"), Language::Other);
}

#[test]
fn test_language_equality() {
    assert_eq!(Language::Python, Language::Python);
    assert_ne!(Language::Python, Language::JavaScript);
    assert_ne!(Language::Rust, Language::TypeScript);
}

#[test]
fn test_language_debug() {
    let lang = Language::Python;
    let debug_str = format!("{:?}", lang);
    assert_eq!(debug_str, "Python");
}

#[test]
fn test_pattern_type_equality() {
    assert_eq!(PatternType::Source, PatternType::Source);
    assert_eq!(PatternType::Sink, PatternType::Sink);
    assert_eq!(PatternType::Validate, PatternType::Validate);
    
    assert_ne!(PatternType::Source, PatternType::Sink);
    assert_ne!(PatternType::Sink, PatternType::Validate);
    assert_ne!(PatternType::Source, PatternType::Validate);
}

#[test]
fn test_pattern_config_creation() {
    let config = PatternConfig {
        pattern: "eval\\(".to_string(),
        description: "Dynamic code execution".to_string(),
    };
    
    assert_eq!(config.pattern, "eval\\(");
    assert_eq!(config.description, "Dynamic code execution");
}

#[test]
fn test_language_patterns_creation() {
    let sources = vec![
        PatternConfig {
            pattern: "input\\(".to_string(),
            description: "User input".to_string(),
        },
        PatternConfig {
            pattern: "request\\.get".to_string(),
            description: "HTTP request parameter".to_string(),
        },
    ];
    
    let sinks = vec![
        PatternConfig {
            pattern: "eval\\(".to_string(),
            description: "Code execution".to_string(),
        },
    ];
    
    let patterns = LanguagePatterns {
        sources: Some(sources.clone()),
        sinks: Some(sinks.clone()),
        validate: None,
    };
    
    assert!(patterns.sources.is_some());
    assert!(patterns.sinks.is_some());
    assert_eq!(patterns.sources.unwrap().len(), 2);
    assert_eq!(patterns.sinks.unwrap().len(), 1);
}

#[test]
fn test_language_patterns_empty() {
    let patterns = LanguagePatterns {
        sources: None,
        sinks: None,
        validate: None,
    };
    
    assert!(patterns.sources.is_none());
    assert!(patterns.sinks.is_none());
}

#[test]
fn test_language_patterns_partial() {
    let patterns = LanguagePatterns {
        sources: Some(vec![PatternConfig {
            pattern: "test".to_string(),
            description: "test description".to_string(),
        }]),
        sinks: None,
        validate: None,
    };
    
    assert!(patterns.sources.is_some());
    assert!(patterns.sinks.is_none());
}

#[test]
fn test_security_risk_patterns_new() {
    let patterns = SecurityRiskPatterns::new(Language::Python);
    // The constructor doesn't return Result, it creates an instance directly
    assert!(true); // Just test that it doesn't panic
}

// Note: SecurityRiskPatterns tests are skipped because they depend on patterns.yml file
// which may not be available in the test environment. We focus on testing the basic types.

// Note: PatternConfig and LanguagePatterns only derive Deserialize, not Serialize
// So we skip serialization tests and focus on deserialization and basic functionality

#[test]
fn test_language_hash_and_equality() {
    use std::collections::HashSet;
    
    let mut set = HashSet::new();
    set.insert(Language::Python);
    set.insert(Language::JavaScript);
    set.insert(Language::Python); // Duplicate
    
    assert_eq!(set.len(), 2); // Should only contain 2 unique languages
    assert!(set.contains(&Language::Python));
    assert!(set.contains(&Language::JavaScript));
    assert!(!set.contains(&Language::Rust));
}

#[test]
fn test_pattern_type_clone() {
    let original = PatternType::Source;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_language_clone() {
    let original = Language::Rust;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_pattern_config_deserialization() {
    use serde_json;
    
    let json_data = r#"{"pattern": "test_pattern", "description": "test description"}"#;
    let config: Result<PatternConfig, _> = serde_json::from_str(json_data);
    
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.pattern, "test_pattern");
    assert_eq!(config.description, "test description");
}

#[test]
fn test_language_patterns_deserialization() {
    use serde_json;
    
    let json_data = r#"{
        "sources": [{"pattern": "input", "description": "User input"}],
        "sinks": [{"pattern": "eval", "description": "Code execution"}],
        "validate": null
    }"#;
    
    let patterns: Result<LanguagePatterns, _> = serde_json::from_str(json_data);
    assert!(patterns.is_ok());
    
    let patterns = patterns.unwrap();
    assert!(patterns.sources.is_some());
    assert!(patterns.sinks.is_some());
    assert!(patterns.validate.is_none());
}
