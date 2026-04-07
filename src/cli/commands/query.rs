use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{IsTerminal, Read as IoRead};
use std::path::Path;

use crate::cli::ui::StatusPrinter;

use parsentry_core::ThreatModel;

use super::common::locate_repository;

#[derive(Debug, Serialize)]
pub struct ResolvedFile {
    pub path: String,
    pub exists: bool,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct SurfaceFiles {
    pub surface_id: String,
    pub identifier: String,
    pub locations: Vec<String>,
    pub resolved_files: Vec<ResolvedFile>,
}

fn load_threat_model(path: Option<&Path>) -> Result<ThreatModel> {
    let json = match path {
        Some(p) => std::fs::read_to_string(p)
            .with_context(|| format!("Failed to read: {}", p.display()))?,
        None => {
            if std::io::stdin().is_terminal() {
                anyhow::bail!("Provide --threat-model <file> or pipe from stdin");
            }
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    serde_json::from_str(json.trim()).context("Failed to parse threat model JSON")
}

pub async fn run_query_command(target: &str, threat_model_path: Option<&Path>) -> Result<()> {
    let printer = StatusPrinter::new();

    let (root_dir, _) = locate_repository(target, &printer)?;
    let threat_model = load_threat_model(threat_model_path)?;

    printer.status("Loaded", &format!("{} surfaces", threat_model.total_surfaces()));

    let results: Vec<SurfaceFiles> = threat_model
        .surfaces
        .iter()
        .map(|surface| {
            let resolved_files: Vec<ResolvedFile> = surface
                .locations
                .iter()
                .map(|loc| {
                    let full = root_dir.join(loc);
                    let exists = full.exists();
                    let size = if exists { std::fs::metadata(&full).ok().map(|m| m.len()) } else { None };
                    ResolvedFile { path: loc.clone(), exists, size }
                })
                .collect();
            SurfaceFiles {
                surface_id: surface.id.clone(),
                identifier: surface.identifier.clone(),
                locations: surface.locations.clone(),
                resolved_files,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
