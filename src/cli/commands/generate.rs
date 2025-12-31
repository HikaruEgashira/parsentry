//! Generate command for creating security patterns.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::cli::args::{GenerateArgs, Agent};
use crate::github::{clone_repo, GitHubSearchClient};
use crate::pattern_generator_claude_code::generate_custom_patterns_with_claude_code;
use parsentry_analyzer::generate_custom_patterns;
use parsentry_claude_code::ClaudeCodeConfig;

/// Benchmark result for pattern generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub benchmark_name: String,
    pub repository: String,
    pub expected_tags: Vec<String>,
    pub generated_patterns: usize,
    pub execution_time_ms: u128,
    pub patterns_file_exists: bool,
    pub accuracy_metrics: Option<AccuracyMetrics>,
}

/// Accuracy metrics for benchmark validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyMetrics {
    pub expected_vuln_types: Vec<String>,
    pub detected_pattern_types: Vec<String>,
    pub principal_count: usize,
    pub action_count: usize,
    pub resource_count: usize,
    pub coverage_score: f64,
}

/// Benchmark configuration from benchmark.json
#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkConfig {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub level: String,
}

/// Summary of all benchmark results
#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkSummary {
    pub total_repositories: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_patterns: usize,
    pub total_time_ms: u128,
    pub avg_time_ms: u128,
    pub avg_coverage: f64,
    pub results: Vec<BenchmarkResult>,
}

