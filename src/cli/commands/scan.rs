use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

use futures::stream::{self, StreamExt};
use tracing::{info, error};

use crate::cli::args::{ScanArgs, validate_scan_args};
use crate::cli::ui::{self, StatusPrinter, SummaryTable, SummaryRow};
use crate::config::ParsentryConfig;
use crate::repo::{RepoOps, clone_github_repo};
use crate::response::{from_claude_code_response, Response, ResponseExt, VulnType};
use crate::mvra::{MvraScanner, MvraRepositoryResult, MvraResults};

use parsentry_analyzer::{analyze_pattern, generate_custom_patterns};

use crate::pattern_generator_claude_code::generate_custom_patterns_with_claude_code;
use parsentry_i18n::{Language, get_messages};
use parsentry_parser::{PatternMatch, SecurityRiskPatterns};
use parsentry_reports::{AnalysisSummary, generate_output_filename, generate_pattern_specific_filename, SarifReport};
use parsentry_utils::FileClassifier;

use parsentry_claude_code::{ClaudeCodeConfig, ClaudeCodeExecutor, PromptBuilder, PatternContext};

/// Analyze a pattern using Claude Code CLI
async fn analyze_with_claude_code(
    executor: &ClaudeCodeExecutor,
    prompt_builder: &PromptBuilder,
    file_path: &PathBuf,
    pattern_match: &PatternMatch,
    _working_dir: &PathBuf,
    printer: &StatusPrinter,
) -> Result<Option<Response>> {
    let file_name = file_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let content = tokio::fs::read_to_string(file_path).await?;

    let pattern_type_str = format!("{:?}", pattern_match.pattern_config.pattern_type);
    let pattern_context = PatternContext::new(
        &pattern_type_str,
        &pattern_match.pattern_config.description,
        &pattern_match.matched_text,
    )
    .with_attack_vectors(pattern_match.pattern_config.attack_vector.clone());

    let prompt = prompt_builder.build_security_analysis_prompt(
        file_path,
        &content,
        Some(&pattern_context),
    );

    let output = executor.execute_with_retry(&prompt, 2).await;

    match output {
        Ok(output) => {
            info!("Claude Code succeeded for {}", file_path.display());
            let mut response = from_claude_code_response(
                output.response,
                file_path.to_string_lossy().to_string(),
            );
            response.pattern_description = Some(pattern_match.pattern_config.description.clone());
            response.matched_source_code = Some(pattern_match.matched_text.clone());
            Ok(Some(response))
        }
        Err(e) => {
            printer.error("Failed", &format!("{}: {}", file_name, e));
            error!("Claude Code execution error for {}: {}", file_path.display(), e);
            Err(anyhow::anyhow!("Claude Code failed: {}", e))
        }
    }
}

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    // Check if MVRA mode is enabled
    if args.mvra {
        return run_mvra_scan(args).await;
    }

    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars
    )?;

    let final_args = config.to_args();

    validate_scan_args(&final_args)?;

    let language = Language::from_string(&final_args.language);
    let messages = get_messages(&language);

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
            printer.status("Cloning", &format!("{} → {}", target, dest.display()));
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
        printer.status("Generating", "custom security patterns");

        if config.claude_code.enabled {
            let claude_path = config.claude_code.path.clone()
                .unwrap_or_else(|| PathBuf::from("claude"));
            let log_dir = final_args.output_dir.as_ref().map(|d| d.join("claude_code_logs"));
            let claude_config = parsentry_claude_code::ClaudeCodeConfig {
                claude_path,
                max_concurrent: config.claude_code.max_concurrent.min(10),
                timeout_secs: config.claude_code.timeout_secs,
                enable_poc: false,
                working_dir: root_dir.clone(),
                log_dir,
            };
            printer.info("Mode", "Claude Code");
            let result = generate_custom_patterns_with_claude_code(&root_dir, claude_config).await?;

            printer.status("Discovered", &format!("{} source files", result.total_files));
            if result.skipped_files > 0 {
                printer.warning("Skipped", &format!("{} files (too large)", result.skipped_files));
            }
            printer.status("Analyzed", &format!("{} files", result.analyzed_files));

            if !result.languages.is_empty() {
                for lang in &result.languages {
                    printer.bullet(&format!("{:?}", lang));
                }
            }

            printer.success("Generated", &format!("{} patterns", result.patterns_generated));
        } else {
            generate_custom_patterns(&root_dir, &final_args.model, api_base_url).await?;
            printer.success("Generated", "custom patterns");
        }
    }

    let repo = RepoOps::new(root_dir.clone());
    let files = repo.get_relevant_files();

    printer.status("Discovered", &format!("{} source files", files.len()));

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

    printer.status("Matched", &format!("{} security patterns", all_pattern_matches.len()));

    // Group patterns by type for display
    let mut pattern_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, pattern_match) in &all_pattern_matches {
        *pattern_types.entry(pattern_match.pattern_config.description.clone()).or_insert(0) += 1;
    }

    for (pattern_desc, count) in &pattern_types {
        printer.bullet(&format!("{} ({})", pattern_desc, count));
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

    let progress_bar = ui::progress::create_bar(total as u64);

    let max_concurrent = if use_claude_code {
        progress_bar.set_message("analyzing with Claude Code");
        config.claude_code.max_concurrent.min(10)
    } else {
        progress_bar.set_message("analyzing patterns");
        50
    };

    let claude_executor = if use_claude_code {
        let claude_path = config.claude_code.path.clone().unwrap_or_else(|| PathBuf::from("claude"));
        let log_dir = output_dir.as_ref().map(|d| d.join("claude_code_logs"));
        let claude_config = ClaudeCodeConfig {
            claude_path: claude_path.clone(),
            max_concurrent: config.claude_code.max_concurrent.min(10),
            timeout_secs: config.claude_code.timeout_secs,
            enable_poc: config.claude_code.enable_poc,
            working_dir: root_dir.as_ref().clone(),
            log_dir: log_dir.clone(),
        };

        printer.info("Mode", "Claude Code enabled");

        Some(Arc::new(ClaudeCodeExecutor::new(claude_config)?))
    } else {
        None
    };

    let prompt_builder = Arc::new(
        PromptBuilder::new()
            .with_poc(config.claude_code.enable_poc)
            .with_language(&final_args.language)
    );

    let printer = Arc::new(printer);

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
            let printer = Arc::clone(&printer);

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
                            &_root_dir,
                            &printer,
                        )
                        .await
                        {
                            Ok(Some(res)) => res,
                            Ok(None) => {
                                info!("Claude Code returned None for {}", file_path.display());
                                progress_bar.inc(1);
                                return None;
                            }
                            Err(e) => {
                                error!(
                                    "Claude Code {}: {}: {}",
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
                printer.error("Error", &format!("Failed to create directory {}: {}", final_output_dir.display(), e));
            } else {
                if !filtered_summary.results.is_empty() {
                    let mut summary_path = final_output_dir.clone();
                    summary_path.push("summary.md");
                    if let Err(e) = std::fs::write(&summary_path, filtered_summary.to_markdown()) {
                        printer.error("Error", &format!("Failed to write {}: {}", summary_path.display(), e));
                    } else {
                        printer.success("Wrote", &format!("{}", summary_path.display()));
                    }
                }
            }
        } else {
            printer.warning("Warning", "Summary report requires --output-dir");
        }
    }

    // Always generate SARIF report
    {
        let sarif_report = SarifReport::from_analysis_summary(&filtered_summary, env!("CARGO_PKG_VERSION"));

        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                printer.error("Error", &format!("Failed to create directory {}: {}", final_output_dir.display(), e));
            } else {
                let mut sarif_path = final_output_dir.clone();
                sarif_path.push("parsentry-results.sarif");
                if let Err(e) = sarif_report.save_to_file(&sarif_path) {
                    printer.error("Error", &format!("Failed to write {}: {}", sarif_path.display(), e));
                } else {
                    printer.success("Wrote", &format!("{}", sarif_path.display()));
                }
            }
        } else {
            // Output SARIF to stdout if no output directory specified
            match sarif_report.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => printer.error("Error", &format!("Failed to output SARIF: {}", e)),
            }
        }
    }

    // Display summary table
    if !filtered_summary.results.is_empty() {
        let mut table = SummaryTable::new();
        for result in &filtered_summary.results {
            table.add(SummaryRow {
                file: result.file_path.to_string_lossy().to_string(),
                pattern: result.response.pattern_description.clone().unwrap_or_default(),
                confidence: result.response.confidence_score as u8,
                vuln_types: result.response.vulnerability_types.iter().map(|v| format!("{:?}", v)).collect(),
            });
        }
        table.print();
    }

    if use_claude_code {
        let success_count = filtered_summary.results.len();
        let high_confidence_count = filtered_summary.results.iter().filter(|r| r.response.confidence_score >= 70).count();
        printer.section("Summary");
        printer.kv("patterns analyzed", &all_pattern_matches.len().to_string());
        printer.kv("vulnerabilities found", &success_count.to_string());
        printer.kv("high confidence", &high_confidence_count.to_string());
    }

    printer.success("Finished", "analysis complete");

    Ok(())
}

