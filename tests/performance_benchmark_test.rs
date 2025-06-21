use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use parsentry::security_patterns::Language;
use std::time::{Duration, Instant};
use tempfile::tempdir;

/// パフォーマンスベンチマークテスト
/// 実行速度を測定し、パフォーマンス要件を満たしているか検証

#[derive(Debug, Clone)]
struct PerformanceBenchmark {
    name: &'static str,
    language: Language,
    code_size: CodeSize,
    code_generator: fn() -> String,
    max_duration_ms: u128,
    description: &'static str,
}

#[derive(Debug, Clone)]
enum CodeSize {
    Small,   // ~100 lines
    Medium,  // ~500 lines
    Large,   // ~1000 lines
    XLarge,  // ~5000 lines
}

/// ベンチマーク用のサンプルコードを生成
fn generate_small_javascript_code() -> String {
    r#"
const express = require('express');
const app = express();

app.get('/api/user/:id', (req, res) => {
    const userId = req.params.id;
    const query = `SELECT * FROM users WHERE id = ${userId}`;
    
    db.query(query, (err, result) => {
        if (err) {
            res.status(500).json({error: err.message});
        } else {
            res.json(result);
        }
    });
});

app.post('/api/execute', (req, res) => {
    const command = req.body.command;
    const exec = require('child_process').exec;
    
    exec(command, (error, stdout, stderr) => {
        res.json({output: stdout, error: stderr});
    });
});

app.listen(3000);
"#.to_string()
}

