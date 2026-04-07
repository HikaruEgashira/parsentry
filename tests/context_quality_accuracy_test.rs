use anyhow::Result;
use parsentry::parser::CodeParser;
use parsentry::security_patterns::Language;
use std::collections::HashMap;
use tempfile::tempdir;

/// コンテキスト品質精度テスト
/// Tree-sitter解析の精度とデータフロー追跡の品質を測定
/// コンテキスト構築の品質が脆弱性検出精度に直接影響するため重要

#[derive(Debug, Clone)]
struct ContextQualityTestCase {
    name: &'static str,
    language: Language,
    code: &'static str,
    expected_definitions: Vec<ExpectedDefinition>,
    expected_references: Vec<ExpectedReference>,
    data_flow_expectations: Vec<DataFlowExpectation>,
    test_rationale: &'static str,
}

#[derive(Debug, Clone)]
struct ExpectedDefinition {
    name: &'static str,
    should_be_found: bool,
    minimum_source_length: usize,
}

#[derive(Debug, Clone)]
struct ExpectedReference {
    name: &'static str,
    expected_count: usize,
    tolerance: usize, // 許容誤差
}

#[derive(Debug, Clone)]
struct DataFlowExpectation {
    from_function: &'static str,
    to_function: &'static str,
    flow_type: FlowType,
}

#[derive(Debug, Clone)]
enum FlowType {
    DirectCall,  // 直接関数呼び出し
    DataPassing, // データの受け渡し
    Reference,   // 参照関係
}

