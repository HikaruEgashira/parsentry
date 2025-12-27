use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};

use crate::analyzer::analyze_pattern;
use crate::cli::args::{ScanArgs, validate_scan_args};
use crate::config::ParsentryConfig;
use crate::file_classifier::FileClassifier;
use crate::locales::{Language, get_messages};
use crate::pattern_generator::generate_custom_patterns;
use crate::repo::{RepoOps, clone_github_repo};
use crate::response::{Response, VulnType};
use crate::reports::{AnalysisSummary, generate_output_filename, generate_pattern_specific_filename, SarifReport};
use crate::security_patterns::{PatternMatch, SecurityRiskPatterns};

use parsentry_claude_code::{ClaudeCodeConfig, ClaudeCodeExecutor, PromptBuilder, PatternContext};

/// Analyze a pattern using Claude Code CLI
async fn analyze_with_claude_code(
    executor: &ClaudeCodeExecutor,
    prompt_builder: &PromptBuilder,
    file_path: &PathBuf,
    pattern_match: &PatternMatch,
) -> Result<Option<Response>> {
    let content = std::fs::read_to_string(file_path)?;

    // Build pattern context
    let pattern_type_str = format!("{:?}", pattern_match.pattern_config.pattern_type);
    let pattern_context = PatternContext::new(
        &pattern_type_str,
        &pattern_match.pattern_config.description,
        &pattern_match.matched_text,
    )
    .with_attack_vectors(pattern_match.pattern_config.attack_vector.clone());

    // Build prompt
    let prompt = prompt_builder.build_security_analysis_prompt(
        file_path,
        &content,
        Some(&pattern_context),
    );

    // Execute Claude Code
    let output = executor.execute_with_retry(&prompt, 2).await;

    match output {
        Ok(output) => {
            println!("✅ Claude Code succeeded for {}", file_path.display());
            let mut response = Response::from_claude_code_response(
                output.response,
                file_path.to_string_lossy().to_string(),
            );
            response.pattern_description = Some(pattern_match.pattern_config.description.clone());
            response.matched_source_code = Some(pattern_match.matched_text.clone());
            Ok(Some(response))
        }
        Err(e) => {
            println!("❌ Claude Code execution error for {}: {}", file_path.display(), e);
            Err(anyhow::anyhow!("Claude Code failed: {}", e))
        }
    }
}

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    // Load configuration with precedence: CLI args > env vars > config file
    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars
    )?;
    
    // Convert config back to args for compatibility with existing code
    let final_args = config.to_args();
    
    validate_scan_args(&final_args)?;

    // Create language configuration
    let language = Language::from_string(&final_args.language);
    let messages = get_messages(&language);

    // Get API base URL from configuration
    let api_base_url = final_args.api_base_url.as_deref();

    let (root_dir, repo_name) = if let Some(target) = &final_args.target {
        if target.contains('/') && !std::path::Path::new(target).exists() {
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
                target,
                dest.display()
            );
            clone_github_repo(target, &dest).map_err(|e| {
                anyhow::anyhow!(
                    "{}: {}",
                    messages
                        .get("github_repo_clone_failed")
                        .map_or("Failed to clone GitHub repository", |s| s),
                    e
                )
            })?;

            let repo_name = target
                .split('/')
                .last()
                .unwrap_or("unknown-repo")
                .replace(".git", "");

            (dest, Some(repo_name))
        } else {
            (PathBuf::from(target), None)
        }
    } else {
        return Err(anyhow::anyhow!("Target must be specified, or configured in a config file"));
    };

    // Handle pattern generation mode
    if final_args.generate_patterns {
        println!(
            "🔧 {}",
            messages
                .get("custom_pattern_generation_start")
                .unwrap_or(&"Starting custom pattern generation mode")
        );
        generate_custom_patterns(&root_dir, &final_args.model, api_base_url).await?;
        println!(
            "✅ {}",
            messages
                .get("pattern_generation_completed")
                .unwrap_or(&"Pattern generation completed")
        );
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
            // Skip files with more than 50,000 characters
            if content.len() > 50_000 {
                continue;
            }
            
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
    let output_dir = if let Some(base_output_dir) = final_args.output_dir.clone() {
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

    let model = final_args.model.clone();
    let files = files.clone();
    let verbosity = final_args.verbosity;
    let debug = final_args.debug;
    let use_claude_code = config.claude_code.enabled;

    let mut summary = AnalysisSummary::new();

    // プログレスバーを設定
    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    // Set concurrency based on mode
    let max_concurrent = if use_claude_code {
        progress_bar.set_message("Analyzing files with Claude Code...");
        config.claude_code.max_concurrent.min(10)
    } else {
        progress_bar.set_message("Analyzing files...");
        50
    };

    // Setup Claude Code executor if enabled
    let claude_executor = if use_claude_code {
        let claude_config = ClaudeCodeConfig {
            claude_path: config.claude_code.path.clone().unwrap_or_else(|| PathBuf::from("claude")),
            max_concurrent: config.claude_code.max_concurrent.min(10),
            timeout_secs: config.claude_code.timeout_secs,
            enable_poc: config.claude_code.enable_poc,
            working_dir: root_dir.as_ref().clone(),
        };
        Some(Arc::new(ClaudeCodeExecutor::new(claude_config)?))
    } else {
        None
    };

    let prompt_builder = Arc::new(
        PromptBuilder::new()
            .with_poc(config.claude_code.enable_poc)
            .with_language(&final_args.language)
    );

    // 並列度を制御してタスクを実行 - パターンごとに分析
    let results = stream::iter(all_pattern_matches.iter().enumerate())
        .map(|(idx, (file_path, pattern_match))| {
            let file_path = file_path.clone();
            let pattern_match = pattern_match.clone();
            let _root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let model = model.clone();
            let files = files.clone();
            let progress_bar = progress_bar.clone();
            let debug = debug;
            let messages = messages.clone();
            let language = language.clone();
            let claude_executor = claude_executor.clone();
            let prompt_builder = prompt_builder.clone();
            let use_claude_code = use_claude_code;

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

                // Choose analysis method based on mode
                let analysis_result = if use_claude_code {
                    if let Some(ref executor) = claude_executor {
                        match analyze_with_claude_code(
                            executor,
                            &prompt_builder,
                            &file_path,
                            &pattern_match,
                        )
                        .await
                        {
                            Ok(Some(res)) => res,
                            Ok(None) => {
                                println!("⚠️ Claude Code returned None for {}", file_path.display());
                                progress_bar.inc(1);
                                return None;
                            }
                            Err(e) => {
                                println!(
                                    "❌ Claude Code {}: {}: {}",
                                    messages
                                        .get("analysis_failed")
                                        .unwrap_or(&"Analysis failed"),
                                    file_path.display(),
                                    e
                                );
                                progress_bar.inc(1);
                                return None;
                            }
                        }
                    } else {
                        progress_bar.inc(1);
                        return None;
                    }
                } else {
                    match analyze_pattern(
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
                        Ok(Some(res)) => res,
                        Ok(None) => {
                            progress_bar.inc(1);
                            return None;
                        }
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
                    let fname = generate_pattern_specific_filename(&file_path, &_root_dir, &pattern_match.pattern_config.description);
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
        .buffer_unordered(max_concurrent) // Claude Code: max 10, API: 50
        .collect::<Vec<_>>()
        .await;
    for result in results.into_iter() {
        if let Some((file_path, response)) = result {
            // Generate the same filename that was used for the actual file output
            let output_filename = if let Some(pattern_desc) = &response.pattern_description {
                generate_pattern_specific_filename(&file_path, &root_dir, pattern_desc)
            } else {
                generate_output_filename(&file_path, &root_dir)
            };
            
            summary.add_result(file_path, response, output_filename);
        }
    }

    progress_bar.finish_with_message(
        messages
            .get("analysis_completed")
            .map_or("Analysis completed!", |s| s),
    );

    summary.sort_by_confidence();

    let mut filtered_summary = if final_args.min_confidence > 0 {
        summary.filter_by_min_confidence(final_args.min_confidence)
    } else {
        summary
    };

    if let Some(types_str) = final_args.vuln_types {
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