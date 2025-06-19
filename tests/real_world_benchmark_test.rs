use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use parsentry::response::VulnType;
use parsentry::security_patterns::Language;
use std::collections::HashMap;
use tempfile::tempdir;

/// 実世界ベンチマークテスト
/// 既知のCVE事例、実際の脆弱性サンプル、業界標準ベンチマークに対する検出精度を測定
/// これはParsentryの実用性を評価する最も重要なテスト

#[derive(Debug, Clone)]
struct RealWorldBenchmark {
    name: &'static str,
    cve_id: Option<&'static str>,
    language: Language,
    code: &'static str,
    expected_vulnerabilities: Vec<VulnType>,
    expected_confidence_min: i32,
    severity: BenchmarkSeverity,
    _description: &'static str,
    _real_world_context: &'static str,
}

#[derive(Debug, Clone)]
enum BenchmarkSeverity {
    Critical,  // CVSS 9.0-10.0
    High,      // CVSS 7.0-8.9  
    Medium,    // CVSS 4.0-6.9
}

fn get_real_world_benchmarks() -> Vec<RealWorldBenchmark> {
    vec![
    // === Critical Severity Benchmarks ===
    RealWorldBenchmark {
        name: "CVE-2021-44228 Log4j类似 - Command Injection",
        cve_id: Some("CVE-2021-44228"),
        language: Language::Python,
        code: r#"
import logging
import subprocess

def log_user_activity(user_input, action):
    # 脆弱：ユーザー入力が直接ログに記録され、JNDIルックアップ類似の処理
    log_message = f"User activity: {user_input} performed {action}"
    
    # さらに危険：ログメッセージがコマンド実行に使用される
    if "exec:" in user_input:
        command = user_input.replace("exec:", "")
        result = subprocess.run(command, shell=True, capture_output=True, text=True)
        logging.info(f"Command executed: {result.stdout}")
    
    logging.info(log_message)
    return log_message

# 悪用例:
# log_user_activity("exec:rm -rf /tmp/*", "file_cleanup")
"#,
        expected_vulnerabilities: vec![VulnType::RCE],
        expected_confidence_min: 8,
        severity: BenchmarkSeverity::Critical,
        _description: "Log4j様のJNDIインジェクション類似パターン - コマンドインジェクション",
        _real_world_context: "Log4jのJNDI lookup脆弱性を模倣。実際のアプリケーションでユーザー入力がログに記録され、特殊な文字列がコマンド実行につながる",
    },

    RealWorldBenchmark {
        name: "SQL Injection - Union-based attack vector",
        cve_id: None,
        language: Language::Python,
        code: r#"
import sqlite3

def get_user_profile(user_id, db_connection):
    # 典型的なSQLインジェクション脆弱性
    query = f"SELECT username, email, role FROM users WHERE id = {user_id}"
    cursor = db_connection.execute(query)
    result = cursor.fetchone()
    
    if result:
        return {
            'username': result[0],
            'email': result[1], 
            'role': result[2]
        }
    return None

def get_user_by_email(email, db_connection):
    # さらに危険：文字列パラメータの直接埋め込み
    query = f"SELECT * FROM users WHERE email = '{email}'"
    cursor = db_connection.execute(query)
    return cursor.fetchall()

# 悪用例:
# get_user_profile("1 UNION SELECT password,secret,admin FROM admin_users--", conn)
# get_user_by_email("admin@test.com' OR '1'='1' --", conn)
"#,
        expected_vulnerabilities: vec![VulnType::SQLI],
        expected_confidence_min: 9,
        severity: BenchmarkSeverity::Critical,
        _description: "典型的なSQLインジェクション - UNIONベース攻撃",
        _real_world_context: "実際のWebアプリケーションでよく見られるSQLインジェクション。攻撃者がUNION文で管理者情報を窃取可能",
    },

    RealWorldBenchmark {
        name: "Deserialization RCE - pickle unsafe loading",
        cve_id: None,
        language: Language::Python,
        code: r#"
import pickle
import base64

def deserialize_user_session(session_data):
    # 極めて危険：信頼できないデータのデシリアライゼーション
    try:
        decoded_data = base64.b64decode(session_data)
        user_session = pickle.loads(decoded_data)
        return user_session
    except Exception as e:
        return {"error": str(e)}

def restore_user_state(user_id, session_cookie):
    # ユーザーセッションをCookieから復元
    if session_cookie:
        session = deserialize_user_session(session_cookie)
        if "user_id" in session and session["user_id"] == user_id:
            return session
    return None

# 悪用例:
# 攻撃者は悪意のあるpickleデータをbase64エンコードしてCookieに設定
# import os; os.system('rm -rf /')  のようなコードが実行される
"#,
        expected_vulnerabilities: vec![VulnType::RCE],
        expected_confidence_min: 9,
        severity: BenchmarkSeverity::Critical,
        _description: "Pythonデシリアライゼーション脆弱性 - pickle RCE",
        _real_world_context: "実際のWebアプリでセッション管理にpickleを使用している場合の典型的な脆弱性。攻撃者が任意コード実行可能",
    },

    // === High Severity Benchmarks ===
    RealWorldBenchmark {
        name: "DOM-based XSS - innerHTML injection",
        cve_id: None,
        language: Language::JavaScript,
        code: r#"
function displayUserMessage(messageId) {
    // URLパラメータから直接値を取得
    const urlParams = new URLSearchParams(window.location.search);
    const userMessage = urlParams.get('message');
    
    // 危険：サニタイズなしでDOM操作
    if (userMessage) {
        document.getElementById('message-display').innerHTML = 
            `<div class="user-message">Message: ${userMessage}</div>`;
    }
}

function updateUserProfile(userData) {
    // さらに危険：オブジェクトプロパティの直接展開
    const profileDiv = document.getElementById('profile');
    profileDiv.innerHTML = `
        <h2>Welcome ${userData.name}</h2>
        <p>Bio: ${userData.bio}</p>
        <p>Website: <a href="${userData.website}">${userData.website}</a></p>
    `;
}

// 悪用例:
// ?message=<script>alert('XSS')</script>
// userData.name = "<img src=x onerror=alert('XSS')>"
"#,
        expected_vulnerabilities: vec![VulnType::XSS],
        expected_confidence_min: 8,
        severity: BenchmarkSeverity::High,
        _description: "DOM-based XSS - innerHTML による直接注入",
        _real_world_context: "SPAアプリケーションでよく見られるDOM-based XSS。URLパラメータやユーザーデータが直接DOM操作に使用",
    },

    RealWorldBenchmark {
        name: "Path Traversal - File include vulnerability",
        cve_id: None,
        language: Language::Python,
        code: r#"
import os

def serve_static_file(filename):
    # 危険：パス検証なしのファイルアクセス
    static_dir = "/var/www/static/"
    file_path = static_dir + filename
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        return content
    except FileNotFoundError:
        return "File not found"

def load_template(template_name):
    # さらに危険：テンプレートファイルの動的読み込み
    template_dir = "/app/templates/"
    full_path = os.path.join(template_dir, template_name)
    
    # os.path.joinでも危険：絶対パス指定で回避される
    with open(full_path, 'r') as f:
        return f.read()

def include_user_file(user_id, file_type):
    # 最も危険：ユーザー制御可能な複数パラメータ
    base_path = f"/app/user_data/{user_id}/"
    file_path = base_path + file_type + ".txt"
    
    return open(file_path).read()

# 悪用例:
# serve_static_file("../../../etc/passwd")
# load_template("../../../../etc/shadow")
# include_user_file("../admin", "../../../etc/passwd")
"#,
        expected_vulnerabilities: vec![VulnType::LFI],
        expected_confidence_min: 8,
        severity: BenchmarkSeverity::High,
        _description: "パストラバーサル攻撃 - ディレクトリトラバーサル",
        _real_world_context: "Webアプリケーションでのファイル提供機能における典型的な脆弱性。攻撃者がシステムファイルにアクセス可能",
    },

    RealWorldBenchmark {
        name: "SSRF - Internal service access",
        cve_id: None,
        language: Language::Python,
        code: r#"
import requests

def fetch_external_image(image_url):
    # 危険：URL検証なしの外部リクエスト
    try:
        response = requests.get(image_url, timeout=10)
        if response.status_code == 200:
            return response.content
        return None
    except Exception:
        return None

def proxy_api_request(api_endpoint, user_token):
    # さらに危険：内部APIへのプロキシ
    internal_base = "http://internal-api:8080"
    full_url = f"{internal_base}/{api_endpoint}"
    
    headers = {
        'Authorization': f'Bearer {user_token}',
        'X-Forwarded-For': '127.0.0.1'
    }
    
    response = requests.get(full_url, headers=headers)
    return response.json()

def health_check_service(service_name):
    # 最も危険：完全にユーザー制御可能なURL
    health_url = f"http://{service_name}/health"
    response = requests.get(health_url)
    return response.status_code == 200

# 悪用例:
# fetch_external_image("http://169.254.169.254/latest/meta-data/iam/security-credentials/")
# proxy_api_request("../../admin/users", "attacker_token")  
# health_check_service("localhost:22")  # ポートスキャン
"#,
        expected_vulnerabilities: vec![VulnType::SSRF],
        expected_confidence_min: 8,
        severity: BenchmarkSeverity::High,
        _description: "SSRF攻撃 - 内部サービスアクセス",
        _real_world_context: "マイクロサービス環境での典型的なSSRF脆弱性。攻撃者が内部ネットワークスキャンやメタデータ窃取可能",
    },

    // === Medium Severity Benchmarks ===
    RealWorldBenchmark {
        name: "IDOR - Direct object reference",
        cve_id: None,
        language: Language::JavaScript,
        code: r#"
// Express.js route handlers

app.get('/api/documents/:docId', (req, res) => {
    const docId = req.params.docId;
    
    // 危険：認可チェックなしの直接アクセス
    const document = database.getDocument(docId);
    if (document) {
        res.json(document);
    } else {
        res.status(404).json({error: 'Document not found'});
    }
});

app.delete('/api/users/:userId/files/:fileId', (req, res) => {
    const userId = req.params.userId;
    const fileId = req.params.fileId;
    
    // さらに危険：ユーザーIDチェックなしの削除
    const deleted = database.deleteFile(fileId);
    if (deleted) {
        res.json({message: 'File deleted successfully'});
    } else {
        res.status(404).json({error: 'File not found'});
    }
});

app.put('/api/profiles/:profileId', (req, res) => {
    const profileId = req.params.profileId;
    const updates = req.body;
    
    // 最も危険：所有者チェックなしのプロファイル更新
    const updated = database.updateProfile(profileId, updates);
    res.json({updated: true, profile: updated});
});

// 悪用例:
// GET /api/documents/12345 (他人の文書アクセス)
// DELETE /api/users/victim/files/important.pdf (他人のファイル削除)
// PUT /api/profiles/admin_profile_id {"role": "admin"} (権限昇格)
"#,
        expected_vulnerabilities: vec![VulnType::IDOR],
        expected_confidence_min: 7,
        severity: BenchmarkSeverity::Medium,
        _description: "IDOR攻撃 - 認可チェック不備",
        _real_world_context: "REST APIでの典型的なIDOR脆弱性。攻撃者が他ユーザーのリソースに直接アクセス可能",
    },

    RealWorldBenchmark {
        name: "Weak file upload validation",
        cve_id: None,
        language: Language::Python,
        code: r#"
import os
from werkzeug.utils import secure_filename

def upload_user_avatar(file, user_id):
    # 弱い検証：拡張子のみチェック
    allowed_extensions = {'.jpg', '.jpeg', '.png', '.gif'}
    filename = file.filename
    
    if not any(filename.lower().endswith(ext) for ext in allowed_extensions):
        return {"error": "Invalid file type"}
    
    # 危険：ファイル名検証不十分
    upload_dir = f"/uploads/avatars/{user_id}/"
    os.makedirs(upload_dir, exist_ok=True)
    
    file_path = os.path.join(upload_dir, filename)
    file.save(file_path)
    
    return {"success": True, "path": file_path}

def process_document_upload(file, document_type):
    # さらに危険：Content-Typeのみ信頼
    allowed_types = ['application/pdf', 'text/plain', 'application/msword']
    
    if file.content_type not in allowed_types:
        return {"error": "Unsupported document type"}
    
    # ファイル内容検証なし
    filename = secure_filename(file.filename)
    upload_path = f"/app/documents/{document_type}/{filename}"
    
    file.save(upload_path)
    return {"uploaded": upload_path}

# 悪用例:
# avatar.php.jpg (二重拡張子)
# document.pdf (実際はPHPスクリプト、Content-Type偽装)
"#,
        expected_vulnerabilities: vec![VulnType::AFO],
        expected_confidence_min: 6,
        severity: BenchmarkSeverity::Medium,
        _description: "ファイルアップロード検証不備",
        _real_world_context: "Webアプリでのファイルアップロード機能における典型的な検証不備。攻撃者が悪意のあるファイル実行可能",
    },

    // === Complex Multi-vulnerability Cases ===
    RealWorldBenchmark {
        name: "E-commerce checkout - Multiple vulnerabilities",
        cve_id: None,
        language: Language::Python,
        code: r#"
import json
import subprocess
import sqlite3

def process_payment(user_id, payment_data, order_details):
    # 1. SQLインジェクション
    user_query = f"SELECT * FROM users WHERE id = {user_id}"
    user = db.execute(user_query).fetchone()
    
    # 2. コマンドインジェクション  
    payment_processor = payment_data.get('processor', 'stripe')
    cmd = f"payment_cli --processor {payment_processor} --amount {order_details['total']}"
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    
    # 3. デシリアライゼーション
    if 'promo_data' in payment_data:
        promo_info = json.loads(payment_data['promo_data'])
        # さらに危険：pickleデータも処理
        if 'serialized_discount' in promo_info:
            import pickle
            discount = pickle.loads(promo_info['serialized_discount'])
    
    # 4. パストラバーサル
    receipt_template = order_details.get('receipt_template', 'default.html')
    with open(f"/app/templates/{receipt_template}", 'r') as f:
        template = f.read()
    
    # 5. SSRF
    webhook_url = payment_data.get('webhook_url')
    if webhook_url:
        import requests
        requests.post(webhook_url, json={"order_id": order_details['id']})
    
    return {"status": "processed", "transaction_id": result.stdout.strip()}

# この一つの関数に複数の深刻な脆弱性が含まれている
"#,
        expected_vulnerabilities: vec![VulnType::SQLI, VulnType::RCE, VulnType::LFI, VulnType::SSRF],
        expected_confidence_min: 8,
        severity: BenchmarkSeverity::Critical,
        _description: "複合脆弱性 - 実際のEコマース決済処理",
        _real_world_context: "実際のEコマースサイトの決済処理に見られる複数脆弱性の組み合わせ。単一関数に多数の攻撃ベクターが存在",
    },
    ]
}

