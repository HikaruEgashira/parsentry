use anyhow::Result;
use parsentry::parser::CodeParser;
use parsentry::security_patterns::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use tempfile::tempdir;

/// Service Level Objectives for prompt performance and accuracy
#[derive(Debug, Clone)]
pub struct PromptSLO {
    /// Minimum accuracy for high-risk security function identification (%)
    pub min_security_detection_accuracy: f64,
    /// Minimum accuracy for PAR pattern classification (%)
    pub min_pattern_classification_accuracy: f64,
    /// Maximum false positive rate for non-security functions (%)
    pub max_false_positive_rate: f64,
    /// Maximum false negative rate for security functions (%)
    pub max_false_negative_rate: f64,
    /// Maximum response time per function analysis (seconds)
    pub max_response_time_per_function: f64,
    /// Minimum consistency across multiple runs (%)
    pub min_consistency_rate: f64,
}

impl Default for PromptSLO {
    fn default() -> Self {
        Self {
            min_security_detection_accuracy: 85.0,
            min_pattern_classification_accuracy: 75.0,
            max_false_positive_rate: 15.0,
            max_false_negative_rate: 10.0,
            max_response_time_per_function: 5.0,
            min_consistency_rate: 90.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLOTestCase {
    pub name: String,
    pub language: Language,
    pub function_code: String,
    pub expected_security_relevant: bool,
    pub expected_risk_level: SecurityRiskLevel,
    pub expected_pattern_type: Option<PARPatternType>,
    pub category: TestCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityRiskLevel {
    High,
    Medium,
    Low,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PARPatternType {
    Principals,
    Actions,
    Resources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCategory {
    UserInput,           // Functions handling direct user input
    SecondOrderData,     // Functions returning data from external sources
    Authentication,      // Auth/authz related functions
    Validation,         // Input validation and sanitization
    FileSystem,         // File operations
    Database,           // Database operations
    Cryptography,       // Crypto operations
    SystemCommands,     // System command execution
    Configuration,      // Config/environment handling
    Logging,            // Logging and error handling
    Utility,            // Non-security utility functions
    BusinessLogic,      // Business logic without security implications
}

#[derive(Debug)]
pub struct SLOTestResult {
    pub test_case: SLOTestCase,
    pub actual_security_relevant: bool,
    pub actual_risk_level: SecurityRiskLevel,
    pub actual_pattern_type: Option<PARPatternType>,
    pub response_time_ms: u128,
    pub security_detection_correct: bool,
    pub pattern_classification_correct: bool,
}

#[derive(Debug)]
pub struct SLOReport {
    pub total_tests: usize,
    pub security_detection_accuracy: f64,
    pub pattern_classification_accuracy: f64,
    pub false_positive_rate: f64,
    pub false_negative_rate: f64,
    pub avg_response_time_ms: f64,
    pub consistency_rate: f64,
    pub slo_compliance: HashMap<String, bool>,
    pub failed_tests: Vec<SLOTestResult>,
}

pub fn get_slo_test_cases() -> Vec<SLOTestCase> {
    vec![
        // High-risk user input handlers
        SLOTestCase {
            name: "HTTP POST handler with body parsing".to_string(),
            language: Language::JavaScript,
            function_code: r#"
function handleLogin(req, res) {
    const { username, password } = req.body;
    if (!username || !password) {
        return res.status(400).json({ error: 'Missing credentials' });
    }
    return authenticateUser(username, password);
}
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Principals),
            category: TestCategory::UserInput,
        },
        
        SLOTestCase {
            name: "File upload handler".to_string(),
            language: Language::Python,
            function_code: r#"
def upload_file(request):
    uploaded_file = request.FILES.get('file')
    if uploaded_file:
        file_path = os.path.join(UPLOAD_DIR, uploaded_file.name)
        with open(file_path, 'wb') as f:
            f.write(uploaded_file.read())
    return file_path
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Resources),
            category: TestCategory::FileSystem,
        },
        
        // Second-order data sources
        SLOTestCase {
            name: "Database query result handler".to_string(),
            language: Language::Python,
            function_code: r#"
def get_user_profile(user_id):
    query = "SELECT * FROM user_profiles WHERE user_id = %s"
    result = db.execute(query, (user_id,))
    return result.fetchone()
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Principals),
            category: TestCategory::SecondOrderData,
        },
        
        // Authentication functions
        SLOTestCase {
            name: "JWT token verification".to_string(),
            language: Language::JavaScript,
            function_code: r#"
function verifyToken(token) {
    try {
        const decoded = jwt.verify(token, process.env.JWT_SECRET);
        return { valid: true, user: decoded };
    } catch (error) {
        return { valid: false, error: error.message };
    }
}
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Actions),
            category: TestCategory::Authentication,
        },
        
        // Validation functions
        SLOTestCase {
            name: "Input sanitization".to_string(),
            language: Language::Python,
            function_code: r#"
def sanitize_html(user_input):
    import html
    import re
    # Remove script tags
    clean_input = re.sub(r'<script.*?</script>', '', user_input, flags=re.IGNORECASE | re.DOTALL)
    # Escape HTML entities
    return html.escape(clean_input)
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Actions),
            category: TestCategory::Validation,
        },
        
        // System command execution
        SLOTestCase {
            name: "System command executor".to_string(),
            language: Language::Python,
            function_code: r#"
def execute_command(command, args):
    import subprocess
    full_command = [command] + args
    result = subprocess.run(full_command, capture_output=True, text=True)
    return result.stdout
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::High,
            expected_pattern_type: Some(PARPatternType::Resources),
            category: TestCategory::SystemCommands,
        },
        
        // Medium-risk configuration handlers
        SLOTestCase {
            name: "Environment variable loader".to_string(),
            language: Language::JavaScript,
            function_code: r#"
function loadConfig() {
    return {
        database: {
            host: process.env.DB_HOST || 'localhost',
            password: process.env.DB_PASSWORD,
        },
        api: {
            key: process.env.API_KEY,
            secret: process.env.API_SECRET,
        }
    };
}
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::Medium,
            expected_pattern_type: Some(PARPatternType::Principals),
            category: TestCategory::Configuration,
        },
        
        // Low-risk utility functions
        SLOTestCase {
            name: "String formatting utility".to_string(),
            language: Language::JavaScript,
            function_code: r#"
function formatUserName(firstName, lastName) {
    return `${firstName.trim()} ${lastName.trim()}`;
}
"#.to_string(),
            expected_security_relevant: false,
            expected_risk_level: SecurityRiskLevel::Low,
            expected_pattern_type: None,
            category: TestCategory::Utility,
        },
        
        // No-risk business logic
        SLOTestCase {
            name: "Price calculation".to_string(),
            language: Language::Python,
            function_code: r#"
def calculate_total_price(items, tax_rate=0.08):
    subtotal = sum(item.price * item.quantity for item in items)
    tax = subtotal * tax_rate
    return subtotal + tax
"#.to_string(),
            expected_security_relevant: false,
            expected_risk_level: SecurityRiskLevel::None,
            expected_pattern_type: None,
            category: TestCategory::BusinessLogic,
        },
        
        // Edge case: Logging with potential sensitive data
        SLOTestCase {
            name: "Error logger with user context".to_string(),
            language: Language::Python,
            function_code: r#"
def log_authentication_error(username, error_details, ip_address):
    logger.error(f"Auth failed for {username} from {ip_address}: {error_details}")
    metrics.increment('auth.failed', tags={'username': username})
"#.to_string(),
            expected_security_relevant: true,
            expected_risk_level: SecurityRiskLevel::Medium,
            expected_pattern_type: Some(PARPatternType::Actions),
            category: TestCategory::Logging,
        },
        
        // False positive test case - looks security-related but isn't
        SLOTestCase {
            name: "Password strength indicator (UI only)".to_string(),
            language: Language::JavaScript,
            function_code: r#"
function getPasswordStrengthLabel(score) {
    const labels = ['Weak', 'Fair', 'Good', 'Strong', 'Very Strong'];
    return labels[Math.min(score, 4)] || 'Weak';
}
"#.to_string(),
            expected_security_relevant: false,
            expected_risk_level: SecurityRiskLevel::None,
            expected_pattern_type: None,
            category: TestCategory::Utility,
        },
    ]
}

pub async fn run_slo_compliance_test(model: &str, slo: &PromptSLO) -> Result<SLOReport> {
    let test_cases = get_slo_test_cases();
    let mut test_results = Vec::new();
    let mut total_response_time = 0u128;
    
    println!("🎯 SLOコンプライアンステスト開始: {}ケース", test_cases.len());
    println!("📋 SLO基準:");
    println!("   セキュリティ検出精度: ≥{:.1}%", slo.min_security_detection_accuracy);
    println!("   パターン分類精度: ≥{:.1}%", slo.min_pattern_classification_accuracy);
    println!("   偽陽性率: ≤{:.1}%", slo.max_false_positive_rate);
    println!("   偽陰性率: ≤{:.1}%", slo.max_false_negative_rate);
    println!("   最大応答時間: ≤{:.1}秒/関数", slo.max_response_time_per_function);
    
    for (i, test_case) in test_cases.iter().enumerate() {
        println!("  [{}/{}] テスト中: {} ({})", 
                 i + 1, test_cases.len(), test_case.name, format!("{:?}", test_case.category));
        
        let start_time = Instant::now();
        
        // Create temporary file for testing
        let temp_dir = tempdir()?;
        let file_extension = match test_case.language {
            Language::JavaScript => "js",
            Language::Python => "py",
            _ => "txt",
        };
        let test_file = temp_dir.path().join(format!("test.{}", file_extension));
        std::fs::write(&test_file, &test_case.function_code)?;
        
        // Parse and analyze
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let context = parser.build_context_from_file(&test_file)?;
        
        let response_time = start_time.elapsed().as_millis();
        total_response_time += response_time;
        
        if let Some(definition) = context.definitions.first() {
            // Test risk assessment
            let definitions_slice = vec![definition];
            // TODO: Replace with actual risk assessment logic when filter_high_risk_definitions is available
            let high_risk_definitions = definitions_slice; // Placeholder: assume all definitions are high risk
            
            let actual_security_relevant = !high_risk_definitions.is_empty();
            let actual_risk_level = if high_risk_definitions.is_empty() {
                SecurityRiskLevel::None
            } else {
                SecurityRiskLevel::High // Simplified for testing
            };
            
            // Test pattern classification
            let actual_pattern_type = if !high_risk_definitions.is_empty() {
                let patterns = parsentry::pattern_generator::analyze_definitions_for_security_patterns(
                    model, &high_risk_definitions, test_case.language, None
                ).await?;
                
                patterns.first().and_then(|p| {
                    p.pattern_type.as_ref().and_then(|pt| match pt.as_str() {
                        "principals" => Some(PARPatternType::Principals),
                        "actions" => Some(PARPatternType::Actions),
                        "resources" => Some(PARPatternType::Resources),
                        _ => None,
                    })
                })
            } else {
                None
            };
            
            let security_detection_correct = test_case.expected_security_relevant == actual_security_relevant;
            let pattern_classification_correct = test_case.expected_pattern_type == actual_pattern_type;
            
            test_results.push(SLOTestResult {
                test_case: test_case.clone(),
                actual_security_relevant,
                actual_risk_level,
                actual_pattern_type,
                response_time_ms: response_time,
                security_detection_correct,
                pattern_classification_correct,
            });
        }
    }
    
    // Calculate metrics
    let total_tests = test_results.len();
    let security_detection_correct = test_results.iter().filter(|r| r.security_detection_correct).count();
    let pattern_classification_correct = test_results.iter().filter(|r| r.pattern_classification_correct).count();
    
    let false_positives = test_results.iter()
        .filter(|r| !r.test_case.expected_security_relevant && r.actual_security_relevant)
        .count();
    let false_negatives = test_results.iter()
        .filter(|r| r.test_case.expected_security_relevant && !r.actual_security_relevant)
        .count();
    
    let non_security_cases = test_results.iter()
        .filter(|r| !r.test_case.expected_security_relevant)
        .count();
    let security_cases = test_results.iter()
        .filter(|r| r.test_case.expected_security_relevant)
        .count();
    
    let security_detection_accuracy = (security_detection_correct as f64 / total_tests as f64) * 100.0;
    let pattern_classification_accuracy = (pattern_classification_correct as f64 / total_tests as f64) * 100.0;
    let false_positive_rate = if non_security_cases > 0 {
        (false_positives as f64 / non_security_cases as f64) * 100.0
    } else { 0.0 };
    let false_negative_rate = if security_cases > 0 {
        (false_negatives as f64 / security_cases as f64) * 100.0
    } else { 0.0 };
    let avg_response_time_ms = total_response_time as f64 / total_tests as f64;
    
    // Check SLO compliance
    let mut slo_compliance = HashMap::new();
    slo_compliance.insert("security_detection_accuracy".to_string(), 
                         security_detection_accuracy >= slo.min_security_detection_accuracy);
    slo_compliance.insert("pattern_classification_accuracy".to_string(), 
                         pattern_classification_accuracy >= slo.min_pattern_classification_accuracy);
    slo_compliance.insert("false_positive_rate".to_string(), 
                         false_positive_rate <= slo.max_false_positive_rate);
    slo_compliance.insert("false_negative_rate".to_string(), 
                         false_negative_rate <= slo.max_false_negative_rate);
    slo_compliance.insert("response_time".to_string(), 
                         (avg_response_time_ms / 1000.0) <= slo.max_response_time_per_function);
    
    let failed_tests = test_results.into_iter()
        .filter(|r| !r.security_detection_correct || !r.pattern_classification_correct)
        .collect();
    
    Ok(SLOReport {
        total_tests,
        security_detection_accuracy,
        pattern_classification_accuracy,
        false_positive_rate,
        false_negative_rate,
        avg_response_time_ms,
        consistency_rate: 100.0, // TODO: Implement consistency testing
        slo_compliance,
        failed_tests,
    })
}

#[tokio::test]
async fn test_prompt_slo_compliance() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping SLO test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";
    let slo = PromptSLO::default();
    let report = run_slo_compliance_test(model, &slo).await?;
    
