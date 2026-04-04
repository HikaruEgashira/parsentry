use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use futures::stream::{self, StreamExt};

use crate::cli::args::{validate_scan_args, ScanArgs};
use crate::cli::ui::{self, StatusPrinter, SummaryRow, SummaryTable};
use crate::config::ParsentryConfig;
use crate::github::clone_repo;
use crate::repo::RepoOps;
use crate::response::{Response, VulnType};

use parsentry_cache::Cache;
use parsentry_claude_code::{ClaudeCodeConfig, ClaudeCodeExecutor, StreamCallback};
use parsentry_core::Language;
use parsentry_threat_model::{
    RepoMetadata, ThreatModelGenerator, render_threat_model_md,
    THREAT_MODEL_SYSTEM_PROMPT, build_threat_model_prompt,
    parse_threat_model_response, threat_model_schema,
};
use parsentry_parser::{PatternMatch, PatternType, SecurityRiskPatterns};
use parsentry_reports::{
    generate_output_filename, generate_pattern_specific_filename, AnalysisSummary, SarifReport,
};
use parsentry_utils::FileClassifier;

/// Cache mode for analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheMode {
    Normal,
    NoCache,
    CacheOnly,
}

impl CacheMode {
    pub fn from_flags(cache: bool, cache_only: bool) -> Self {
        if cache_only {
            CacheMode::CacheOnly
        } else if cache {
            CacheMode::Normal
        } else {
            CacheMode::NoCache
        }
    }
}

/// Build a prompt for analyzing a file with a detected pattern.
fn build_analysis_prompt(
    file_path: &Path,
    pattern_type: Option<&str>,
    matched_code: Option<&str>,
    sarif_output_path: Option<&Path>,
) -> String {
    let mut parts = Vec::new();

    match (pattern_type, matched_code) {
        (Some(pt), Some(mc)) => {
            parts.push(format!(
                "Analyze {} for security vulnerabilities.\n{} pattern detected: `{}`",
                file_path.display(), pt, mc
            ));
        }
        _ => {
            parts.push(format!(
                "Analyze {} for security vulnerabilities.",
                file_path.display()
            ));
        }
    }

    if let Some(path) = sarif_output_path {
        parts.push(format!("Write SARIF v2.1.0 results to {}.", path.display()));
    }

    parts.join("\n")
}

