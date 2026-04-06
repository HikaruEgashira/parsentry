use parsentry_core::Language;
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

// Threat model metadata collection tests
#[test]
fn test_repo_metadata_collection() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("src/app.py"),
        "from flask import Flask\napp = Flask(__name__)",
    )
    .unwrap();
    fs::write(root.join("requirements.txt"), "flask==3.0\nsqlalchemy==2.0").unwrap();

    let meta = parsentry_core::RepoMetadata::collect(root).unwrap();

    assert_eq!(meta.total_files, 1);
    assert!(meta.languages.contains_key(&Language::Python));
    assert_eq!(meta.dependency_manifests.len(), 1);
    assert!(meta.dependency_manifests[0].content.contains("flask"));
}

#[test]
fn test_repo_metadata_prompt_context() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    fs::create_dir_all(root.join("api")).unwrap();
    fs::write(root.join("api/app.py"), "from flask import Flask").unwrap();
    fs::write(root.join("requirements.txt"), "flask").unwrap();

    let meta = parsentry_core::RepoMetadata::collect(root).unwrap();
    let ctx = meta.to_prompt_context();

    assert!(ctx.contains("Directory Structure"));
    assert!(ctx.contains("Languages"));
    assert!(ctx.contains("Dependencies"));
    assert!(ctx.contains("flask"));
}

#[test]
fn test_threat_model_report_rendering() {
    use parsentry_core::{AttackSurface, ThreatModel, render_threat_model_md};

    let model = ThreatModel {
        repository: "test/repo".to_string(),
        generated_at: "2025-01-01T00:00:00Z".to_string(),
        app_type: "web_application".to_string(),
        summary: "A Flask web app with user-facing endpoints.".to_string(),
        surfaces: vec![AttackSurface {
            id: "SURFACE-001".to_string(),
            kind: "endpoint".to_string(),
            identifier: "POST /api/users".to_string(),
            locations: vec!["src/routes/users.py".to_string()],
            description: "User registration endpoint accepting untrusted input".to_string(),
            query: r#"(call function: (identifier) @func (#eq? @func "route")) @call"#.to_string(),
        }],
    };

    let md = render_threat_model_md(&model);

    assert!(md.contains("# Threat Model: test/repo"));
    assert!(md.contains("SURFACE-001"));
    assert!(md.contains("POST /api/users"));
    assert!(md.contains("web_application"));
}

#[test]
fn test_threat_model_all_locations() {
    use parsentry_core::{AttackSurface, ThreatModel};

    let model = ThreatModel {
        repository: "test".to_string(),
        generated_at: "now".to_string(),
        app_type: "web_application".to_string(),
        summary: "test".to_string(),
        surfaces: vec![
            AttackSurface {
                id: "S1".to_string(),
                kind: "endpoint".to_string(),
                identifier: "GET /api/users".to_string(),
                locations: vec!["src/routes.py".to_string(), "src/models.py".to_string()],
                description: "test".to_string(),
                query: "(call) @call".to_string(),
            },
            AttackSurface {
                id: "S2".to_string(),
                kind: "db_table".to_string(),
                identifier: "users table".to_string(),
                locations: vec!["src/models.py".to_string(), "src/db.py".to_string()],
                description: "test".to_string(),
                query: "(call) @call".to_string(),
            },
        ],
    };

    let locations = model.all_locations();
    // Deduplicated: src/routes.py, src/models.py, src/db.py
    assert_eq!(locations.len(), 3);
    assert_eq!(model.total_surfaces(), 2);
}

#[test]
fn test_add_dynamic_query_to_patterns() {
    use parsentry_parser::SecurityRiskPatterns;

    let mut patterns = SecurityRiskPatterns::new(Language::Python);

    // Valid query should succeed
    let ok = patterns.add_query(
        "reference",
        r#"(call function: (identifier) @func (#eq? @func "eval")) @call"#,
        "eval() calls",
        vec!["T1059".to_string()],
    );
    assert!(ok);

    // Invalid query should fail gracefully
    let bad = patterns.add_query(
        "reference",
        "this is not a valid query !!!",
        "bad query",
        vec![],
    );
    assert!(!bad);
}
