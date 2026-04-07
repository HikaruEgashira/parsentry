use anyhow::Result;
use std::io::Read as _;
use std::path::PathBuf;

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;
use crate::prompt::build_all_surface_prompts;

use parsentry_core::{RepoMetadata, ThreatModel};

use super::common::{locate_repository, resolve_output_dir};

pub async fn run_scan_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let _config = ParsentryConfig::load_with_precedence(
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

    // Phase 2: Load threat model
    let threat_model = load_threat_model(&args, &printer)?;

    // Resolve output directory
    let output_dir = resolve_output_dir(&args.output_dir, &repo_name)
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
        // Write prompt file (e.g., SURFACE-001.prompt.md)
        let prompt_path = output_dir.join(format!("{}.prompt.md", sp.surface_id));
        std::fs::write(&prompt_path, &sp.prompt)?;

        printer.bullet(&format!("{} → {}", sp.surface_id, prompt_path.display()));

        // Output prompt file path to stdout for piping
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

/// Load threat model from --threat-model file or stdin.
fn load_threat_model(args: &ScanArgs, printer: &StatusPrinter) -> Result<ThreatModel> {
    let json = match args.threat_model.as_deref() {
        Some(path) if path.to_string_lossy() != "-" => {
            std::fs::read_to_string(path)?
        }
        _ => {
            // Read from stdin
            if atty::is(atty::Stream::Stdin) {
                anyhow::bail!(
                    "Threat model required. Pipe from stdin or use --threat-model:\n  \
                     parsentry model <target> | claude -p | parsentry <target>\n  \
                     parsentry --threat-model model.json <target>"
                );
            }
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            if buf.trim().is_empty() {
                anyhow::bail!("Empty input on stdin");
            }
            buf
        }
    };

    let model: ThreatModel = serde_json::from_str(&json)?;
    printer.status(
        "Loaded",
        &format!("threat model with {} surfaces", model.total_surfaces()),
    );
    Ok(model)
}
