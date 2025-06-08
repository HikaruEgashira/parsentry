use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use std::path::PathBuf;
use parsentry::analyzer::analyze_file;
use parsentry::file_classifier::FileClassifier;
use parsentry::parser;
use parsentry::pattern_generator::generate_custom_patterns;
use parsentry::sarif::SarifReport;
use parsentry::security_patterns::SecurityRiskPatterns;

use parsentry::repo::RepoOps;
use parsentry::repo_clone::clone_github_repo;
use parsentry::response::{AnalysisSummary, VulnType};

use futures::stream::{self, StreamExt};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    group = clap::ArgGroup::new("target")
        .required(true)
        .args(&["root", "repo"])
)]
struct Args {
    /// Path to the root directory of the project
    #[arg(short, long, group = "target")]
    root: Option<PathBuf>,

    /// GitHub repository (owner/repo or URL)
    #[arg(long, group = "target")]
    repo: Option<String>,

    /// Specific path or file within the project to analyze
    #[arg(short, long)]
    analyze: Option<PathBuf>,

    /// LLM model to use (default: o4-mini)
    #[arg(short, long, default_value = "o4-mini")]
    model: String,

    /// Increase output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Enable evaluation mode for example vulnerable apps
    #[arg(short, long)]
    evaluate: bool,

    /// Output directory for markdown reports
    #[arg(long)]
    output_dir: Option<PathBuf>,

    /// 最小信頼度スコア（これ以上のスコアを持つ脆弱性のみ表示）
    #[arg(long, default_value = "70")]
    min_confidence: i32,

    /// 特定の脆弱性タイプでフィルタリング（カンマ区切りで複数指定可）
    #[arg(long)]
    vuln_types: Option<String>,

    /// カスタムパターンを生成する（現在のディレクトリを解析してセキュリティパターンを自動検出）
    #[arg(long)]
    generate_patterns: bool,

    /// Debug mode (save LLM input/output to debug folder)
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    println!("🔍 Parsentry - PAR-based security scanner");

    let args = Args::parse();

    let (root_dir, repo_name) = if let Some(repo) = &args.repo {
        let dest = PathBuf::from("repo");
        if dest.exists() {
            std::fs::remove_dir_all(&dest)
                .map_err(|e| anyhow::anyhow!("クローン先ディレクトリの削除に失敗: {}", e))?;
        }
        println!(
            "🛠️  GitHubリポジトリをクローン中: {} → {}",
            repo,
            dest.display()
        );
        clone_github_repo(repo, &dest)
            .map_err(|e| anyhow::anyhow!("GitHubリポジトリのクローンに失敗: {}", e))?;
        
        // Extract repository name for output directory
        let repo_name = if repo.contains('/') {
            repo.split('/').last().unwrap_or("unknown-repo").replace(".git", "")
        } else {
            repo.replace(".git", "")
        };
        
        (dest, Some(repo_name))
    } else if let Some(root) = &args.root {
        (root.clone(), None)
    } else {
        panic!("root path or repo must be set");
    };

    // Handle pattern generation mode
    if args.generate_patterns {
        println!("🔧 カスタムパターン生成モードを開始します");
        return generate_custom_patterns(&root_dir, &args.model).await;
    }


    let repo = RepoOps::new(root_dir.clone());

    let files = repo.get_relevant_files();
    println!(
        "📁 関連するソースファイルを検出しました ({}件)",
        files.len()
    );

