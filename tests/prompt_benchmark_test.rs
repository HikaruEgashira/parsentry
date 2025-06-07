use anyhow::Result;
use parsentry::parser::CodeParser;
use parsentry::security_patterns::Language;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCase {
    name: String,
    language: Language,
    function_definition: String,
    expected_risk_level: String, // "high", "medium", "low", "none"
    expected_pattern_type: Option<String>, // "principals", "actions", "resources", null
    reasoning: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BenchmarkResults {
    total_cases: usize,
    correct_risk_assessments: usize,
    correct_pattern_classifications: usize,
    risk_accuracy: f64,
    pattern_accuracy: f64,
    false_positives: Vec<String>,
    false_negatives: Vec<String>,
    details: Vec<TestResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestResult {
    case_name: String,
    expected_risk: String,
    actual_risk: String,
    expected_pattern: Option<String>,
    actual_pattern: Option<String>,
    risk_correct: bool,
    pattern_correct: bool,
}

fn get_benchmark_test_cases() -> Vec<TestCase> {
    vec![
        // High-risk principals (user input handlers)
        TestCase {
            name: "HTTP request handler".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
function handleUserLogin(req, res) {
    const { username, password } = req.body;
    return authenticateUser(username, password);
}
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("principals".to_string()),
            reasoning: "Directly handles user input from HTTP request body".to_string(),
        },
        TestCase {
            name: "SQL query function".to_string(),
            language: Language::Python,
            function_definition: r#"
def get_user_by_id(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return db.execute(query)
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("principals".to_string()),
            reasoning: "Returns data from database, potential SQL injection vulnerability".to_string(),
        },
        
        // High-risk actions (security operations)
        TestCase {
            name: "Password validation".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
function validatePassword(password) {
    const minLength = 8;
    const hasSpecialChar = /[!@#$%^&*]/.test(password);
    return password.length >= minLength && hasSpecialChar;
}
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("actions".to_string()),
            reasoning: "Performs security validation of passwords".to_string(),
        },
        TestCase {
            name: "Input sanitization".to_string(),
            language: Language::Python,
            function_definition: r#"
def sanitize_html_input(user_input):
    import html
    return html.escape(user_input)
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("actions".to_string()),
            reasoning: "Sanitizes user input to prevent XSS".to_string(),
        },
        
        // High-risk resources (file/system access)
        TestCase {
            name: "File operation".to_string(),
            language: Language::Python,
            function_definition: r#"
def read_user_file(filename):
    with open(filename, 'r') as f:
        return f.read()
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("resources".to_string()),
            reasoning: "Directly accesses file system, potential path traversal vulnerability".to_string(),
        },
        TestCase {
            name: "Command execution".to_string(),
            language: Language::Python,
            function_definition: r#"
def execute_user_command(command):
    import subprocess
    return subprocess.run(command, shell=True, capture_output=True)
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("resources".to_string()),
            reasoning: "Executes system commands, potential command injection vulnerability".to_string(),
        },
        
        // Medium-risk functions
        TestCase {
            name: "Configuration loader".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
function loadConfiguration() {
    return {
        dbHost: process.env.DB_HOST,
        apiKey: process.env.API_KEY
    };
}
"#.to_string(),
            expected_risk_level: "medium".to_string(),
            expected_pattern_type: Some("principals".to_string()),
            reasoning: "Loads configuration that might contain sensitive data".to_string(),
        },
        TestCase {
            name: "Error logging".to_string(),
            language: Language::Python,
            function_definition: r#"
def log_error(error_message, user_context):
    logger.error(f"Error: {error_message}, User: {user_context}")
"#.to_string(),
            expected_risk_level: "medium".to_string(),
            expected_pattern_type: Some("actions".to_string()),
            reasoning: "Logs data that might contain sensitive information".to_string(),
        },
        
        // Low-risk functions
        TestCase {
            name: "String utility".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
function capitalizeString(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}
"#.to_string(),
            expected_risk_level: "low".to_string(),
            expected_pattern_type: None,
            reasoning: "Simple string manipulation with no security implications".to_string(),
        },
        TestCase {
            name: "Math calculation".to_string(),
            language: Language::Python,
            function_definition: r#"
def calculate_total(items):
    return sum(item.price for item in items)
"#.to_string(),
            expected_risk_level: "none".to_string(),
            expected_pattern_type: None,
            reasoning: "Pure mathematical calculation with no security relevance".to_string(),
        },
        
        // Edge cases
        TestCase {
            name: "Second-order data source".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
async function getUserPosts(userId) {
    const user = await db.query('SELECT * FROM users WHERE id = ?', [userId]);
    return user.posts;
}
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("principals".to_string()),
            reasoning: "Second-order data source - returns data from database that could contain untrusted user input".to_string(),
        },
        TestCase {
            name: "JWT token handler".to_string(),
            language: Language::JavaScript,
            function_definition: r#"
function verifyJWTToken(token) {
    try {
        return jwt.verify(token, process.env.JWT_SECRET);
    } catch (err) {
        return null;
    }
}
"#.to_string(),
            expected_risk_level: "high".to_string(),
            expected_pattern_type: Some("actions".to_string()),
            reasoning: "Handles authentication tokens and cryptographic verification".to_string(),
        },
    ]
}

async fn run_risk_assessment_benchmark(model: &str) -> Result<BenchmarkResults> {
    let test_cases = get_benchmark_test_cases();
    let mut results = Vec::new();
    let mut correct_risk = 0;
    let mut correct_pattern = 0;
    let mut false_positives = Vec::new();
    let mut false_negatives = Vec::new();

    println!("🧪 プロンプトベンチマークテスト開始: {}ケース", test_cases.len());

    for test_case in &test_cases {
        println!("  テスト中: {}", test_case.name);
        
        // Create a temporary file with the test function
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
        std::fs::write(&test_file, &test_case.function_definition)?;
        
        // Parse the function to get Definition
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let context = parser.build_context_from_file(&test_file)?;
        
        if let Some(definition) = context.definitions.first() {
            // Test the risk assessment filtering
            let definitions_slice = vec![definition];
            let high_risk_definitions = parsentry::pattern_generator::filter_high_risk_definitions(
                model, &definitions_slice, test_case.language
            ).await?;
            
            // Determine actual risk level based on whether function was filtered
            let actual_risk = if high_risk_definitions.is_empty() {
                "none".to_string()
            } else {
                "high".to_string() // Our filter only keeps high/medium, so assume high for simplicity
            };
            
            // Test pattern classification if function was kept
            let actual_pattern = if !high_risk_definitions.is_empty() {
                let patterns = parsentry::pattern_generator::analyze_definitions_for_security_patterns(
                    model, &high_risk_definitions, test_case.language
                ).await?;
                patterns.first().map(|p| p.pattern_type.clone()).flatten()
            } else {
                None
            };
            
            let risk_correct = match (test_case.expected_risk_level.as_str(), actual_risk.as_str()) {
                ("high", "high") | ("medium", "high") => true, // Our filter combines high/medium
                ("low", "none") | ("none", "none") => true,
                _ => false,
            };
            
            let pattern_correct = test_case.expected_pattern_type == actual_pattern;
            
            if risk_correct {
                correct_risk += 1;
            } else {
                if test_case.expected_risk_level == "none" && actual_risk == "high" {
                    false_positives.push(test_case.name.clone());
                } else if (test_case.expected_risk_level == "high" || test_case.expected_risk_level == "medium") && actual_risk == "none" {
                    false_negatives.push(test_case.name.clone());
                }
            }
            
            if pattern_correct {
                correct_pattern += 1;
            }
            
            results.push(TestResult {
                case_name: test_case.name.clone(),
                expected_risk: test_case.expected_risk_level.clone(),
                actual_risk,
                expected_pattern: test_case.expected_pattern_type.clone(),
                actual_pattern,
                risk_correct,
                pattern_correct,
            });
        }
    }

    let total_cases = test_cases.len();
    let risk_accuracy = correct_risk as f64 / total_cases as f64;
    let pattern_accuracy = correct_pattern as f64 / total_cases as f64;

    Ok(BenchmarkResults {
        total_cases,
        correct_risk_assessments: correct_risk,
        correct_pattern_classifications: correct_pattern,
        risk_accuracy,
        pattern_accuracy,
        false_positives,
        false_negatives,
        details: results,
    })
}

#[tokio::test]
async fn test_prompt_benchmark() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping benchmark test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";
    let results = run_risk_assessment_benchmark(model).await?;
    
    println!("\n📊 ベンチマーク結果:");
    println!("  総テストケース: {}", results.total_cases);
    println!("  リスク評価精度: {:.1}% ({}/{})", 
             results.risk_accuracy * 100.0, 
             results.correct_risk_assessments, 
             results.total_cases);
    println!("  パターン分類精度: {:.1}% ({}/{})", 
             results.pattern_accuracy * 100.0, 
             results.correct_pattern_classifications, 
             results.total_cases);
    
    if !results.false_positives.is_empty() {
        println!("  誤検知 (False Positives): {:?}", results.false_positives);
    }
    
    if !results.false_negatives.is_empty() {
        println!("  見逃し (False Negatives): {:?}", results.false_negatives);
    }
    
    // Detailed results for debugging
    for result in &results.details {
        if !result.risk_correct || !result.pattern_correct {
            println!("  ❌ {}: 期待[{:?}/{:?}] 実際[{}/{:?}]", 
                     result.case_name,
                     result.expected_risk,
                     result.expected_pattern,
                     result.actual_risk,
                     result.actual_pattern);
        }
    }
    
    // Assert minimum accuracy thresholds
    assert!(results.risk_accuracy >= 0.7, "Risk assessment accuracy too low: {:.1}%", results.risk_accuracy * 100.0);
    assert!(results.pattern_accuracy >= 0.6, "Pattern classification accuracy too low: {:.1}%", results.pattern_accuracy * 100.0);
    
    Ok(())
}

#[tokio::test]
async fn test_prompt_consistency() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping consistency test");
        return Ok(());
    }
    
    // Test same input multiple times to check consistency
    let test_case = TestCase {
        name: "Consistency test".to_string(),
        language: Language::JavaScript,
        function_definition: r#"
function handleUserLogin(req, res) {
    const { username, password } = req.body;
    return authenticateUser(username, password);
}
"#.to_string(),
        expected_risk_level: "high".to_string(),
        expected_pattern_type: Some("principals".to_string()),
        reasoning: "Should consistently identify as high-risk principal".to_string(),
    };
    
    let model = "gpt-4.1-mini";
    let runs = 3;
    let mut risk_results = Vec::new();
    let mut pattern_results = Vec::new();
    
    for i in 0..runs {
        println!("一貫性テスト実行 {}/{}", i + 1, runs);
        
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.js");
        std::fs::write(&test_file, &test_case.function_definition)?;
        
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let context = parser.build_context_from_file(&test_file)?;
        
        if let Some(definition) = context.definitions.first() {
            let definitions_slice = vec![definition];
            let high_risk_definitions = parsentry::pattern_generator::filter_high_risk_definitions(
                model, &definitions_slice, test_case.language
            ).await?;
            
            let risk_identified = !high_risk_definitions.is_empty();
            risk_results.push(risk_identified);
            
            if risk_identified {
                let patterns = parsentry::pattern_generator::analyze_definitions_for_security_patterns(
                    model, &high_risk_definitions, test_case.language
                ).await?;
                let pattern_type = patterns.first().map(|p| p.pattern_type.clone()).flatten();
                pattern_results.push(pattern_type);
            }
        }
    }
    
    // Check consistency
    let risk_consistency = risk_results.iter().all(|&x| x == risk_results[0]);
    let pattern_consistency = if pattern_results.len() > 1 {
        pattern_results.iter().all(|x| x == &pattern_results[0])
    } else {
        true
    };
    
    println!("リスク識別一貫性: {} (結果: {:?})", risk_consistency, risk_results);
    println!("パターン分類一貫性: {} (結果: {:?})", pattern_consistency, pattern_results);
    
    assert!(risk_consistency, "Risk assessment should be consistent across runs");
    assert!(pattern_consistency, "Pattern classification should be consistent across runs");
    
    Ok(())
}
