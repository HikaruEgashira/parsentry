use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::ui::StatusPrinter;
use parsentry_reports::{SarifReport, SarifResult};

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

    bail!("pdf-report tool not found. Set PARSENTRY_PDF_TOOL or run from the repository root.")
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

/// Render `report.pdf` from the report files in `cache_dir` to `output`.
pub fn render_pdf(cache_dir: &Path, output: &str, printer: &StatusPrinter) -> Result<PathBuf> {
    let tool_dir = pdf_tool_dir()?;
    ensure_deps(&tool_dir, printer)?;

    let output = PathBuf::from(output);
    let output_path = std::fs::canonicalize(output.parent().unwrap_or(Path::new(".")))
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(output.file_name().unwrap_or_default());
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    printer.status("Render", "generating PDF report...");
    let status = Command::new("npx")
        .args(["tsx", "src/index.tsx"])
        .arg(cache_dir)
        .arg(&output_path)
        .current_dir(&tool_dir)
        .status()
        .context("failed to run pdf-report tool — is Node.js installed?")?;

    if !status.success() {
        bail!("PDF generation failed (exit {})", status);
    }
    Ok(output_path)
}

/// Render the Markdown report from a merged SARIF.
///
/// Results marked `absent` (present in the baseline, gone now) are excluded
/// from the findings list and only appear in the model diff section, which
/// is rendered whenever the merge ran against a previous model's results.
pub fn render_report_markdown(merged: &SarifReport, has_baseline: bool) -> String {
    let mut current = merged.clone();
    for run in &mut current.runs {
        run.results
            .retain(|r| r.baseline_state.as_deref() != Some("absent"));
    }
    let mut md = current.to_markdown();
    if !has_baseline {
        return md;
    }

    let results: Vec<&SarifResult> = merged.runs.iter().flat_map(|r| &r.results).collect();
    let by_state = |state: &str| -> Vec<&SarifResult> {
        results
            .iter()
            .copied()
            .filter(|r| r.baseline_state.as_deref() == Some(state))
            .collect()
    };
    let new = by_state("new");
    let absent = by_state("absent");
    let unchanged = by_state("unchanged");

    md.push_str("\n# Model Diff\n\n");
    md.push_str(
        "Compared against the results of the previous threat model. \
         `new` findings were detected only by the current model; \
         `absent` findings were detected only by the previous model.\n\n",
    );
    md.push_str("| State | Count |\n|---|---|\n");
    md.push_str(&format!("| new | {} |\n", new.len()));
    md.push_str(&format!("| absent | {} |\n", absent.len()));
    md.push_str(&format!("| unchanged | {} |\n\n", unchanged.len()));
    for (title, items) in [("New findings", &new), ("Absent findings", &absent)] {
        if items.is_empty() {
            continue;
        }
        md.push_str(&format!("## {}\n\n", title));
        for r in items.iter() {
            let uri = r
                .locations
                .first()
                .map(|l| l.physical_location.artifact_location.uri.as_str())
                .unwrap_or("-");
            md.push_str(&format!("- `{}` ({}) — {}\n", r.rule_id, r.level, uri));
        }
        md.push('\n');
    }
    md
}