    let mut pattern_files = Vec::new();
    for file_path in &files {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            let filename = file_path.to_string_lossy();
            let lang = FileClassifier::classify(&filename, &content);
            
            let patterns = SecurityRiskPatterns::new(lang);
            if patterns.matches(&content) {
                pattern_files.push(file_path.clone());
            }
        }
    }

    println!(
        "🔎 セキュリティパターン該当ファイルを検出しました ({}件)",
        pattern_files.len()
    );
    for (i, f) in pattern_files.iter().enumerate() {
        println!("  [P{}] {}", i + 1, f.display());
    }

    let total = pattern_files.len();
    let root_dir = Arc::new(root_dir);
    
    // Create repository-specific output directory
    let output_dir = if let Some(base_output_dir) = args.output_dir.clone() {
        if let Some(ref name) = repo_name {
            let mut repo_output_dir = base_output_dir;
            repo_output_dir.push(name);
            Some(repo_output_dir)
        } else {
            Some(base_output_dir)
        }
    } else {
        None
    };
    
    let model = args.model.clone();
    let files = files.clone();
    let verbosity = args.verbosity;
    let debug = args.debug;

    let mut summary = AnalysisSummary::new();

    // プログレスバーを設定
    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    progress_bar.set_message("Analyzing files...");

    // 並列度を制御してタスクを実行
    let results = stream::iter(pattern_files.iter().enumerate())
        .map(|(idx, file_path)| {
            let file_path = file_path.clone();
            let root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let model = model.clone();
            let files = files.clone();
            let progress_bar = progress_bar.clone();
            let debug = debug;

            async move {
                let file_name = file_path.display().to_string();
                progress_bar.set_message(format!("Analyzing: {}", file_name));
                if verbosity > 0 {
                    println!("📄 解析対象: {} ({} / {})", file_name, idx + 1, total);
                    println!("{}", "=".repeat(80));
                }

                let mut repo = RepoOps::new((*root_dir).clone());
                if let Err(e) = repo.add_file_to_parser(&file_path) {
                    if verbosity > 0 {
                        println!(
                            "❌ ファイルのパース追加に失敗: {}: {}",
                            file_path.display(),
                            e
                        );
                    }
                    progress_bar.inc(1);
                    return None;
                }
                let context = match repo.collect_context_for_security_pattern(&file_path) {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        println!("⚠️  コンテキスト収集に失敗（空のコンテキストで継続）: {}: {}", file_path.display(), e);
                        // For IaC files and other unsupported file types, continue with empty context
                        parser::Context { definitions: Vec::new() }
                    }
                };

                let analysis_result =
                    match analyze_file(&file_path, &model, &files, verbosity, &context, 0, debug, &output_dir).await {
                        Ok(res) => res,
                        Err(e) => {
                            if verbosity > 0 {
                                println!("❌ 解析に失敗: {}: {}", file_path.display(), e);
                            }
                            progress_bar.inc(1);
                            return None;
                        }
                    };

                if analysis_result.vulnerability_types.is_empty()
                    || analysis_result.confidence_score < 1
                {
                    progress_bar.inc(1);
                    return None;
                }

                if let Some(ref output_dir) = output_dir {
                    if let Err(e) = std::fs::create_dir_all(output_dir) {
                        if verbosity > 0 {
                            println!(
                                "❌ 出力ディレクトリ作成に失敗: {}: {}",
                                output_dir.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                    let fname = file_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string() + ".md")
                        .unwrap_or_else(|| "report.md".to_string());
                    let mut out_path = output_dir.clone();
                    out_path.push(fname);
                    if let Err(e) = std::fs::write(&out_path, analysis_result.to_markdown()) {
                        if verbosity > 0 {
                            println!(
                                "❌ Markdownレポート出力に失敗: {}: {}",
                                out_path.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                    if verbosity > 0 {
                        println!("📝 Markdownレポートを出力: {}", out_path.display());
                    }
                }

                if verbosity > 0 {
                    analysis_result.print_readable();
                }

                progress_bar.inc(1);
                Some((file_path, analysis_result))
            }
        })
        .buffer_unordered(10)  // API制限を考慮した並列処理
        .collect::<Vec<_>>()
        .await;
    for result in results.into_iter() {
        if let Some((file_path, response)) = result {
            summary.add_result(file_path, response);
        }
    }

    progress_bar.finish_with_message("Analysis completed!");

    summary.sort_by_confidence();

    let mut filtered_summary = if args.min_confidence > 0 {
        summary.filter_by_min_confidence(args.min_confidence)
    } else {
        summary
    };

    if let Some(types_str) = args.vuln_types {
        let vuln_types: Vec<VulnType> = types_str
            .split(',')
            .map(|s| match s.trim() {
                "LFI" => VulnType::LFI,
                "RCE" => VulnType::RCE,
                "SSRF" => VulnType::SSRF,
                "AFO" => VulnType::AFO,
                "SQLI" => VulnType::SQLI,
                "XSS" => VulnType::XSS,
                "IDOR" => VulnType::IDOR,
                other => VulnType::Other(other.to_string()),
            })
            .collect();

        filtered_summary = filtered_summary.filter_by_vuln_types(&vuln_types);
    }

    // Always generate summary report
    {
        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                println!(
                    "❌ 出力ディレクトリ作成に失敗: {}: {}",
                    final_output_dir.display(),
                    e
                );
            } else {
                if !filtered_summary.results.is_empty() {
                    let mut summary_path = final_output_dir.clone();
                    summary_path.push("summary.md");
                    if let Err(e) = std::fs::write(&summary_path, filtered_summary.to_markdown()) {
                        println!(
                            "❌ サマリーレポート出力に失敗: {}: {}",
                            summary_path.display(),
                            e
                        );
                    } else {
                        println!("📊 サマリーレポートを出力: {}", summary_path.display());
                    }
                }
            }
        } else {
            println!("⚠ サマリーレポートを出力するには --output-dir オプションが必要です");
        }
    }

    // Always generate SARIF report
    {
        let sarif_report = SarifReport::from_analysis_summary(&filtered_summary);
        
        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                println!(
                    "❌ 出力ディレクトリ作成に失敗: {}: {}",
                    final_output_dir.display(),
                    e
                );
            } else {
                let mut sarif_path = final_output_dir.clone();
                sarif_path.push("parsentry-results.sarif");
                if let Err(e) = sarif_report.save_to_file(&sarif_path) {
                    println!(
                        "❌ SARIFレポート出力に失敗: {}: {}",
                        sarif_path.display(),
                        e
                    );
                } else {
                    println!("📋 SARIFレポートを出力: {}", sarif_path.display());
                }
            }
        } else {
            // Output SARIF to stdout if no output directory specified
            match sarif_report.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => println!("❌ SARIF出力に失敗: {}", e),
            }
        }
    }

    println!("✅ 解析が完了しました");

    Ok(())
}