#[derive(Debug)]
struct BenchmarkResult {
    detected_vulnerabilities: Vec<VulnType>,
    confidence_score: i32,
    analysis_quality: f64,
    poc_quality: f64,
    detection_accuracy: f64,
}

async fn test_benchmark_case(
    benchmark: &RealWorldBenchmark,
    model: &str,
) -> Result<BenchmarkResult> {
    // 一時ファイル作成
    let temp_dir = tempdir()?;
    let file_extension = match benchmark.language {
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
    std::fs::write(&test_file, benchmark.code)?;

    // パーサーでコンテキスト構築
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let context = parser.build_context_from_file(&test_file)?;

    // 解析実行
    let response = analyze_file(
        &test_file,
        model,
        &[test_file.clone()],
        0,
        &context,
        0,
        false,
        &None,
        None,
        &LocaleLanguage::Japanese,
    ).await?;

    // 結果分析
    let detected_vulnerabilities = response.vulnerability_types;
    let confidence_score = response.confidence_score;

    // 検出精度計算
    let expected_set: std::collections::HashSet<_> = benchmark.expected_vulnerabilities.iter().collect();
    let detected_set: std::collections::HashSet<_> = detected_vulnerabilities.iter().collect();
    
    let true_positives = expected_set.intersection(&detected_set).count();
    let _false_positives = detected_set.difference(&expected_set).count();
    let _false_negatives = expected_set.difference(&detected_set).count();
    
    let precision = if detected_set.len() > 0 {
        true_positives as f64 / detected_set.len() as f64
    } else {
        if expected_set.is_empty() { 1.0 } else { 0.0 }
    };
    
    let recall = if expected_set.len() > 0 {
        true_positives as f64 / expected_set.len() as f64
    } else {
        1.0
    };
    
    let f1_score = if precision + recall > 0.0 {
        2.0 * (precision * recall) / (precision + recall)
    } else {
        0.0
    };

    // 解析品質評価（簡易版）
    let analysis_quality = if response.analysis.len() > 100 && 
                             response.analysis.to_lowercase().contains("脆弱") {
        85.0
    } else if response.analysis.len() > 50 {
        70.0
    } else {
        40.0
    };

    // PoC品質評価（簡易版）
    let poc_quality = if response.poc.len() > 50 &&
                        (response.poc.contains("curl") || 
                         response.poc.contains("SELECT") ||
                         response.poc.contains("<script>") ||
                         response.poc.contains("../") ||
                         response.poc.contains("http://")) {
        80.0
    } else if response.poc.len() > 20 {
        60.0
    } else {
        30.0
    };

    Ok(BenchmarkResult {
        detected_vulnerabilities,
        confidence_score,
        analysis_quality,
        poc_quality,
        detection_accuracy: f1_score * 100.0,
    })
}

#[tokio::test]
async fn test_critical_severity_benchmarks() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping critical severity benchmark test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let benchmarks = get_real_world_benchmarks();
    let critical_benchmarks: Vec<_> = benchmarks
        .iter()
        .filter(|b| matches!(b.severity, BenchmarkSeverity::Critical))
        .collect();

    println!("🔥 クリティカル脆弱性ベンチマークテスト: {}ケース", critical_benchmarks.len());

    let mut detection_scores = Vec::new();
    let mut confidence_scores = Vec::new();
    let mut failed_cases = Vec::new();

    for benchmark in critical_benchmarks {
        println!("  テスト中: {} - {}", benchmark.name, 
                if let Some(cve) = benchmark.cve_id { cve } else { "No CVE" });
        
        let result = test_benchmark_case(benchmark, model).await?;
        
        detection_scores.push(result.detection_accuracy);
        confidence_scores.push(result.confidence_score as f64);

        if result.detection_accuracy >= 90.0 && result.confidence_score >= benchmark.expected_confidence_min {
            println!("    ✅ 検出成功: {:.1}% (信頼度={})", 
                    result.detection_accuracy, result.confidence_score);
            println!("       検出された脆弱性: {:?}", result.detected_vulnerabilities);
        } else {
            println!("    ❌ 検出失敗: {:.1}% (信頼度={})", 
                    result.detection_accuracy, result.confidence_score);
            println!("       期待: {:?}", benchmark.expected_vulnerabilities);
            println!("       実際: {:?}", result.detected_vulnerabilities);
            failed_cases.push(benchmark.name);
        }
    }

    let avg_detection = detection_scores.iter().sum::<f64>() / detection_scores.len() as f64;
    let avg_confidence = confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64;

    println!("\n📊 クリティカル脆弱性ベンチマーク結果:");
    println!("  平均検出精度: {:.1}%", avg_detection);
    println!("  平均信頼度: {:.1}", avg_confidence);

    if !failed_cases.is_empty() {
        println!("\n❌ 失敗したケース:");
        for case in &failed_cases {
            println!("    - {}", case);
        }
    }

    // クリティカル脆弱性は95%以上の検出精度を要求
    assert!(
        avg_detection >= 95.0,
        "クリティカル脆弱性検出精度が基準を下回っています: {:.1}% (要求: 95.0%)",
        avg_detection
    );

    println!("\n🎉 クリティカル脆弱性ベンチマークテスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_benchmark_suite() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping comprehensive benchmark test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let benchmarks = get_real_world_benchmarks();
    println!("🧪 包括的ベンチマークスイート: {}ケース", benchmarks.len());

    let mut severity_stats = HashMap::new();
    let mut total_detection_score = 0.0;
    let mut total_analysis_quality = 0.0;
    let mut total_poc_quality = 0.0;
    let mut total_tests = 0;

    for benchmark in &benchmarks {
        println!(
            "  [{}/{}] テスト中: {}",
            total_tests + 1,
            benchmarks.len(),
            benchmark.name
        );

        let result = test_benchmark_case(benchmark, model).await?;

        // 統計更新
        let severity_key = format!("{:?}", benchmark.severity);
        let entry = severity_stats.entry(severity_key).or_insert((0.0, 0.0, 0.0, 0));
        entry.0 += result.detection_accuracy;
        entry.1 += result.analysis_quality;
        entry.2 += result.poc_quality;
        entry.3 += 1;

        total_detection_score += result.detection_accuracy;
        total_analysis_quality += result.analysis_quality;
        total_poc_quality += result.poc_quality;
        total_tests += 1;
    }

    let avg_detection = total_detection_score / total_tests as f64;
    let avg_analysis = total_analysis_quality / total_tests as f64;
    let avg_poc = total_poc_quality / total_tests as f64;

    println!("\n📊 包括的ベンチマーク結果:");
    println!("  全体検出精度: {:.1}%", avg_detection);
    println!("  全体解析品質: {:.1}%", avg_analysis);
    println!("  全体PoC品質: {:.1}%", avg_poc);

    println!("\n深刻度別結果:");
    for (severity, (detection_sum, analysis_sum, poc_sum, count)) in severity_stats {
        println!("  {}: 検出={:.1}%, 解析={:.1}%, PoC={:.1}% ({}ケース)",
                severity,
                detection_sum / count as f64,
                analysis_sum / count as f64,
                poc_sum / count as f64,
                count);
    }

    // 総合スコア計算（重み付き平均）
    let comprehensive_score = (avg_detection * 0.5) + (avg_analysis * 0.3) + (avg_poc * 0.2);

    println!("\n総合評価スコア: {:.1}%", comprehensive_score);

    // 包括的ベンチマークは85%以上のスコアを要求
    assert!(
        comprehensive_score >= 85.0,
        "包括的ベンチマークスコアが基準を下回っています: {:.1}% (要求: 85.0%)",
        comprehensive_score
    );

    println!("\n🎉 包括的ベンチマークスイート合格!");
    Ok(())
}

#[tokio::test]
async fn test_multi_vulnerability_detection() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping multi-vulnerability test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let benchmarks = get_real_world_benchmarks();
    // 複数脆弱性を含むケースのみテスト
    let multi_vuln_cases: Vec<_> = benchmarks
        .iter()
        .filter(|b| b.expected_vulnerabilities.len() > 1)
        .collect();

    println!("🔍 複数脆弱性検出テスト: {}ケース", multi_vuln_cases.len());

    let mut total_recall = 0.0;
    let mut total_precision = 0.0;
    let mut total_tests = 0;

    for benchmark in multi_vuln_cases {
        println!("  テスト中: {} (期待脆弱性数: {})", 
                benchmark.name, benchmark.expected_vulnerabilities.len());
        
        let result = test_benchmark_case(benchmark, model).await?;

        let expected_set: std::collections::HashSet<_> = benchmark.expected_vulnerabilities.iter().collect();
        let detected_set: std::collections::HashSet<_> = result.detected_vulnerabilities.iter().collect();
        
        let true_positives = expected_set.intersection(&detected_set).count();
        let false_positives = detected_set.difference(&expected_set).count();
        let false_negatives = expected_set.difference(&detected_set).count();
        
        let precision = if detected_set.len() > 0 {
            true_positives as f64 / detected_set.len() as f64
        } else {
            0.0
        };
        
        let recall = if expected_set.len() > 0 {
            true_positives as f64 / expected_set.len() as f64
        } else {
            1.0
        };

        total_recall += recall;
        total_precision += precision;
        total_tests += 1;

        println!("    再現率: {:.1}%, 適合率: {:.1}% (TP={}, FP={}, FN={})",
                recall * 100.0, precision * 100.0, true_positives, false_positives, false_negatives);
    }

    let avg_recall = (total_recall / total_tests as f64) * 100.0;
    let avg_precision = (total_precision / total_tests as f64) * 100.0;
    let f1_score = if avg_recall + avg_precision > 0.0 {
        2.0 * (avg_recall * avg_precision) / (avg_recall + avg_precision)
    } else {
        0.0
    };

    println!("\n📊 複数脆弱性検出結果:");
    println!("  平均再現率 (Recall): {:.1}%", avg_recall);
    println!("  平均適合率 (Precision): {:.1}%", avg_precision);
    println!("  F1スコア: {:.1}%", f1_score);

    // 複数脆弱性検出はF1スコア80%以上を要求
    assert!(
        f1_score >= 80.0,
        "複数脆弱性検出F1スコアが基準を下回っています: {:.1}% (要求: 80.0%)",
        f1_score
    );

    println!("✅ 複数脆弱性検出テスト合格!");
    Ok(())
}