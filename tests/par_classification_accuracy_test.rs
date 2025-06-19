use anyhow::Result;
use parsentry::parser::CodeParser;
use parsentry::security_patterns::Language;
use std::collections::HashMap;
use tempfile::tempdir;

/// PAR分類精度テスト
/// このテストは、Principal-Action-Resource分類の正確性を測定し、
/// セキュリティ解析の根幹となるPAR分類の精度を向上させることを目的とする

#[derive(Debug, Clone)]
struct PARTestCase {
    name: &'static str,
    language: Language,
    code: &'static str,
    expected_par_type: Option<PARType>,
    confidence_level: ConfidenceLevel,
    test_rationale: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
enum PARType {
    Principal,   // データソース・実行主体
    Action,      // データ処理・セキュリティ制御  
    Resource,    // データの最終出力先・危険な操作対象
}

#[derive(Debug, Clone)]
enum ConfidenceLevel {
    High,      // 明確に分類できる
    Medium,    // コンテキスト依存
}

fn get_par_classification_test_cases() -> Vec<PARTestCase> {
    vec![
    // === PRINCIPAL分類テストケース ===
    // ユーザー入力ソース
    PARTestCase {
        name: "HTTP request parameter access",
        language: Language::Python,
        code: r#"
def get_user_input():
    return request.args.get('user_id')
"#,
        expected_par_type: Some(PARType::Principal),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "HTTP requestからのパラメータ取得 - 典型的なPrincipal",
    },
    PARTestCase {
        name: "Environment variable access",
        language: Language::Python,
        code: r#"
def get_database_url():
    import os
    return os.environ.get('DATABASE_URL')
"#,
        expected_par_type: Some(PARType::Principal),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "環境変数アクセス - 外部設定ソースのPrincipal",
    },
    PARTestCase {
        name: "File content reading",
        language: Language::JavaScript,
        code: r#"
function readConfigFile(filename) {
    const fs = require('fs');
    return fs.readFileSync(filename, 'utf8');
}
"#,
        expected_par_type: Some(PARType::Principal),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "ファイル読み取り - データソースとしてのPrincipal",
    },
    PARTestCase {
        name: "Database query result",
        language: Language::Python,
        code: r#"
def fetch_user_data(user_id):
    query = "SELECT * FROM users WHERE id = %s"
    return db.execute(query, (user_id,))
"#,
        expected_par_type: Some(PARType::Principal),
        confidence_level: ConfidenceLevel::Medium,
        test_rationale: "DB結果取得 - データソースだが、中間処理要素でもある",
    },

    // === ACTION分類テストケース ===
    // データ検証・サニタイゼーション
    PARTestCase {
        name: "Input validation function",
        language: Language::JavaScript,
        code: r#"
function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "入力検証 - 典型的なセキュリティAction",
    },
    PARTestCase {
        name: "Data sanitization",
        language: Language::Python,
        code: r#"
def sanitize_html(user_input):
    import html
    return html.escape(user_input)
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "HTMLエスケープ - データサニタイゼーションAction",
    },
    PARTestCase {
        name: "Password hashing",
        language: Language::Python,
        code: r#"
def hash_password(password):
    import hashlib
    return hashlib.pbkdf2_hmac('sha256', password.encode(), b'salt', 100000)
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "パスワードハッシュ化 - 暗号化Action",
    },
    PARTestCase {
        name: "Authentication check",
        language: Language::JavaScript,
        code: r#"
function authenticateUser(username, password) {
    return users.find(u => u.username === username && u.password === password);
}
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "認証チェック - セキュリティ制御Action",
    },

    // === RESOURCE分類テストケース ===
    // システムリソース・危険な操作
    PARTestCase {
        name: "Command execution",
        language: Language::Python,
        code: r#"
def execute_command(cmd):
    import subprocess
    return subprocess.run(cmd, shell=True, capture_output=True)
"#,
        expected_par_type: Some(PARType::Resource),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "コマンド実行 - 典型的な危険なResource操作",
    },
    PARTestCase {
        name: "File write operation",
        language: Language::JavaScript,
        code: r#"
function writeToFile(filename, data) {
    const fs = require('fs');
    fs.writeFileSync(filename, data);
}
"#,
        expected_par_type: Some(PARType::Resource),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "ファイル書き込み - ファイルシステムResource操作",
    },
    PARTestCase {
        name: "Database modification",
        language: Language::Python,
        code: r#"
def update_user(user_id, data):
    query = f"UPDATE users SET name='{data}' WHERE id={user_id}"
    return db.execute(query)
"#,
        expected_par_type: Some(PARType::Resource),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "DB更新 - データベースResource操作",
    },
    PARTestCase {
        name: "Dynamic code execution",
        language: Language::Python,
        code: r#"
def execute_dynamic_code(code_string):
    return eval(code_string)
"#,
        expected_par_type: Some(PARType::Resource),
        confidence_level: ConfidenceLevel::High,
        test_rationale: "動的コード実行 - 最も危険なResource操作",
    },

    // === 境界ケース・難しい分類 ===
    PARTestCase {
        name: "Configuration parser (boundary case)",
        language: Language::JavaScript,
        code: r#"
function parseConfig(configString) {
    return JSON.parse(configString);
}
"#,
        expected_par_type: Some(PARType::Principal),
        confidence_level: ConfidenceLevel::Medium,
        test_rationale: "設定解析 - Principal(データソース)とAction(解析処理)の境界",
    },
    PARTestCase {
        name: "Logging function (boundary case)",
        language: Language::Python,
        code: r#"
def log_user_action(user_id, action):
    logger.info(f"User {user_id} performed {action}")
"#,
        expected_par_type: Some(PARType::Resource),
        confidence_level: ConfidenceLevel::Medium,
        test_rationale: "ログ出力 - 情報漏洩の観点でResource、監査の観点でAction",
    },
    PARTestCase {
        name: "Data transformation (boundary case)",
        language: Language::JavaScript,
        code: r#"
function transformUserData(userData) {
    return {
        id: userData.id,
        name: userData.firstName + ' ' + userData.lastName,
        email: userData.email.toLowerCase()
    };
}
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::Medium,
        test_rationale: "データ変換 - ActionとPrincipalの境界、処理の観点でAction",
    },

    // === 非セキュリティ関数（PARに分類されないもの） ===
    PARTestCase {
        name: "Pure math function",
        language: Language::Python,
        code: r#"
def calculate_area(width, height):
    return width * height
"#,
        expected_par_type: None,
        confidence_level: ConfidenceLevel::High,
        test_rationale: "純粋な数学計算 - セキュリティに関連しない",
    },
    PARTestCase {
        name: "String utility function",
        language: Language::JavaScript,
        code: r#"
function formatCurrency(amount) {
    return `$${amount.toFixed(2)}`;
}
"#,
        expected_par_type: None,
        confidence_level: ConfidenceLevel::High,
        test_rationale: "文字列フォーマット - セキュリティに関連しない",
    },

    // === 複合的なケース ===
    PARTestCase {
        name: "Combined Principal and Action",
        language: Language::Python,
        code: r#"
def process_user_input(user_data):
    # Principal: user_data 取得
    raw_input = user_data.get('input')
    # Action: 検証とサニタイゼーション
    if not raw_input or len(raw_input) > 1000:
        return None
    return html.escape(raw_input)
"#,
        expected_par_type: Some(PARType::Action),
        confidence_level: ConfidenceLevel::Medium,
        test_rationale: "複合機能 - PrincipalとActionの両方を含むが、主要機能は検証/サニタイゼーション",
    },
    ]
}

async fn test_par_classification_accuracy(
    test_case: &PARTestCase,
    model: &str,
) -> Result<(bool, Option<PARType>)> {
    // 一時ファイル作成
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

    // パーサーでコンテキスト構築
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let context = parser.build_context_from_file(&test_file)?;

    if let Some(definition) = context.definitions.first() {
        // パターン解析でPAR分類を実行
        let definitions_slice = vec![definition];
        let patterns = parsentry::pattern_generator::analyze_definitions_for_security_patterns(
            model,
            &definitions_slice,
            test_case.language,
            None,
        )
        .await?;

        let detected_par_type = if let Some(pattern) = patterns.first() {
            pattern.pattern_type.as_ref().and_then(|pt| match pt.as_str() {
                "principals" => Some(PARType::Principal),
                "actions" => Some(PARType::Action),
                "resources" => Some(PARType::Resource),
                _ => None,
            })
        } else {
            None
        };

        let classification_correct = test_case.expected_par_type == detected_par_type;
        
        Ok((classification_correct, detected_par_type))
    } else {
        // 関数定義が見つからない場合
        Ok((test_case.expected_par_type.is_none(), None))
    }
}

#[tokio::test]
async fn test_par_classification_accuracy_high_confidence() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping PAR classification test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let test_cases = get_par_classification_test_cases();
    // 高信頼度ケースのみテスト
    let high_confidence_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| matches!(case.confidence_level, ConfidenceLevel::High))
        .collect();

    println!("🎯 PAR分類精度テスト (高信頼度): {}ケース", high_confidence_cases.len());

    let mut correct_classifications = 0;
    let mut total_tests = 0;
    let mut classification_stats = HashMap::new();

    for test_case in high_confidence_cases {
        println!("  テスト中: {} ({:?})", test_case.name, test_case.expected_par_type);
        
        let (classification_correct, detected_type) = 
            test_par_classification_accuracy(test_case, model).await?;

        // 統計更新
        let expected_key = format!("{:?}", test_case.expected_par_type);
        let entry = classification_stats.entry(expected_key.clone()).or_insert((0, 0));
        entry.1 += 1; // total

        if classification_correct {
            correct_classifications += 1;
            entry.0 += 1; // correct
            println!("    ✅ 正しく分類: {:?}", detected_type);
        } else {
            println!("    ❌ 分類失敗: 期待={:?}, 実際={:?}", 
                    test_case.expected_par_type, detected_type);
            println!("       理由: {}", test_case.test_rationale);
        }
        total_tests += 1;
    }

    let overall_accuracy = (correct_classifications as f64 / total_tests as f64) * 100.0;
    
    println!("\n📊 PAR分類精度結果 (高信頼度):");
    println!("  全体精度: {:.1}% ({}/{})", overall_accuracy, correct_classifications, total_tests);
    
    for (par_type, (correct, total)) in classification_stats {
        let accuracy = (correct as f64 / total as f64) * 100.0;
        println!("  {} 精度: {:.1}% ({}/{})", par_type, accuracy, correct, total);
    }

    // 高信頼度ケースは90%以上の精度を要求
    assert!(
        overall_accuracy >= 90.0,
        "PAR分類精度が基準を下回っています: {:.1}% (要求: 90.0%)",
        overall_accuracy
    );

    println!("\n🎉 PAR分類精度テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_par_boundary_cases() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping PAR boundary test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let test_cases = get_par_classification_test_cases();
    // 境界ケースのみテスト
    let boundary_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| matches!(case.confidence_level, ConfidenceLevel::Medium))
        .collect();

    println!("🔍 PAR境界ケーステスト: {}ケース", boundary_cases.len());

    let mut correct_classifications = 0;
    let mut total_tests = 0;

    for test_case in boundary_cases {
        println!("  境界ケース: {}", test_case.name);
        
        let (classification_correct, detected_type) = 
            test_par_classification_accuracy(test_case, model).await?;

        if classification_correct {
            correct_classifications += 1;
            println!("    ✅ 正しく分類: {:?}", detected_type);
        } else {
            println!("    ⚠️  分類相違: 期待={:?}, 実際={:?}", 
                    test_case.expected_par_type, detected_type);
            println!("       境界理由: {}", test_case.test_rationale);
        }
        total_tests += 1;
    }

    let boundary_accuracy = (correct_classifications as f64 / total_tests as f64) * 100.0;
    
    println!("\n📊 PAR境界ケース結果:");
    println!("  境界精度: {:.1}% ({}/{})", boundary_accuracy, correct_classifications, total_tests);

    // 境界ケースは70%以上の精度があれば合格
    assert!(
        boundary_accuracy >= 70.0,
        "PAR境界ケース精度が基準を下回っています: {:.1}% (要求: 70.0%)",
        boundary_accuracy
    );

    println!("✅ PAR境界ケーステスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_non_security_function_rejection() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping non-security rejection test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let test_cases = get_par_classification_test_cases();
    // 非セキュリティ関数のみテスト
    let non_security_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| case.expected_par_type.is_none())
        .collect();

    println!("🚫 非セキュリティ関数拒否テスト: {}ケース", non_security_cases.len());

    let mut correct_rejections = 0;
    let mut total_tests = 0;

    for test_case in non_security_cases {
        println!("  テスト中: {}", test_case.name);
        
        let (classification_correct, detected_type) = 
            test_par_classification_accuracy(test_case, model).await?;

        if classification_correct {
            correct_rejections += 1;
            println!("    ✅ 正しく非セキュリティとして識別");
        } else {
            println!("    ❌ 偽陽性: 非セキュリティ関数を{:?}として誤分類", detected_type);
        }
        total_tests += 1;
    }

    let rejection_accuracy = (correct_rejections as f64 / total_tests as f64) * 100.0;
    
    println!("\n📊 非セキュリティ関数拒否結果:");
    println!("  拒否精度: {:.1}% ({}/{})", rejection_accuracy, correct_rejections, total_tests);

    // 非セキュリティ関数は85%以上の精度で拒否されるべき
    assert!(
        rejection_accuracy >= 85.0,
        "非セキュリティ関数拒否精度が基準を下回っています: {:.1}% (要求: 85.0%)",
        rejection_accuracy
    );

    println!("✅ 非セキュリティ関数拒否テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_par_comprehensive_accuracy() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping comprehensive PAR test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let test_cases = get_par_classification_test_cases();
    println!("🧪 PAR分類包括的精度テスト: {}ケース", test_cases.len());

    let mut correct_classifications = 0;
    let mut total_tests = 0;
    let mut failed_cases = Vec::new();
    let mut par_type_stats = HashMap::new();

    for test_case in &test_cases {
        println!(
            "  [{}/{}] テスト中: {}",
            total_tests + 1,
            test_cases.len(),
            test_case.name
        );

        let (classification_correct, detected_type) = 
            test_par_classification_accuracy(test_case, model).await?;

        // 統計更新
        let expected_key = format!("{:?}", test_case.expected_par_type);
        let entry = par_type_stats.entry(expected_key).or_insert((0, 0));
        entry.1 += 1;

        if classification_correct {
            correct_classifications += 1;
            entry.0 += 1;
        } else {
            failed_cases.push(format!(
                "{}: 期待={:?}, 実際={:?} (信頼度={:?})",
                test_case.name, test_case.expected_par_type, detected_type, test_case.confidence_level
            ));
        }

        total_tests += 1;
    }

    let overall_accuracy = (correct_classifications as f64 / total_tests as f64) * 100.0;

    println!("\n📊 PAR分類包括的結果:");
    println!("  全体精度: {:.1}% ({}/{})", overall_accuracy, correct_classifications, total_tests);

    println!("\nPARタイプ別精度:");
    for (par_type, (correct, total)) in par_type_stats {
        let accuracy = (correct as f64 / total as f64) * 100.0;
        println!("  {}: {:.1}% ({}/{})", par_type, accuracy, correct, total);
    }

    if !failed_cases.is_empty() {
        println!("\n❌ 失敗したケース:");
        for case in &failed_cases {
            println!("    - {}", case);
        }
    }

    // 全体精度は80%以上を要求
    assert!(
        overall_accuracy >= 80.0,
        "PAR分類全体精度が基準を下回っています: {:.1}% (要求: 80.0%)",
        overall_accuracy
    );

    println!("\n🎉 PAR分類包括的テスト合格!");
    Ok(())
}