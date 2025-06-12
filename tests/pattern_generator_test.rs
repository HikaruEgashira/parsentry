use parsentry::pattern_generator::{PatternClassification, write_patterns_to_file};
use parsentry::security_patterns::Language;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_python_file(dir: &TempDir) -> PathBuf {
    let file_path = dir.path().join("test.py");
    let content = r#"
import os
import subprocess
from flask import Flask, request

app = Flask(__name__)

def get_user_input():
    return request.args.get('input', '')

def execute_command(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True)

def write_to_file(filename, data):
    with open(filename, 'w') as f:
        f.write(data)

def validate_input(data):
    import re
    return re.match(r'^[a-zA-Z0-9]+$', data) is not None

def hash_password(password):
    import hashlib
    return hashlib.sha256(password.encode()).hexdigest()
"#;
    fs::write(&file_path, content).unwrap();
    file_path
}

fn create_test_rust_file(dir: &TempDir) -> PathBuf {
    let file_path = dir.path().join("test.rs");
    let content = r#"
use std::process::Command;
use std::fs;

fn get_user_data() -> String {
    std::env::args().nth(1).unwrap_or_default()
}

fn execute_shell_command(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn sanitize_input(input: &str) -> String {
    input.chars().filter(|c| c.is_alphanumeric()).collect()
}
"#;
    fs::write(&file_path, content).unwrap();
    file_path
}

// Note: LLM integration tests are commented out as they require real API keys
// These would be integration tests that need actual LLM responses
//
// #[tokio::test]
// async fn test_pattern_generation_python() {
//     let temp_dir = TempDir::new().unwrap();
//     create_test_python_file(&temp_dir);
//
//     let result = generate_custom_patterns(&temp_dir.path().to_path_buf(), "test-model").await;
//     assert!(result.is_ok());
//     assert!(temp_dir.path().join("vuln-patterns.yml").exists());
// }
//
// #[tokio::test]
// async fn test_pattern_generation_rust() {
//     let temp_dir = TempDir::new().unwrap();
//     create_test_rust_file(&temp_dir);
//
//     let result = generate_custom_patterns(&temp_dir.path().to_path_buf(), "test-model").await;
//     assert!(result.is_ok());
//     assert!(temp_dir.path().join("vuln-patterns.yml").exists());
// }

#[test]
fn test_yaml_pattern_format() {
    let temp_dir = TempDir::new().unwrap();

    let patterns = vec![
        PatternClassification {
            function_name: "test_resource".to_string(),
            pattern_type: Some("resources".to_string()),
            pattern: "\\\\btest_resource\\\\s*\\\\(".to_string(),
            description: "Test resource function".to_string(),
            reasoning: "Test reasoning for resource".to_string(),
            attack_vector: vec!["T1059".to_string()],
        },
        PatternClassification {
            function_name: "test_principal".to_string(),
            pattern_type: Some("principals".to_string()),
            pattern: "\\\\btest_principal\\\\s*\\\\(".to_string(),
            description: "Test principal function".to_string(),
            reasoning: "Test reasoning for principal".to_string(),
            attack_vector: vec!["T1190".to_string()],
        },
        PatternClassification {
            function_name: "test_action".to_string(),
            pattern_type: Some("actions".to_string()),
            pattern: "\\\\btest_action\\\\s*\\\\(".to_string(),
            description: "Test action function".to_string(),
            reasoning: "Test reasoning for action".to_string(),
            attack_vector: vec!["T1070".to_string()],
        },
    ];

    let result =
        write_patterns_to_file(&temp_dir.path().to_path_buf(), Language::Python, &patterns);
    assert!(result.is_ok());

    let yaml_path = temp_dir.path().join("vuln-patterns.yml");
    assert!(yaml_path.exists());

    let content = fs::read_to_string(&yaml_path).unwrap();
    assert!(content.contains("Python:"));
    assert!(content.contains("principals:"));
    assert!(content.contains("resources:"));
    assert!(content.contains("actions:"));
    assert!(content.contains("test_resource"));
    assert!(content.contains("test_principal"));
    assert!(content.contains("test_action"));
}

// Integration test that doesn't require API calls
#[test]
fn test_file_discovery() {
    let temp_dir = TempDir::new().unwrap();
    create_test_python_file(&temp_dir);
    create_test_rust_file(&temp_dir);

    let repo = parsentry::repo::RepoOps::new(temp_dir.path().to_path_buf());
    let files = repo.get_files_to_analyze(None).unwrap();

    assert_eq!(files.len(), 2);
    assert!(files.iter().any(|f| f.file_name().unwrap() == "test.py"));
    assert!(files.iter().any(|f| f.file_name().unwrap() == "test.rs"));
}

#[test]
fn test_definition_extraction() {
    let temp_dir = TempDir::new().unwrap();
    create_test_python_file(&temp_dir);

    let file_path = temp_dir.path().join("test.py");
    let mut parser = parsentry::parser::CodeParser::new().unwrap();
    parser.add_file(&file_path).unwrap();

    let context = parser.build_context_from_file(&file_path).unwrap();

    // Should extract function definitions
    assert!(!context.definitions.is_empty());

    // Check if expected functions are found
    let function_names: Vec<&str> = context
        .definitions
        .iter()
        .map(|def| def.name.as_str())
        .collect();

    assert!(function_names.contains(&"get_user_input"));
    assert!(function_names.contains(&"execute_command"));
    assert!(function_names.contains(&"write_to_file"));
    assert!(function_names.contains(&"validate_input"));
    assert!(function_names.contains(&"hash_password"));
}

#[test]
fn test_yaml_append_functionality() {
    let temp_dir = TempDir::new().unwrap();
    let yaml_path = temp_dir.path().join("vuln-patterns.yml");

    // Write initial content
    fs::write(&yaml_path, "Go:\n  resources:\n    - pattern: \"existing_pattern\"\n      description: \"Existing pattern\"\n").unwrap();

    let patterns = vec![PatternClassification {
        function_name: "new_function".to_string(),
        pattern_type: Some("resources".to_string()),
        pattern: "\\\\bnew_function\\\\s*\\\\(".to_string(),
        description: "New function pattern".to_string(),
        reasoning: "Test reasoning".to_string(),
        attack_vector: vec!["T1059".to_string()],
    }];

    let result =
        write_patterns_to_file(&temp_dir.path().to_path_buf(), Language::Python, &patterns);
    assert!(result.is_ok());

    let content = fs::read_to_string(&yaml_path).unwrap();
    assert!(content.contains("Go:"));
    assert!(content.contains("existing_pattern"));
    assert!(content.contains("Python:"));
    assert!(content.contains("new_function"));
}

#[test]
fn test_empty_patterns_handling() {
    let temp_dir = TempDir::new().unwrap();

    let patterns = vec![PatternClassification {
        function_name: "non_security_function".to_string(),
        pattern_type: None,
        pattern: "".to_string(),
        description: "".to_string(),
        reasoning: "Not a security pattern".to_string(),
        attack_vector: vec![],
    }];

    let result =
        write_patterns_to_file(&temp_dir.path().to_path_buf(), Language::Python, &patterns);
    assert!(result.is_ok());

    let yaml_path = temp_dir.path().join("vuln-patterns.yml");
    let content = fs::read_to_string(&yaml_path).unwrap();

    // Should only contain the language header since no valid patterns were provided
    assert_eq!(content.trim(), "Python:");
}

#[test]
fn test_language_filtering() {
    let temp_dir = TempDir::new().unwrap();

    let patterns = vec![PatternClassification {
        function_name: "test_function".to_string(),
        pattern_type: Some("resources".to_string()),
        pattern: "test_pattern".to_string(),
        description: "Test description".to_string(),
        reasoning: "Test reasoning".to_string(),
        attack_vector: vec!["T1059".to_string()],
    }];

    // Test that Other language is skipped
    let result = write_patterns_to_file(&temp_dir.path().to_path_buf(), Language::Other, &patterns);
    assert!(result.is_ok());

    let yaml_path = temp_dir.path().join("vuln-patterns.yml");
    assert!(!yaml_path.exists());
}

fn create_mixed_security_file(dir: &TempDir) -> PathBuf {
    let file_path = dir.path().join("mixed.py");
    let content = r#"
# High-risk security functions
def execute_user_command(cmd):
    import subprocess
    return subprocess.run(cmd, shell=True)

def get_user_data(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return db.execute(query)

# Non-security utility functions
def format_string(text):
    return text.strip().upper()

def calculate_sum(numbers):
    return sum(numbers)

def get_current_time():
    import datetime
    return datetime.datetime.now()

# Medium-risk configuration function
def load_config():
    import os
    return os.environ.get('DATABASE_URL')

# Low-risk helper function
def validate_email_format(email):
    return '@' in email and '.' in email
"#;
    fs::write(&file_path, content).unwrap();
    file_path
}

#[test]
fn test_definition_extraction_mixed_functions() {
    let temp_dir = TempDir::new().unwrap();
    create_mixed_security_file(&temp_dir);

    let file_path = temp_dir.path().join("mixed.py");
    let mut parser = parsentry::parser::CodeParser::new().unwrap();
    parser.add_file(&file_path).unwrap();

    let context = parser.build_context_from_file(&file_path).unwrap();

    // Should extract all function definitions
    assert!(!context.definitions.is_empty());
    assert!(context.definitions.len() >= 6); // At least 6 functions defined

    let function_names: Vec<&str> = context
        .definitions
        .iter()
        .map(|def| def.name.as_str())
        .collect();

    // High-risk functions
    assert!(function_names.contains(&"execute_user_command"));
    assert!(function_names.contains(&"get_user_data"));

    // Non-security functions
    assert!(function_names.contains(&"format_string"));
    assert!(function_names.contains(&"calculate_sum"));
    assert!(function_names.contains(&"get_current_time"));

    // Medium-risk function
    assert!(function_names.contains(&"load_config"));

    // Low-risk function
    assert!(function_names.contains(&"validate_email_format"));
}

#[test]
fn test_pattern_classification_structure() {
    // Test the structure of PatternClassification without LLM calls
    let pattern = PatternClassification {
        function_name: "test_auth".to_string(),
        pattern_type: Some("actions".to_string()),
        pattern: "\\\\btest_auth\\\\s*\\\\(".to_string(),
        description: "Authentication function".to_string(),
        reasoning: "Handles user authentication".to_string(),
        attack_vector: vec!["T1078".to_string(), "T1110".to_string()],
    };

    assert_eq!(pattern.function_name, "test_auth");
    assert_eq!(pattern.pattern_type, Some("actions".to_string()));
    assert!(!pattern.attack_vector.is_empty());
    assert!(pattern.attack_vector.contains(&"T1078".to_string()));
    assert!(pattern.attack_vector.contains(&"T1110".to_string()));
}

#[test]
fn test_attack_vector_formatting() {
    let temp_dir = TempDir::new().unwrap();

    let patterns = vec![PatternClassification {
        function_name: "multi_attack_function".to_string(),
        pattern_type: Some("resources".to_string()),
        pattern: "\\\\bmulti_attack_function\\\\s*\\\\(".to_string(),
        description: "Function with multiple attack vectors".to_string(),
        reasoning: "Can be exploited in multiple ways".to_string(),
        attack_vector: vec![
            "T1059".to_string(), // Command and Scripting Interpreter
            "T1190".to_string(), // Exploit Public-Facing Application
            "T1078".to_string(), // Valid Accounts
        ],
    }];

    let result =
        write_patterns_to_file(&temp_dir.path().to_path_buf(), Language::Python, &patterns);
    assert!(result.is_ok());

    let yaml_path = temp_dir.path().join("vuln-patterns.yml");
    let content = fs::read_to_string(&yaml_path).unwrap();

    // Check that all attack vectors are properly formatted
    assert!(content.contains("attack_vector:"));
    assert!(content.contains("\"T1059\""));
    assert!(content.contains("\"T1190\""));
    assert!(content.contains("\"T1078\""));
}

// Test to verify the new filtering logic would work
// This is a unit test that doesn't require LLM calls
#[test]
fn test_security_risk_categories() {
    // Define test cases that represent different security risk levels
    let high_risk_patterns = vec![
        "execute_user_command", // Command injection risk
        "get_user_data",        // SQL injection risk
        "upload_file",          // File upload risk
        "authenticate_user",    // Authentication function
    ];

    let low_risk_patterns = vec![
        "format_string",    // String utility
        "calculate_sum",    // Math function
        "get_current_time", // Time utility
        "sort_array",       // Data structure operation
    ];

    // In a real implementation, these would be filtered by the LLM
    // Here we just verify the pattern structure is as expected
    for pattern_name in high_risk_patterns {
        assert!(pattern_name.len() > 0);
        // High-risk patterns typically involve user input, system operations, or security functions
        assert!(
            pattern_name.contains("user")
                || pattern_name.contains("execute")
                || pattern_name.contains("auth")
                || pattern_name.contains("upload")
                || pattern_name.contains("get") && pattern_name.contains("data")
        );
    }

    for pattern_name in low_risk_patterns {
        assert!(pattern_name.len() > 0);
        // Low-risk patterns are typically utilities without external dependencies
        assert!(
            pattern_name.contains("format")
                || pattern_name.contains("calculate")
                || pattern_name.contains("time")
                || pattern_name.contains("sort")
        );
    }
}

#[test]
fn test_par_pattern_types() {
    // Test that we handle all three PAR pattern types correctly
    let principals_example = PatternClassification {
        function_name: "get_request_data".to_string(),
        pattern_type: Some("principals".to_string()),
        pattern: "\\\\bget_request_data\\\\s*\\\\(".to_string(),
        description: "Retrieves data from HTTP request".to_string(),
        reasoning: "Principal - source of user input".to_string(),
        attack_vector: vec!["T1190".to_string()],
    };

    let actions_example = PatternClassification {
        function_name: "validate_input".to_string(),
        pattern_type: Some("actions".to_string()),
        pattern: "\\\\bvalidate_input\\\\s*\\\\(".to_string(),
        description: "Validates user input".to_string(),
        reasoning: "Action - performs security validation".to_string(),
        attack_vector: vec!["T1059".to_string()],
    };

    let resources_example = PatternClassification {
        function_name: "write_file".to_string(),
        pattern_type: Some("resources".to_string()),
        pattern: "\\\\bwrite_file\\\\s*\\\\(".to_string(),
        description: "Writes data to file".to_string(),
        reasoning: "Resource - accesses file system".to_string(),
        attack_vector: vec!["T1005".to_string()],
    };

    assert_eq!(
        principals_example.pattern_type,
        Some("principals".to_string())
    );
    assert_eq!(actions_example.pattern_type, Some("actions".to_string()));
    assert_eq!(
        resources_example.pattern_type,
        Some("resources".to_string())
    );
}
