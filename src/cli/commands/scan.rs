use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;
use crate::executor::{AgentExecutor, ExecutionResult};
use crate::pipeline::Pipeline;
use crate::prompt::build_all_surface_prompts;

use parsentry_core::{RepoMetadata, ThreatModel};

use super::common::{build_threat_model_cli_prompt, locate_repository, resolve_output_dir};

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

    let pipeline = Pipeline::new(&config)?;
    pipeline.maybe_cleanup();

    let executor = AgentExecutor::new(
        config.agent.path.clone(),
        config.agent.max_concurrent,
        config.agent.timeout_secs,
    );

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

    // Phase 2: Generate threat model (auto, with cache)
    let threat_model = generate_threat_model(
        &repo_metadata,
        &executor,
        &pipeline,
        &printer,
    )
    .await?;

    // Resolve output directory
    let output_dir = resolve_output_dir(&args.output_dir, &repo_name)
        .unwrap_or_else(|| PathBuf::from("parsentry-output"));
    std::fs::create_dir_all(&output_dir)?;

    // Phase 3: Generate per-surface prompts (directly from source)
    let surface_prompts = build_all_surface_prompts(
        &threat_model,
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
        printer.warning("Scan", "no surfaces had readable source files — nothing to dispatch");
        return Ok(());
    }

    // Phase 4: Cache check + parallel agent execution
    let mut cached_results: Vec<ExecutionResult> = Vec::new();
    let mut tasks_to_execute: Vec<(String, String, PathBuf)> = Vec::new();

    for sp in &surface_prompts {
        if let Some(cached) = pipeline.cache_get(&sp.cache_key) {
            printer.info("Cache hit", &sp.surface_id);
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

    // Phase 5: Summary
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

/// Generate threat model automatically via agent, with cache.
///
/// Cache key is based on repo metadata (file structure, languages, deps).
/// If the repo hasn't changed structurally, the same threat model is reused.
async fn generate_threat_model(
    metadata: &RepoMetadata,
    executor: &AgentExecutor,
    pipeline: &Pipeline,
    printer: &StatusPrinter,
) -> Result<ThreatModel> {
    use sha2::{Digest, Sha256};

    let prompt = build_threat_model_cli_prompt(metadata);

    // Cache key = hash of the prompt (repo structure, languages, deps)
    let mut hasher = Sha256::new();
    hasher.update(prompt.as_bytes());
    let cache_key = format!("{:x}", hasher.finalize());

    // Check cache (namespace = "threat-model")
    if let Some(cached) = pipeline.cache_get_ns("threat-model", &cache_key) {
        let model: ThreatModel = serde_json::from_str(&cached)
            .context("cached threat model is invalid JSON")?;
        printer.success(
            "Threat model",
            &format!("{} surfaces (cached)", model.total_surfaces()),
        );
        return Ok(model);
    }

    printer.status("Generating", "threat model via agent");

    // Use a temp file for agent output
    let tmp_dir = tempfile::tempdir()?;
    let output_path = tmp_dir.path().join("threat-model.json");

    // Prompt instructs the agent to write raw JSON (no markdown) to the file
    let full_prompt = format!(
        "{}\n\nWrite ONLY valid JSON to the file: {}\n\
         No markdown, no code fences, no explanation — raw JSON only.",
        prompt,
        output_path.display()
    );

    let result = executor
        .execute_one("threat-model", &full_prompt, &output_path)
        .await
        .context("failed to generate threat model")?;

    if !result.success || result.output.is_empty() {
        anyhow::bail!(
            "Agent failed to generate threat model. \
             Ensure '{}' is installed and accessible.",
            executor.agent_path().display()
        );
    }

    let mut model: ThreatModel = serde_json::from_str(&result.output)
        .context("agent wrote invalid JSON to threat model file")?;

    // Fill in fields the agent may omit
    if model.repository.is_empty() {
        model.repository = metadata.root_dir.display().to_string();
    }
    if model.generated_at.is_empty() {
        model.generated_at = chrono::Utc::now().to_rfc3339();
    }

    printer.success(
        "Threat model",
        &format!("{} surfaces", model.total_surfaces()),
    );

    // Cache the parsed model
    let model_json = serde_json::to_string(&model)?;
    pipeline.cache_set_ns("threat-model", &cache_key, &model_json, prompt.len());

    Ok(model)
}
