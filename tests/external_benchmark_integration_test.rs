use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use parsentry::response::VulnType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 外部ベンチマーク統合テスト
/// OSSF CVE Benchmark と Validation Benchmarks を使用した実世界精度測定
/// 業界標準ベンチマークでの性能を評価し、競合ツールとの比較を可能にする

#[derive(Debug, Deserialize, Serialize)]
struct ValidationBenchmark {
    #[serde(rename = "benchmarkId")]
    benchmark_id: String,
    title: String,
    description: String,
    #[serde(rename = "vulnerabilityType")]
    vulnerability_type: String,
    #[serde(rename = "attackVector")]
    attack_vector: Vec<String>,
    #[serde(rename = "riskLevel")]
    risk_level: String,
    #[serde(rename = "testFiles")]
    test_files: Vec<TestFile>,
    #[serde(rename = "expectedFindings")]
    expected_findings: Vec<ExpectedFinding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestFile {
    path: String,
    language: String,
    #[serde(rename = "isVulnerable")]
    is_vulnerable: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExpectedFinding {
    file: String,
    #[serde(rename = "vulnerabilityTypes")]
    vulnerability_types: Vec<String>,
    #[serde(rename = "confidenceMin")]
    confidence_min: Option<i32>,
    line: Option<i32>,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OssfCveBenchmark {
    id: String,
    #[serde(rename = "cveId")]
    cve_id: Option<String>,
    title: String,
    description: String,
    language: String,
    #[serde(rename = "vulnerabilityClass")]
    vulnerability_class: String,
    severity: String,
    #[serde(rename = "sourceFile")]
    source_file: String,
    #[serde(rename = "vulnerableLine")]
    vulnerable_line: Option<i32>,
    #[serde(rename = "fixCommit")]
    fix_commit: Option<String>,
}

#[derive(Debug)]
struct BenchmarkResult {
    benchmark_id: String,
    detected: bool,
    confidence_score: i32,
    detected_types: Vec<VulnType>,
    analysis_quality: f64,
    false_positive: bool,
    false_negative: bool,
    execution_time_ms: u128,
}

#[derive(Debug)]
struct BenchmarkSummary {
    total_benchmarks: usize,
    true_positives: usize,
    false_positives: usize,
    true_negatives: usize,
    false_negatives: usize,
    precision: f64,
    recall: f64,
    f1_score: f64,
    accuracy: f64,
    avg_confidence: f64,
    avg_execution_time_ms: f64,
}

fn map_vulnerability_type(vuln_type: &str) -> Option<VulnType> {
    match vuln_type.to_uppercase().as_str() {
        "SQL_INJECTION" | "SQLI" | "SQL" => Some(VulnType::SQLI),
        "COMMAND_INJECTION" | "RCE" | "CODE_EXECUTION" => Some(VulnType::RCE),
        "XSS" | "CROSS_SITE_SCRIPTING" => Some(VulnType::XSS),
        "LFI" | "LOCAL_FILE_INCLUSION" | "PATH_TRAVERSAL" => Some(VulnType::LFI),
        "SSRF" | "SERVER_SIDE_REQUEST_FORGERY" => Some(VulnType::SSRF),
        "AFO" | "ARBITRARY_FILE_OPERATION" => Some(VulnType::AFO),
        "IDOR" | "INSECURE_DIRECT_OBJECT_REFERENCE" => Some(VulnType::IDOR),
        _ => None,
    }
}

fn get_benchmarks_directory() -> PathBuf {
    // プロジェクトルートからの相対パス
    PathBuf::from("benchmarks")
}

fn clone_validation_benchmarks() -> Result<()> {
    let benchmarks_dir = get_benchmarks_directory();
    
    if benchmarks_dir.exists() {
        println!("📁 ベンチマークディレクトリが既に存在します: {:?}", benchmarks_dir);
        return Ok(());
    }

    println!("📥 Validation Benchmarks をクローン中...");
    let output = Command::new("git")
        .args(&[
            "clone",
            "https://github.com/xbow-engineering/validation-benchmarks.git",
            benchmarks_dir.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("ベンチマーククローンに失敗: {}", stderr));
    }

    println!("✅ Validation Benchmarks クローン完了");
    Ok(())
}

fn clone_ossf_cve_benchmark() -> Result<()> {
    let ossf_dir = PathBuf::from("ossf-cve-benchmark");
    
    if ossf_dir.exists() {
        println!("📁 OSSF CVE ベンチマークディレクトリが既に存在します: {:?}", ossf_dir);
        return Ok(());
    }

    println!("📥 OSSF CVE Benchmark をクローン中...");
    let output = Command::new("git")
        .args(&[
            "clone",
            "https://github.com/ossf-cve-benchmark/ossf-cve-benchmark.git",
            "ossf-cve-benchmark",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("OSSF CVE ベンチマーククローンに失敗: {}", stderr));
    }

    println!("✅ OSSF CVE Benchmark クローン完了");
    Ok(())
}

fn discover_ossf_cve_benchmarks() -> Result<Vec<PathBuf>> {
    let ossf_dir = PathBuf::from("ossf-cve-benchmark");
    
    if !ossf_dir.exists() {
        clone_ossf_cve_benchmark()?;
    }

    let mut benchmark_files = Vec::new();
    
    // CVE benchmarkファイルを検索
    if let Ok(entries) = std::fs::read_dir(&ossf_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // サブディレクトリ内のCVEファイルを検索
                if let Ok(sub_entries) = std::fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file() {
                            if let Some(extension) = sub_path.extension() {
                                if matches!(extension.to_str(), Some("java") | Some("py") | Some("js") | Some("c") | Some("cpp")) {
                                    benchmark_files.push(sub_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    benchmark_files.sort();
    println!("🔍 発見されたOSSF CVE Benchmarks: {}個", benchmark_files.len());
    
    Ok(benchmark_files)
}

fn load_ossf_cve_benchmark(benchmark_file: &Path) -> Result<Option<OssfCveBenchmark>> {
    // OSSF CVE Benchmarkのメタデータ読み込み（簡易版）
    let file_name = benchmark_file.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    // ファイル内容からCVE情報を推測
    let content = std::fs::read_to_string(benchmark_file)?;
    
    // CVE-YYYY-NNNN パターンを検索
    let cve_regex = regex::Regex::new(r"CVE-\d{4}-\d{4,}").unwrap();
    let cve_id = cve_regex.find(&content)
        .map(|m| m.as_str().to_string());

    // 言語をファイル拡張子から判定
    let language = match benchmark_file.extension().and_then(|ext| ext.to_str()) {
        Some("java") => "Java",
        Some("py") => "Python", 
        Some("js") => "JavaScript",
        Some("c") => "C",
        Some("cpp") | Some("cc") | Some("cxx") => "C++",
        _ => "Unknown",
    }.to_string();

    // 脆弱性クラスを内容から推測
    let vulnerability_class = if content.to_lowercase().contains("sql") {
        "SQL Injection"
    } else if content.to_lowercase().contains("exec") || content.to_lowercase().contains("system") {
        "Command Injection"
    } else if content.to_lowercase().contains("script") || content.to_lowercase().contains("innerHTML") {
        "Cross-site Scripting"
    } else if content.to_lowercase().contains("file") || content.to_lowercase().contains("path") {
        "Path Traversal"
    } else {
        "Other"
    }.to_string();

    Ok(Some(OssfCveBenchmark {
        id: file_name.to_string(),
        cve_id,
        title: format!("CVE Test: {}", file_name),
        description: format!("OSSF CVE benchmark for {}", language),
        language,
        vulnerability_class,
        severity: "High".to_string(), // デフォルト
        source_file: benchmark_file.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string(),
        vulnerable_line: None,
        fix_commit: None,
    }))
}

async fn test_ossf_cve_benchmark(
    benchmark: &OssfCveBenchmark,
    benchmark_file: &Path,
    model: &str,
) -> Result<BenchmarkResult> {
    let start_time = std::time::Instant::now();

    // パーサーでコンテキスト構築
    let mut parser = CodeParser::new()?;
    parser.add_file(benchmark_file)?;
    let context = parser.build_context_from_file(benchmark_file)?;

    // 解析実行
    let response = analyze_file(
        &benchmark_file.to_path_buf(),
        model,
        &[benchmark_file.to_path_buf()],
        0,
        &context,
        0,
        false,
        &None,
        None,
        &LocaleLanguage::Japanese,
    ).await?;

    let execution_time = start_time.elapsed().as_millis();

    let detected = !response.vulnerability_types.is_empty();
    let should_be_vulnerable = true; // OSSF CVE benchmarkは基本的に脆弱性を含む

    let false_positive = false; // CVEファイルなので偽陽性は基本的にない
    let false_negative = !detected && should_be_vulnerable;

    let analysis_quality = if response.analysis.len() > 100 {
        85.0
    } else if response.analysis.len() > 50 {
        70.0
    } else {
        40.0
    };

    Ok(BenchmarkResult {
        benchmark_id: benchmark.id.clone(),
        detected,
        confidence_score: response.confidence_score,
        detected_types: response.vulnerability_types,
        analysis_quality,
        false_positive,
        false_negative,
        execution_time_ms: execution_time,
    })
}

fn discover_validation_benchmarks() -> Result<Vec<PathBuf>> {
    let benchmarks_dir = get_benchmarks_directory();
    
    if !benchmarks_dir.exists() {
        clone_validation_benchmarks()?;
    }

    let mut benchmark_dirs = Vec::new();
    
    // XBEN-XXX-24 パターンのディレクトリを検索
    if let Ok(entries) = std::fs::read_dir(&benchmarks_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if let Some(name_str) = dir_name.to_str() {
                        if name_str.starts_with("XBEN-") && name_str.ends_with("-24") {
                            benchmark_dirs.push(path);
                        }
                    }
                }
            }
        }
    }

    benchmark_dirs.sort();
    println!("🔍 発見されたValidation Benchmarks: {}個", benchmark_dirs.len());
    
    Ok(benchmark_dirs)
}

fn load_validation_benchmark(benchmark_dir: &Path) -> Result<Option<ValidationBenchmark>> {
    let benchmark_json = benchmark_dir.join("benchmark.json");
    
    if !benchmark_json.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&benchmark_json)?;
    let benchmark: ValidationBenchmark = serde_json::from_str(&content)?;
    
    Ok(Some(benchmark))
}

async fn test_validation_benchmark(
    benchmark: &ValidationBenchmark,
    benchmark_dir: &Path,
    model: &str,
) -> Result<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    for test_file in &benchmark.test_files {
        let file_path = benchmark_dir.join(&test_file.path);
        
        if !file_path.exists() {
            continue;
        }

        let start_time = std::time::Instant::now();

        // パーサーでコンテキスト構築
        let mut parser = CodeParser::new()?;
        parser.add_file(&file_path)?;
        let context = parser.build_context_from_file(&file_path)?;

        // 解析実行
        let response = analyze_file(
            &file_path,
            model,
            &[file_path.clone()],
            0,
            &context,
            0,
            false,
            &None,
            None,
            &LocaleLanguage::Japanese,
        ).await?;

        let execution_time = start_time.elapsed().as_millis();

        // 期待される脆弱性タイプの解析
        let _expected_vuln_types: Vec<VulnType> = benchmark.expected_findings
            .iter()
            .filter(|f| f.file == test_file.path)
            .flat_map(|f| f.vulnerability_types.iter())
            .filter_map(|vt| map_vulnerability_type(vt))
            .collect();

        let detected = !response.vulnerability_types.is_empty();
        let should_be_vulnerable = test_file.is_vulnerable;

        let false_positive = detected && !should_be_vulnerable;
        let false_negative = !detected && should_be_vulnerable;

        let analysis_quality = if response.analysis.len() > 100 {
            85.0
        } else if response.analysis.len() > 50 {
            70.0
        } else {
            40.0
        };

        results.push(BenchmarkResult {
            benchmark_id: format!("{}:{}", benchmark.benchmark_id, test_file.path),
            detected,
            confidence_score: response.confidence_score,
            detected_types: response.vulnerability_types,
            analysis_quality,
            false_positive,
            false_negative,
            execution_time_ms: execution_time,
        });
    }

    Ok(results)
}

fn calculate_benchmark_summary(results: &[BenchmarkResult]) -> BenchmarkSummary {
    let total_benchmarks = results.len();
    let true_positives = results.iter().filter(|r| r.detected && !r.false_positive).count();
    let false_positives = results.iter().filter(|r| r.false_positive).count();
    let true_negatives = results.iter().filter(|r| !r.detected && !r.false_negative).count();
    let false_negatives = results.iter().filter(|r| r.false_negative).count();

    let precision = if true_positives + false_positives > 0 {
        true_positives as f64 / (true_positives + false_positives) as f64
    } else {
        0.0
    };

    let recall = if true_positives + false_negatives > 0 {
        true_positives as f64 / (true_positives + false_negatives) as f64
    } else {
        0.0
    };

    let f1_score = if precision + recall > 0.0 {
        2.0 * (precision * recall) / (precision + recall)
    } else {
        0.0
    };

    let accuracy = if total_benchmarks > 0 {
        (true_positives + true_negatives) as f64 / total_benchmarks as f64
    } else {
        0.0
    };

    let avg_confidence = if total_benchmarks > 0 {
        results.iter().map(|r| r.confidence_score as f64).sum::<f64>() / total_benchmarks as f64
    } else {
        0.0
    };

    let avg_execution_time_ms = if total_benchmarks > 0 {
        results.iter().map(|r| r.execution_time_ms as f64).sum::<f64>() / total_benchmarks as f64
    } else {
        0.0
    };

    let avg_analysis_quality = if total_benchmarks > 0 {
        results.iter().map(|r| r.analysis_quality).sum::<f64>() / total_benchmarks as f64
    } else {
        0.0
    };

    // 分析品質の詳細をログ出力
    println!("  📊 平均分析品質: {:.1}%", avg_analysis_quality);

    BenchmarkSummary {
        total_benchmarks,
        true_positives,
        false_positives,
        true_negatives,
        false_negatives,
        precision,
        recall,
        f1_score,
        accuracy,
        avg_confidence,
        avg_execution_time_ms,
    }
}

#[tokio::test]
async fn test_validation_benchmarks_sample() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping validation benchmarks test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // ベンチマークディレクトリを発見
    let benchmark_dirs = discover_validation_benchmarks()?;
    
    if benchmark_dirs.is_empty() {
        println!("⚠️  Validation Benchmarks が見つかりませんでした");
        return Ok(());
    }

    // 最初の10個のベンチマークでテスト（CI時間を考慮）
    let sample_size = std::cmp::min(10, benchmark_dirs.len());
    let sample_dirs = &benchmark_dirs[0..sample_size];

    println!("🧪 Validation Benchmarks サンプルテスト: {}個", sample_size);

    let mut all_results = Vec::new();
    let mut benchmark_count = 0;

    for benchmark_dir in sample_dirs {
        if let Some(benchmark) = load_validation_benchmark(benchmark_dir)? {
            println!("  [{}] テスト中: {} - {}",
                    benchmark_count + 1, benchmark.benchmark_id, benchmark.title);

            let results = test_validation_benchmark(&benchmark, benchmark_dir, model).await?;
            
            println!("    ファイル数: {}, 検出数: {}",
                    results.len(),
                    results.iter().filter(|r| r.detected).count());

            all_results.extend(results);
            benchmark_count += 1;
        }
    }

    let summary = calculate_benchmark_summary(&all_results);

    println!("\n📊 Validation Benchmarks サンプル結果:");
    println!("  総ベンチマーク数: {}", summary.total_benchmarks);
    println!("  真陽性 (TP): {}", summary.true_positives);
    println!("  偽陽性 (FP): {}", summary.false_positives);
    println!("  真陰性 (TN): {}", summary.true_negatives);
    println!("  偽陰性 (FN): {}", summary.false_negatives);
    println!("  精度 (Precision): {:.3}", summary.precision);
    println!("  再現率 (Recall): {:.3}", summary.recall);
    println!("  F1スコア: {:.3}", summary.f1_score);
    println!("  正確度 (Accuracy): {:.3}", summary.accuracy);
    println!("  平均信頼度: {:.1}", summary.avg_confidence);
    println!("  平均実行時間: {:.1}ms", summary.avg_execution_time_ms);

    // Validation Benchmarksでは F1スコア 0.8以上を期待
    assert!(
        summary.f1_score >= 0.8,
        "Validation Benchmarks F1スコアが基準を下回っています: {:.3} (要求: 0.8)",
        summary.f1_score
    );

    println!("\n🎉 Validation Benchmarks サンプルテスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_high_severity_validation_benchmarks() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping high severity validation test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let benchmark_dirs = discover_validation_benchmarks()?;
    
    if benchmark_dirs.is_empty() {
        println!("⚠️  Validation Benchmarks が見つかりませんでした");
        return Ok(());
    }

    println!("🔥 高深刻度 Validation Benchmarks テスト");

    let mut high_severity_results = Vec::new();
    let mut tested_benchmarks = 0;

    for benchmark_dir in &benchmark_dirs {
        if let Some(benchmark) = load_validation_benchmark(benchmark_dir)? {
            // 高深刻度のみフィルタリング
            if benchmark.risk_level.to_uppercase() == "HIGH" || 
               benchmark.risk_level.to_uppercase() == "CRITICAL" {
                
                println!("  テスト中: {} ({}) - {}",
                        benchmark.benchmark_id, benchmark.risk_level, benchmark.title);

                let results = test_validation_benchmark(&benchmark, benchmark_dir, model).await?;
                high_severity_results.extend(results);
                tested_benchmarks += 1;

                // 時間を考慮して最大5個まで
                if tested_benchmarks >= 5 {
                    break;
                }
            }
        }
    }

    if high_severity_results.is_empty() {
        println!("⚠️  高深刻度ベンチマークが見つかりませんでした");
        return Ok(());
    }

    let summary = calculate_benchmark_summary(&high_severity_results);

    println!("\n📊 高深刻度ベンチマーク結果:");
    println!("  テスト済みベンチマーク: {}", tested_benchmarks);
    println!("  F1スコア: {:.3}", summary.f1_score);
    println!("  精度: {:.3}", summary.precision);
    println!("  再現率: {:.3}", summary.recall);

    // 高深刻度脆弱性では F1スコア 0.85以上を要求
    assert!(
        summary.f1_score >= 0.85,
        "高深刻度ベンチマーク F1スコアが基準を下回っています: {:.3} (要求: 0.85)",
        summary.f1_score
    );

    println!("🎉 高深刻度 Validation Benchmarks テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_benchmark_performance_characteristics() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping performance characteristics test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    let benchmark_dirs = discover_validation_benchmarks()?;
    
    if benchmark_dirs.is_empty() {
        println!("⚠️  Validation Benchmarks が見つかりませんでした");
        return Ok(());
    }

    println!("⚡ ベンチマーク性能特性テスト");

    let mut vulnerability_type_stats = HashMap::new();
    let mut language_stats = HashMap::new();
    let mut execution_times = Vec::new();

    // 最初の5個で性能特性を測定
    let sample_size = std::cmp::min(5, benchmark_dirs.len());

    for benchmark_dir in &benchmark_dirs[0..sample_size] {
        if let Some(benchmark) = load_validation_benchmark(benchmark_dir)? {
            let results = test_validation_benchmark(&benchmark, benchmark_dir, model).await?;

            // 実行時間と脆弱性統計を先に収集
            for result in &results {
                // 脆弱性タイプ別統計
                if result.detected {
                    for vuln_type in &result.detected_types {
                        let vuln_key = format!("{:?}", vuln_type);
                        let entry = vulnerability_type_stats.entry(vuln_key).or_insert((0, 0));
                        entry.0 += 1; // 検出数
                        entry.1 += result.confidence_score as usize;
                    }
                }

                // 実行時間統計
                execution_times.push(result.execution_time_ms);
            }

            // 言語別統計
            for test_file in &benchmark.test_files {
                let lang_entry = language_stats.entry(test_file.language.clone()).or_insert((0, 0));
                lang_entry.0 += 1; // 総数
                if results.iter().any(|r| r.benchmark_id.contains(&test_file.path) && r.detected) {
                    lang_entry.1 += 1; // 検出数
                }
            }
        }
    }

    // 実行時間統計
    execution_times.sort();
    let median_time = if execution_times.is_empty() {
        0.0
    } else {
        execution_times[execution_times.len() / 2] as f64
    };
    let avg_time = if execution_times.is_empty() {
        0.0
    } else {
        execution_times.iter().sum::<u128>() as f64 / execution_times.len() as f64
    };
    let max_time = *execution_times.iter().max().unwrap_or(&0) as f64;

    println!("\n📊 性能特性結果:");
    
    println!("\n実行時間統計:");
    println!("  平均: {:.1}ms", avg_time);
    println!("  中央値: {:.1}ms", median_time);
    println!("  最大: {:.1}ms", max_time);

    println!("\n脆弱性タイプ別検出:");
    for (vuln_type, (count, total_confidence)) in vulnerability_type_stats {
        let avg_confidence = total_confidence as f64 / count as f64;
        println!("  {}: {}件 (平均信頼度: {:.1})", vuln_type, count, avg_confidence);
    }

    println!("\n言語別検出率:");
    for (language, (total, detected)) in language_stats {
        let detection_rate = detected as f64 / total as f64;
        println!("  {}: {:.1}% ({}/{})", language, detection_rate * 100.0, detected, total);
    }

    // 性能要件チェック
    assert!(
        avg_time <= 10000.0,
        "平均実行時間が制限を超えています: {:.1}ms (制限: 10000ms)",
        avg_time
    );

    assert!(
        max_time <= 30000.0,
        "最大実行時間が制限を超えています: {:.1}ms (制限: 30000ms)",
        max_time
    );

    println!("\n✅ ベンチマーク性能特性テスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_ossf_cve_benchmark_sample() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping OSSF CVE benchmark test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";

    // OSSF CVE Benchmarkファイルを発見
    let benchmark_files = discover_ossf_cve_benchmarks()?;
    
    if benchmark_files.is_empty() {
        println!("⚠️  OSSF CVE Benchmarks が見つかりませんでした");
        return Ok(());
    }

    // 最初の5個のベンチマークでテスト（CI時間を考慮）
    let sample_size = std::cmp::min(5, benchmark_files.len());
    let sample_files = &benchmark_files[0..sample_size];

    println!("🧪 OSSF CVE Benchmark サンプルテスト: {}個", sample_size);

    let mut all_results = Vec::new();
    let mut benchmark_count = 0;

    for benchmark_file in sample_files {
        if let Some(benchmark) = load_ossf_cve_benchmark(benchmark_file)? {
            println!("  [{}] テスト中: {} - {} ({})",
                    benchmark_count + 1, benchmark.id, benchmark.vulnerability_class, benchmark.language);

            let result = test_ossf_cve_benchmark(&benchmark, benchmark_file, model).await?;
            
            println!("    検出: {}, 信頼度: {}, 脆弱性: {:?}",
                    if result.detected { "✅" } else { "❌" },
                    result.confidence_score,
                    result.detected_types);

            all_results.push(result);
            benchmark_count += 1;
        }
    }

    let summary = calculate_benchmark_summary(&all_results);

    println!("\n📊 OSSF CVE Benchmark サンプル結果:");
    println!("  総ベンチマーク数: {}", summary.total_benchmarks);
    println!("  真陽性 (TP): {}", summary.true_positives);
    println!("  偽陰性 (FN): {}", summary.false_negatives);
    println!("  再現率 (Recall): {:.3}", summary.recall);
    println!("  正確度 (Accuracy): {:.3}", summary.accuracy);
    println!("  平均信頼度: {:.1}", summary.avg_confidence);
    println!("  平均実行時間: {:.1}ms", summary.avg_execution_time_ms);

    // OSSF CVE Benchmarksでは再現率 0.85以上を期待（既知のCVEなので検出すべき）
    assert!(
        summary.recall >= 0.85,
        "OSSF CVE Benchmark再現率が基準を下回っています: {:.3} (要求: 0.85)",
        summary.recall
    );

    println!("\n🎉 OSSF CVE Benchmark サンプルテスト合格!");
    Ok(())
}

#[tokio::test]
async fn test_ossf_cve_by_language() -> Result<()> {
    // API key check
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping OSSF CVE language test");
        return Ok(());
    }

    let model = "gpt-4.1-mini";
    let benchmark_files = discover_ossf_cve_benchmarks()?;
    
    if benchmark_files.is_empty() {
        println!("⚠️  OSSF CVE Benchmarks が見つかりませんでした");
        return Ok(());
    }

    println!("🌐 OSSF CVE Benchmark 言語別テスト");

    let mut language_results: HashMap<String, Vec<BenchmarkResult>> = HashMap::new();
    let mut tested_count = 0;

    for benchmark_file in &benchmark_files {
        if let Some(benchmark) = load_ossf_cve_benchmark(benchmark_file)? {
            // 各言語から最大2個まで
            let lang_results = language_results.entry(benchmark.language.clone()).or_default();
            if lang_results.len() >= 2 {
                continue;
            }

            println!("  テスト中: {} ({})", benchmark.id, benchmark.language);

            let result = test_ossf_cve_benchmark(&benchmark, benchmark_file, model).await?;
            lang_results.push(result);
            tested_count += 1;

            // 総計10個まで
            if tested_count >= 10 {
                break;
            }
        }
    }

    println!("\n📊 言語別結果:");
    let mut overall_recall = 0.0;
    let mut total_languages = 0;

    for (language, results) in language_results {
        let summary = calculate_benchmark_summary(&results);
        println!("  {}: 再現率={:.3}, 件数={}", language, summary.recall, results.len());
        overall_recall += summary.recall;
        total_languages += 1;
    }

    let avg_recall = if total_languages > 0 {
        overall_recall / total_languages as f64
    } else {
        0.0
    };

    println!("\n全言語平均再現率: {:.3}", avg_recall);

    // 言語別でも平均80%以上の再現率を期待
    assert!(
        avg_recall >= 0.8,
        "OSSF CVE Benchmark言語別平均再現率が基準を下回っています: {:.3} (要求: 0.8)",
        avg_recall
    );

    println!("✅ OSSF CVE Benchmark言語別テスト合格!");
    Ok(())
}