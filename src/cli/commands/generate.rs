use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

use super::common::cache_dir_for;
use crate::cli::ui::StatusPrinter;
use parsentry_reports::merge_sarif_dir;

/// Resolve the reports directory for a given target.
/// Accepts: local directory path (containing *.sarif.json) or owner/repo cache key.
fn resolve_reports_dir(target: &str) -> PathBuf {
    let local = PathBuf::from(target);
    // If target is a local directory containing SARIF files, use it directly
    if local.is_dir() {
        let has_sarif = std::fs::read_dir(&local)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| {
                        e.path()
                            .extension()
                            .is_some_and(|ext| ext == "json")
                            && e.path()
                                .to_str()
                                .is_some_and(|s| s.ends_with(".sarif.json"))
                    })
            })
            .unwrap_or(false);
        if has_sarif {
            return local;
        }
        // Check for reports/ subdirectory
        let sub = local.join("reports");
        if sub.is_dir() {
            return sub;
        }
    }
    cache_dir_for(target).join("reports")
}

/// Locate the pdf-report tool.
fn pdf_tool_dir() -> Result<PathBuf> {
    // 1. PARSENTRY_PDF_TOOL env var (explicit override)
    if let Ok(dir) = std::env::var("PARSENTRY_PDF_TOOL") {
        let p = PathBuf::from(dir);
        if p.join("package.json").exists() {
            return Ok(p);
        }
    }

    // 2. Relative to CWD (development: running from repo root)
    let cwd = std::env::current_dir().unwrap_or_default();
    let from_cwd = cwd.join("tools/pdf-report");
    if from_cwd.join("package.json").exists() {
        return Ok(from_cwd);
    }

    // 3. Relative to the executable (installed layout)
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe;
        for _ in 0..5 {
            dir.pop();
            let candidate = dir.join("tools/pdf-report");
            if candidate.join("package.json").exists() {
                return Ok(candidate);
            }
        }
    }

    bail!(
        "pdf-report tool not found. Set PARSENTRY_PDF_TOOL or run from the repository root."
    )
}

/// Ensure npm dependencies are installed.
fn ensure_deps(tool_dir: &Path, printer: &StatusPrinter) -> Result<()> {
    let node_modules = tool_dir.join("node_modules");
    if node_modules.exists() {
        return Ok(());
    }

    printer.status("Install", "pdf-report dependencies...");
    let status = Command::new("npm")
        .arg("install")
        .arg("--silent")
        .current_dir(tool_dir)
        .status()
        .context("failed to run `npm install` — is Node.js installed?")?;

    if !status.success() {
        bail!("npm install failed (exit {})", status);
    }
    Ok(())
}

/// Merge SARIF + generate report.md + render PDF.
pub async fn run_generate_command(target: &str, output: Option<&str>) -> Result<()> {
    let printer = StatusPrinter::with_service(super::common::repo_name_from_target(target));

    let reports_dir = std::fs::canonicalize(resolve_reports_dir(target))
        .unwrap_or_else(|_| resolve_reports_dir(target));
    if !reports_dir.exists() {
        bail!(
            "Reports directory not found: {}\nRun `parsentry scan` first.",
            reports_dir.display()
        );
    }

    // Phase 1: Merge SARIF
    printer.status("Merge", "merging per-surface SARIF files...");
    let merged = merge_sarif_dir(&reports_dir, None)?;
    let cache_dir = cache_dir_for(target);
    std::fs::create_dir_all(&cache_dir).ok();
    let merged_path = cache_dir.join("merged.sarif.json");
    std::fs::write(&merged_path, serde_json::to_string_pretty(&merged)?)
        .context("failed to write merged.sarif.json")?;
    printer.success(
        "Merged",
        &format!(
            "{} results → {}",
            merged.runs.first().map_or(0, |r| r.results.len()),
            merged_path.display()
        ),
    );

    // Phase 2: Generate report.md
    // Check both source reports_dir and cache_dir; prefer existing one
    let report_md_src = reports_dir.join("report.md");
    let report_md = cache_dir.join("report.md");
    if report_md_src.exists() {
        std::fs::copy(&report_md_src, &report_md).ok();
        printer.status("Report", &format!("using {}", report_md_src.display()));
    } else if report_md.exists() {
        printer.status("Report", &format!("using existing {}", report_md.display()));
    } else {
        printer.status("Report", "report.md not found, generating from SARIF...");
        let md = merged.to_markdown();
        std::fs::write(&report_md, &md).context("failed to write report.md")?;
        printer.success("Report", &format!("generated {}", report_md.display()));
    }

    // Phase 3: Render PDF
    let tool_dir = pdf_tool_dir()?;
    ensure_deps(&tool_dir, &printer)?;

    let output_path = match output {
        Some(p) => std::fs::canonicalize(PathBuf::from(p).parent().unwrap_or(Path::new(".")))
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(PathBuf::from(p).file_name().unwrap_or_default()),
        None => cache_dir_for(target).join("report.pdf"),
    };
    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    printer.status("Render", "generating PDF report...");
    let status = Command::new("npx")
        .args(["tsx", "src/index.tsx"])
        .arg(cache_dir.to_str().unwrap())
        .arg(output_path.to_str().unwrap())
        .current_dir(&tool_dir)
        .status()
        .context("failed to run pdf-report tool — is Node.js installed?")?;

    if !status.success() {
        bail!("PDF generation failed (exit {})", status);
    }

    printer.success("Saved", &format!("{}", output_path.display()));
    Ok(())
}
