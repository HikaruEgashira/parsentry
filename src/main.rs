use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use parsentry::analyzer::{analyze_file, analyze_pattern};
use parsentry::args::{Args, validate_args};
use parsentry::file_classifier::FileClassifier;
use parsentry::language::Language;
use parsentry::locales;
use parsentry::parser;
use parsentry::pattern_generator::generate_custom_patterns;
use parsentry::sarif::SarifReport;
use parsentry::security_patterns::SecurityRiskPatterns;
use parsentry::benchmark::BenchmarkRunner;
use std::path::PathBuf;

use parsentry::repo::RepoOps;
use parsentry::repo_clone::clone_github_repo;
use parsentry::response::{AnalysisSummary, VulnType};

use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    println!(
        r#"
                ▲
               ╱ ╲
              ╱   ╲
             ╱ ░░░ ╲
            ╱ ░▓▓▓░ ╲
           ╱ ░▓███▓░ ╲
          ╱ ░▓█████▓░ ╲
         ╱_░▓███████▓░_╲
           ─────┬─────
                │
          P A R S E N T R Y
                │
             v{}
"#,
        env!("CARGO_PKG_VERSION")
    );

    let args = Args::parse();

    validate_args(&args)?;

    // Create language configuration
    let language = Language::from_string(&args.language);
    let messages = locales::get_messages(&language);

    // Get API base URL from CLI arg or environment variable
    let env_base_url = std::env::var("API_BASE_URL").ok();
    let api_base_url = args
        .api_base_url
        .as_deref()
        .or_else(|| env_base_url.as_deref());

    let (root_dir, repo_name) = if args.benchmark {
        // In benchmark mode, use current directory
        (PathBuf::from("."), None)
    } else if let Some(repo) = &args.repo {
        let dest = PathBuf::from("repo");
        if dest.exists() {
            std::fs::remove_dir_all(&dest).map_err(|e| {
                anyhow::anyhow!(
                    "{}: {}",
                    messages
                        .get("error_clone_failed")
                        .map_or("Failed to delete clone directory", |s| s),
                    e
                )
            })?;
        }
        println!(
            "🛠️  {}: {} → {}",
            messages
                .get("cloning_repo")
                .map_or("Cloning GitHub repository", |s| s),
            repo,
            dest.display()
        );
        clone_github_repo(repo, &dest).map_err(|e| {
            anyhow::anyhow!(
                "{}: {}",
                messages
                    .get("github_repo_clone_failed")
                    .map_or("Failed to clone GitHub repository", |s| s),
                e
            )
        })?;

        // Extract repository name for output directory
        let repo_name = if repo.contains('/') {
            repo.split('/')
                .last()
                .unwrap_or("unknown-repo")
                .replace(".git", "")
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
        println!(
            "🔧 {}",
            messages
                .get("custom_pattern_generation_start")
                .unwrap_or(&"Starting custom pattern generation mode")
        );
        generate_custom_patterns(&root_dir, &args.model, api_base_url).await?;
        println!(
            "✅ {}",
            messages
                .get("pattern_generation_completed")
                .unwrap_or(&"Pattern generation completed")
        );
    }

    // Handle benchmark mode
    if args.benchmark {
        println!("🎯 Starting benchmark mode");
        
        let benchmarks_dir = root_dir.join("repo").join("benchmarks");
        let output_dir = args.output_dir.unwrap_or_else(|| PathBuf::from("benchmark_results"));
        
        let runner = BenchmarkRunner::new(benchmarks_dir, output_dir);
        
        // Run benchmark scoring
        let score = runner.run_full_benchmark().await?;
        
        // Save results
        let results_file = PathBuf::from("benchmark_results.json");
        runner.save_results(&score, &results_file).await?;
        
        // Print summary
        runner.print_summary(&score);
        
        return Ok(());
    }

    let repo = RepoOps::new(root_dir.clone());

    let files = repo.get_relevant_files();
    println!(
        "📁 {} ({}件)",
        messages
            .get("relevant_files_detected")
            .unwrap_or(&"Detected relevant source files"),
        files.len()
    );

    // Collect all pattern matches across all files
    let mut all_pattern_matches = Vec::new();
    
    for file_path in &files {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            let filename = file_path.to_string_lossy();
            let lang = FileClassifier::classify(&filename, &content);

            let patterns = SecurityRiskPatterns::new_with_root(lang, Some(&root_dir));
            let matches = patterns.get_pattern_matches(&content);
            
            for pattern_match in matches {
                all_pattern_matches.push((file_path.clone(), pattern_match));
            }
        }
    }

    println!(
        "🔎 {} ({}件のパターンマッチ)",
        messages
            .get("security_pattern_files_detected")
            .unwrap_or(&"Detected security patterns"),
        all_pattern_matches.len()
    );
    
    // Group patterns by type for display
    let mut pattern_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, pattern_match) in &all_pattern_matches {
        *pattern_types.entry(pattern_match.pattern_config.description.clone()).or_insert(0) += 1;
    }
    
    for (pattern_desc, count) in pattern_types {
        println!("  [{}] {} matches", count, pattern_desc);
    }

    let total = all_pattern_matches.len();
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
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    progress_bar.set_message("Analyzing files...");

    // 並列度を制御してタスクを実行 - パターンごとに分析
    let results = stream::iter(all_pattern_matches.iter().enumerate())
        .map(|(idx, (file_path, pattern_match))| {
            let file_path = file_path.clone();
            let pattern_match = pattern_match.clone();
            let root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let model = model.clone();
            let files = files.clone();
            let progress_bar = progress_bar.clone();
            let debug = debug;
            let messages = messages.clone();
            let language = language.clone();

            async move {
                let file_name = file_path.display().to_string();
                let pattern_desc = &pattern_match.pattern_config.description;
                progress_bar.set_message(format!("Analyzing pattern '{}' in: {}", pattern_desc, file_name));
                if verbosity > 0 {
                    println!(
                        "📄 {}: {} - Pattern: {} ({} / {})",
                        messages
                            .get("analysis_target")
                            .unwrap_or(&"Analysis target"),
                        file_name,
                        pattern_desc,
                        idx + 1,
                        total
                    );
                    println!("{}", "=".repeat(80));
                }

                let analysis_result = match analyze_pattern(
                    &file_path,
                    &pattern_match,
                    &model,
                    &files,
                    verbosity,
                    0,
                    debug,
                    &output_dir,
                    api_base_url,
                    &language,
                )
                .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        if verbosity > 0 {
                            println!(
                                "❌ {}: {}: {}",
                                messages
                                    .get("analysis_failed")
                                    .unwrap_or(&"Analysis failed"),
                                file_path.display(),
                                e
                            );
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
                                "❌ {}: {}: {}",
                                messages
                                    .get("error_directory_creation")
                                    .map_or("Failed to create directory", |s| s),
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
                                "❌ {}: {}: {}",
                                messages
                                    .get("markdown_report_output_failed")
                                    .map_or("Failed to output Markdown report", |s| s),
                                out_path.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                    if verbosity > 0 {
                        println!(
                            "📝 {}: {}",
                            messages
                                .get("markdown_report_output")
                                .map_or("Output Markdown report", |s| s),
                            out_path.display()
                        );
                    }
                }

                if verbosity > 0 {
                    analysis_result.print_readable();
                }

                progress_bar.inc(1);
                Some((file_path, analysis_result))
            }
        })
        .buffer_unordered(10) // API制限を考慮した並列処理
        .collect::<Vec<_>>()
        .await;
    for result in results.into_iter() {
        if let Some((file_path, response)) = result {
            summary.add_result(file_path, response);
        }
    }

    progress_bar.finish_with_message(
        messages
            .get("analysis_completed")
            .map_or("Analysis completed!", |s| s),
    );

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
                    "❌ {}: {}: {}",
                    messages
                        .get("error_directory_creation")
                        .map_or("Failed to create directory", |s| s),
                    final_output_dir.display(),
                    e
                );
            } else {
                if !filtered_summary.results.is_empty() {
                    let mut summary_path = final_output_dir.clone();
                    summary_path.push("summary.md");
                    if let Err(e) = std::fs::write(&summary_path, filtered_summary.to_markdown()) {
                        println!(
                            "❌ {}: {}: {}",
                            messages
                                .get("summary_report_output_failed")
                                .map_or("Failed to output summary report", |s| s),
                            summary_path.display(),
                            e
                        );
                    } else {
                        println!(
                            "📊 {}: {}",
                            messages
                                .get("summary_report_output")
                                .map_or("Output summary report", |s| s),
                            summary_path.display()
                        );
                    }
                }
            }
        } else {
            println!(
                "⚠ {}",
                messages
                    .get("summary_report_needs_output_dir")
                    .map_or("Summary report output requires --output-dir option", |s| s)
            );
        }
    }

    // Always generate SARIF report
    {
        let sarif_report = SarifReport::from_analysis_summary(&filtered_summary);

        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                println!(
                    "❌ {}: {}: {}",
                    messages
                        .get("error_directory_creation")
                        .map_or("Failed to create directory", |s| s),
                    final_output_dir.display(),
                    e
                );
            } else {
                let mut sarif_path = final_output_dir.clone();
                sarif_path.push("parsentry-results.sarif");
                if let Err(e) = sarif_report.save_to_file(&sarif_path) {
                    println!(
                        "❌ {}: {}: {}",
                        messages
                            .get("sarif_report_output_failed")
                            .map_or("Failed to output SARIF report", |s| s),
                        sarif_path.display(),
                        e
                    );
                } else {
                    println!(
                        "📋 {}: {}",
                        messages
                            .get("sarif_report_output")
                            .map_or("Output SARIF report", |s| s),
                        sarif_path.display()
                    );
                }
            }
        } else {
            // Output SARIF to stdout if no output directory specified
            match sarif_report.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => println!(
                    "❌ {}: {}",
                    messages
                        .get("sarif_output_failed")
                        .map_or("Failed to output SARIF", |s| s),
                    e
                ),
            }
        }
    }

    println!(
        "✅ {}",
        messages
            .get("analysis_completed")
            .map_or("Analysis completed", |s| s)
    );

    Ok(())
}