/// Analyze a pattern using Claude Code CLI with direct SARIF output.
async fn analyze_with_claude_code<C: StreamCallback>(
    executor: &ClaudeCodeExecutor,
    cache: Option<&Cache>,
    cache_mode: CacheMode,
    file_path: &PathBuf,
    pattern_match: &PatternMatch,
    sarif_output_path: &PathBuf,
    printer: &StatusPrinter,
    streaming_callback: &C,
) -> Result<bool> {
    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let pattern_type_str = format!("{:?}", pattern_match.pattern_config.pattern_type);
    let model = "claude-code";

    // Try pattern-based cache first
    if cache_mode != CacheMode::NoCache {
        if let Some(cache) = cache {
            if let Ok(Some(cached_sarif)) = cache.get_by_pattern(&pattern_type_str, &pattern_match.matched_text, model) {
                if let Some(parent) = sarif_output_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(sarif_output_path, &cached_sarif).is_ok() {
                    printer.status("Cache hit", &file_name);
                    return Ok(true);
                }
            }
        }
    }

    if sarif_output_path.exists() {
        printer.status("Cached", &file_name);
        return Ok(true);
    }

    if cache_mode == CacheMode::CacheOnly {
        return Ok(false);
    }

    let prompt = build_analysis_prompt(
        file_path,
        Some(&pattern_type_str),
        Some(pattern_match.matched_text.as_str()),
        Some(sarif_output_path),
    );

    printer.status("Analyzing", &file_name);

    let result = executor
        .execute_streaming_file_output(&prompt, streaming_callback)
        .await;

    match result {
        Ok(output) => {
            if output.success && sarif_output_path.exists() {
                if cache_mode != CacheMode::NoCache {
                    if let Some(cache) = cache {
                        if let Ok(sarif_json) = std::fs::read_to_string(sarif_output_path) {
                            let _ = cache.set_by_pattern(&pattern_type_str, &pattern_match.matched_text, model, &sarif_json);
                        }
                    }
                }
                printer.status("Completed", &file_name);
                Ok(true)
            } else if output.success {
                printer.status("No findings", &file_name);
                Ok(false)
            } else {
                printer.error("Failed", &format!("{}: non-zero exit", file_name));
                Ok(false)
            }
        }
        Err(e) => {
            printer.error("Failed", &format!("{}: {}", file_name, e));
            Err(anyhow::anyhow!("Claude Code SARIF output failed: {}", e))
        }
    }
}

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars
    )?;

    let final_args = config.to_args();
    validate_scan_args(&final_args)?;

    let api_base_url = final_args.api_base_url.as_deref();

    // Phase 0: Locate repository
    let (root_dir, repo_name) = if let Some(target) = &final_args.target {
        if target.contains('/') && !std::path::Path::new(target).exists() {
            let dest = PathBuf::from("repo");
            if dest.exists() {
                std::fs::remove_dir_all(&dest)?;
            }
            printer.status("Cloning", &format!("{} → {}", target, dest.display()));
            clone_repo(target, &dest)?;
            let repo_name = target.split('/').last().unwrap_or("unknown-repo").replace(".git", "");
            (dest, Some(repo_name))
        } else {
            (PathBuf::from(target), None)
        }
    } else {
        return Err(anyhow::anyhow!("Target must be specified"));
    };

    // Phase 1: Enumerate attack surfaces
    printer.status("Phase 1", "enumerating attack surfaces");
    let repo_metadata = RepoMetadata::collect(&root_dir)?;

    printer.status("Discovered", &format!("{} source files across {} languages",
        repo_metadata.total_files,
        repo_metadata.languages.len()
    ));

    // Threat model cache: key is derived from repo metadata (structure, languages, manifests)
    let tm_cache_mode = CacheMode::from_flags(final_args.cache, final_args.cache_only);
    let tm_cache = if config.cache.enabled && tm_cache_mode != CacheMode::NoCache {
        Cache::new(&config.cache.directory).ok()
    } else {
        None
    };
    let tm_cache_prompt = repo_metadata.to_prompt_context();
    let tm_cache_model = if config.agent.is_claude_code() { "claude-code" } else { &final_args.model };
    let tm_cache_agent = "threat-model";

    // Try cache first
    let cached_threat_model = if tm_cache_mode != CacheMode::NoCache {
        tm_cache.as_ref().and_then(|c| {
            c.get(&tm_cache_prompt, tm_cache_model, tm_cache_agent).ok().flatten()
        }).and_then(|json| {
            serde_json::from_str::<parsentry_threat_model::ThreatModel>(&json).ok()
        })
    } else {
        None
    };

    let threat_model = if let Some(tm) = cached_threat_model {
        printer.status("Cache hit", "threat model loaded from cache");
        tm
    } else if tm_cache_mode == CacheMode::CacheOnly {
        return Err(anyhow::anyhow!("Threat model not found in cache (cache-only mode)"));
    } else {
        printer.status("Generating", "threat model via LLM");
        let tm = if config.agent.is_claude_code() {
            let claude_path = config.agent.path.clone().unwrap_or_else(|| PathBuf::from("claude"));
            let tm_config = ClaudeCodeConfig {
                claude_path,
                max_concurrent: 1,
                timeout_secs: config.agent.timeout_secs,
                enable_poc: false,
                working_dir: root_dir.clone(),
                log_dir: None,
                model: Some("sonnet".to_string()),
            };
            let tm_executor = ClaudeCodeExecutor::new(tm_config)?;
            let prompt = build_threat_model_cli_prompt(&repo_metadata);
            let raw = tm_executor.execute_raw(&prompt).await
                .map_err(|e| anyhow::anyhow!("Claude Code threat model generation failed: {}", e))?;
            let json_str = extract_json_from_text(&raw)
                .ok_or_else(|| anyhow::anyhow!("Failed to extract JSON from Claude Code response"))?;
            parse_threat_model_response(&json_str, &root_dir.to_string_lossy())?
        } else {
            let threat_generator = ThreatModelGenerator::new(&final_args.model, api_base_url);
            threat_generator.generate(&repo_metadata).await?
        };

        // Store in cache
        if tm_cache_mode != CacheMode::NoCache {
            if let Some(cache) = &tm_cache {
                if let Ok(json) = serde_json::to_string(&tm) {
                    let _ = cache.set(&tm_cache_prompt, tm_cache_model, tm_cache_agent, &json);
                }
            }
        }

        tm
    };

    printer.success("Phase 1", &format!(
        "{} attack surfaces identified ({})",
        threat_model.total_surfaces(),
        threat_model.app_type
    ));

    for surface in &threat_model.surfaces {
        printer.bullet(&format!("{} — [{}] {}", surface.id, surface.kind, surface.identifier));
    }

    // Write threat-model.md
    let output_dir = resolve_output_dir(&final_args.output_dir, &repo_name);
    if let Some(ref dir) = output_dir {
        let _ = std::fs::create_dir_all(dir);
        let md = render_threat_model_md(&threat_model);
        let md_path = dir.join("threat-model.md");
        if let Err(e) = std::fs::write(&md_path, &md) {
            printer.error("Error", &format!("Failed to write threat model: {}", e));
        } else {
            printer.success("Wrote", &format!("{}", md_path.display()));
        }
    }

    // Phase 2: Run tree-sitter queries per surface in parallel
    printer.status("Phase 2", "scanning attack surfaces");

    let repo = RepoOps::new(root_dir.clone());
    let files = repo.get_relevant_files();

    // Filter to changed files when --diff-base is specified
    let files = if let Some(ref diff_base) = final_args.diff_base {
        let changed = get_diff_files(&root_dir, diff_base)?;
        if changed.is_empty() {
            printer.success("Finished", "no changed files found");
            return Ok(());
        }
        let filtered: Vec<_> = files.into_iter().filter(|f| changed.contains(f)).collect();
        printer.status("Diff filtered", &format!("{} changed files (base: {})", filtered.len(), diff_base));
        filtered
    } else {
        files
    };

    let lang_filter: Option<HashSet<Language>> = final_args.filter_lang.as_ref().and_then(|filter_str| {
        let languages: HashSet<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();
        if languages.is_empty() { None } else { Some(languages) }
    });

    // Collect files referenced by attack surfaces
    let surface_locations: HashSet<String> = threat_model.all_locations().into_iter().collect();

    let root_dir_arc = Arc::new(root_dir.clone());
    let lang_filter = Arc::new(lang_filter);
    let concurrency = (num_cpus::get() * 4).max(16);

    let file_results: Vec<_> = stream::iter(files.clone())
        .map(|file_path| {
            let root_dir = Arc::clone(&root_dir_arc);
            let lang_filter = Arc::clone(&lang_filter);
            let surface_locations = surface_locations.clone();
            async move {
                tokio::task::spawn_blocking(move || {
                    // Only scan files referenced by attack surfaces
                    let rel_path = file_path.strip_prefix(root_dir.as_ref())
                        .unwrap_or(&file_path)
                        .to_string_lossy()
                        .to_string();
                    let is_surface_file = surface_locations.iter().any(|loc| {
                        rel_path.contains(loc) || loc.contains(&rel_path)
                    });
                    if !is_surface_file {
                        return None;
                    }

                    std::fs::read_to_string(&file_path)
                        .ok()
                        .and_then(|content| {
                            if content.len() > 50_000 {
                                return None;
                            }

                            let filename = file_path.to_string_lossy();
                            let lang = FileClassifier::classify(&filename, &content);

                            if let Some(filter) = lang_filter.as_ref() {
                                if !filter.contains(&lang) {
                                    return None;
                                }
                            }

                            let patterns = SecurityRiskPatterns::new_with_root(lang, Some(&root_dir));
                            let matches = patterns.get_pattern_matches(&content);

                            Some(
                                matches
                                    .into_iter()
                                    .map(|pattern_match| (file_path.clone(), pattern_match))
                                    .collect::<Vec<_>>()
                            )
                        })
                })
                .await
                .ok()
                .flatten()
            }
        })
        .buffer_unordered(concurrency)
        .collect()
        .await;

    let mut all_pattern_matches: Vec<(PathBuf, PatternMatch)> = Vec::new();
    for results in file_results {
        if let Some(matches) = results {
            all_pattern_matches.extend(matches);
        }
    }

    // Filter: only Resource patterns in files that also have a Principal
    let files_with_principals: HashSet<PathBuf> = all_pattern_matches
        .iter()
        .filter(|(_, pm)| pm.pattern_type == PatternType::Principal)
        .map(|(path, _)| path.clone())
        .collect();

    let all_pattern_matches: Vec<(PathBuf, PatternMatch)> = if files_with_principals.is_empty() {
        printer.warning("Skipped", "No Principal (input source) detected — no attack vectors");
        Vec::new()
    } else {
        all_pattern_matches
            .into_iter()
            .filter(|(file_path, pm)| {
                pm.pattern_type == PatternType::Resource && files_with_principals.contains(file_path)
            })
            .collect()
    };

    printer.status("Phase 2", &format!("{} patterns matched for analysis", all_pattern_matches.len()));

    // Phase 3: Analyze each match with Claude Code
    printer.status("Phase 3", "analyzing with Claude Code");

    let total = all_pattern_matches.len();
    let root_dir = Arc::new(root_dir);

    let cache_mode = CacheMode::from_flags(final_args.cache, final_args.cache_only);
    let cache = if config.cache.enabled && cache_mode != CacheMode::NoCache {
        Cache::new(&config.cache.directory).ok().map(Arc::new)
    } else {
        None
    };

    let claude_path = config.agent.path.clone().unwrap_or_else(|| PathBuf::from("claude"));
    let claude_config = ClaudeCodeConfig {
        claude_path,
        max_concurrent: config.agent.max_concurrent.min(10),
        timeout_secs: config.agent.timeout_secs,
        enable_poc: config.agent.enable_poc,
        working_dir: root_dir.as_ref().clone(),
        log_dir: None,
        model: Some("haiku".to_string()),
    };
    let claude_executor = Arc::new(ClaudeCodeExecutor::new(claude_config)?);

    let progress_bar = ui::progress::create_bar(total as u64);
    progress_bar.set_message("analyzing with Claude Code");

    let printer = Arc::new(printer);
    let max_concurrent = config.agent.max_concurrent.min(10);

    let mut summary = AnalysisSummary::new();

    let results = stream::iter(all_pattern_matches.iter().enumerate())
        .map(|(_idx, (file_path, pattern_match))| {
            let file_path = file_path.clone();
            let pattern_match = pattern_match.clone();
            let _root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let progress_bar = progress_bar.clone();
            let executor = Arc::clone(&claude_executor);
            let printer = Arc::clone(&printer);
            let cache = cache.clone();

            async move {
                let sarif_filename = format!(
                    "{}-{}.sarif",
                    file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown"),
                    pattern_match.pattern_config.description.replace(' ', "-").to_lowercase()
                );
                let sarif_output_path = output_dir
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("./reports"))
                    .join(&sarif_filename);

                if let Some(parent) = sarif_output_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                let analysis_result = match analyze_with_claude_code(
                    &executor,
                    cache.as_deref(),
                    cache_mode,
                    &file_path,
                    &pattern_match,
                    &sarif_output_path,
                    &printer,
                    &parsentry_claude_code::NoOpCallback,
                )
                .await
                {
                    Ok(true) => {
                        match SarifReport::from_file(&sarif_output_path) {
                            Ok(sarif) => {
                                let first_result = sarif.runs.first()
                                    .and_then(|r| r.results.first());

                                if let Some(result) = first_result {
                                    let confidence = result.properties.as_ref()
                                        .and_then(|p| p.confidence)
                                        .map(|c| (c * 100.0) as i32)
                                        .unwrap_or(0);

                                    Response {
                                        scratchpad: String::new(),
                                        analysis: result.message.markdown.clone()
                                            .unwrap_or_else(|| result.message.text.clone()),
                                        poc: String::new(),
                                        confidence_score: confidence,
                                        vulnerability_types: vec![
                                            VulnType::from_str(&result.rule_id).unwrap_or(VulnType::Other(result.rule_id.clone()))
                                        ],
                                        par_analysis: extract_par_from_sarif_result(result),
                                        remediation_guidance: parsentry_core::RemediationGuidance {
                                            policy_enforcement: vec![],
                                        },
                                        file_path: Some(file_path.to_string_lossy().to_string()),
                                        pattern_description: Some(pattern_match.pattern_config.description.clone()),
                                        matched_source_code: Some(pattern_match.matched_text.clone()),
                                        full_source_code: None,
                                    }
                                } else {
                                    progress_bar.inc(1);
                                    return None;
                                }
                            }
                            Err(_) => {
                                progress_bar.inc(1);
                                return None;
                            }
                        }
                    }
                    Ok(false) | Err(_) => {
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

                progress_bar.inc(1);
                Some((file_path, analysis_result))
            }
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;

    for result in results {
        if let Some((file_path, response)) = result {
            let output_filename = if let Some(pattern_desc) = &response.pattern_description {
                generate_pattern_specific_filename(&file_path, &root_dir, pattern_desc)
            } else {
                generate_output_filename(&file_path, &root_dir)
            };
            summary.add_result(file_path, response, output_filename);
        }
    }

    progress_bar.finish_with_message("Analysis completed!");

    // Phase 4: Generate report
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

    // SARIF report
    let sarif_report = SarifReport::from_analysis_summary(&filtered_summary, env!("CARGO_PKG_VERSION"));

    if let Some(ref dir) = output_dir {
        let _ = std::fs::create_dir_all(dir);

        if !filtered_summary.results.is_empty() {
            let summary_path = dir.join("summary.md");
            let _ = std::fs::write(&summary_path, filtered_summary.to_markdown());
            printer.success("Wrote", &format!("{}", summary_path.display()));
        }

        let sarif_path = dir.join("parsentry-results.sarif");
        if let Err(e) = sarif_report.save_to_file(&sarif_path) {
            printer.error("Error", &format!("Failed to write SARIF: {}", e));
        } else {
            printer.success("Wrote", &format!("{}", sarif_path.display()));
        }
    } else {
        match sarif_report.to_json() {
            Ok(json) => println!("{}", json),
            Err(e) => printer.error("Error", &format!("Failed to output SARIF: {}", e)),
        }
    }

    // Summary table
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

    let success_count = filtered_summary.results.len();
    let high_confidence_count = filtered_summary.results.iter().filter(|r| r.response.confidence_score >= 70).count();
    printer.section("Summary");
    printer.kv("attack surfaces", &threat_model.total_surfaces().to_string());
    printer.kv("patterns analyzed", &all_pattern_matches.len().to_string());
    printer.kv("vulnerabilities found", &success_count.to_string());
    printer.kv("high confidence", &high_confidence_count.to_string());

    printer.success("Finished", "analysis complete");
    Ok(())
}

fn resolve_output_dir(base: &Option<PathBuf>, repo_name: &Option<String>) -> Option<PathBuf> {
    base.as_ref().map(|dir| {
        if let Some(name) = repo_name {
            dir.join(name)
        } else {
            dir.clone()
        }
    })
}

fn get_diff_files(root_dir: &Path, diff_base: &str) -> Result<HashSet<PathBuf>> {
    let three_dot = format!("{}...HEAD", diff_base);
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", "--diff-filter=ACMR", &three_dot])
        .current_dir(root_dir)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => {
            std::process::Command::new("git")
                .args(["diff", "--name-only", "--diff-filter=ACMR", diff_base])
                .current_dir(root_dir)
                .output()
                .map_err(|e| anyhow::anyhow!("git diff failed: {}", e))?
        }
    };

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "git diff failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().filter(|l| !l.is_empty()).map(|l| root_dir.join(l.trim())).collect())
}

fn extract_par_from_sarif_result(result: &parsentry_reports::SarifResult) -> parsentry_core::ParAnalysis {
    let mut principals = Vec::new();
    let mut actions = Vec::new();
    let mut resources = Vec::new();
    let mut policy_violations = Vec::new();

    if let Some(props) = &result.properties {
        if let Some(principal) = &props.principal {
            principals.push(parsentry_core::PrincipalInfo {
                identifier: principal.clone(),
                trust_level: parsentry_core::TrustLevel::Untrusted,
                source_context: String::new(),
                risk_factors: vec![],
            });
        }

        if let Some(action) = &props.action {
            let quality = if action.to_lowercase().contains("なし")
                || action.to_lowercase().contains("no ")
                || action.to_lowercase().contains("missing") {
                parsentry_core::SecurityFunctionQuality::Missing
            } else if action.to_lowercase().contains("insufficient") {
                parsentry_core::SecurityFunctionQuality::Insufficient
            } else {
                parsentry_core::SecurityFunctionQuality::Bypassed
            };

            actions.push(parsentry_core::ActionInfo {
                identifier: action.clone(),
                security_function: String::new(),
                implementation_quality: quality,
                detected_weaknesses: vec![],
                bypass_vectors: vec![],
            });
        }

        if let Some(resource) = &props.resource {
            resources.push(parsentry_core::ResourceInfo {
                identifier: resource.clone(),
                sensitivity_level: parsentry_core::SensitivityLevel::Critical,
                operation_type: String::new(),
                protection_mechanisms: vec![],
            });
        }

        if let Some(data_flow) = &props.data_flow {
            policy_violations.push(parsentry_core::PolicyViolation {
                rule_id: result.rule_id.clone(),
                rule_description: format!("Unsafe data flow: {}", data_flow),
                violation_path: data_flow.clone(),
                severity: result.level.clone(),
                confidence: props.confidence.unwrap_or(0.0),
            });
        }
    }

    parsentry_core::ParAnalysis {
        principals,
        actions,
        resources,
        policy_violations,
    }
}

fn build_threat_model_cli_prompt(metadata: &RepoMetadata) -> String {
    let repo_context = metadata.to_prompt_context();
    let languages: Vec<String> = metadata
        .languages
        .keys()
        .map(|l| format!("{:?}", l))
        .collect();
    let user_prompt = build_threat_model_prompt(&repo_context, &languages);
    let schema = serde_json::to_string_pretty(&threat_model_schema()).unwrap_or_default();

    format!(
        "{system}\n\n{user}\n\nIMPORTANT: Return ONLY valid JSON matching this schema, with no markdown wrapping or extra text:\n{schema}",
        system = THREAT_MODEL_SYSTEM_PROMPT,
        user = user_prompt,
        schema = schema,
    )
}

fn extract_json_from_text(text: &str) -> Option<String> {
    // Try markdown code block first
    if let Some(start) = text.find("```json") {
        let json_start = start + 7;
        if let Some(end) = text[json_start..].find("```") {
            return Some(text[json_start..json_start + end].trim().to_string());
        }
    }
    if let Some(start) = text.find("```") {
        let json_start = start + 3;
        let json_start = text[json_start..]
            .find('\n')
            .map(|n| json_start + n + 1)
            .unwrap_or(json_start);
        if let Some(end) = text[json_start..].find("```") {
            return Some(text[json_start..json_start + end].trim().to_string());
        }
    }

    // Try raw JSON object
    if let Some(start) = text.find('{') {
        let mut depth = 0;
        for (i, c) in text[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(text[start..start + i + 1].to_string());
                    }
                }
                _ => {}
            }
        }
    }

    None
}