    println!("\n📊 SLOコンプライアンス結果:");
    println!("==================================");
    println!("セキュリティ検出精度: {:.1}% (基準: ≥{:.1}%) {}", 
             report.security_detection_accuracy, 
             slo.min_security_detection_accuracy,
             if report.slo_compliance["security_detection_accuracy"] { "✅" } else { "❌" });
             
    println!("パターン分類精度: {:.1}% (基準: ≥{:.1}%) {}", 
             report.pattern_classification_accuracy, 
             slo.min_pattern_classification_accuracy,
             if report.slo_compliance["pattern_classification_accuracy"] { "✅" } else { "❌" });
             
    println!("偽陽性率: {:.1}% (基準: ≤{:.1}%) {}", 
             report.false_positive_rate, 
             slo.max_false_positive_rate,
             if report.slo_compliance["false_positive_rate"] { "✅" } else { "❌" });
             
    println!("偽陰性率: {:.1}% (基準: ≤{:.1}%) {}", 
             report.false_negative_rate, 
             slo.max_false_negative_rate,
             if report.slo_compliance["false_negative_rate"] { "✅" } else { "❌" });
             
    println!("平均応答時間: {:.1}ms (基準: ≤{:.1}ms) {}", 
             report.avg_response_time_ms, 
             slo.max_response_time_per_function * 1000.0,
             if report.slo_compliance["response_time"] { "✅" } else { "❌" });
    
