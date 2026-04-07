use anyhow::Result;
use std::path::Path;

use crate::cli::ui::StatusPrinter;
use crate::prompt::{build_all_surface_prompts, build_orchestrator_prompt, SurfacePrompt};

use parsentry_core::{RepoMetadata, ThreatModel};

use super::common::{cache_dir_for, locate_repository};

/// Check if a surface has a cached SARIF result with a matching cache key.
fn is_cached(output_dir: &Path, sp: &SurfacePrompt) -> bool {
    let sarif_path = output_dir.join(format!("{}.sarif.json", sp.surface_id));
    let cache_key_path = output_dir.join(format!("{}.cache_key", sp.surface_id));

    if !sarif_path.exists() || !cache_key_path.exists() {
        return false;
    }

    match std::fs::read_to_string(&cache_key_path) {
        Ok(stored_key) => stored_key.trim() == sp.cache_key,
        Err(_) => false,
    }
}

/// Write the cache key sidecar file for a surface.
fn write_cache_key(output_dir: &Path, sp: &SurfacePrompt) -> Result<()> {
    let cache_key_path = output_dir.join(format!("{}.cache_key", sp.surface_id));
    std::fs::write(&cache_key_path, &sp.cache_key)?;
    Ok(())
}

pub async fn run_scan_command(
    target: &str,
    _diff_base: Option<&str>,
    _filter_lang: Option<&str>,
) -> Result<()> {
    let printer = StatusPrinter::new();

    let (root_dir, _repo_name) = locate_repository(target, &printer)?;

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

    // Phase 2: Load threat model from per-repo cache
    let project_cache = cache_dir_for(target);
    let threat_model_path = project_cache.join("model.json");
    let json = std::fs::read_to_string(&threat_model_path)
        .map_err(|e| anyhow::anyhow!("Failed to read threat model {}: {}. Run `parsentry model {}` first.", threat_model_path.display(), e, target))?;
    let threat_model: ThreatModel = serde_json::from_str(&json)
        .map_err(|e| anyhow::anyhow!("Invalid threat model JSON in {}: {}", threat_model_path.display(), e))?;
    printer.status(
        "Loaded",
        &format!("threat model with {} surfaces", threat_model.total_surfaces()),
    );

    // Phase 3: Generate per-surface prompts
    let output_dir = project_cache.join("reports");
    std::fs::create_dir_all(&output_dir)?;

    let surface_prompts = build_all_surface_prompts(
        &threat_model,
        &root_dir,
    );

    if surface_prompts.is_empty() {
        printer.warning("Scan", "no surfaces had readable source files");
        return Ok(());
    }

    // Partition into cached and new surfaces
    let mut cached: Vec<&SurfacePrompt> = Vec::new();
    let mut pending: Vec<&SurfacePrompt> = Vec::new();
    for sp in &surface_prompts {
        if is_cached(&output_dir, sp) {
            cached.push(sp);
        } else {
            pending.push(sp);
        }
    }

    if !cached.is_empty() {
        printer.status(
            "Cached",
            &format!("{} surfaces unchanged (SARIF results reused)", cached.len()),
        );
    }

    if pending.is_empty() {
        printer.success(
            "Complete",
            &format!(
                "all {} surfaces cached, no analysis needed ({})",
                surface_prompts.len(),
                output_dir.display()
            ),
        );
        return Ok(());
    }

    // Write prompts only for pending (non-cached) surfaces
    printer.section("Prompts");
    for sp in &pending {
        let prompt_path = output_dir.join(format!("{}.prompt.md", sp.surface_id));
        let sarif_path = output_dir.join(format!("{}.sarif.json", sp.surface_id));

        let full_prompt = format!(
            "{}\n\nWrite the SARIF JSON output to: {}\n\
             Write ONLY valid JSON. No markdown, no code fences, no explanation.\n",
            sp.prompt,
            sarif_path.display()
        );

        std::fs::write(&prompt_path, &full_prompt)?;
        write_cache_key(&output_dir, sp)?;

        printer.bullet(&format!("{} → {}", sp.surface_id, prompt_path.display()));
    }

    // Phase 4: Generate orchestrator prompt only for pending surfaces
    let pending_owned: Vec<SurfacePrompt> = pending.iter().map(|s| (*s).clone()).collect();
    let orchestrator_content = build_orchestrator_prompt(&pending_owned, &output_dir);
    let orchestrator_path = output_dir.join("orchestrator.prompt.md");
    std::fs::write(&orchestrator_path, &orchestrator_content)?;
    printer.bullet(&format!("orchestrator → {}", orchestrator_path.display()));
    println!("{}", orchestrator_content);

    printer.success(
        "Complete",
        &format!(
            "{} prompts written ({} cached) to {}",
            pending.len(),
            cached.len(),
            output_dir.display()
        ),
    );

    Ok(())
}
