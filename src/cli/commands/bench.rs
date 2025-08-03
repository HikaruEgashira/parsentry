use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::BenchCommands;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchConfig {
    pub models: Vec<String>,
    pub suites: Vec<String>,
    pub repetitions: u32,
    pub output_dir: PathBuf,
    pub parallel: bool,
    pub verbose: bool,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            models: vec!["o4-mini".to_string()],
            suites: vec![
                "performance".to_string(),
                "external".to_string(),
                "real-world".to_string(),
                "accuracy".to_string(),
            ],
            repetitions: 1,
            output_dir: PathBuf::from("docs/benchmark/results"),
            parallel: false,
            verbose: false,
        }
    }
}

pub async fn run_bench_command(subcommand: &BenchCommands) -> Result<()> {
    match subcommand {
        BenchCommands::List => {
            list_benchmarks().await
        }
        BenchCommands::Run {
            category,
            output_dir,
            config,
            parallel,
            verbose,
        } => {
            run_benchmarks(category, output_dir, config, *parallel, *verbose).await
        }
        BenchCommands::InitConfig { name } => {
            init_config(name).await
        }
    }
}

async fn list_benchmarks() -> Result<()> {
    println!("🏃 使用可能なベンチマークカテゴリ:");
    println!("  📊 performance    - パフォーマンスベンチマーク（処理速度測定）");
    println!("  🔗 external       - 外部ベンチマーク統合（validation-benchmarks）");
    println!("  🌍 real-world     - 実世界ベンチマーク（実際のCVE事例）");
    println!("  🎯 accuracy       - 精度テスト統合スイート");
    println!("  🏷️  par           - PAR分類精度テスト");
    println!("  📝 context        - コンテキスト品質精度テスト");
    println!("  🔄 end-to-end     - エンドツーエンド精度テスト");
    println!();
    println!("💡 使用例:");
    println!("  parsentry bench run --category performance");
    println!("  parsentry bench run --category external --parallel");
    println!("  parsentry bench run --config bench-config.json");
    Ok(())
}

async fn run_benchmarks(
    category: &Option<String>,
    output_dir: &Option<PathBuf>,
    config_path: &Option<PathBuf>,
    parallel: bool,
    verbose: bool,
) -> Result<()> {
    let config = if let Some(config_path) = config_path {
        load_config(config_path)?
    } else {
        BenchConfig::default()
    };

    let output_dir = output_dir.as_ref().unwrap_or(&config.output_dir);
    
    // 出力ディレクトリを作成
    fs::create_dir_all(output_dir)?;

    match category {
        Some(cat) => run_specific_benchmark(cat, output_dir, parallel, verbose).await?,
        None => run_all_benchmarks(&config).await?,
    }

    Ok(())
}

async fn run_specific_benchmark(
    category: &str,
    output_dir: &Path,
    _parallel: bool,
    verbose: bool,
) -> Result<()> {
    println!("🚀 {}ベンチマーク実行中...", category);

    let test_name = match category {
        "performance" => "performance_benchmark_test",
        "external" => "external_benchmark_integration_test", 
        "real-world" => "real_world_benchmark_test",
        "accuracy" => "accuracy_test_suite",
        "par" => "par_classification_accuracy_test",
        "context" => "context_quality_accuracy_test",
        "end-to-end" => "end_to_end_accuracy_test",
        _ => return Err(anyhow::anyhow!("不明なベンチマークカテゴリ: {}", category)),
    };

    let mut cmd = Command::new("cargo");
    cmd.args(&["test", "--features", "benchmark", test_name]);
    
    if verbose {
        cmd.arg("--");
        cmd.arg("--nocapture");
    }

    let output = cmd.output()?;

    if !output.status.success() {
        eprintln!("❌ ベンチマーク実行失敗:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("ベンチマーク実行に失敗しました"));
    }

    if verbose {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // 結果サマリーを出力
    let result_file = output_dir.join(format!("{}_result.json", category));
    if result_file.exists() {
        let result_content = fs::read_to_string(&result_file)?;
        let result: Value = serde_json::from_str(&result_content)?;
        print_result_summary(&result, category);
    }

    println!("✅ {}ベンチマーク完了", category);
    Ok(())
}

async fn run_all_benchmarks(config: &BenchConfig) -> Result<()> {
    println!("🌟 全ベンチマークスイート実行中...");
    
    let mut results = HashMap::new();
    
    for suite in &config.suites {
        println!("\n📋 {} ベンチマーク実行中...", suite);
        
        for repetition in 1..=config.repetitions {
            println!("  🔄 実行 {}/{}", repetition, config.repetitions);
            
            match run_specific_benchmark(
                suite,
                &config.output_dir,
                config.parallel,
                config.verbose,
            ).await {
                Ok(_) => {
                    results.insert(format!("{}_rep_{}", suite, repetition), "success");
                }
                Err(e) => {
                    eprintln!("❌ {} (実行 {}) 失敗: {}", suite, repetition, e);
                    results.insert(format!("{}_rep_{}", suite, repetition), "failed");
                }
            }
        }
    }

    // 総合結果を保存
    let summary_file = config.output_dir.join("benchmark_summary.json");
    let summary = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "config": config,
        "results": results
    });
    
    fs::write(&summary_file, serde_json::to_string_pretty(&summary)?)?;
    
    println!("\n🎉 全ベンチマーク完了!");
    println!("📄 結果サマリー: {}", summary_file.display());
    
    Ok(())
}

async fn init_config(name: &Option<PathBuf>) -> Result<()> {
    let default_path = PathBuf::from("bench-config.json");
    let config_path = name.as_ref().unwrap_or(&default_path);
    
    let config = BenchConfig::default();
    let config_json = serde_json::to_string_pretty(&config)?;
    
    fs::write(config_path, config_json)?;
    
    println!("📝 ベンチマーク設定ファイルを作成しました: {}", config_path.display());
    println!("💡 設定を編集してからベンチマークを実行してください:");
    println!("   parsentry bench run --config {}", config_path.display());
    
    Ok(())
}

fn load_config(config_path: &Path) -> Result<BenchConfig> {
    let config_content = fs::read_to_string(config_path)?;
    let config: BenchConfig = serde_json::from_str(&config_content)?;
    Ok(config)
}

fn print_result_summary(result: &Value, category: &str) {
    println!("\n📊 {} ベンチマーク結果サマリー:", category);
    
    if let Some(obj) = result.as_object() {
        for (key, value) in obj {
            match value {
                Value::Number(n) => println!("  {}: {}", key, n),
                Value::String(s) => println!("  {}: {}", key, s),
                Value::Bool(b) => println!("  {}: {}", key, b),
                _ => println!("  {}: {:?}", key, value),
            }
        }
    }
}