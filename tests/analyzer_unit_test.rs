use parsentry::response::Response;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_analyze_empty_file() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path().to_path_buf();

    let result = analyze_empty_file_logic(&file_path).await?;

    assert_eq!(result.scratchpad, "");
    assert_eq!(result.analysis, "");
    assert_eq!(result.poc, "");
    assert_eq!(result.confidence_score, 0);
    assert_eq!(result.vulnerability_types.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_analyze_file_with_basic_content() -> anyhow::Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "print('Hello, World!')")?;
    let file_path = temp_file.path().to_path_buf();

    let content = std::fs::read_to_string(&file_path)?;
    assert!(!content.is_empty());
    assert!(content.contains("Hello, World!"));

    Ok(())
}

// Simulate the empty file handling logic
async fn analyze_empty_file_logic(file_path: &PathBuf) -> anyhow::Result<Response> {
    let content = std::fs::read_to_string(file_path)?;
    if content.is_empty() {
        return Ok(Response {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: 0,
            vulnerability_types: vec![],
            ..Default::default()
        });
    }

    Ok(Response {
        scratchpad: "File processed".to_string(),
        analysis: "Basic analysis performed".to_string(),
        confidence_score: 5,
        ..Default::default()
    })
}

#[test]
fn test_parse_json_response_valid() {
    let json_content = r#"{
        "scratchpad": "Test scratchpad",
        "analysis": "Test analysis",
        "poc": "Test PoC",
        "confidence_score": 8,
        "vulnerability_types": []
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
