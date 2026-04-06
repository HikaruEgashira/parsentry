use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;

use parsentry_parser::PatternMatch;
use parsentry_core::RepoMetadata;

use super::common::locate_repository;
use super::query::run_pattern_matching;

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let _config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars
    )?;

    // Phase 0: Locate repository (default: current directory)
    let target = args.target.clone().unwrap_or_else(|| ".".to_string());
    let (root_dir, _repo_name) = locate_repository(&target, &printer)?;

    // Collect repository metadata
    let repo_metadata = RepoMetadata::collect(&root_dir)?;

    printer.status(
        "Collected",
        &format!(
            "{} source files across {} languages",
            repo_metadata.total_files,
            repo_metadata.languages.len()
        ),
    );

    // Pattern matching (all files, no threat model filtering)
    printer.status("Scanning", "tree-sitter pattern matching");

    let all_pattern_matches = run_pattern_matching(
        &root_dir,
        None,
        args.diff_base.as_deref(),
        args.filter_lang.as_deref(),
        &printer,
    ).await?;

    printer.status("Matched", &format!("{} patterns", all_pattern_matches.len()));

    // Output prompt to stdout
    emit_prompt(&repo_metadata, &all_pattern_matches, &root_dir, &printer)
}

/// Emit analysis prompt to stdout for piping (e.g. `parsentry | claude`).
fn emit_prompt(
    metadata: &RepoMetadata,
    pattern_matches: &[(PathBuf, PatternMatch)],
    root_dir: &Path,
    printer: &StatusPrinter,
) -> Result<()> {
    if pattern_matches.is_empty() {
        printer.warning("Prompt", "no patterns to analyze");
        return Ok(());
    }

    let repo_context = metadata.to_prompt_context();

    // Deduplicate: same file + same matched_text
    let mut seen = std::collections::HashSet::new();
    let deduped: Vec<_> = pattern_matches
        .iter()
        .filter(|(fp, pm)| {
            let key = (
                fp.strip_prefix(root_dir).unwrap_or(fp).to_string_lossy().to_string(),
                pm.matched_text.clone(),
            );
            seen.insert(key)
        })
        .collect();

    let mut prompt = String::new();
    prompt.push_str("You are a security auditor. Analyze the following code patterns for vulnerabilities and output SARIF v2.1.0 results.\n\n");
    prompt.push_str("## Repository Context\n\n");
    prompt.push_str(&repo_context);
    prompt.push_str("\n\n## Detected Patterns\n\n");

    for (file_path, pattern_match) in &deduped {
        let rel_path = file_path
            .strip_prefix(root_dir)
            .unwrap_or(file_path)
            .display();
        let pat_type = format!("{:?}", pattern_match.pattern_type);

        prompt.push_str(&format!(
            "### {} — {}\n\n**Type**: {} | **Pattern**: {}\n```\n{}\n```\n\n",
            rel_path,
            pattern_match.pattern_config.description,
            pat_type,
            pattern_match.pattern_config.description,
            pattern_match.matched_text,
        ));
    }

    prompt.push_str("## Output Instructions\n\n");
    prompt.push_str("For each finding, provide:\n");
    prompt.push_str("- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)\n");
    prompt.push_str("- level: error/warning/note\n");
    prompt.push_str("- confidence: 0.0-1.0\n");
    prompt.push_str("- PAR analysis: principal (input source), action (security function), resource (target)\n\n");
    prompt.push_str("Output valid SARIF v2.1.0 JSON.\n");

    print!("{}", prompt);

    printer.success("Prompt", &format!("{} unique patterns (from {} total)", deduped.len(), pattern_matches.len()));
    Ok(())
}
