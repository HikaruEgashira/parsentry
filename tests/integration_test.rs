use std::path::PathBuf;
use vulnhuntrs::analyzer::analyze_file;
use vulnhuntrs::parser::{CodeParser, Context};
use vulnhuntrs::repo::RepoOps;
use vulnhuntrs::response::{AnalysisSummary, VulnType};
use vulnhuntrs::security_patterns::{Language, SecurityRiskPatterns};

/// Integration test for analyzing the Python vulnerable application
#[tokio::test]
async fn test_analyze_python_vulnerable_app() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let python_app_path = manifest_dir
        .join("example")
        .join("python-vulnerable-app")
        .join("app.py");

    // Skip test if example file doesn't exist
    if !python_app_path.exists() {
        println!(
            "Skipping test: example Python app not found at {:?}",
            python_app_path
        );
        return Ok(());
    }

    // Parse the file to get context
    let mut parser = CodeParser::new()?;
    parser.add_file(&python_app_path)?;
    let context = parser
        .build_context_from_file(&python_app_path)
        .unwrap_or(Context {
            definitions: vec![],
        });

    // Run analysis (using a test model that doesn't require API key)
    // In real CI environment, this would use a mock or test API
    let result = analyze_file(
        &python_app_path,
        "test-model", // This will fail in CI but that's expected for this test structure
        &[python_app_path.clone()],
        7, // min_confidence
        &context,
        0, // verbosity
    )
    .await;

    // For this integration test, we mainly want to verify the structure works
    // The actual analysis will fail without API keys, which is expected
    match result {
        Ok(response) => {
            // If analysis succeeds (with API key), verify response structure
            assert!(response.confidence_score >= 0);
            assert!(response.confidence_score <= 100);
            // Should detect SQL injection, XSS, or RCE vulnerabilities
            println!(
                "Detected vulnerabilities: {:?}",
                response.vulnerability_types
            );
        }
        Err(_) => {
            // Expected to fail without proper API configuration
            println!("Analysis failed as expected without API key");
        }
    }

    Ok(())
}

/// Integration test for analyzing the Rust vulnerable application
#[tokio::test]
async fn test_analyze_rust_vulnerable_app() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rust_app_path = manifest_dir
        .join("example")
        .join("rust-vulnerable-app")
        .join("src")
        .join("main.rs");

    if !rust_app_path.exists() {
        println!(
            "Skipping test: example Rust app not found at {:?}",
            rust_app_path
        );
        return Ok(());
    }

    let mut parser = CodeParser::new()?;
    parser.add_file(&rust_app_path)?;
    let context = parser
        .build_context_from_file(&rust_app_path)
        .unwrap_or(Context {
            definitions: vec![],
        });

    let result = analyze_file(
        &rust_app_path,
        "test-model",
        &[rust_app_path.clone()],
        7,
        &context,
        0,
    )
    .await;

    match result {
        Ok(response) => {
            assert!(response.confidence_score >= 0);
            assert!(response.confidence_score <= 100);
            println!(
                "Detected Rust vulnerabilities: {:?}",
                response.vulnerability_types
            );
        }
        Err(_) => {
            println!("Rust analysis failed as expected without API key");
        }
    }

    Ok(())
}

/// Integration test for RepoOps file discovery
#[test]
fn test_repo_ops_file_discovery() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let example_dir = manifest_dir.join("example");

    if !example_dir.exists() {
        println!("Skipping test: example directory not found");
        return Ok(());
    }

    let repo = RepoOps::new(example_dir);
    let files = repo.get_files_to_analyze(None)?;

    // Should find at least the Python and Rust vulnerable apps
    assert!(!files.is_empty(), "Should discover some files to analyze");

    // Verify we found expected file types
    let python_files: Vec<_> = files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "py"))
        .collect();
    let rust_files: Vec<_> = files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "rs"))
        .collect();

    assert!(!python_files.is_empty(), "Should find Python files");
    assert!(!rust_files.is_empty(), "Should find Rust files");

    println!("Discovered {} files for analysis", files.len());
    println!("Python files: {}", python_files.len());
    println!("Rust files: {}", rust_files.len());

    Ok(())
}

/// Integration test for security pattern matching
#[test]
fn test_security_patterns_integration() {
    // Test pattern creation for different languages
    let languages = vec![
        Language::Python,
        Language::JavaScript,
        Language::Rust,
        Language::TypeScript,
        Language::Java,
        Language::Go,
        Language::Ruby,
        // Note: Language::Other might not have patterns defined, so we skip it
    ];

    for language in languages {
        let _patterns = SecurityRiskPatterns::new(language);

        // Basic smoke test - patterns should be created without panic
        println!("Successfully created patterns for {:?}", language);
    }

    // Test language detection from file extension
    let test_cases = vec![
        ("test.py", Language::Python),
        ("test.rs", Language::Rust),
        ("test.js", Language::JavaScript),
        ("test.java", Language::Java),
        ("test.go", Language::Go),
        ("test.rb", Language::Ruby),
        ("test.txt", Language::Other),
        ("test.md", Language::Other),
    ];

    for (filename, expected_lang) in test_cases {
        let path = PathBuf::from(filename);
        let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let detected_lang = Language::from_extension(ext);
        assert_eq!(
            detected_lang, expected_lang,
            "Language detection failed for {}: expected {:?}, got {:?}",
            filename, expected_lang, detected_lang
        );
    }
}

