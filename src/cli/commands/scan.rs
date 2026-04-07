use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::cli::ui::StatusPrinter;
use crate::prompt::build_all_surface_prompts;

use parsentry_core::{RepoMetadata, ThreatModel};

use super::common::{locate_repository, resolve_output_dir};

pub async fn run_scan_command(
    threat_model_path: &Path,
    target: &str,
    output_dir: Option<&Path>,
    _diff_base: Option<&str>,
    _filter_lang: Option<&str>,
) -> Result<()> {
    let printer = StatusPrinter::new();

    let (root_dir, repo_name) = locate_repository(target, &printer)?;

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

    // Phase 2: Load threat model
    let json = std::fs::read_to_string(threat_model_path)?;
    let threat_model: ThreatModel = serde_json::from_str(&json)?;
    printer.status(
        "Loaded",
        &format!("threat model with {} surfaces", threat_model.total_surfaces()),
    );

    // Resolve output directory
    let output_dir = output_dir
        .map(|p| p.to_path_buf())
        .or_else(|| resolve_output_dir(&None, &repo_name))
        .unwrap_or_else(|| PathBuf::from("parsentry-output"));
    std::fs::create_dir_all(&output_dir)?;

    // Phase 3: Generate per-surface prompts and write to files
    let surface_prompts = build_all_surface_prompts(
        &threat_model,
        &repo_metadata,
        &root_dir,
    );

    if surface_prompts.is_empty() {
        printer.warning("Scan", "no surfaces had readable source files");
        return Ok(());
    }

    printer.section("Prompts");
    for sp in &surface_prompts {
        let prompt_path = output_dir.join(format!("{}.prompt.md", sp.surface_id));
        let abs_sarif = std::fs::canonicalize(&output_dir)
            .unwrap_or(output_dir.clone())
            .join(format!("{}.sarif.json", sp.surface_id));

        // Append output instruction with the concrete SARIF file path
        let full_prompt = format!(
            "{}\n\nWrite the SARIF JSON output to: {}\n\
             Write ONLY valid JSON. No markdown, no code fences, no explanation.\n",
            sp.prompt,
            abs_sarif.display()
        );

        std::fs::write(&prompt_path, &full_prompt)?;

        printer.bullet(&format!("{} → {}", sp.surface_id, prompt_path.display()));
        println!("{}", prompt_path.display());
    }

    printer.success(
        "Complete",
        &format!(
            "{} prompts written to {}",
            surface_prompts.len(),
            output_dir.display()
        ),
    );

    Ok(())
}
