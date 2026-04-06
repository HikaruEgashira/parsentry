use anyhow::Result;
use std::path::PathBuf;

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;
use crate::executor::{AgentExecutor, ExecutionResult};
use crate::pipeline::Pipeline;
use crate::prompt::build_all_surface_prompts;

use parsentry_core::{RepoMetadata, ThreatModel};

use super::common::{locate_repository, resolve_output_dir};
use super::query::run_pattern_matching;

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(
        args.config.clone(),
        &args,
        &env_vars,
    )?;

    let target = args.target.clone().unwrap_or_else(|| ".".to_string());
    let (root_dir, repo_name) = locate_repository(&target, &printer)?;

    // Phase 1: Collect repository metadata
    let repo_metadata = RepoMetadata::collect(&root_dir)?;
    printer.status(
        "Collected",
        &format!(
            "{} source files across {} languages",
            repo_metadata.total_files,
            repo_metadata.languages.len()
        ),
    );

    // Phase 2: Load or require threat model
    let threat_model = load_threat_model_for_scan(&args, &printer)?;

    // Phase 3: Tree-sitter pattern matching (filtered by threat model surfaces)
    printer.status("Scanning", "tree-sitter pattern matching");
    let all_pattern_matches = run_pattern_matching(
        &root_dir,
        Some(&threat_model),
        args.diff_base.as_deref(),
        args.filter_lang.as_deref(),
        &printer,
    )
    .await?;
    printer.status("Matched", &format!("{} patterns", all_pattern_matches.len()));

    if all_pattern_matches.is_empty() {
        printer.warning("Scan", "no patterns found — nothing to analyze");
        return Ok(());
    }

    // Resolve output directory
    let output_dir = resolve_output_dir(&args.output_dir, &repo_name)
        .unwrap_or_else(|| PathBuf::from("parsentry-output"));
    std::fs::create_dir_all(&output_dir)?;

    // Phase 4: Generate per-surface prompts
    let surface_prompts = build_all_surface_prompts(
        &threat_model,
        &all_pattern_matches,
        &repo_metadata,
        &root_dir,
        &output_dir,
    );
    printer.status(
        "Prompts",
        &format!(
            "{} surfaces (of {} total)",
            surface_prompts.len(),
            threat_model.total_surfaces()
        ),
    );

    if surface_prompts.is_empty() {
        printer.warning("Scan", "no surfaces matched patterns — nothing to dispatch");
        return Ok(());
    }

    // Phase 5: Cache check + parallel agent execution
    let pipeline = Pipeline::new(&config)?;
    pipeline.maybe_cleanup();

    let mut cached_results: Vec<ExecutionResult> = Vec::new();
    let mut tasks_to_execute: Vec<(String, String, PathBuf)> = Vec::new();

    for sp in &surface_prompts {
        if let Some(cached) = pipeline.cache_get(&sp.cache_key) {
            printer.info("Cache hit", &sp.surface_id);
            // Write cached result to output file
            std::fs::write(&sp.output_path, &cached)?;
            cached_results.push(ExecutionResult {
                surface_id: sp.surface_id.clone(),
                output_path: sp.output_path.clone(),
                output: cached,
                success: true,
                cached: true,
            });
        } else {
            tasks_to_execute.push((
                sp.surface_id.clone(),
                sp.prompt.clone(),
                sp.output_path.clone(),
            ));
        }
    }

    if !cached_results.is_empty() {
        printer.success(
            "Cache",
            &format!("{} surfaces served from cache", cached_results.len()),
        );
    }

    // Execute uncached tasks in parallel
    let mut execution_results = if !tasks_to_execute.is_empty() {
        let executor = AgentExecutor::new(
            config.agent.path.clone(),
            config.agent.max_concurrent,
            config.agent.timeout_secs,
        );
        executor.execute_all(tasks_to_execute).await
    } else {
        vec![]
    };

    // Store successful results in cache
    for result in &execution_results {
        if result.success {
            if let Some(sp) = surface_prompts.iter().find(|sp| sp.surface_id == result.surface_id) {
                pipeline.cache_set(&sp.cache_key, &result.output, sp.prompt.len());
            }
        }
    }

    // Combine all results
    cached_results.append(&mut execution_results);

    // Phase 6: Summary
    let succeeded = cached_results.iter().filter(|r| r.success).count();
    let failed = cached_results.len() - succeeded;

    printer.section("Results");
    for result in &cached_results {
        if result.success {
            let label = if result.cached { "cached" } else { "analyzed" };
            printer.bullet(&format!(
                "{} ({}) → {}",
                result.surface_id,
                label,
                result.output_path.display()
            ));
        } else {
            printer.bullet(&format!("{} — FAILED", result.surface_id));
        }
    }

    printer.success(
        "Complete",
        &format!(
            "{} succeeded, {} failed → {}",
            succeeded,
            failed,
            output_dir.display()
        ),
    );

    Ok(())
}

/// Load threat model from --threat-model flag.
/// When running the full scan, threat model is required.
fn load_threat_model_for_scan(args: &ScanArgs, printer: &StatusPrinter) -> Result<ThreatModel> {
    let path = args.threat_model.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "Threat model required for full scan.\n\
             Generate one first:\n  \
             parsentry model <target> | claude -p > threat-model.json\n  \
             parsentry --threat-model threat-model.json <target>"
        )
    })?;

    let json = if path.to_string_lossy() == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(path)?
    };

    let model: ThreatModel = serde_json::from_str(&json)?;
    printer.status(
        "Loaded",
        &format!("threat model with {} surfaces", model.total_surfaces()),
    );
    Ok(model)
}
