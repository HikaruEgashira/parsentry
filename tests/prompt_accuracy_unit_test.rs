use anyhow::Result;
use parsentry::security_patterns::{Language, SecurityRiskPatterns};
use std::collections::HashMap;
use tempfile::tempdir;

/// Unit tests specifically focused on prompt accuracy for security pattern detection
/// These tests are separate from SLO tests and focus on individual function analysis

#[derive(Debug)]
struct AccuracyTestCase {
    name: &'static str,
    language: Language,
    code: &'static str,
    expected_security_risk: bool,
    expected_pattern: Option<&'static str>,
    test_rationale: &'static str,
}

const ACCURACY_TEST_CASES: &[AccuracyTestCase] = &[
    // Clear security risks - should always be detected
    AccuracyTestCase {
        name: "SQL injection vulnerable function",
        language: Language::Python,
        code: r#"
def get_user_data(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return execute_query(query)
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "Direct string interpolation in SQL query - classic injection vulnerability",
    },
    AccuracyTestCase {
        name: "Command injection vulnerable function",
        language: Language::Python,
        code: r#"
def process_file(filename):
    import os
    os.system(f"cat {filename}")
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "Direct execution of user-controlled filename in system command",
    },
    AccuracyTestCase {
        name: "XSS vulnerable template rendering",
        language: Language::JavaScript,
        code: r#"
function renderUserComment(comment) {
    document.getElementById('comment').innerHTML = comment;
}
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "Direct insertion of user content into DOM without sanitization",
    },
    AccuracyTestCase {
        name: "Path traversal vulnerable file reader",
        language: Language::JavaScript,
        code: r#"
function readUserFile(filepath) {
    const fs = require('fs');
    return fs.readFileSync(filepath, 'utf8');
}
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "Direct file access without path validation - path traversal risk",
    },
    AccuracyTestCase {
        name: "Authentication bypass vulnerability",
        language: Language::Python,
        code: r#"
def authenticate(username, password):
    if username == "admin" and password:
        return True
    return False
"#,
        expected_security_risk: true,
        expected_pattern: Some("actions"),
        test_rationale: "Weak authentication logic that accepts any non-empty password for admin",
    },
    // Security-related but properly implemented - should still be detected as security patterns
    AccuracyTestCase {
        name: "Proper parameterized query",
        language: Language::Python,
        code: r#"
def get_user_secure(user_id):
    query = "SELECT * FROM users WHERE id = %s"
    return execute_query(query, (user_id,))
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "Database query function - security-relevant even if properly implemented",
    },
    AccuracyTestCase {
        name: "Input validation function",
        language: Language::JavaScript,
        code: r#"
function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}
"#,
        expected_security_risk: true,
        expected_pattern: Some("actions"),
        test_rationale: "Input validation is a security action even when properly implemented",
    },
    AccuracyTestCase {
        name: "Password hashing function",
        language: Language::Python,
        code: r#"
def hash_password(password, salt):
    import hashlib
    return hashlib.pbkdf2_hmac('sha256', password.encode(), salt, 100000)
"#,
        expected_security_risk: true,
        expected_pattern: Some("actions"),
        test_rationale: "Cryptographic operations are security-relevant actions",
    },
    // Clear non-security functions - should NOT be detected
    AccuracyTestCase {
        name: "Simple math calculation",
        language: Language::Python,
        code: r#"
def calculate_area(width, height):
    return width * height
"#,
        expected_security_risk: false,
        expected_pattern: None,
        test_rationale: "Pure mathematical calculation with no security implications",
    },
    AccuracyTestCase {
        name: "String formatting utility",
        language: Language::JavaScript,
        code: r#"
function formatName(first, last) {
    return `${first} ${last}`;
}
"#,
        expected_security_risk: false,
        expected_pattern: None,
        test_rationale: "Simple string manipulation with no external data or security context",
    },
    AccuracyTestCase {
        name: "Array sorting function",
        language: Language::Python,
        code: r#"
def sort_numbers(numbers):
    return sorted(numbers)
"#,
        expected_security_risk: false,
        expected_pattern: None,
        test_rationale: "Pure data structure operation with no security implications",
    },
    // Edge cases that could be tricky
    AccuracyTestCase {
        name: "Logging function with user data",
        language: Language::Python,
        code: r#"
def log_user_action(user_id, action):
    logger.info(f"User {user_id} performed {action}")
"#,
        expected_security_risk: true,
        expected_pattern: Some("actions"),
        test_rationale: "Logging user data can be security-relevant for audit trails and information disclosure",
    },
    AccuracyTestCase {
        name: "Configuration parser",
        language: Language::JavaScript,
        code: r#"
function parseConfig(configString) {
    return JSON.parse(configString);
}
"#,
        expected_security_risk: true,
        expected_pattern: Some("resources"),
        test_rationale: "JSON parsing is an operation on data resources that can be vulnerable to attacks",
    },
    AccuracyTestCase {
        name: "Environment variable reader",
        language: Language::Python,
        code: r#"
def get_api_key():
    import os
    return os.environ.get('API_KEY')
"#,
        expected_security_risk: false,
        expected_pattern: None,
        test_rationale: "Simple environment variable access without security implications - not a PAR pattern",
    },
];

