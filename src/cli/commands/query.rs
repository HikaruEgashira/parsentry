use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{IsTerminal, Read as IoRead};
use std::path::Path;

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;

use parsentry_core::ThreatModel;

use super::common::locate_repository;

/// A resolved file entry for a surface location.
#[derive(Debug, Serialize)]
pub struct ResolvedFile {
    pub path: String,
    pub exists: bool,
    pub size: Option<u64>,
}

/// Output entry per surface showing resolved file locations.
#[derive(Debug, Serialize)]
pub struct SurfaceFiles {
    pub surface_id: String,
    pub identifier: String,
    pub locations: Vec<String>,
    pub resolved_files: Vec<ResolvedFile>,
}

/// Load threat model from file path or stdin.
fn load_threat_model(path: Option<&Path>) -> Result<ThreatModel> {
    let json = match path {
        Some(p) if p.to_string_lossy() != "-" => std::fs::read_to_string(p)
            .with_context(|| format!("Failed to read threat model file: {}", p.display()))?,
        Some(_) | None => {
            if std::io::stdin().is_terminal() {
                anyhow::bail!(
                    "No threat model provided. Use --threat-model <file> or pipe from stdin:\n  \
                     parsentry model <repo> | parsentry query <repo>"
                );
            }
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            if buf.trim().is_empty() {
                anyhow::bail!(
                    "Empty input on stdin. Provide a threat model JSON:\n  \
                     parsentry model <repo> | parsentry query <repo>"
                );
            }
            buf
        }
    };
    // Try parsing as-is first, then try extracting JSON from markdown code blocks
    let json_str = json.trim();
    if let Ok(model) = serde_json::from_str::<ThreatModel>(json_str) {
        return Ok(model);
    }
    // Extract JSON from markdown code block (```json ... ``` or ``` ... ```)
    let extracted = extract_json_from_markdown(json_str);
    serde_json::from_str(&extracted).context(
        "Failed to parse threat model JSON. Ensure the input is valid JSON \
         (markdown code blocks are automatically stripped)",
    )
}

/// Extract JSON content from a markdown code block if present.
fn extract_json_from_markdown(text: &str) -> String {
    // Try ```json ... ``` first, then ``` ... ```
    if let Some(start) = text.find("```json") {
        let after = &text[start + 7..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }
    if let Some(start) = text.find("```") {
        let after = &text[start + 3..];
        // Skip the language identifier line
        let after = if let Some(nl) = after.find('\n') {
            &after[nl + 1..]
        } else {
            after
        };
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }
    // Try finding the first { ... } block
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return text[start..=end].to_string();
        }
    }
    text.to_string()
}

/// Resolve a location string to actual files under root_dir.
fn resolve_location(root_dir: &Path, location: &str) -> ResolvedFile {
    let full_path = root_dir.join(location);
    let exists = full_path.exists();
    let size = if exists {
        std::fs::metadata(&full_path).ok().map(|m| m.len())
    } else {
        None
    };
    ResolvedFile {
        path: location.to_string(),
        exists,
        size,
    }
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

    // Resolve each surface's locations to actual files
    let results: Vec<SurfaceFiles> = threat_model
        .surfaces
        .iter()
        .map(|surface| {
            let resolved_files: Vec<ResolvedFile> = surface
                .locations
                .iter()
                .map(|loc| resolve_location(&root_dir, loc))
                .collect();
            SurfaceFiles {
                surface_id: surface.id.clone(),
                identifier: surface.identifier.clone(),
                locations: surface.locations.clone(),
                resolved_files,
            }
        })
        .collect();

    let total_resolved = results
        .iter()
        .flat_map(|s| &s.resolved_files)
        .filter(|f| f.exists)
        .count();

    printer.status(
        "Resolved",
        &format!(
            "{} files across {} surfaces",
            total_resolved,
            results.len()
        ),
    );

    // Output JSON to stdout
    println!("{}", serde_json::to_string_pretty(&results)?);

    Ok(())
}