pub async fn run_generate_command(args: GenerateArgs) -> Result<()> {
    let mut all_targets: Vec<String> = vec![args.target.clone()];

    // Add additional repos
    all_targets.extend(args.repos.clone());

    // Search for repositories if search query is provided
    if let Some(search_query) = &args.search {
        println!("🔍 GitHub リポジトリを検索中: {}", search_query);

        match GitHubSearchClient::new() {
            Ok(client) => {
                match client.search_repositories(search_query, args.max_repos).await {
                    Ok(repos) => {
                        println!("📦 {} 件のリポジトリが見つかりました", repos.len());
                        for repo in &repos {
                            all_targets.push(repo.full_name.clone());
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  検索エラー: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️  GitHub クライアント初期化エラー: {}", e);
            }
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("🔧 パターン生成ベンチマーク");
    println!("═══════════════════════════════════════════════════════════════");
    println!("📋 対象リポジトリ数: {}", all_targets.len());
    println!("🤖 モデル: {}", args.model);
    println!();

    let mut results: Vec<BenchmarkResult> = Vec::new();
    let total_start = Instant::now();

    for (idx, target) in all_targets.iter().enumerate() {
        println!("───────────────────────────────────────────────────────────────");
        println!("[{}/{}] {}", idx + 1, all_targets.len(), target);
        println!("───────────────────────────────────────────────────────────────");

        match process_single_target(target, &args).await {
            Ok(result) => {
                print_single_result(&result);
                results.push(result);
            }
            Err(e) => {
                eprintln!("❌ エラー: {}", e);
                results.push(BenchmarkResult {
                    benchmark_name: target.clone(),
                    repository: target.clone(),
                    expected_tags: Vec::new(),
                    generated_patterns: 0,
                    execution_time_ms: 0,
                    patterns_file_exists: false,
                    accuracy_metrics: None,
                });
            }
        }
        println!();
    }

    let total_elapsed = total_start.elapsed();

    // Print summary
    print_summary(&results, total_elapsed.as_millis());

    Ok(())
}

async fn process_single_target(target: &str, args: &GenerateArgs) -> Result<BenchmarkResult> {
    let start_time = Instant::now();

    // Determine if target is a GitHub repo or local path
    // Local path exists on filesystem, GitHub repo is owner/repo format (exactly one slash, no path separators)
    let target_path = PathBuf::from(target);
    let is_local_path = target_path.exists();
    let is_github_repo = !is_local_path
        && target.contains('/')
        && !target.starts_with('/')
        && !target.starts_with('.')
        && target.matches('/').count() == 1;

    let local_path: PathBuf = if is_github_repo {
        // Clone the repository
        let temp_dir = std::env::temp_dir().join("parsentry-generate").join(target.replace('/', "_"));

        if temp_dir.exists() {
            println!("📁 キャッシュを使用: {}", temp_dir.display());
        } else {
            println!("📥 クローン中: {}", target);
            fs::create_dir_all(temp_dir.parent().unwrap())?;
            clone_repo(target, &temp_dir)?;
        }
        temp_dir
    } else {
        PathBuf::from(target)
    };

    // Generate patterns
    println!("🔧 パターン生成中...");

    match args.agent {
        Agent::ClaudeCode => {
            let claude_config = ClaudeCodeConfig {
                working_dir: local_path.clone(),
                ..ClaudeCodeConfig::default()
            };
            generate_custom_patterns_with_claude_code(&local_path, claude_config).await?;
        }
        Agent::Genai => {
            generate_custom_patterns(&local_path, &args.model, args.api_base_url.as_deref()).await?;
        }
    }

    let elapsed = start_time.elapsed();

    // Run benchmark validation
    let benchmark_result = run_benchmark_validation(
        &local_path,
        target,
        args.benchmark_file.as_deref(),
        elapsed.as_millis(),
    )?;

    Ok(benchmark_result)
}

fn run_benchmark_validation(
    target_path: &Path,
    repository: &str,
    benchmark_file: Option<&Path>,
    execution_time_ms: u128,
) -> Result<BenchmarkResult> {
    // Try to find benchmark.json
    let benchmark_path = benchmark_file
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| target_path.join("benchmark.json"));

    let benchmark_config = if benchmark_path.exists() {
        let content = fs::read_to_string(&benchmark_path)?;
        Some(serde_json::from_str::<BenchmarkConfig>(&content)?)
    } else {
        None
    };

    // Check patterns file
    let patterns_file = target_path.join("vuln-patterns.yml");
    let patterns_file_exists = patterns_file.exists();

    let (generated_patterns, pattern_counts) = if patterns_file_exists {
        let counts = count_patterns_by_type(&patterns_file)?;
        let total = counts.0 + counts.1 + counts.2;
        (total, counts)
    } else {
        (0, (0, 0, 0))
    };

    // Calculate accuracy metrics
    let accuracy_metrics = {
        let detected_types = if patterns_file_exists {
            extract_pattern_types_from_file(&patterns_file).unwrap_or_default()
        } else {
            Vec::new()
        };

        let expected_tags = benchmark_config.as_ref().map(|c| c.tags.clone()).unwrap_or_default();
        let coverage = calculate_coverage(&expected_tags, &detected_types);

        Some(AccuracyMetrics {
            expected_vuln_types: expected_tags,
            detected_pattern_types: detected_types,
            principal_count: pattern_counts.0,
            action_count: pattern_counts.1,
            resource_count: pattern_counts.2,
            coverage_score: coverage,
        })
    };

    Ok(BenchmarkResult {
        benchmark_name: benchmark_config
            .as_ref()
            .map(|c| c.name.clone())
            .unwrap_or_else(|| repository.to_string()),
        repository: repository.to_string(),
        expected_tags: benchmark_config.map(|c| c.tags).unwrap_or_default(),
        generated_patterns,
        execution_time_ms,
        patterns_file_exists,
        accuracy_metrics,
    })
}

fn count_patterns_by_type(path: &Path) -> Result<(usize, usize, usize)> {
    let content = fs::read_to_string(path)?;
    let mut principals = 0;
    let mut actions = 0;
    let mut resources = 0;

    let mut current_section = String::new();
    for line in content.lines() {
        if line.starts_with("  principals:") {
            current_section = "principals".to_string();
        } else if line.starts_with("  actions:") {
            current_section = "actions".to_string();
        } else if line.starts_with("  resources:") {
            current_section = "resources".to_string();
        } else if line.starts_with("    - definition:") || line.starts_with("    - reference:") {
            match current_section.as_str() {
                "principals" => principals += 1,
                "actions" => actions += 1,
                "resources" => resources += 1,
                _ => {}
            }
        }
    }

    Ok((principals, actions, resources))
}

fn extract_pattern_types_from_file(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let mut types = Vec::new();

    let mut current_section = String::new();
    for line in content.lines() {
        if line.starts_with("  principals:") {
            current_section = "principals".to_string();
        } else if line.starts_with("  actions:") {
            current_section = "actions".to_string();
        } else if line.starts_with("  resources:") {
            current_section = "resources".to_string();
        } else if line.starts_with("    - ") && !current_section.is_empty() {
            if !types.contains(&current_section) {
                types.push(current_section.clone());
            }
        }
    }

    Ok(types)
}

fn calculate_coverage(expected_tags: &[String], detected_types: &[String]) -> f64 {
    if expected_tags.is_empty() {
        // No expected tags - base score on pattern diversity
        return match detected_types.len() {
            0 => 0.0,
            1 => 0.3,
            2 => 0.6,
            _ => 0.8,
        };
    }

    // Map vulnerability tags to PAR types
    let tag_to_par: Vec<(&str, &str)> = vec![
        ("idor", "resources"),
        ("sqli", "resources"),
        ("sql_injection", "resources"),
        ("xss", "resources"),
        ("command_injection", "resources"),
        ("lfi", "resources"),
        ("rce", "resources"),
        ("ssrf", "resources"),
        ("path_traversal", "resources"),
        ("file", "resources"),
        ("auth", "actions"),
        ("authentication", "actions"),
        ("authorization", "actions"),
        ("validation", "actions"),
        ("input", "principals"),
        ("user_input", "principals"),
        ("request", "principals"),
    ];

    let mut matched = 0;
    for tag in expected_tags {
        let tag_lower = tag.to_lowercase();
        for (pattern, par_type) in &tag_to_par {
            if tag_lower.contains(pattern) && detected_types.contains(&par_type.to_string()) {
                matched += 1;
                break;
            }
        }
    }

    // Base score for having any patterns
    let base_score = if detected_types.is_empty() { 0.0 } else { 0.3 };
    let tag_score = (matched as f64) / (expected_tags.len() as f64) * 0.7;

    base_score + tag_score
}

fn print_single_result(result: &BenchmarkResult) {
    let speed_icon = if result.execution_time_ms < 5000 {
        "🟢"
    } else if result.execution_time_ms < 15000 {
        "🟡"
    } else {
        "🔴"
    };

    println!("   ⏱️  時間: {} ms {}", result.execution_time_ms, speed_icon);
    println!("   📝 パターン: {} 個", result.generated_patterns);

    if let Some(metrics) = &result.accuracy_metrics {
        println!(
            "   📊 PAR: P={} A={} R={}",
            metrics.principal_count, metrics.action_count, metrics.resource_count
        );

        let coverage_icon = if metrics.coverage_score >= 0.7 {
            "🟢"
        } else if metrics.coverage_score >= 0.4 {
            "🟡"
        } else {
            "🔴"
        };
        println!(
            "   🎯 カバレッジ: {:.1}% {}",
            metrics.coverage_score * 100.0,
            coverage_icon
        );
    }
}

fn print_summary(results: &[BenchmarkResult], total_time_ms: u128) {
    let successful: Vec<_> = results.iter().filter(|r| r.patterns_file_exists).collect();
    let total_patterns: usize = results.iter().map(|r| r.generated_patterns).sum();
    let avg_time = if !results.is_empty() {
        total_time_ms / results.len() as u128
    } else {
        0
    };

    let coverages: Vec<f64> = results
        .iter()
        .filter_map(|r| r.accuracy_metrics.as_ref().map(|m| m.coverage_score))
        .collect();
    let avg_coverage = if !coverages.is_empty() {
        coverages.iter().sum::<f64>() / coverages.len() as f64
    } else {
        0.0
    };

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("📊 ベンチマーク総合結果");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("## 概要");
    println!("| 指標 | 値 |");
    println!("|------|-----|");
    println!("| 総リポジトリ数 | {} |", results.len());
    println!("| 成功 | {} |", successful.len());
    println!("| 失敗 | {} |", results.len() - successful.len());
    println!("| 総パターン数 | {} |", total_patterns);
    println!("| 総実行時間 | {:.2} 秒 |", total_time_ms as f64 / 1000.0);
    println!("| 平均実行時間 | {} ms |", avg_time);
    println!("| 平均カバレッジ | {:.1}% |", avg_coverage * 100.0);
    println!();

    println!("## 詳細結果");
    println!("| リポジトリ | パターン | 時間(ms) | カバレッジ |");
    println!("|-----------|---------|---------|-----------|");

    for result in results {
        let coverage_str = result
            .accuracy_metrics
            .as_ref()
            .map(|m| format!("{:.1}%", m.coverage_score * 100.0))
            .unwrap_or_else(|| "-".to_string());

        let name = if result.repository.len() > 30 {
            format!("{}...", &result.repository[..27])
        } else {
            result.repository.clone()
        };

        println!(
            "| {} | {} | {} | {} |",
            name, result.generated_patterns, result.execution_time_ms, coverage_str
        );
    }

    println!();

    // Rating
    let overall_rating = if avg_coverage >= 0.7 && avg_time < 10000 {
        "🏆 優秀"
    } else if avg_coverage >= 0.5 && avg_time < 20000 {
        "✅ 良好"
    } else if avg_coverage >= 0.3 {
        "🟡 改善の余地あり"
    } else {
        "🔴 要改善"
    };

    println!("## 総合評価: {}", overall_rating);
    println!();
}