/// Run multi-repository variant analysis (MVRA) scan
async fn run_mvra_scan(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    printer.section("Multi-Repository Variant Analysis (MVRA)");

    // Load configuration
    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars
    )?;

    // Build MVRA configuration from CLI args
    let mut mvra_config = config.mvra.clone();

    // Override with CLI arguments if provided
    if let Some(ref query) = args.mvra_search_query {
        mvra_config.search_query = Some(query.clone());
    }

    if let Some(ref query) = args.mvra_code_query {
        mvra_config.code_query = Some(query.clone());
    }

    if args.mvra_max_repos > 0 {
        mvra_config.max_repos = args.mvra_max_repos;
    }

    if let Some(ref repos_str) = args.mvra_repositories {
        let repos: Vec<String> = repos_str.split(',').map(|s| s.trim().to_string()).collect();
        mvra_config.repositories = Some(repos);
    }

    if let Some(min_stars) = args.mvra_min_stars {
        mvra_config.min_stars = Some(min_stars);
    }

    if let Some(ref cache_dir) = args.mvra_cache_dir {
        mvra_config.cache_dir = cache_dir.clone();
    }

    if args.mvra_no_cache {
        mvra_config.use_cache = false;
    }

    // Validate MVRA configuration
    if mvra_config.search_query.is_none()
        && mvra_config.code_query.is_none()
        && mvra_config.repositories.is_none()
    {
        return Err(anyhow::anyhow!(
            "MVRA requires at least one of: --mvra-search-query, --mvra-code-query, or --mvra-repositories"
        ));
    }

    // Create MVRA scanner
    let scanner = MvraScanner::new(mvra_config.clone())?;

    // Get target repositories
    printer.status("Searching", "for target repositories");
    let repositories = scanner.get_target_repositories().await?;

    printer.success("Found", &format!("{} repositories to analyze", repositories.len()));
    for repo in &repositories {
        printer.bullet(&format!("{} (⭐ {})", repo.full_name, repo.stars));
    }

    // Create output directory
    let output_dir = args.output_dir.clone().unwrap_or_else(|| PathBuf::from("./reports/mvra"));
    std::fs::create_dir_all(&output_dir)?;

    printer.status("Output", &format!("{}", output_dir.display()));

    // Scan each repository
    let mut mvra_results = Vec::new();
    let total_repos = repositories.len();

    for (idx, repo) in repositories.iter().enumerate() {
        printer.section(&format!("[{}/{}] Analyzing {}", idx + 1, total_repos, repo.full_name));

        // Clone or get cached repository
        let local_path = match scanner.get_repository(repo) {
            Ok(path) => path,
            Err(e) => {
                printer.error("Failed", &format!("to clone {}: {}", repo.full_name, e));
                mvra_results.push(MvraRepositoryResult {
                    owner: repo.owner.clone(),
                    repo: repo.repo.clone(),
                    full_name: repo.full_name.clone(),
                    html_url: repo.html_url.clone(),
                    stars: repo.stars,
                    description: repo.description.clone(),
                    local_path: PathBuf::new(),
                    analysis: AnalysisSummary::new(),
                    scan_error: Some(format!("Clone failed: {}", e)),
                });
                continue;
            }
        };

        // Create a temporary scan args for this repository
        let mut repo_args = args.clone();
        repo_args.target = Some(local_path.to_string_lossy().to_string());
        repo_args.mvra = false; // Disable MVRA for individual repo scans
        repo_args.output_dir = Some(output_dir.join(&repo.full_name));

        // Run scan for this repository
        match run_single_repo_scan(&repo_args).await {
            Ok(summary) => {
                let vuln_count = summary.results.len();
                printer.success("Completed", &format!("{} (found {} vulnerabilities)", repo.full_name, vuln_count));

                mvra_results.push(MvraRepositoryResult {
                    owner: repo.owner.clone(),
                    repo: repo.repo.clone(),
                    full_name: repo.full_name.clone(),
                    html_url: repo.html_url.clone(),
                    stars: repo.stars,
                    description: repo.description.clone(),
                    local_path,
                    analysis: summary,
                    scan_error: None,
                });
            }
            Err(e) => {
                printer.error("Failed", &format!("to scan {}: {}", repo.full_name, e));
                mvra_results.push(MvraRepositoryResult {
                    owner: repo.owner.clone(),
                    repo: repo.repo.clone(),
                    full_name: repo.full_name.clone(),
                    html_url: repo.html_url.clone(),
                    stars: repo.stars,
                    description: repo.description.clone(),
                    local_path,
                    analysis: AnalysisSummary::new(),
                    scan_error: Some(format!("Scan failed: {}", e)),
                });
            }
        }
    }

    // Analyze variant patterns across repositories
    printer.section("Analyzing Variant Patterns");
    let variant_patterns = scanner.analyze_variants(&mvra_results);

    let successful_scans = mvra_results.iter().filter(|r| r.scan_error.is_none()).count();
    let failed_scans = mvra_results.len() - successful_scans;

    let results = MvraResults {
        repositories: mvra_results,
        total_repositories: total_repos,
        successful_scans,
        failed_scans,
        variant_patterns: variant_patterns.clone(),
    };

    // Generate MVRA report
    printer.status("Generating", "MVRA report");
    let report_path = output_dir.join("mvra-report.md");
    let report_content = generate_mvra_report(&results);
    std::fs::write(&report_path, report_content)?;
    printer.success("Wrote", &format!("{}", report_path.display()));

    // Generate JSON results
    let json_path = output_dir.join("mvra-results.json");
    let json_content = serde_json::to_string_pretty(&results)?;
    std::fs::write(&json_path, json_content)?;
    printer.success("Wrote", &format!("{}", json_path.display()));

    // Print summary
    printer.section("Summary");
    printer.kv("total repositories", &total_repos.to_string());
    printer.kv("successful scans", &successful_scans.to_string());
    printer.kv("failed scans", &failed_scans.to_string());
    printer.kv("variant patterns", &variant_patterns.len().to_string());

    // Display top variant patterns
    if !variant_patterns.is_empty() {
        printer.section("Top Variant Patterns");
        for (idx, pattern) in variant_patterns.iter().take(10).enumerate() {
            printer.bullet(&format!(
                "{}: {} (found in {} repositories, {} occurrences)",
                idx + 1,
                pattern.vulnerability_type,
                pattern.repository_count,
                pattern.occurrences.len()
            ));
        }
    }

    printer.success("Finished", "MVRA analysis complete");

    Ok(())
}