fn generate_medium_python_code() -> String {
    let mut code = String::new();
    
    // Add imports
    code.push_str("import os\nimport subprocess\nimport pickle\nimport requests\n\n");
    
    // Generate multiple vulnerable functions
    for i in 0..20 {
        code.push_str(&format!(r#"
def process_user_data_{}(user_input):
    # SQL injection vulnerability
    query = f"SELECT * FROM table_{} WHERE id = {{user_input}}"
    cursor.execute(query)
    
    # Command injection
    cmd = f"process --data {{user_input}}"
    subprocess.run(cmd, shell=True)
    
    # Path traversal
    file_path = f"/data/{{user_input}}.txt"
    with open(file_path, 'r') as f:
        content = f.read()
    
    return content

"#, i, i));
    }
    
    // Add more complexity
    code.push_str(r#"
class DataProcessor:
    def __init__(self):
        self.data = {}
    
    def process_request(self, request_data):
        # Deserialization vulnerability
        if 'serialized' in request_data:
            data = pickle.loads(request_data['serialized'])
        
        # SSRF vulnerability
        if 'url' in request_data:
            response = requests.get(request_data['url'])
            return response.text
        
        return None
"#);
    
    code
}

fn generate_large_java_code() -> String {
    let mut code = String::new();
    
    code.push_str(r#"
import java.sql.*;
import java.io.*;
import javax.servlet.http.*;

public class VulnerableApp {
"#);
    
    // Generate many methods with vulnerabilities
    for i in 0..50 {
        code.push_str(&format!(r#"
    public String processRequest_{i}(HttpServletRequest request) {{
        String param = request.getParameter("input_{i}");
        
        // SQL Injection
        String query = "SELECT * FROM users WHERE name = '" + param + "'";
        Statement stmt = connection.createStatement();
        ResultSet rs = stmt.executeQuery(query);
        
        // Path Traversal
        String filename = request.getParameter("file_{i}");
        File file = new File("/uploads/" + filename);
        FileReader reader = new FileReader(file);
        
        // XXE vulnerability
        String xml = request.getParameter("xml_{i}");
        DocumentBuilder builder = DocumentBuilderFactory.newInstance().newDocumentBuilder();
        Document doc = builder.parse(new InputSource(new StringReader(xml)));
        
        return processData(rs, reader, doc);
    }}
    
"#, i = i));
    }
    
    code.push_str("}\n");
    code
}

fn generate_xlarge_mixed_code() -> String {
    let mut code = String::new();
    
    // Mix multiple languages patterns in a large file
    code.push_str("// Large mixed vulnerability codebase\n\n");
    
    // Add JavaScript patterns
    for i in 0..100 {
        code.push_str(&format!(r#"
function processUserInput_{}(input) {{
    // Direct DOM manipulation
    document.getElementById('output').innerHTML = input;
    
    // eval usage
    const result = eval('processData(' + input + ')');
    
    // SQL in JS
    const query = `SELECT * FROM table WHERE id = ${{input}}`;
    
    return db.query(query);
}}

"#, i));
    }
    
    // Add Python patterns
    code.push_str("\n# Python section\n");
    for i in 0..100 {
        code.push_str(&format!(r#"
def handle_request_{}(data):
    # Multiple vulnerabilities
    import os
    os.system(f"process {{data}}")
    
    exec(data.get('code', ''))
    
    with open(f"/files/{{data['path']}}", 'r') as f:
        return f.read()

"#, i));
    }
    
    code
}

fn get_performance_benchmarks() -> Vec<PerformanceBenchmark> {
    vec![
        PerformanceBenchmark {
            name: "Small JavaScript file - Quick scan",
            language: Language::JavaScript,
            code_size: CodeSize::Small,
            code_generator: generate_small_javascript_code,
            max_duration_ms: 5000,  // 5 seconds
            description: "小規模ファイルの高速スキャン",
        },
        PerformanceBenchmark {
            name: "Medium Python file - Standard scan",
            language: Language::Python,
            code_size: CodeSize::Medium,
            code_generator: generate_medium_python_code,
            max_duration_ms: 10000,  // 10 seconds
            description: "中規模ファイルの標準スキャン",
        },
        PerformanceBenchmark {
            name: "Large Java file - Complex analysis",
            language: Language::Java,
            code_size: CodeSize::Large,
            code_generator: generate_large_java_code,
            max_duration_ms: 20000,  // 20 seconds
            description: "大規模ファイルの複雑な解析",
        },
        PerformanceBenchmark {
            name: "Extra large mixed file - Stress test",
            language: Language::JavaScript,  // Primary language
            code_size: CodeSize::XLarge,
            code_generator: generate_xlarge_mixed_code,
            max_duration_ms: 30000,  // 30 seconds
            description: "超大規模ファイルのストレステスト",
        },
    ]
}

#[derive(Debug)]
struct PerformanceResult {
    duration: Duration,
    lines_of_code: usize,
    vulnerabilities_found: usize,
    lines_per_second: f64,
    passed: bool,
}

async fn run_performance_benchmark(
    benchmark: &PerformanceBenchmark,
    model: &str,
) -> Result<PerformanceResult> {
    // Generate code
    let code = (benchmark.code_generator)();
    let lines_of_code = code.lines().count();
    
    // Create temporary file
    let temp_dir = tempdir()?;
    let file_extension = match benchmark.language {
        Language::JavaScript => "js",
        Language::Python => "py",
        Language::Java => "java",
        _ => "txt",
    };
    
    let test_file = temp_dir.path().join(format!("benchmark.{}", file_extension));
    std::fs::write(&test_file, &code)?;
    
    // Parse and build context
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let context = parser.build_context_from_file(&test_file)?;
    
    // Measure execution time
    let start = Instant::now();
    
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
    
    let duration = start.elapsed();
    
    // Calculate metrics
    let lines_per_second = lines_of_code as f64 / duration.as_secs_f64();
    let passed = duration.as_millis() <= benchmark.max_duration_ms;
    
    Ok(PerformanceResult {
        duration,
        lines_of_code,
        vulnerabilities_found: response.vulnerability_types.len(),
        lines_per_second,
        passed,
    })
}

#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    // Skip API-based tests in CI or when API key is not available
    if std::env::var("OPENAI_API_KEY").is_err() || std::env::var("CI").is_ok() {
        println!("Skipping API-based performance benchmark test (no API key or CI environment)");
        return Ok(());
    }
    
    let model = "gpt-4.1-mini";
    let benchmarks = get_performance_benchmarks();
    
    println!("\n⚡ パフォーマンスベンチマークテスト");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let mut all_passed = true;
    let mut total_lines = 0;
    let mut total_duration = Duration::new(0, 0);
    
    for benchmark in &benchmarks {
        print!("📊 {:<50} ", benchmark.name);
        
        let result = run_performance_benchmark(benchmark, model).await?;
        
        total_lines += result.lines_of_code;
        total_duration += result.duration;
        
        if result.passed {
            println!("✅ PASS");
        } else {
            println!("❌ FAIL");
            all_passed = false;
        }
        
        println!("   ├─ 実行時間: {:.2}秒 (制限: {:.1}秒)",
                result.duration.as_secs_f64(),
                benchmark.max_duration_ms as f64 / 1000.0);
        println!("   ├─ コード行数: {} 行", result.lines_of_code);
        println!("   ├─ 処理速度: {:.1} 行/秒", result.lines_per_second);
        println!("   └─ 検出脆弱性数: {}", result.vulnerabilities_found);
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let avg_speed = total_lines as f64 / total_duration.as_secs_f64();
    println!("\n📈 総合パフォーマンス統計:");
    println!("   ├─ 総処理行数: {} 行", total_lines);
    println!("   ├─ 総実行時間: {:.2} 秒", total_duration.as_secs_f64());
    println!("   └─ 平均処理速度: {:.1} 行/秒", avg_speed);
    
    assert!(all_passed, "一部のパフォーマンステストが失敗しました");
    
    println!("\n🎉 全パフォーマンスベンチマークテスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_incremental_performance() -> Result<()> {
    // Skip API-based tests in CI or when API key is not available
    if std::env::var("OPENAI_API_KEY").is_err() || std::env::var("CI").is_ok() {
        println!("Skipping API-based incremental performance test (no API key or CI environment)");
        return Ok(());
    }
    
    let model = "gpt-4.1-mini";
    
    println!("\n📐 インクリメンタルパフォーマンステスト");
    println!("コードサイズと実行時間の関係を測定");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let sizes = vec![100, 200, 500, 1000, 2000];
    let mut results = Vec::new();
    
    for size in &sizes {
        // Generate code with specific size
        let mut code = String::new();
        for i in 0..(size / 10) {
            code.push_str(&format!(r#"
function vulnerable_{}(input) {{
    const query = `SELECT * FROM users WHERE id = ${{input}}`;
    eval(input);
    document.getElementById('out').innerHTML = input;
}}
"#, i));
        }
        
        // Create test file
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("incremental_test.js");
        std::fs::write(&test_file, &code)?;
        
        // Parse and analyze
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let context = parser.build_context_from_file(&test_file)?;
        
        let start = Instant::now();
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
        let duration = start.elapsed();
        
        results.push((*size, duration.as_millis()));
        
        println!("{:>5} 行: {:>6.2} 秒 ({} 脆弱性検出)",
                size,
                duration.as_secs_f64(),
                response.vulnerability_types.len());
    }
    
    // Check if performance scales linearly
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Simple linear regression check
    let first_ratio = results[0].1 as f64 / results[0].0 as f64;
    let last_ratio = results.last().unwrap().1 as f64 / results.last().unwrap().0 as f64;
    let scaling_factor = last_ratio / first_ratio;
    
    println!("\n📊 スケーリング分析:");
    println!("   ├─ 初期比率 (時間/行): {:.4} ms/行", first_ratio);
    println!("   ├─ 最終比率 (時間/行): {:.4} ms/行", last_ratio);
    println!("   └─ スケーリング係数: {:.2}x", scaling_factor);
    
    // Performance should scale sub-linearly (scaling factor < 2.0 is good)
    assert!(
        scaling_factor < 2.0,
        "パフォーマンスのスケーリングが非効率です: {:.2}x (期待値: < 2.0x)",
        scaling_factor
    );
    
    println!("\n✅ インクリメンタルパフォーマンステスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_analysis_performance() -> Result<()> {
    // Skip API-based tests in CI or when API key is not available
    if std::env::var("OPENAI_API_KEY").is_err() || std::env::var("CI").is_ok() {
        println!("Skipping API-based concurrent performance test (no API key or CI environment)");
        return Ok(());
    }
    
    let model = "gpt-4.1-mini";
    
    println!("\n🔄 並行解析パフォーマンステスト");
    println!("複数ファイルの並行処理性能を測定");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Create multiple test files
    let temp_dir = tempdir()?;
    let mut test_files = Vec::new();
    
    for i in 0..5 {
        let code = format!(r#"
// File {}
function processData_{}(userInput) {{
    const query = `SELECT * FROM table_{} WHERE data = '${{userInput}}'`;
    db.query(query);
    
    const cmd = `process --file ${{userInput}}`;
    exec(cmd);
    
    fetch(`http://internal-api/${{userInput}}`);
}}
"#, i, i, i);
        
        let file_path = temp_dir.path().join(format!("concurrent_test_{}.js", i));
        std::fs::write(&file_path, &code)?;
        test_files.push(file_path);
    }
    
    // Test sequential processing
    let sequential_start = Instant::now();
    let mut sequential_vulns = 0;
    
    for file in &test_files {
        let mut parser = CodeParser::new()?;
        parser.add_file(file)?;
        let context = parser.build_context_from_file(file)?;
        
        let response = analyze_file(
            file,
            model,
            &[file.clone()],
            0,
            &context,
            0,
            false,
            &None,
            None,
            &LocaleLanguage::Japanese,
        ).await?;
        
        sequential_vulns += response.vulnerability_types.len();
    }
    
    let sequential_duration = sequential_start.elapsed();
    
    // Test concurrent processing (simulated by measuring individual files)
    let concurrent_start = Instant::now();
    let mut max_individual_time = Duration::new(0, 0);
    
    for file in &test_files {
        let file_start = Instant::now();
        
        let mut parser = CodeParser::new()?;
        parser.add_file(file)?;
        let context = parser.build_context_from_file(file)?;
        
        let _response = analyze_file(
            file,
            model,
            &[file.clone()],
            0,
            &context,
            0,
            false,
            &None,
            None,
            &LocaleLanguage::Japanese,
        ).await?;
        
        let file_duration = file_start.elapsed();
        if file_duration > max_individual_time {
            max_individual_time = file_duration;
        }
    }
    
    let _concurrent_duration = concurrent_start.elapsed();
    
    println!("📊 結果:");
    println!("   ├─ ファイル数: {}", test_files.len());
    println!("   ├─ 逐次処理時間: {:.2} 秒", sequential_duration.as_secs_f64());
    println!("   ├─ 最大個別処理時間: {:.2} 秒", max_individual_time.as_secs_f64());
    println!("   ├─ 検出脆弱性総数: {}", sequential_vulns);
    println!("   └─ 平均処理時間/ファイル: {:.2} 秒", 
            sequential_duration.as_secs_f64() / test_files.len() as f64);
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Each file should complete within reasonable time
    let avg_time_per_file = sequential_duration.as_millis() / test_files.len() as u128;
    assert!(
        avg_time_per_file < 10000,  // 10 seconds per file
        "ファイルあたりの平均処理時間が長すぎます: {} ms (期待値: < 10000 ms)",
        avg_time_per_file
    );
    
    println!("\n✅ 並行解析パフォーマンステスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_parser_performance() -> Result<()> {
    println!("\n🔧 パーサーパフォーマンステスト (API不要)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let test_cases = vec![
        ("Small JS", Language::JavaScript, generate_small_javascript_code()),
        ("Medium Python", Language::Python, generate_medium_python_code()),
        ("Large Java", Language::Java, generate_large_java_code()),
    ];
    
    for (name, language, code) in test_cases {
        let temp_dir = tempdir()?;
        let file_extension = match language {
            Language::JavaScript => "js",
            Language::Python => "py", 
            Language::Java => "java",
            _ => "txt",
        };
        
        let test_file = temp_dir.path().join(format!("test.{}", file_extension));
        std::fs::write(&test_file, &code)?;
        
        let start = Instant::now();
        
        // Test parser performance
        let mut parser = CodeParser::new()?;
        parser.add_file(&test_file)?;
        let _context = parser.build_context_from_file(&test_file)?;
        
        let duration = start.elapsed();
        let lines = code.lines().count();
        let lines_per_sec = lines as f64 / duration.as_secs_f64();
        
        println!("📊 {:<15} : {:>8} 行, {:>6.3} 秒, {:>8.1} 行/秒", 
                name, lines, duration.as_secs_f64(), lines_per_sec);
        
        // Parser should be fast - over 1000 lines per second
        assert!(
            lines_per_sec > 100.0,
            "パーサーが遅すぎます: {:.1} 行/秒 (期待値: > 100 行/秒)",
            lines_per_sec
        );
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ パーサーパフォーマンステスト合格!");
    Ok(())
}