fn get_context_quality_test_cases() -> Vec<ContextQualityTestCase> {
    vec![
        // === Python テストケース ===
        ContextQualityTestCase {
            name: "Python basic function definitions",
            language: Language::Python,
            code: r#"
import os
import subprocess

def get_user_input():
    return input("Enter value: ")

def validate_input(user_data):
    if len(user_data) > 100:
        return False
    return True

def process_command(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True)
    return result.stdout

def main():
    user_input = get_user_input()
    if validate_input(user_input):
        output = process_command(user_input)
        print(output)
"#,
            expected_definitions: vec![
                ExpectedDefinition {
                    name: "get_user_input",
                    should_be_found: true,
                    minimum_source_length: 20,
                },
                ExpectedDefinition {
                    name: "validate_input",
                    should_be_found: true,
                    minimum_source_length: 30,
                },
                ExpectedDefinition {
                    name: "process_command",
                    should_be_found: true,
                    minimum_source_length: 40,
                },
                ExpectedDefinition {
                    name: "main",
                    should_be_found: true,
                    minimum_source_length: 50,
                },
            ],
            expected_references: vec![
                ExpectedReference {
                    name: "get_user_input",
                    expected_count: 1, // main関数で1回呼び出し
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "validate_input",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "process_command",
                    expected_count: 1,
                    tolerance: 0,
                },
            ],
            data_flow_expectations: vec![
                DataFlowExpectation {
                    from_function: "get_user_input",
                    to_function: "validate_input",
                    flow_type: FlowType::DataPassing,
                },
                DataFlowExpectation {
                    from_function: "get_user_input",
                    to_function: "process_command",
                    flow_type: FlowType::DataPassing,
                },
                DataFlowExpectation {
                    from_function: "validate_input",
                    to_function: "process_command",
                    flow_type: FlowType::DirectCall,
                },
            ],
            test_rationale: "基本的なPython関数定義と呼び出しの解析精度",
        },
        // === JavaScript テストケース ===
        ContextQualityTestCase {
            name: "JavaScript function expressions and arrow functions",
            language: Language::JavaScript,
            code: r#"
const express = require('express');
const app = express();

// 通常の関数宣言
function authenticateUser(username, password) {
    return users.find(u => u.username === username && u.password === password);
}

// 関数式
const validateEmail = function(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
};

// アロー関数
const sanitizeInput = (input) => {
    return input.replace(/[<>]/g, '');
};

// ルートハンドラー
app.post('/login', (req, res) => {
    const { username, password } = req.body;
    
    if (!validateEmail(username)) {
        return res.status(400).json({ error: 'Invalid email' });
    }
    
    const sanitizedPassword = sanitizeInput(password);
    const user = authenticateUser(username, sanitizedPassword);
    
    if (user) {
        res.json({ success: true, user: user });
    } else {
        res.status(401).json({ error: 'Authentication failed' });
    }
});
"#,
            expected_definitions: vec![
                ExpectedDefinition {
                    name: "authenticateUser",
                    should_be_found: true,
                    minimum_source_length: 30,
                },
                ExpectedDefinition {
                    name: "validateEmail",
                    should_be_found: true,
                    minimum_source_length: 25,
                },
                ExpectedDefinition {
                    name: "sanitizeInput",
                    should_be_found: true,
                    minimum_source_length: 20,
                },
            ],
            expected_references: vec![
                ExpectedReference {
                    name: "validateEmail",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "sanitizeInput",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "authenticateUser",
                    expected_count: 1,
                    tolerance: 0,
                },
            ],
            data_flow_expectations: vec![DataFlowExpectation {
                from_function: "validateEmail",
                to_function: "authenticateUser",
                flow_type: FlowType::DataPassing,
            }],
            test_rationale: "JavaScript関数式、アロー関数、ルートハンドラーの解析精度",
        },
        // === Rust テストケース ===
        ContextQualityTestCase {
            name: "Rust function definitions with error handling",
            language: Language::Rust,
            code: r#"
use std::fs;
use std::process::Command;

fn read_config_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn parse_config(content: &str) -> serde_json::Value {
    serde_json::from_str(content).unwrap_or_default()
}

fn execute_shell_command(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn process_user_request(config_path: &str, user_command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config_content = read_config_file(config_path)?;
    let config = parse_config(&config_content);
    
    if config["allow_commands"].as_bool().unwrap_or(false) {
        execute_shell_command(user_command)
    } else {
        Err("Commands not allowed".into())
    }
}
"#,
            expected_definitions: vec![
                ExpectedDefinition {
                    name: "read_config_file",
                    should_be_found: true,
                    minimum_source_length: 25,
                },
                ExpectedDefinition {
                    name: "parse_config",
                    should_be_found: true,
                    minimum_source_length: 30,
                },
                ExpectedDefinition {
                    name: "execute_shell_command",
                    should_be_found: true,
                    minimum_source_length: 50,
                },
                ExpectedDefinition {
                    name: "process_user_request",
                    should_be_found: true,
                    minimum_source_length: 80,
                },
            ],
            expected_references: vec![
                ExpectedReference {
                    name: "read_config_file",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "parse_config",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "execute_shell_command",
                    expected_count: 1,
                    tolerance: 0,
                },
            ],
            data_flow_expectations: vec![
                DataFlowExpectation {
                    from_function: "read_config_file",
                    to_function: "parse_config",
                    flow_type: FlowType::DataPassing,
                },
                DataFlowExpectation {
                    from_function: "parse_config",
                    to_function: "execute_shell_command",
                    flow_type: FlowType::DataPassing,
                },
            ],
            test_rationale: "Rust関数定義、エラーハンドリング、型推論の解析精度",
        },
        // === 複雑なネストケース ===
        ContextQualityTestCase {
            name: "Complex nested functions with closures",
            language: Language::JavaScript,
            code: r#"
function createUserProcessor(config) {
    const validator = function(data) {
        return data && typeof data === 'object';
    };
    
    const sanitizer = (input) => {
        if (typeof input === 'string') {
            return input.replace(/[<>'"]/g, '');
        }
        return input;
    };
    
    return function processUser(userData) {
        if (!validator(userData)) {
            throw new Error('Invalid user data');
        }
        
        const processedData = {};
        for (const [key, value] of Object.entries(userData)) {
            processedData[key] = sanitizer(value);
        }
        
        // ネストした関数
        function logProcessing() {
            console.log('Processing user:', processedData.username || 'unknown');
        }
        
        logProcessing();
        return processedData;
    };
}

const processor = createUserProcessor({ strict: true });
const result = processor({ username: 'test<script>', email: 'test@example.com' });
"#,
            expected_definitions: vec![
                ExpectedDefinition {
                    name: "createUserProcessor",
                    should_be_found: true,
                    minimum_source_length: 100,
                },
                ExpectedDefinition {
                    name: "validator",
                    should_be_found: true,
                    minimum_source_length: 20,
                },
                ExpectedDefinition {
                    name: "sanitizer",
                    should_be_found: true,
                    minimum_source_length: 30,
                },
                ExpectedDefinition {
                    name: "processUser",
                    should_be_found: true,
                    minimum_source_length: 80,
                },
                ExpectedDefinition {
                    name: "logProcessing",
                    should_be_found: true,
                    minimum_source_length: 20,
                },
            ],
            expected_references: vec![
                ExpectedReference {
                    name: "validator",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "sanitizer",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "logProcessing",
                    expected_count: 1,
                    tolerance: 0,
                },
                ExpectedReference {
                    name: "createUserProcessor",
                    expected_count: 1,
                    tolerance: 0,
                },
            ],
            data_flow_expectations: vec![
                DataFlowExpectation {
                    from_function: "validator",
                    to_function: "processUser",
                    flow_type: FlowType::Reference,
                },
                DataFlowExpectation {
                    from_function: "sanitizer",
                    to_function: "processUser",
                    flow_type: FlowType::Reference,
                },
            ],
            test_rationale: "複雑なネスト関数、クロージャ、スコープの解析精度",
        },
    ]
}

fn analyze_context_quality(
    context: &parsentry::parser::Context,
    test_case: &ContextQualityTestCase,
) -> ContextQualityResult {
    let mut result = ContextQualityResult {
        definition_accuracy: 0.0,
        reference_accuracy: 0.0,
        data_flow_accuracy: 0.0,
        failed_definitions: Vec::new(),
        failed_references: Vec::new(),
        failed_data_flows: Vec::new(),
    };

    // 1. 定義抽出精度の測定
    let mut correct_definitions = 0;
    let total_definitions = test_case.expected_definitions.len();

    for expected_def in &test_case.expected_definitions {
        let found_definition = context
            .definitions
            .iter()
            .find(|def| def.name == expected_def.name);

        match (found_definition, expected_def.should_be_found) {
            (Some(def), true) => {
                if def.source.len() >= expected_def.minimum_source_length {
                    correct_definitions += 1;
                } else {
                    result.failed_definitions.push(format!(
                        "{}: ソースが短すぎます ({}文字, 期待>={}文字)",
                        expected_def.name,
                        def.source.len(),
                        expected_def.minimum_source_length
                    ));
                }
            }
            (None, true) => {
                result
                    .failed_definitions
                    .push(format!("{}: 定義が見つかりません", expected_def.name));
            }
            (Some(_), false) => {
                result.failed_definitions.push(format!(
                    "{}: 見つからないはずの定義が見つかりました",
                    expected_def.name
                ));
            }
            (None, false) => {
                correct_definitions += 1; // 正しく見つからなかった
            }
        }
    }

    result.definition_accuracy = if total_definitions > 0 {
        (correct_definitions as f64 / total_definitions as f64) * 100.0
    } else {
        100.0
    };

    // 2. 参照追跡精度の測定
    let mut correct_references = 0;
    let total_references = test_case.expected_references.len();

    for expected_ref in &test_case.expected_references {
        let actual_count = context
            .references
            .iter()
            .filter(|ref_def| ref_def.name == expected_ref.name)
            .count();

        let expected_min = expected_ref
            .expected_count
            .saturating_sub(expected_ref.tolerance);
        let expected_max = expected_ref.expected_count + expected_ref.tolerance;

        if actual_count >= expected_min && actual_count <= expected_max {
            correct_references += 1;
        } else {
            result.failed_references.push(format!(
                "{}: 参照数不一致 (実際={}, 期待={}±{})",
                expected_ref.name,
                actual_count,
                expected_ref.expected_count,
                expected_ref.tolerance
            ));
        }
    }

    result.reference_accuracy = if total_references > 0 {
        (correct_references as f64 / total_references as f64) * 100.0
    } else {
        100.0
    };

    // 3. データフロー追跡精度の測定
    let mut correct_flows = 0;
    let total_flows = test_case.data_flow_expectations.len();

    for expected_flow in &test_case.data_flow_expectations {
        // 両方の関数が定義に存在するかチェック
        let from_exists = context
            .definitions
            .iter()
            .any(|def| def.name == expected_flow.from_function);
        let to_exists = context
            .definitions
            .iter()
            .any(|def| def.name == expected_flow.to_function);

        if from_exists && to_exists {
            // 簡易的なデータフロー検証：to_function内でfrom_functionが参照されているか
            let flow_detected = context
                .definitions
                .iter()
                .filter(|def| def.name == expected_flow.to_function)
                .any(|def| def.source.contains(expected_flow.from_function));

            if flow_detected {
                correct_flows += 1;
            } else {
                result.failed_data_flows.push(format!(
                    "{}→{}: データフローが検出されませんでした (期待タイプ: {:?})",
                    expected_flow.from_function, expected_flow.to_function, expected_flow.flow_type
                ));
            }
        } else {
            result.failed_data_flows.push(format!(
                "{}→{}: 関数定義が見つかりません (from={}, to={})",
                expected_flow.from_function, expected_flow.to_function, from_exists, to_exists
            ));
        }
    }

    result.data_flow_accuracy = if total_flows > 0 {
        (correct_flows as f64 / total_flows as f64) * 100.0
    } else {
        100.0
    };

    result
}

#[derive(Debug)]
struct ContextQualityResult {
    definition_accuracy: f64,
    reference_accuracy: f64,
    data_flow_accuracy: f64,
    failed_definitions: Vec<String>,
    failed_references: Vec<String>,
    failed_data_flows: Vec<String>,
}

async fn test_context_quality_for_case(
    test_case: &ContextQualityTestCase,
) -> Result<ContextQualityResult> {
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

    Ok(analyze_context_quality(&context, test_case))
}

#[tokio::test]
async fn test_definition_extraction_accuracy() -> Result<()> {
    println!("🔍 関数定義抽出精度テスト");

    let mut total_accuracy = 0.0;
    let mut total_tests = 0;
    let mut language_stats = HashMap::new();

    let test_cases = get_context_quality_test_cases();
    for test_case in &test_cases {
        println!(
            "  テスト中: {} ({:?}) - {}",
            test_case.name, test_case.language, test_case.test_rationale
        );

        let result = test_context_quality_for_case(test_case).await?;

        // 言語別統計更新
        let language_key = format!("{:?}", test_case.language);
        let entry = language_stats.entry(language_key).or_insert((0.0, 0));
        entry.0 += result.definition_accuracy;
        entry.1 += 1;

        total_accuracy += result.definition_accuracy;
        total_tests += 1;

        if result.definition_accuracy >= 90.0 {
            println!("    ✅ 定義抽出精度: {:.1}%", result.definition_accuracy);
        } else {
            println!("    ⚠️  定義抽出精度: {:.1}%", result.definition_accuracy);
            for failure in &result.failed_definitions {
                println!("       - {}", failure);
            }
        }
    }

    let overall_accuracy = total_accuracy / total_tests as f64;

    println!("\n📊 関数定義抽出結果:");
    println!("  全体精度: {:.1}%", overall_accuracy);

    println!("\n言語別精度:");
    for (language, (accuracy_sum, count)) in language_stats {
        let avg_accuracy = accuracy_sum / count as f64;
        println!("  {}: {:.1}%", language, avg_accuracy);
    }

    // 定義抽出は95%以上の精度を要求
    assert!(
        overall_accuracy >= 95.0,
        "関数定義抽出精度が基準を下回っています: {:.1}% (要求: 95.0%)",
        overall_accuracy
    );

    println!("\n🎉 関数定義抽出精度テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_reference_tracking_accuracy() -> Result<()> {
    println!("🔗 参照追跡精度テスト");

    let mut total_accuracy = 0.0;
    let mut total_tests = 0;

    let test_cases = get_context_quality_test_cases();
    for test_case in &test_cases {
        if test_case.expected_references.is_empty() {
            continue; // 参照期待値がないケースはスキップ
        }

        println!(
            "  テスト中: {} - {}",
            test_case.name, test_case.test_rationale
        );

        let result = test_context_quality_for_case(test_case).await?;

        total_accuracy += result.reference_accuracy;
        total_tests += 1;

        if result.reference_accuracy >= 85.0 {
            println!("    ✅ 参照追跡精度: {:.1}%", result.reference_accuracy);
        } else {
            println!("    ⚠️  参照追跡精度: {:.1}%", result.reference_accuracy);
            for failure in &result.failed_references {
                println!("       - {}", failure);
            }
        }
    }

    let overall_accuracy = if total_tests > 0 {
        total_accuracy / total_tests as f64
    } else {
        100.0
    };

    println!("\n📊 参照追跡結果:");
    println!("  全体精度: {:.1}%", overall_accuracy);

    // 参照追跡は85%以上の精度を要求
    assert!(
        overall_accuracy >= 85.0,
        "参照追跡精度が基準を下回っています: {:.1}% (要求: 85.0%)",
        overall_accuracy
    );

    println!("✅ 参照追跡精度テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_data_flow_tracking_accuracy() -> Result<()> {
    println!("🌊 データフロー追跡精度テスト");

    let mut total_accuracy = 0.0;
    let mut total_tests = 0;

    let test_cases = get_context_quality_test_cases();
    for test_case in &test_cases {
        if test_case.data_flow_expectations.is_empty() {
            continue; // データフロー期待値がないケースはスキップ
        }

        println!(
            "  テスト中: {} - {}",
            test_case.name, test_case.test_rationale
        );

        let result = test_context_quality_for_case(test_case).await?;

        total_accuracy += result.data_flow_accuracy;
        total_tests += 1;

        if result.data_flow_accuracy >= 75.0 {
            println!("    ✅ データフロー精度: {:.1}%", result.data_flow_accuracy);
        } else {
            println!(
                "    ⚠️  データフロー精度: {:.1}%",
                result.data_flow_accuracy
            );
            for failure in &result.failed_data_flows {
                println!("       - {}", failure);
            }
        }
    }

    let overall_accuracy = if total_tests > 0 {
        total_accuracy / total_tests as f64
    } else {
        100.0
    };

    println!("\n📊 データフロー追跡結果:");
    println!("  全体精度: {:.1}%", overall_accuracy);

    // データフロー追跡は75%以上の精度を要求。複雑な解析のため基準を下げている
    assert!(
        overall_accuracy >= 75.0,
        "データフロー追跡精度が基準を下回っています: {:.1}% (要求: 75.0%)",
        overall_accuracy
    );

    println!("✅ データフロー追跡精度テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_context_quality() -> Result<()> {
    println!("🧪 コンテキスト品質包括テスト");

    let mut definition_total = 0.0;
    let mut reference_total = 0.0;
    let mut data_flow_total = 0.0;
    let mut total_tests = 0;
    let mut failed_cases = Vec::new();

    let test_cases = get_context_quality_test_cases();
    for test_case in &test_cases {
        println!(
            "  [{}/{}] テスト中: {}",
            total_tests + 1,
            test_cases.len(),
            test_case.name
        );

        let result = test_context_quality_for_case(test_case).await?;

        definition_total += result.definition_accuracy;
        reference_total += result.reference_accuracy;
        data_flow_total += result.data_flow_accuracy;
        total_tests += 1;

        // 総合スコア計算
        let comprehensive_score = (result.definition_accuracy * 0.5)
            + (result.reference_accuracy * 0.3)
            + (result.data_flow_accuracy * 0.2);

        if comprehensive_score < 85.0 {
            failed_cases.push(format!(
                "{}: {:.1}% (定義={:.1}%, 参照={:.1}%, フロー={:.1}%)",
                test_case.name,
                comprehensive_score,
                result.definition_accuracy,
                result.reference_accuracy,
                result.data_flow_accuracy
            ));
        }
    }

    let avg_definition = definition_total / total_tests as f64;
    let avg_reference = reference_total / total_tests as f64;
    let avg_data_flow = data_flow_total / total_tests as f64;
    let comprehensive_average =
        (avg_definition * 0.5) + (avg_reference * 0.3) + (avg_data_flow * 0.2);

    println!("\n📊 コンテキスト品質包括結果:");
    println!("  定義抽出精度: {:.1}%", avg_definition);
    println!("  参照追跡精度: {:.1}%", avg_reference);
    println!("  データフロー精度: {:.1}%", avg_data_flow);
    println!("  総合スコア: {:.1}%", comprehensive_average);

    if !failed_cases.is_empty() {
        println!("\n❌ 基準を下回ったケース:");
        for case in &failed_cases {
            println!("    - {}", case);
        }
    }

    // 総合スコアは85%以上を要求
    assert!(
        comprehensive_average >= 85.0,
        "コンテキスト品質総合スコアが基準を下回っています: {:.1}% (要求: 85.0%)",
        comprehensive_average
    );

    println!("\n🎉 コンテキスト品質包括テスト合格!");
    Ok(())
}
