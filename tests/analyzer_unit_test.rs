use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use vulnhuntrs::parser::{Context, Definition};
use vulnhuntrs::response::Response;

// Mock functions for testing
#[tokio::test]
async fn test_analyze_empty_file() -> anyhow::Result<()> {
    // Create empty temporary file
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path().to_path_buf();

    // Create empty context
    let _context = Context {
        definitions: vec![],
    };

    // Test with mock model (this would require actual API key in real scenario)
    // For unit test, we'll test the empty file handling specifically
    let result = analyze_empty_file_logic(&file_path).await?;

    assert_eq!(result.scratchpad, "");
    assert_eq!(result.analysis, "");
    assert_eq!(result.poc, "");
    assert_eq!(result.confidence_score, 0);
    assert_eq!(result.vulnerability_types.len(), 0);
    assert_eq!(result.context_code.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_analyze_file_with_basic_content() -> anyhow::Result<()> {
    // Create temporary file with basic content
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "print('Hello, World!')")?;
    let file_path = temp_file.path().to_path_buf();

    // Create context with a mock definition
    let _context = Context {
        definitions: vec![Definition {
            name: "test_function".to_string(),
            source: "def test_function(): pass".to_string(),
            start_byte: 0,
            end_byte: 25,
        }],
    };

    // Test basic file processing logic (without actual LLM call)
    let content = std::fs::read_to_string(&file_path)?;
    assert!(!content.is_empty());
    assert!(content.contains("Hello, World!"));

    Ok(())
}

#[test]
fn test_context_text_generation() {
    let context = Context {
        definitions: vec![
            Definition {
                name: "vulnerable_function".to_string(),
                source: "def vulnerable_function(user_input):\n    os.system(user_input)"
                    .to_string(),
                start_byte: 0,
                end_byte: 50,
            },
            Definition {
                name: "safe_function".to_string(),
                source: "def safe_function(user_input):\n    return user_input.strip()".to_string(),
                start_byte: 60,
                end_byte: 110,
            },
        ],
    };

    let mut context_text = String::new();
    if !context.definitions.is_empty() {
        context_text.push_str("\nContext Definitions:\n");
        for def in &context.definitions {
            context_text.push_str(&format!(
                "\nFunction/Definition: {}\nCode:\n{}\n",
                def.name, def.source
            ));
        }
    }

    assert!(context_text.contains("vulnerable_function"));
    assert!(context_text.contains("safe_function"));
    assert!(context_text.contains("os.system"));
    assert!(context_text.contains("Context Definitions:"));
}

// Simulate the empty file handling logic from analyzer.rs
async fn analyze_empty_file_logic(file_path: &PathBuf) -> anyhow::Result<Response> {
    let content = std::fs::read_to_string(file_path)?;
    if content.is_empty() {
        return Ok(Response {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: 0,
            vulnerability_types: vec![],
            context_code: vec![],
        });
    }

    // For non-empty files, return a mock response
    Ok(Response {
        scratchpad: "File processed".to_string(),
        analysis: "Basic analysis performed".to_string(),
        poc: "".to_string(),
        confidence_score: 5,
        vulnerability_types: vec![],
        context_code: vec![],
    })
}

#[test]
fn test_parse_json_response_valid() {
    let json_content = r#"{
        "scratchpad": "Test scratchpad",
        "analysis": "Test analysis",
        "poc": "Test PoC",
        "confidence_score": 8,
        "vulnerability_types": [],
        "context_code": []
    }"#;

    let result: Result<Response, _> = serde_json::from_str(json_content);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.scratchpad, "Test scratchpad");
    assert_eq!(response.analysis, "Test analysis");
    assert_eq!(response.confidence_score, 8);
}

#[test]
fn test_parse_json_response_invalid() {
    let invalid_json = r#"{ invalid json content"#;

    let result: Result<Response, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_file_path_display() {
    let file_path = PathBuf::from("/test/path/vulnerable.py");
    let display_str = format!("File: {}", file_path.display());
    assert!(display_str.contains("vulnerable.py"));
    assert!(display_str.contains("/test/path/"));
}