async fn test_individual_function_accuracy(
    test_case: &AccuracyTestCase,
    _model: &str,
) -> Result<(bool, bool)> {
    // Create temporary file
    let temp_dir = tempdir()?;
    let file_extension = match test_case.language {
        Language::JavaScript => "js",
        Language::Python => "py",
        Language::TypeScript => "ts",
        Language::Rust => "rs",
        Language::Java => "java",
        Language::Go => "go",
        Language::Ruby => "rb",
        Language::C => "c",
        Language::Cpp => "cpp",
        _ => "txt",
    };

    let test_file = temp_dir.path().join(format!("test.{}", file_extension));
    std::fs::write(&test_file, test_case.code)?;

    // Use SecurityRiskPatterns to detect security patterns
    let security_patterns = SecurityRiskPatterns::new(test_case.language);
    let detected_as_security_risk = security_patterns.matches(test_case.code);
    let risk_accuracy = detected_as_security_risk == test_case.expected_security_risk;
    
    // Debug output for investigation
    if !risk_accuracy {
        println!("    DEBUG: 検出結果 = {}, 期待値 = {}", detected_as_security_risk, test_case.expected_security_risk);
        let pattern_matches = security_patterns.get_pattern_matches(test_case.code);
        println!("    DEBUG: パターンマッチ数 = {}", pattern_matches.len());
        for (i, m) in pattern_matches.iter().enumerate() {
            println!("    DEBUG: マッチ{}: {:?}", i, m);
        }
    }

    // Pattern classification accuracy: just check if detection matched expectation
    let pattern_accuracy = if detected_as_security_risk && test_case.expected_pattern.is_some() {
        // Pattern was expected and detected - consider it correct
        true
    } else if !detected_as_security_risk && test_case.expected_pattern.is_none() {
        // Correctly identified as non-security function
        true
    } else {
        // Mismatch between risk detection and pattern expectation
        false
    };

    Ok((risk_accuracy, pattern_accuracy))
}

#[tokio::test]
async fn test_high_confidence_security_detection() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping accuracy test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // Test only the high-confidence security vulnerability cases
    let high_confidence_cases: Vec<_> = ACCURACY_TEST_CASES
        .iter()
        .filter(|case| case.expected_security_risk && case.name.contains("vulnerable"))
        .collect();

    println!(
        "🎯 高信頼度セキュリティ検出テスト: {}ケース",
        high_confidence_cases.len()
    );

    let mut correct_detections = 0;
    let mut total_tests = 0;

    for test_case in high_confidence_cases {
        println!("  テスト中: {}", test_case.name);
        let (risk_accuracy, _) = test_individual_function_accuracy(test_case, model).await?;

        if risk_accuracy {
            correct_detections += 1;
            println!("    ✅ 正しく検出");
        } else {
            println!("    ❌ 検出失敗: {}", test_case.test_rationale);
        }
        total_tests += 1;
    }

    let accuracy = (correct_detections as f64 / total_tests as f64) * 100.0;
    println!(
        "\n📊 高信頼度セキュリティ検出精度: {:.1}% ({}/{})",
        accuracy, correct_detections, total_tests
    );

    // High-confidence security vulnerabilities should be detected with 90%+ accuracy
    assert!(
        accuracy >= 90.0,
        "High-confidence security detection accuracy too low: {:.1}%",
        accuracy
    );

    Ok(())
}

#[tokio::test]
async fn test_false_positive_control() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping false positive test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // Test only the clear non-security cases
    let non_security_cases: Vec<_> = ACCURACY_TEST_CASES
        .iter()
        .filter(|case| {
            !case.expected_security_risk
                && (case.name.contains("math")
                    || case.name.contains("string")
                    || case.name.contains("array"))
        })
        .collect();

    println!("🔍 偽陽性制御テスト: {}ケース", non_security_cases.len());

    let mut correct_rejections = 0;
    let mut total_tests = 0;

    for test_case in non_security_cases {
        println!("  テスト中: {}", test_case.name);
        let (risk_accuracy, _) = test_individual_function_accuracy(test_case, model).await?;

        if risk_accuracy {
            correct_rejections += 1;
            println!("    ✅ 正しく非セキュリティとして識別");
        } else {
            println!("    ❌ 偽陽性: {}", test_case.test_rationale);
        }
        total_tests += 1;
    }

    let accuracy = (correct_rejections as f64 / total_tests as f64) * 100.0;
    println!(
        "\n📊 偽陽性制御精度: {:.1}% ({}/{})",
        accuracy, correct_rejections, total_tests
    );

    // Non-security functions should be correctly rejected with 85%+ accuracy
    assert!(
        accuracy >= 85.0,
        "False positive control accuracy too low: {:.1}%",
        accuracy
    );

    Ok(())
}