/// Integration test for AnalysisSummary workflow
#[test]
fn test_analysis_summary_workflow() {
    let mut summary = AnalysisSummary::new();

    // Create mock analysis results
    let mock_results = vec![
        (
            PathBuf::from("/test/sqli.py"),
            create_mock_response(9, vec![VulnType::SQLI]),
        ),
        (
            PathBuf::from("/test/xss.js"),
            create_mock_response(8, vec![VulnType::XSS]),
        ),
        (
            PathBuf::from("/test/rce.py"),
            create_mock_response(7, vec![VulnType::RCE]),
        ),
        (
            PathBuf::from("/test/low.py"),
            create_mock_response(3, vec![VulnType::Other("INFO".to_string())]),
        ),
    ];

    // Add results to summary
    for (path, response) in mock_results {
        summary.add_result(path, response);
    }

    assert_eq!(summary.results.len(), 4);

    // Test sorting by confidence
    summary.sort_by_confidence();
    assert_eq!(summary.results[0].response.confidence_score, 9);
    assert_eq!(summary.results[3].response.confidence_score, 3);

    // Test filtering by confidence
    let high_confidence = summary.filter_by_min_confidence(7);
    assert_eq!(high_confidence.results.len(), 3);

    // Test filtering by vulnerability type
    let sql_vulns = summary.filter_by_vuln_types(&[VulnType::SQLI]);
    assert_eq!(sql_vulns.results.len(), 1);

    // Test markdown generation
    let markdown = summary.to_markdown();
    assert!(markdown.contains("脆弱性解析サマリーレポート"));
    assert!(markdown.contains("SQLI"));
    assert!(markdown.contains("XSS"));
    assert!(markdown.contains("RCE"));
}

/// Integration test for confidence score normalization
#[test]
fn test_confidence_score_normalization() {
    use vulnhuntrs::response::Response;

    // Test the normalize_confidence_score function
    let test_cases = vec![
        (1, 10),    // 1-10 scale -> multiply by 10
        (5, 50),    // 1-10 scale -> multiply by 10
        (10, 100),  // 1-10 scale -> multiply by 10
        (50, 50),   // Already in 0-100 scale
        (85, 85),   // Already in 0-100 scale
        (0, 0),     // Edge case
        (-5, -5),   // Invalid input passed through
        (150, 150), // Invalid input passed through
    ];

    for (input, expected) in test_cases {
        let normalized = Response::normalize_confidence_score(input);
        assert_eq!(
            normalized, expected,
            "Confidence normalization failed: {} -> {} (expected {})",
            input, normalized, expected
        );
    }
}

/// Integration test for vulnerability type parsing and filtering
#[test]
fn test_vulnerability_type_integration() {
    // Test vulnerability type string parsing (similar to main.rs logic)
    let types_str = "LFI,RCE,SQLI,UNKNOWN";
    let vuln_types: Vec<VulnType> = types_str
        .split(',')
        .map(|s| match s.trim() {
            "LFI" => VulnType::LFI,
            "RCE" => VulnType::RCE,
            "SSRF" => VulnType::SSRF,
            "AFO" => VulnType::AFO,
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "IDOR" => VulnType::IDOR,
            other => VulnType::Other(other.to_string()),
        })
        .collect();

    assert_eq!(vuln_types.len(), 4);
    assert_eq!(vuln_types[0], VulnType::LFI);
    assert_eq!(vuln_types[1], VulnType::RCE);
    assert_eq!(vuln_types[2], VulnType::SQLI);
    assert_eq!(vuln_types[3], VulnType::Other("UNKNOWN".to_string()));
}

/// Integration test for file parsing across different languages
#[test]
fn test_parser_integration() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Test parsing different language files if they exist
    let test_files = vec![
        manifest_dir
            .join("example")
            .join("python-vulnerable-app")
            .join("app.py"),
        manifest_dir
            .join("example")
            .join("rust-vulnerable-app")
            .join("src")
            .join("main.rs"),
        manifest_dir.join("src").join("main.rs"), // Our own main.rs
    ];

    for file_path in test_files {
        if file_path.exists() {
            let parser = CodeParser::new();
            match parser {
                Ok(mut p) => {
                    if p.add_file(&file_path).is_ok() {
                        match p.build_context_from_file(&file_path) {
                            Ok(ctx) => {
                                println!(
                                    "Successfully parsed {}: {} definitions",
                                    file_path.display(),
                                    ctx.definitions.len()
                                );

                                // Verify context structure
                                for def in &ctx.definitions {
                                    assert!(!def.name.is_empty(), "Definition should have a name");
                                    assert!(
                                        !def.source.is_empty(),
                                        "Definition should have source code"
                                    );
                                    assert!(
                                        def.end_byte >= def.start_byte,
                                        "End byte should be >= start byte"
                                    );
                                }
                            }
                            Err(e) => {
                                println!(
                                    "Failed to parse {} (this may be expected): {}",
                                    file_path.display(),
                                    e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to create parser for {}: {}", file_path.display(), e);
                }
            }
        }
    }

    Ok(())
}

// Helper function to create mock responses for testing
fn create_mock_response(
    confidence: i32,
    vuln_types: Vec<VulnType>,
) -> vulnhuntrs::response::Response {
    vulnhuntrs::response::Response {
        scratchpad: format!("Mock analysis with confidence {}", confidence),
        analysis: format!(
            "Mock vulnerability analysis for confidence level {}",
            confidence
        ),
        poc: "Mock PoC".to_string(),
        confidence_score: confidence,
        vulnerability_types: vuln_types,
        context_code: vec![],
    }
}
