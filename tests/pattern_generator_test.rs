use parsentry::pattern_generator::generate_custom_patterns;
use parsentry::repo::RepoOps;
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

#[tokio::test]
async fn test_generate_custom_patterns_outputs_summary() {
    let temp_dir = TempDir::new().unwrap();
    create_test_python_file(&temp_dir);

    let result = generate_custom_patterns(&temp_dir.path().to_path_buf(), "dummy", None).await;
    assert!(result.is_ok());

    // summary mode should not create legacy pattern files
    assert!(!temp_dir.path().join("vuln-patterns.yml").exists());
}

#[test]
fn test_file_discovery() {
    let temp_dir = TempDir::new().unwrap();
    create_test_python_file(&temp_dir);
    create_test_rust_file(&temp_dir);

    let repo = RepoOps::new(temp_dir.path().to_path_buf());
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

    assert!(!context.definitions.is_empty());

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
fn test_definition_extraction_mixed_functions() {
    let temp_dir = TempDir::new().unwrap();
    create_mixed_security_file(&temp_dir);

    let file_path = temp_dir.path().join("mixed.py");
    let mut parser = parsentry::parser::CodeParser::new().unwrap();
    parser.add_file(&file_path).unwrap();

    let context = parser.build_context_from_file(&file_path).unwrap();

    assert!(!context.definitions.is_empty());
    assert!(context.definitions.len() >= 6);

    let function_names: Vec<&str> = context
        .definitions
        .iter()
        .map(|def| def.name.as_str())
        .collect();

    assert!(function_names.contains(&"execute_user_command"));
    assert!(function_names.contains(&"get_user_data"));
    assert!(function_names.contains(&"format_string"));
    assert!(function_names.contains(&"calculate_sum"));
    assert!(function_names.contains(&"get_current_time"));
    assert!(function_names.contains(&"load_config"));
    assert!(function_names.contains(&"validate_email_format"));
}
