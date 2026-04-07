use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use parsentry::response::VulnType;
use parsentry::security_patterns::{Language, SecurityRiskPatterns};
use std::collections::HashMap;
use tempfile::tempdir;

/// エンドツーエンド精度テスト
/// パターンマッチング→コンテキスト構築→LLM解析の全パイプラインの精度を測定
/// 実際のワークフローでの総合的な検出精度を評価

#[derive(Debug, Clone)]
struct EndToEndTestCase {
    name: &'static str,
    language: Language,
    files: Vec<FileSpec>,
    expected_findings: Vec<ExpectedFinding>,
    pipeline_expectations: PipelineExpectation,
    test_scenario: &'static str,
}

#[derive(Debug, Clone)]
struct FileSpec {
    name: &'static str,
    content: &'static str,
}

#[derive(Debug, Clone)]
struct ExpectedFinding {
    file_name: &'static str,
    vulnerability_types: Vec<VulnType>,
    minimum_confidence: i32,
    should_be_detected: bool,
}

#[derive(Debug, Clone)]
struct PipelineExpectation {
    pattern_matching_should_trigger: bool,
    context_building_quality_min: f64,
    llm_analysis_should_succeed: bool,
    expected_total_findings: usize,
}

fn get_end_to_end_test_cases() -> Vec<EndToEndTestCase> {
    vec![
        // === 単一ファイルシナリオ ===
        EndToEndTestCase {
            name: "Single file SQL injection detection",
            language: Language::Python,
            files: vec![FileSpec {
                name: "database.py",
                content: r#"
import sqlite3

def get_user_by_id(user_id):
    """ユーザーIDでユーザー情報を取得"""
    conn = sqlite3.connect('users.db')
    
    # 脆弱：SQLインジェクション
    query = f"SELECT * FROM users WHERE id = {user_id}"
    cursor = conn.execute(query)
    result = cursor.fetchone()
    
    conn.close()
    return result

def get_user_by_email(email):
    """メールアドレスでユーザー情報を取得"""
    conn = sqlite3.connect('users.db')
    
    # さらに脆弱：文字列パラメータの直接埋め込み
    query = f"SELECT * FROM users WHERE email = '{email}'"
    cursor = conn.execute(query)
    result = cursor.fetchone()
    
    conn.close()
    return result

def safe_get_user(user_id):
    """安全なユーザー取得。比較用"""
    conn = sqlite3.connect('users.db')
    cursor = conn.execute("SELECT * FROM users WHERE id = ?", (user_id,))
    result = cursor.fetchone()
    conn.close()
    return result
"#,
            }],
            expected_findings: vec![ExpectedFinding {
                file_name: "database.py",
                vulnerability_types: vec![VulnType::SQLI],
                minimum_confidence: 8,
                should_be_detected: true,
            }],
            pipeline_expectations: PipelineExpectation {
                pattern_matching_should_trigger: true,
                context_building_quality_min: 90.0,
                llm_analysis_should_succeed: true,
                expected_total_findings: 1,
            },
            test_scenario: "単一ファイルでの典型的なSQLインジェクション検出",
        },
        // === マルチファイルシナリオ ===
        EndToEndTestCase {
            name: "Multi-file cross-reference vulnerability",
            language: Language::Python,
            files: vec![
                FileSpec {
                    name: "web_handler.py",
                    content: r#"
from flask import Flask, request
from utils import process_user_input
from database import store_user_data

app = Flask(__name__)

@app.route('/api/user', methods=['POST'])
def create_user():
    """ユーザー作成API"""
    # ユーザー入力を取得
    raw_data = request.get_json()
    
    # 入力処理。utils.pyで定義
    processed_data = process_user_input(raw_data)

    # データベース保存。database.pyで定義
    user_id = store_user_data(processed_data)
    
    return {"user_id": user_id, "status": "created"}

@app.route('/api/search', methods=['GET'])
def search_users():
    """ユーザー検索API"""
    search_term = request.args.get('q', '')
    
    # 危険：直接検索語を処理に渡す
    results = process_search_query(search_term)
    return {"results": results}
"#,
                },
                FileSpec {
                    name: "utils.py",
                    content: r#"
import re
import subprocess

def process_user_input(user_data):
    """ユーザー入力の処理"""
    if not user_data:
        return None
    
    # 基本的な検証
    if 'name' in user_data:
        user_data['name'] = user_data['name'].strip()
    
    # 危険：ユーザーデータでコマンド実行
    if 'profile_image_url' in user_data:
        url = user_data['profile_image_url']
        # 画像ダウンロードコマンド実行
        cmd = f"wget -O /tmp/profile.jpg {url}"
        subprocess.run(cmd, shell=True)
    
    return user_data

def validate_email(email):
    """メール検証"""
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None

def process_search_query(query):
    """検索クエリ処理"""
    # 危険：動的コード実行
    filter_code = f"lambda user: '{query}' in user['name'].lower()"
    filter_func = eval(filter_code)
    
    # 実際の検索処理は省略
    return []
"#,
                },
                FileSpec {
                    name: "database.py",
                    content: r#"
import sqlite3
import os

def store_user_data(user_data):
    """ユーザーデータの保存"""
    conn = sqlite3.connect('app.db')
    
    # 危険：SQLインジェクション
    name = user_data.get('name', '')
    email = user_data.get('email', '')
    
    query = f"INSERT INTO users (name, email) VALUES ('{name}', '{email}')"
    cursor = conn.execute(query)
    user_id = cursor.lastrowid
    
    conn.commit()
    conn.close()
    
    return user_id

def get_user_files(user_id, file_type):
    """ユーザーファイル取得"""
    # 危険：パストラバーサル
    base_path = "/app/user_files/"
    file_path = f"{base_path}{user_id}/{file_type}.txt"
    
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            return f.read()
    return None
"#,
                },
            ],
            expected_findings: vec![
                ExpectedFinding {
                    file_name: "utils.py",
                    vulnerability_types: vec![VulnType::RCE],
                    minimum_confidence: 8,
                    should_be_detected: true,
                },
                ExpectedFinding {
                    file_name: "database.py",
                    vulnerability_types: vec![VulnType::SQLI, VulnType::LFI],
                    minimum_confidence: 7,
                    should_be_detected: true,
                },
            ],
            pipeline_expectations: PipelineExpectation {
                pattern_matching_should_trigger: true,
                context_building_quality_min: 85.0,
                llm_analysis_should_succeed: true,
                expected_total_findings: 2,
            },
            test_scenario: "マルチファイル間の関数呼び出しと複数脆弱性の検出",
        },
        // === JavaScript/Node.js Webアプリケーション ===
        EndToEndTestCase {
            name: "Node.js Express application vulnerabilities",
            language: Language::JavaScript,
            files: vec![
                FileSpec {
                    name: "app.js",
                    content: r#"
const express = require('express');
const userController = require('./controllers/user');
const authMiddleware = require('./middleware/auth');

const app = express();
app.use(express.json());

// 認証が必要なルート
app.use('/api/protected', authMiddleware);

// パブリックルート
app.post('/api/register', userController.register);
app.post('/api/login', userController.login);

// 保護されたルート
app.get('/api/protected/profile/:id', userController.getProfile);
app.delete('/api/protected/user/:id', userController.deleteUser);

// 管理者ルート
app.get('/api/admin/users', userController.getAllUsers);
app.get('/api/admin/logs', userController.getLogs);

app.listen(3000, () => {
    console.log('Server running on port 3000');
});
"#,
                },
                FileSpec {
                    name: "controllers/user.js",
                    content: r#"
const db = require('../database/connection');
const fs = require('fs');
const { exec } = require('child_process');

exports.register = async (req, res) => {
    const { username, email, password } = req.body;
    
    // 危険：SQLインジェクション
    const query = `INSERT INTO users (username, email, password) VALUES ('${username}', '${email}', '${password}')`;
    
    try {
        const result = await db.query(query);
        res.json({ success: true, userId: result.insertId });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
};

exports.getProfile = async (req, res) => {
    const userId = req.params.id;
    
    // 危険：IDOR - 認可チェックなし
    const query = `SELECT username, email, created_at FROM users WHERE id = ${userId}`;
    const user = await db.query(query);
    
    if (user.length > 0) {
        res.json(user[0]);
    } else {
        res.status(404).json({ error: 'User not found' });
    }
};

exports.getLogs = async (req, res) => {
    const { date, level } = req.query;
    
    // 危険：コマンドインジェクション
    const logFile = `/var/log/app-${date}.log`;
    const grepCommand = `grep "${level}" ${logFile}`;
    
    exec(grepCommand, (error, stdout, stderr) => {
        if (error) {
            return res.status(500).json({ error: 'Failed to read logs' });
        }
        res.json({ logs: stdout.split('\n') });
    });
};

exports.deleteUser = async (req, res) => {
    const userId = req.params.id;
    const currentUserId = req.user.id;
    
    // 危険：不十分な認可チェック
    if (userId !== currentUserId && req.user.role !== 'admin') {
        return res.status(403).json({ error: 'Unauthorized' });
    }
    
    // さらに危険：SQLインジェクション
    const query = `DELETE FROM users WHERE id = ${userId}`;
    await db.query(query);
    
    res.json({ success: true });
};
"#,
                },
                FileSpec {
                    name: "middleware/auth.js",
                    content: r#"
const jwt = require('jsonwebtoken');

module.exports = (req, res, next) => {
    const token = req.headers.authorization?.replace('Bearer ', '');
    
    if (!token) {
        return res.status(401).json({ error: 'No token provided' });
    }
    
    try {
        // 危険：JWT secret がハードコード
        const decoded = jwt.verify(token, 'super-secret-key-123');
        req.user = decoded;
        next();
    } catch (error) {
        res.status(401).json({ error: 'Invalid token' });
    }
};
"#,
                },
            ],
            expected_findings: vec![ExpectedFinding {
                file_name: "controllers/user.js",
                vulnerability_types: vec![VulnType::SQLI, VulnType::IDOR, VulnType::RCE],
                minimum_confidence: 7,
                should_be_detected: true,
            }],
            pipeline_expectations: PipelineExpectation {
                pattern_matching_should_trigger: true,
                context_building_quality_min: 80.0,
                llm_analysis_should_succeed: true,
                expected_total_findings: 1,
            },
            test_scenario: "Express.jsアプリケーションでの複合的な認証・認可・インジェクション脆弱性",
        },
        // === パターンマッチングエッジケース ===
        EndToEndTestCase {
            name: "Pattern matching edge cases",
            language: Language::Python,
            files: vec![FileSpec {
                name: "edge_cases.py",
                content: r#"
import math
import datetime

def calculate_interest(principal, rate, time):
    """純粋な数学計算 - セキュリティ関連なし"""
    return principal * (1 + rate) ** time

def format_date(date_obj):
    """日付フォーマット - セキュリティ関連なし"""
    return date_obj.strftime("%Y-%m-%d")

def hidden_vulnerability(user_input):
    """一見無害だが実際は脆弱"""
    # パターンマッチングでは検出困難な微妙な脆弱性
    import pickle
    import base64
    
    # 複雑な条件分岐の奥に隠された脆弱性
    if user_input and len(user_input) > 10:
        if user_input.startswith('data:'):
            encoded_part = user_input[5:]  # 'data:' プレフィックスを除去
            try:
                decoded = base64.b64decode(encoded_part)
                # 危険：pickleデシリアライゼーション
                result = pickle.loads(decoded)
                return {"status": "processed", "data": result}
            except:
                return {"status": "error"}
    
    return {"status": "ignored"}

def complex_file_operation(base_path, user_dir, filename):
    """複雑なファイル操作 - パターンでは検出困難"""
    import os
    
    # 一見安全に見えるファイル操作
    if base_path and user_dir and filename:
        # しかし実際にはパストラバーサル可能
        full_path = os.path.join(base_path, user_dir, filename)
        
        if os.path.exists(full_path):
            with open(full_path, 'r') as f:
                return f.read()
    
    return None
"#,
            }],
            expected_findings: vec![ExpectedFinding {
                file_name: "edge_cases.py",
                vulnerability_types: vec![VulnType::RCE, VulnType::LFI],
                minimum_confidence: 6,
                should_be_detected: true,
            }],
            pipeline_expectations: PipelineExpectation {
                pattern_matching_should_trigger: false, // 微妙な脆弱性はパターンで検出困難
                context_building_quality_min: 85.0,
                llm_analysis_should_succeed: true, // LLMは文脈で検出すべき
                expected_total_findings: 1,
            },
            test_scenario: "パターンマッチングでは検出困難だがLLMで検出すべき微妙な脆弱性",
        },
    ]
}

#[derive(Debug)]
struct EndToEndResult {
    pattern_matching_triggered: bool,
    context_quality_score: f64,
    llm_analysis_success: bool,
    detected_findings: Vec<DetectedFinding>,
    pipeline_performance: PipelinePerformance,
}

#[derive(Debug)]
struct DetectedFinding {
    file_name: String,
    vulnerability_types: Vec<VulnType>,
    confidence_score: i32,
    analysis_quality: f64,
}

#[derive(Debug)]
struct PipelinePerformance {
    pattern_stage_accuracy: f64,
    context_stage_accuracy: f64,
    llm_stage_accuracy: f64,
    overall_accuracy: f64,
}

async fn test_end_to_end_case(test_case: &EndToEndTestCase, model: &str) -> Result<EndToEndResult> {
    // テンポラリディレクトリ作成
    let temp_dir = tempdir()?;
    let project_path = temp_dir.path();

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

    // テストシナリオの詳細をログ出力
    println!(
        "  テスト中: {} - {}",
        test_case.name, test_case.test_scenario
    );

    // ファイル作成
    let mut created_files = Vec::new();
    for file_spec in &test_case.files {
        let file_path = if file_spec.name.contains('/') {
            // サブディレクトリ処理
            let full_path = project_path.join(file_spec.name);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            full_path
        } else {
            project_path.join(format!(
                "{}.{}",
                file_spec
                    .name
                    .trim_end_matches(&format!(".{}", file_extension)),
                file_extension
            ))
        };

        std::fs::write(&file_path, file_spec.content)?;
        created_files.push(file_path);
    }

    // === Stage 1: パターンマッチング ===
    let security_patterns = SecurityRiskPatterns::new(test_case.language);
    let mut pattern_triggered_files = Vec::new();

    for file_path in &created_files {
        let content = std::fs::read_to_string(file_path)?;
        if security_patterns.matches(&content) {
            pattern_triggered_files.push(file_path.clone());
        }
    }

    let pattern_matching_triggered = !pattern_triggered_files.is_empty();

    // === Stage 2: コンテキスト構築 ===
    let mut context_quality_scores = Vec::new();
    let mut all_contexts = HashMap::new();

    for file_path in &created_files {
        let mut parser = CodeParser::new()?;
        parser.add_file(file_path)?;
        let context = parser.build_context_from_file(file_path)?;

        // コンテキスト品質評価。定義数、参照数、平均長さから簡易評価
        let definition_count = context.definitions.len() as f64;
        let reference_count = context.references.len() as f64;
        let avg_definition_length = if definition_count > 0.0 {
            context
                .definitions
                .iter()
                .map(|d| d.source.len())
                .sum::<usize>() as f64
                / definition_count
        } else {
            0.0
        };

        let quality_score =
            ((definition_count * 20.0) + (reference_count * 10.0) + (avg_definition_length / 10.0))
                .min(100.0);
        context_quality_scores.push(quality_score);
        all_contexts.insert(file_path.clone(), context);
    }

    let context_quality_score = if context_quality_scores.is_empty() {
        0.0
    } else {
        context_quality_scores.iter().sum::<f64>() / context_quality_scores.len() as f64
    };

    // === Stage 3: LLM解析 ===
    let mut detected_findings = Vec::new();
    let mut llm_analysis_success = true;

    for file_path in &created_files {
        if let Some(context) = all_contexts.get(file_path) {
            match analyze_file(
                file_path,
                model,
                &created_files,
                0,
                context,
                0,
                false,
                &None,
                None,
                &LocaleLanguage::Japanese,
            )
            .await
            {
                Ok(response) => {
                    if !response.vulnerability_types.is_empty() {
                        let analysis_quality = if response.analysis.len() > 100 {
                            85.0
                        } else if response.analysis.len() > 50 {
                            70.0
                        } else {
                            40.0
                        };

                        detected_findings.push(DetectedFinding {
                            file_name: file_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            vulnerability_types: response.vulnerability_types,
                            confidence_score: response.confidence_score,
                            analysis_quality,
                        });
                    }
                }
                Err(_) => {
                    llm_analysis_success = false;
                }
            }
        }
    }

    // === パイプライン性能評価 ===
    let pattern_stage_accuracy = if test_case
        .pipeline_expectations
        .pattern_matching_should_trigger
    {
        if pattern_matching_triggered {
            100.0
        } else {
            0.0
        }
    } else {
        if !pattern_matching_triggered {
            100.0
        } else {
            50.0
        } // パターンで検出されなくても部分点
    };

    let context_stage_accuracy =
        if context_quality_score >= test_case.pipeline_expectations.context_building_quality_min {
            100.0
        } else {
            (context_quality_score / test_case.pipeline_expectations.context_building_quality_min)
                * 100.0
        };

    let llm_stage_accuracy = if test_case.pipeline_expectations.llm_analysis_should_succeed {
        if llm_analysis_success
            && detected_findings.len() >= test_case.pipeline_expectations.expected_total_findings
        {
            100.0
        } else {
            50.0
        }
    } else {
        if !llm_analysis_success { 100.0 } else { 50.0 }
    };

    // === 期待される検出結果の検証 ===
    for expected in &test_case.expected_findings {
        let found_in_file = detected_findings
            .iter()
            .any(|finding| finding.file_name.contains(expected.file_name));

        // should_be_detected フィールドを使用した検証
        match (expected.should_be_detected, found_in_file) {
            (true, false) => {
                println!(
                    "    ❌ 検出されるべき脆弱性が未検出: {} (期待タイプ: {:?}, 最小信頼度: {})",
                    expected.file_name, expected.vulnerability_types, expected.minimum_confidence
                );
            }
            (false, true) => {
                println!("    ⚠️  偽陽性検出: {}", expected.file_name);
            }
            (true, true) => {
                // 脆弱性タイプと信頼度の詳細チェック
                if let Some(detected) = detected_findings
                    .iter()
                    .find(|f| f.file_name.contains(expected.file_name))
                {
                    let type_match = expected
                        .vulnerability_types
                        .iter()
                        .any(|expected_type| detected.vulnerability_types.contains(expected_type));
                    let confidence_ok = detected.confidence_score >= expected.minimum_confidence;

                    if !type_match {
                        println!(
                            "    ⚠️  期待された脆弱性タイプが検出されませんでした: {:?}",
                            expected.vulnerability_types
                        );
                    }
                    if !confidence_ok {
                        println!(
                            "    ⚠️  信頼度が基準を下回っています: {} < {}",
                            detected.confidence_score, expected.minimum_confidence
                        );
                    }
                    if type_match && confidence_ok {
                        println!(
                            "    ✅ 正常検出: {} (分析品質: {:.1}%)",
                            expected.file_name, detected.analysis_quality
                        );
                    }
                }
            }
            (false, false) => {
                println!("    ✅ 正常非検出: {}", expected.file_name);
            }
        }
    }

    let overall_accuracy = (pattern_stage_accuracy * 0.2)
        + (context_stage_accuracy * 0.3)
        + (llm_stage_accuracy * 0.5);

    // context_quality_scoreをログ出力
    println!(
        "    📊 コンテキスト品質スコア: {:.1}%",
        context_quality_score
    );

    Ok(EndToEndResult {
        pattern_matching_triggered,
        context_quality_score,
        llm_analysis_success,
        detected_findings,
        pipeline_performance: PipelinePerformance {
            pattern_stage_accuracy,
            context_stage_accuracy,
            llm_stage_accuracy,
            overall_accuracy,
        },
    })
}

#[tokio::test]
async fn test_single_file_end_to_end() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping single file E2E test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // 単一ファイルケースのみテスト
    let test_cases = get_end_to_end_test_cases();
    let single_file_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| case.files.len() == 1)
        .collect();

    println!(
        "📄 単一ファイル E2E テスト: {}ケース",
        single_file_cases.len()
    );

    let mut total_accuracy = 0.0;
    let mut total_tests = 0;

    for test_case in single_file_cases {
        println!("  テスト中: {}", test_case.name);

        let result = test_end_to_end_case(test_case, model).await?;

        total_accuracy += result.pipeline_performance.overall_accuracy;
        total_tests += 1;

        println!(
            "    パイプライン精度: {:.1}% (コンテキスト品質: {:.1}%)",
            result.pipeline_performance.overall_accuracy, result.context_quality_score
        );
        println!(
            "      パターン: {:.1}%, コンテキスト: {:.1}%, LLM: {:.1}%",
            result.pipeline_performance.pattern_stage_accuracy,
            result.pipeline_performance.context_stage_accuracy,
            result.pipeline_performance.llm_stage_accuracy
        );

        if result.pipeline_performance.overall_accuracy >= 85.0 {
            println!("    ✅ 合格");
        } else {
            println!("    ⚠️  要改善");
        }
    }

    let avg_accuracy = total_accuracy / total_tests as f64;

    println!("\n📊 単一ファイル E2E 結果:");
    println!("  平均精度: {:.1}%", avg_accuracy);

    // 単一ファイルE2Eは90%以上の精度を要求
    assert!(
        avg_accuracy >= 90.0,
        "単一ファイルE2E精度が基準を下回っています: {:.1}% (要求: 90.0%)",
        avg_accuracy
    );

    println!("🎉 単一ファイル E2E テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_multi_file_end_to_end() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping multi-file E2E test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // マルチファイルケースのみテスト
    let test_cases = get_end_to_end_test_cases();
    let multi_file_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| case.files.len() > 1)
        .collect();

    println!(
        "📁 マルチファイル E2E テスト: {}ケース",
        multi_file_cases.len()
    );

    let mut total_accuracy = 0.0;
    let mut total_tests = 0;

    for test_case in multi_file_cases {
        println!(
            "  テスト中: {} ({}ファイル)",
            test_case.name,
            test_case.files.len()
        );

        let result = test_end_to_end_case(test_case, model).await?;

        total_accuracy += result.pipeline_performance.overall_accuracy;
        total_tests += 1;

        println!(
            "    パイプライン精度: {:.1}% (コンテキスト品質: {:.1}%)",
            result.pipeline_performance.overall_accuracy, result.context_quality_score
        );
        println!(
            "      検出ファイル数: {}/{}",
            result.detected_findings.len(),
            test_case.expected_findings.len()
        );

        for finding in &result.detected_findings {
            println!(
                "        {}: {:?} (信頼度={})",
                finding.file_name, finding.vulnerability_types, finding.confidence_score
            );
        }

        if result.pipeline_performance.overall_accuracy >= 80.0 {
            println!("    ✅ 合格");
        } else {
            println!("    ⚠️  要改善");
        }
    }

    let avg_accuracy = total_accuracy / total_tests as f64;

    println!("\n📊 マルチファイル E2E 結果:");
    println!("  平均精度: {:.1}%", avg_accuracy);

    // マルチファイルE2Eは85%以上の精度を要求。複雑性を考慮して基準を下げている
    assert!(
        avg_accuracy >= 85.0,
        "マルチファイルE2E精度が基準を下回っています: {:.1}% (要求: 85.0%)",
        avg_accuracy
    );

    println!("🎉 マルチファイル E2E テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_pattern_matching_edge_cases() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping pattern edge case test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // パターンマッチングエッジケースのみテスト
    let test_cases = get_end_to_end_test_cases();
    let edge_cases: Vec<_> = test_cases
        .iter()
        .filter(|case| !case.pipeline_expectations.pattern_matching_should_trigger)
        .collect();

    println!(
        "🔍 パターンマッチングエッジケーステスト: {}ケース",
        edge_cases.len()
    );

    let mut llm_compensated = 0;
    let mut total_tests = 0;

    for test_case in edge_cases {
        println!("  テスト中: {}", test_case.name);

        let result = test_end_to_end_case(test_case, model).await?;

        total_tests += 1;

        // パターンマッチングで検出されなくてもLLMで検出できたかチェック
        if !result.pattern_matching_triggered
            && result.llm_analysis_success
            && !result.detected_findings.is_empty()
        {
            llm_compensated += 1;
            println!("    ✅ LLMが補完: パターン未検出でも脆弱性発見");
        } else if result.pattern_matching_triggered {
            println!("    📋 パターン検出: 予期しないパターンマッチ");
        } else {
            println!("    ❌ 未検出: パターン・LLM共に検出失敗");
        }
    }

    let compensation_rate = if total_tests > 0 {
        (llm_compensated as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };

    println!("\n📊 パターンエッジケース結果:");
    println!(
        "  LLM補完率: {:.1}% ({}/{})",
        compensation_rate, llm_compensated, total_tests
    );

    // エッジケースではLLMが80%以上補完することを期待
    assert!(
        compensation_rate >= 80.0,
        "LLM補完率が基準を下回っています: {:.1}% (要求: 80.0%)",
        compensation_rate
    );

    println!("✅ パターンマッチングエッジケーステスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_end_to_end() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping comprehensive E2E test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let test_cases = get_end_to_end_test_cases();
    println!("🧪 包括的 E2E テスト: {}ケース", test_cases.len());

    let mut pattern_accuracy_total = 0.0;
    let mut context_accuracy_total = 0.0;
    let mut llm_accuracy_total = 0.0;
    let mut overall_accuracy_total = 0.0;
    let mut total_tests = 0;
    let mut failed_cases = Vec::new();

    for test_case in &test_cases {
        println!(
            "  [{}/{}] テスト中: {}",
            total_tests + 1,
            test_cases.len(),
            test_case.name
        );

        let result = test_end_to_end_case(test_case, model).await?;

        pattern_accuracy_total += result.pipeline_performance.pattern_stage_accuracy;
        context_accuracy_total += result.pipeline_performance.context_stage_accuracy;
        llm_accuracy_total += result.pipeline_performance.llm_stage_accuracy;
        overall_accuracy_total += result.pipeline_performance.overall_accuracy;
        total_tests += 1;

        if result.pipeline_performance.overall_accuracy < 85.0 {
            failed_cases.push(format!(
                "{}: {:.1}% (パターン={:.1}%, コンテキスト={:.1}%, LLM={:.1}%)",
                test_case.name,
                result.pipeline_performance.overall_accuracy,
                result.pipeline_performance.pattern_stage_accuracy,
                result.pipeline_performance.context_stage_accuracy,
                result.pipeline_performance.llm_stage_accuracy
            ));
        }
    }

    let avg_pattern = pattern_accuracy_total / total_tests as f64;
    let avg_context = context_accuracy_total / total_tests as f64;
    let avg_llm = llm_accuracy_total / total_tests as f64;
    let avg_overall = overall_accuracy_total / total_tests as f64;

    println!("\n📊 包括的 E2E 結果:");
    println!("  パターンステージ精度: {:.1}%", avg_pattern);
    println!("  コンテキストステージ精度: {:.1}%", avg_context);
    println!("  LLMステージ精度: {:.1}%", avg_llm);
    println!("  総合精度: {:.1}%", avg_overall);

    if !failed_cases.is_empty() {
        println!("\n❌ 基準を下回ったケース:");
        for case in &failed_cases {
            println!("    - {}", case);
        }
    }

    // 包括的E2Eは総合85%以上の精度を要求
    assert!(
        avg_overall >= 85.0,
        "包括的E2E総合精度が基準を下回っています: {:.1}% (要求: 85.0%)",
        avg_overall
    );

    println!("\n🎉 包括的 E2E テスト合格!");
    Ok(())
}
