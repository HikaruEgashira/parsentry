use anyhow::Result;
use serde::Serialize;
use std::collections::HashSet;
use std::io::Read as IoRead;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use futures::stream::{self, StreamExt};

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;
use crate::repo::RepoOps;

use parsentry_core::{FileClassifier, Language, ThreatModel};
use parsentry_parser::{PatternMatch, SecurityRiskPatterns};

use super::common::locate_repository;

/// A serializable pattern match result for JSON output.
#[derive(Debug, Serialize)]
pub struct MatchResult {
    pub file: String,
    pub description: String,
    pub matched_text: String,
}

/// Load threat model from file path or stdin.
fn load_threat_model(path: Option<&Path>) -> Result<ThreatModel> {
    let json = match path {
        Some(p) if p.to_string_lossy() != "-" => std::fs::read_to_string(p)?,
        _ => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    Ok(serde_json::from_str(&json)?)
}

/// Get files changed relative to a diff base ref.
fn get_diff_files(root_dir: &Path, diff_base: &str) -> Result<HashSet<PathBuf>> {
    let three_dot = format!("{}...HEAD", diff_base);
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", "--diff-filter=ACMR", &three_dot])
        .current_dir(root_dir)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => std::process::Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=ACMR", diff_base])
            .current_dir(root_dir)
            .output()
            .map_err(|e| anyhow::anyhow!("git diff failed: {}", e))?,
    };

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "git diff failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| root_dir.join(l.trim()))
        .collect())
}

/// Run tree-sitter pattern matching.
///
/// When `threat_model` is provided, only scans files referenced by surfaces.
/// When `None`, scans all relevant files in the repository.
pub async fn run_pattern_matching(
    root_dir: &Path,
    threat_model: Option<&ThreatModel>,
    diff_base: Option<&str>,
    filter_lang: Option<&str>,
    printer: &StatusPrinter,
) -> Result<Vec<(PathBuf, PatternMatch)>> {
    let repo = RepoOps::new(root_dir.to_path_buf());
    let files = repo.get_relevant_files();

    // Filter to changed files when diff_base is specified
    let files = if let Some(diff_base) = diff_base {
        let changed = get_diff_files(root_dir, diff_base)?;
        if changed.is_empty() {
            printer.success("Finished", "no changed files found");
            return Ok(Vec::new());
        }
        let filtered: Vec<_> = files.into_iter().filter(|f| changed.contains(f)).collect();
        printer.status(
            "Diff filtered",
            &format!("{} changed files (base: {})", filtered.len(), diff_base),
        );
        filtered
    } else {
        files
    };

    let lang_filter: Option<HashSet<Language>> = filter_lang.and_then(|filter_str| {
        let languages: HashSet<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();
        if languages.is_empty() {
            None
        } else {
            Some(languages)
        }
    });

    // Surface location filter (empty = scan all files)
    let surface_locations: Option<HashSet<String>> = threat_model
        .map(|tm| tm.all_locations().into_iter().collect());

    let root_dir_arc = Arc::new(root_dir.to_path_buf());
    let lang_filter = Arc::new(lang_filter);
    let surface_locations = Arc::new(surface_locations);
    let concurrency = (num_cpus::get() * 4).max(16);

    let file_results: Vec<_> = stream::iter(files.clone())
        .map(|file_path| {
            let root_dir = Arc::clone(&root_dir_arc);
            let lang_filter = Arc::clone(&lang_filter);
            let surface_locations = Arc::clone(&surface_locations);
            async move {
                tokio::task::spawn_blocking(move || {
                    // Filter by surface locations if threat model is provided
                    if let Some(ref locations) = *surface_locations {
                        let rel_path = file_path
                            .strip_prefix(root_dir.as_ref())
                            .unwrap_or(&file_path)
                            .to_string_lossy()
                            .to_string();
                        let is_surface_file = locations
                            .iter()
                            .any(|loc| rel_path.contains(loc) || loc.contains(&rel_path));
                        if !is_surface_file {
                            return None;
                        }
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

                            let patterns =
                                SecurityRiskPatterns::new_with_root(lang, Some(&root_dir));
                            let matches = patterns.get_pattern_matches(&content);

                            Some(
                                matches
                                    .into_iter()
                                    .map(|pattern_match| (file_path.clone(), pattern_match))
                                    .collect::<Vec<_>>(),
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

    Ok(all_pattern_matches)
}

/// Entry point for the `query` subcommand.
pub async fn run_query_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(args.config.clone(), &args, &env_vars)?;
    let final_args = config.to_args();

    let target = final_args.target.clone().unwrap_or_else(|| ".".to_string());
    let (root_dir, _repo_name) = locate_repository(&target, &printer)?;

    // Load threat model from --threat-model flag or stdin
    let threat_model = load_threat_model(args.threat_model.as_deref())?;

    printer.status(
        "Loaded",
        &format!(
            "threat model with {} surfaces",
            threat_model.total_surfaces()
        ),
    );

    let pattern_matches = run_pattern_matching(
        &root_dir,
        Some(&threat_model),
        args.diff_base.as_deref(),
        args.filter_lang.as_deref(),
        &printer,
    )
    .await?;

    printer.status("Matched", &format!("{} patterns", pattern_matches.len()));

    // Output JSON to stdout
    let results: Vec<MatchResult> = pattern_matches
        .iter()
        .map(|(file_path, pm)| MatchResult {
            file: file_path
                .strip_prefix(&root_dir)
                .unwrap_or(file_path)
                .to_string_lossy()
                .to_string(),
            description: pm.pattern_config.description.clone(),
            matched_text: pm.matched_text.clone(),
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&results)?);

    Ok(())
}