/// Run a single repository scan and return the analysis summary
async fn run_single_repo_scan(args: &ScanArgs) -> Result<AnalysisSummary> {
    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        args,
        &env_vars
    )?;

    let final_args = config.to_args();
    let language = Language::from_string(&final_args.language);
    let api_base_url = final_args.api_base_url.as_deref();

    let (root_dir, _repo_name) = if let Some(target) = &final_args.target {
        (PathBuf::from(target), None::<String>)
    } else {
        return Err(anyhow::anyhow!("Target must be specified"));
    };

    let repo = RepoOps::new(root_dir.clone());
    let files = repo.get_relevant_files();

    // Collect all pattern matches
    let mut all_pattern_matches = Vec::new();

    for file_path in &files {
        if let Ok(content) = std::fs::read_to_string(file_path) {
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

    let root_dir = Arc::new(root_dir);
    let output_dir = final_args.output_dir.clone();
    let model = final_args.model.clone();
    let files = files.clone();
    let verbosity = final_args.verbosity;
    let debug = final_args.debug;
    let use_claude_code = config.claude_code.enabled;

    let mut summary = AnalysisSummary::new();

    let max_concurrent = if use_claude_code {
        config.claude_code.max_concurrent.min(10)
    } else {
        50
    };

    let claude_executor = if use_claude_code {
        let claude_path = config.claude_code.path.clone().unwrap_or_else(|| PathBuf::from("claude"));
        let log_dir = output_dir.as_ref().map(|d| d.join("claude_code_logs"));
        let claude_config = ClaudeCodeConfig {
            claude_path: claude_path.clone(),
            max_concurrent: config.claude_code.max_concurrent.min(10),
            timeout_secs: config.claude_code.timeout_secs,
            enable_poc: config.claude_code.enable_poc,
            working_dir: root_dir.as_ref().clone(),
            log_dir: log_dir.clone(),
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

    let printer = Arc::new(StatusPrinter::new());

    let results = stream::iter(all_pattern_matches.iter().enumerate())
        .map(|(idx, (file_path, pattern_match))| {
            let file_path = file_path.clone();
            let pattern_match = pattern_match.clone();
            let _root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let model = model.clone();
            let files = files.clone();
            let debug = debug;
            let language = language.clone();
            let claude_executor = claude_executor.clone();
            let prompt_builder = prompt_builder.clone();
            let use_claude_code = use_claude_code;
            let printer = Arc::clone(&printer);

            async move {
                // Choose analysis method based on mode
                let analysis_result = if use_claude_code {
                    if let Some(ref executor) = claude_executor {
                        match analyze_with_claude_code(
                            executor,
                            &prompt_builder,
                            &file_path,
                            &pattern_match,
                            &_root_dir,
                            &printer,
                        )
                        .await
                        {
                            Ok(Some(res)) => res,
                            Ok(None) | Err(_) => return None,
                        }
                    } else {
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
                        Ok(None) | Err(_) => return None,
                    }
                };

                if analysis_result.vulnerability_types.is_empty()
                    || analysis_result.confidence_score < 1
                {
                    return None;
                }

                Some((file_path, analysis_result))
            }
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;

    for result in results.into_iter() {
        if let Some((file_path, response)) = result {
            let output_filename = if let Some(pattern_desc) = &response.pattern_description {
                generate_pattern_specific_filename(&file_path, &root_dir, pattern_desc)
            } else {
                generate_output_filename(&file_path, &root_dir)
            };

            summary.add_result(file_path, response, output_filename);
        }
    }

    summary.sort_by_confidence();

    let filtered_summary = if final_args.min_confidence > 0 {
        summary.filter_by_min_confidence(final_args.min_confidence)
    } else {
        summary
    };

    Ok(filtered_summary)
}

/// Generate MVRA markdown report
fn generate_mvra_report(results: &MvraResults) -> String {
    let mut report = String::new();

    report.push_str("# Multi-Repository Variant Analysis Report\n\n");

    // Summary section
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- **Total Repositories**: {}\n", results.total_repositories));
    report.push_str(&format!("- **Successful Scans**: {}\n", results.successful_scans));
    report.push_str(&format!("- **Failed Scans**: {}\n", results.failed_scans));
    report.push_str(&format!("- **Variant Patterns Found**: {}\n\n", results.variant_patterns.len()));

    // Variant patterns section
    if !results.variant_patterns.is_empty() {
        report.push_str("## Variant Patterns\n\n");
        report.push_str("Security vulnerability patterns found across multiple repositories:\n\n");

        for (idx, pattern) in results.variant_patterns.iter().enumerate() {
            report.push_str(&format!("### {}. {} ({})\n\n", idx + 1, pattern.vulnerability_type, pattern.description));
            report.push_str(&format!("- **Found in**: {} repositories\n", pattern.repository_count));
            report.push_str(&format!("- **Total Occurrences**: {}\n\n", pattern.occurrences.len()));

            // Group occurrences by repository
            let mut repo_occurrences: std::collections::HashMap<String, Vec<&crate::mvra::PatternOccurrence>> = std::collections::HashMap::new();
            for occurrence in &pattern.occurrences {
                repo_occurrences.entry(occurrence.repository.clone())
                    .or_insert_with(Vec::new)
                    .push(occurrence);
            }

            report.push_str("#### Repositories Affected:\n\n");
            for (repo, occurrences) in repo_occurrences.iter() {
                report.push_str(&format!("**{}** ({} occurrences):\n", repo, occurrences.len()));
                for occurrence in occurrences {
                    report.push_str(&format!("- `{}` (confidence: {})\n",
                        occurrence.file_path, occurrence.confidence_score));
                }
                report.push_str("\n");
            }
        }
    }

    // Repository details section
    report.push_str("## Repository Analysis Results\n\n");

    for repo_result in &results.repositories {
        report.push_str(&format!("### {}\n\n", repo_result.full_name));
        report.push_str(&format!("- **URL**: {}\n", repo_result.html_url));
        report.push_str(&format!("- **Stars**: {}\n", repo_result.stars));

        if let Some(ref desc) = repo_result.description {
            report.push_str(&format!("- **Description**: {}\n", desc));
        }

        if let Some(ref error) = repo_result.scan_error {
            report.push_str(&format!("- **Status**: ❌ Failed - {}\n\n", error));
        } else {
            let vuln_count = repo_result.analysis.results.len();
            report.push_str(&format!("- **Status**: ✅ Success\n"));
            report.push_str(&format!("- **Vulnerabilities Found**: {}\n\n", vuln_count));

            if vuln_count > 0 {
                report.push_str("#### Findings:\n\n");
                for result in &repo_result.analysis.results {
                    let vuln_types: Vec<String> = result.response.vulnerability_types
                        .iter()
                        .map(|v| format!("{:?}", v))
                        .collect();

                    report.push_str(&format!(
                        "- `{}`: {} (confidence: {})\n",
                        result.file_path.display(),
                        vuln_types.join(", "),
                        result.response.confidence_score
                    ));
                }
                report.push_str("\n");
            }
        }
    }

    report
}