    if !report.failed_tests.is_empty() {
        println!("\n❌ 失敗したテストケース:");
        for test in &report.failed_tests {
            println!("  - {}: セキュリティ検出[{}] パターン分類[{}]",
                     test.test_case.name,
                     if test.security_detection_correct { "✅" } else { "❌" },
                     if test.pattern_classification_correct { "✅" } else { "❌" });
        }
    }
    
    // Assert SLO compliance
    let all_slos_met = report.slo_compliance.values().all(|&compliant| compliant);
    
    if !all_slos_met {
        let failed_slos: Vec<String> = report.slo_compliance.iter()
            .filter(|&(_, &compliant)| !compliant)
            .map(|(metric, _)| metric.clone())
            .collect();
        panic!("SLO violations detected in: {:?}", failed_slos);
    }
    
    println!("\n🎉 全てのSLO基準を満たしています!");
    Ok(())
}

#[tokio::test] 
async fn test_prompt_consistency_slo() -> Result<()> {
    // Skip if no API key is available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping consistency SLO test");
        return Ok(());
    }
    
    let slo = PromptSLO::default();
    let model = "gpt-4.1-mini";
    let runs = 3;
    
    // Select a representative test case
    let test_case = SLOTestCase {
        name: "Consistency test case".to_string(),
        language: Language::JavaScript,
        function_code: r#"
function authenticateUser(username, password) {
    const hashedPassword = crypto.createHash('sha256').update(password).digest('hex');
    return database.query('SELECT * FROM users WHERE username = ? AND password = ?', 
                         [username, hashedPassword]);
}
"#.to_string(),
        expected_security_relevant: true,
        expected_risk_level: SecurityRiskLevel::High,
        expected_pattern_type: Some(PARPatternType::Actions),
        category: TestCategory::Authentication,
    };
    
    let mut results = Vec::new();
    
    for i in 0..runs {
        println!("一貫性テスト実行 {}/{}", i + 1, runs);
        
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.js");
        std::fs::write(&test_file, &test_case.function_code)?;
        
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let context = parser.build_context_from_file(&test_file)?;
        
        if let Some(definition) = context.definitions.first() {
            let definitions_slice = vec![definition];
            // TODO: Replace with actual risk assessment logic when filter_high_risk_definitions is available
            let high_risk_definitions = definitions_slice; // Placeholder: assume all definitions are high risk
            
            let security_relevant = !high_risk_definitions.is_empty();
            
            let pattern_type = if !high_risk_definitions.is_empty() {
                let patterns = parsentry::pattern_generator::analyze_definitions_for_security_patterns(
                    model, &high_risk_definitions, test_case.language, None
                ).await?;
                patterns.first().and_then(|p| p.pattern_type.clone())
            } else {
                None
            };
            
            results.push((security_relevant, pattern_type));
        }
    }
    
    // Check consistency
    let security_consistent = results.iter().all(|(sec, _)| *sec == results[0].0);
    let pattern_consistent = results.iter().all(|(_, pat)| *pat == results[0].1);
    
    let consistency_rate = if security_consistent && pattern_consistent {
        100.0
    } else if security_consistent || pattern_consistent {
        50.0
    } else {
        0.0
    };
    
    println!("一貫性率: {:.1}% (基準: ≥{:.1}%)", consistency_rate, slo.min_consistency_rate);
    
    assert!(consistency_rate >= slo.min_consistency_rate, 
           "Consistency rate {:.1}% below SLO threshold {:.1}%", 
           consistency_rate, slo.min_consistency_rate);
    
    println!("✅ 一貫性SLOを満たしています");
    Ok(())
}