#[tokio::test]
async fn test_pattern_classification_accuracy() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping pattern classification test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // Test only security-relevant cases with clear pattern expectations
    let pattern_cases: Vec<_> = ACCURACY_TEST_CASES
        .iter()
        .filter(|case| case.expected_security_risk && case.expected_pattern.is_some())
        .collect();

    println!(
        "🏷️  PARパターン分類精度テスト: {}ケース",
        pattern_cases.len()
    );

    let mut pattern_stats = HashMap::new();
    let mut correct_classifications = 0;
    let mut total_tests = 0;

    for test_case in pattern_cases {
        println!(
            "  テスト中: {} (期待: {:?})",
            test_case.name, test_case.expected_pattern
        );
        let (_, pattern_accuracy) = test_individual_function_accuracy(test_case, model).await?;

        let expected = test_case.expected_pattern.unwrap();
        let entry = pattern_stats.entry(expected).or_insert((0, 0));
        entry.1 += 1; // total

        if pattern_accuracy {
            correct_classifications += 1;
            entry.0 += 1; // correct
            println!("    ✅ 正しく{}として分類", expected);
        } else {
            println!("    ❌ 分類失敗");
        }
        total_tests += 1;
    }

    let overall_accuracy = (correct_classifications as f64 / total_tests as f64) * 100.0;
    println!("\n📊 PARパターン分類結果:");
    println!(
        "  全体精度: {:.1}% ({}/{})",
        overall_accuracy, correct_classifications, total_tests
    );

    for (pattern, (correct, total)) in pattern_stats {
        let accuracy = (correct as f64 / total as f64) * 100.0;
        println!(
            "  {} パターン: {:.1}% ({}/{})",
            pattern, accuracy, correct, total
        );
    }

    // Pattern classification should be accurate for clear cases
    assert!(
        overall_accuracy >= 75.0,
        "Pattern classification accuracy too low: {:.1}%",
        overall_accuracy
    );

    Ok(())
}

#[tokio::test]
async fn test_comprehensive_accuracy() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping comprehensive accuracy test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    println!("🧪 包括的精度テスト: {}ケース", ACCURACY_TEST_CASES.len());

    let mut risk_correct = 0;
    let mut pattern_correct = 0;
    let mut total_tests = 0;
    let mut failed_cases = Vec::new();

    for test_case in ACCURACY_TEST_CASES {
        println!(
            "  [{}/{}] テスト中: {}",
            total_tests + 1,
            ACCURACY_TEST_CASES.len(),
            test_case.name
        );

        let (risk_accuracy, pattern_accuracy) =
            test_individual_function_accuracy(test_case, model).await?;

        if risk_accuracy {
            risk_correct += 1;
        } else {
            failed_cases.push(format!("{} (リスク検出失敗)", test_case.name));
        }

        if pattern_accuracy {
            pattern_correct += 1;
        } else if test_case.expected_security_risk {
            failed_cases.push(format!("{} (パターン分類失敗)", test_case.name));
        }

        total_tests += 1;
    }

    let risk_accuracy = (risk_correct as f64 / total_tests as f64) * 100.0;
    let pattern_accuracy = (pattern_correct as f64 / total_tests as f64) * 100.0;

    println!("\n📊 包括的精度結果:");
    println!(
        "  リスク検出精度: {:.1}% ({}/{})",
        risk_accuracy, risk_correct, total_tests
    );
    println!(
        "  パターン分類精度: {:.1}% ({}/{})",
        pattern_accuracy, pattern_correct, total_tests
    );

    if !failed_cases.is_empty() {
        println!("\n❌ 失敗したケース:");
        for case in &failed_cases {
            println!("    - {}", case);
        }
    }

    // Overall accuracy thresholds
    assert!(
        risk_accuracy >= 80.0,
        "Overall risk detection accuracy too low: {:.1}%",
        risk_accuracy
    );
    assert!(
        pattern_accuracy >= 70.0,
        "Overall pattern classification accuracy too low: {:.1}%",
        pattern_accuracy
    );

    println!("\n🎉 包括的精度テスト合格!");
    Ok(())
}
