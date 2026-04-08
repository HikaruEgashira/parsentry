use anyhow::{Context, Result};
use std::path::Path;

use crate::sarif::{SarifReport, SarifResult};

/// HTML comment used to embed fingerprint in issue/ticket body for deduplication.
pub const FINGERPRINT_MARKER: &str = "<!-- parsentry-fp:";

/// HTML comment used to embed surface name in parent issue body for deduplication.
pub const SURFACE_MARKER: &str = "<!-- parsentry-surface:";

/// A surface and its filtered results, loaded from a per-surface `result.sarif.json`.
pub struct SurfaceReport {
    pub surface_name: String,
    pub results: Vec<SarifResult>,
}

/// Load per-surface SARIF reports from `reports_dir`.
///
/// Each immediate subdirectory of `reports_dir` that contains a
/// `result.sarif.json` file is treated as one surface.  Results are
/// filtered by `min_level` before being returned.
pub fn load_surface_reports(reports_dir: &Path, min_level: &str) -> Result<Vec<SurfaceReport>> {
    let mut surfaces = Vec::new();

    let entries = std::fs::read_dir(reports_dir)
        .with_context(|| format!("cannot read reports directory: {}", reports_dir.display()))?;

    let mut dirs: Vec<std::path::PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let sarif_path = dir.join("result.sarif.json");
        if !sarif_path.exists() {
            continue;
        }

        let surface_name = dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let content = std::fs::read_to_string(&sarif_path)
            .with_context(|| format!("cannot read {}", sarif_path.display()))?;
        let report: SarifReport = serde_json::from_str(&content)
            .with_context(|| format!("invalid SARIF JSON in {}", sarif_path.display()))?;

        let results: Vec<SarifResult> = report
            .runs
            .into_iter()
            .flat_map(|r| r.results.into_iter())
            .filter(|r| level_passes(&r.level, min_level))
            .collect();

        surfaces.push(SurfaceReport {
            surface_name,
            results,
        });
    }

    Ok(surfaces)
}

pub fn collect_results<'a>(report: &'a SarifReport, min_level: &str) -> Vec<&'a SarifResult> {
    report
        .runs
        .iter()
        .flat_map(|r| r.results.iter())
        .filter(|r| level_passes(&r.level, min_level))
        .collect()
}

pub fn extract_fingerprint(result: &SarifResult) -> Option<String> {
    result
        .fingerprints
        .as_ref()
        .and_then(|fp| fp.get("parsentry/v1"))
        .cloned()
}

/// error > warning > note > none
pub fn level_passes(level: &str, min_level: &str) -> bool {
    fn rank(l: &str) -> u8 {
        match l {
            "error" => 3,
            "warning" => 2,
            "note" => 1,
            _ => 0,
        }
    }
    rank(level) >= rank(min_level)
}

pub fn build_title(result: &SarifResult) -> String {
    let location = result
        .locations
        .first()
        .map(|l| l.physical_location.artifact_location.uri.as_str())
        .unwrap_or("unknown");
    let file = location.split('/').next_back().unwrap_or(location);
    format!(
        "[{}] {} in {}",
        result.level.to_uppercase(),
        result.rule_id,
        file
    )
}

/// Markdown body shared by GitHub Issues, Jira, and Linear.
pub fn build_markdown_body(result: &SarifResult, fingerprint: Option<&str>) -> String {
    let mut body = String::new();

    body.push_str(&format!("## {}\n\n", result.rule_id));
    body.push_str(&format!("{}\n\n", result.message.text));

    if !result.locations.is_empty() {
        body.push_str("## Locations\n\n");
        for loc in &result.locations {
            let uri = &loc.physical_location.artifact_location.uri;
            let line = loc
                .physical_location
                .region
                .as_ref()
                .map(|r| format!(":{}", r.start_line))
                .unwrap_or_default();
            body.push_str(&format!("- `{uri}{line}`\n"));
            if let Some(region) = &loc.physical_location.region {
                if let Some(snippet) = &region.snippet {
                    body.push_str(&format!("\n````\n{}\n````\n\n", snippet.text));
                }
            }
        }
        body.push('\n');
    }

    if let Some(props) = &result.properties {
        body.push_str("## Classification\n\n");
        if let Some(c) = props.confidence {
            body.push_str(&format!("- **Confidence**: {c:.0}%\n"));
        }
        if let Some(cwe) = &props.cwe {
            body.push_str(&format!("- **CWE**: {}\n", cwe.join(", ")));
        }
        if let Some(owasp) = &props.owasp {
            body.push_str(&format!("- **OWASP**: {}\n", owasp.join(", ")));
        }
        if let Some(mitre) = &props.mitre_attack {
            body.push_str(&format!("- **MITRE ATT&CK**: {}\n", mitre.join(", ")));
        }
        body.push('\n');
    }

    body.push_str("---\n*Generated by [parsentry](https://github.com/HikaruEgashira/parsentry)*\n");

    if let Some(fp) = fingerprint {
        let fp_escaped = escape_html_comment(fp);
        body.push_str(&format!("\n{FINGERPRINT_MARKER} {} -->", fp_escaped));
    }

    body
}

pub fn parse_fingerprint_from_body(body: &str) -> Option<String> {
    let start = body.find(FINGERPRINT_MARKER)?;
    let rest = &body[start + FINGERPRINT_MARKER.len()..];
    let end = rest.find("-->")?;
    Some(rest[..end].trim().to_string())
}

pub fn parse_surface_from_body(body: &str) -> Option<String> {
    let start = body.find(SURFACE_MARKER)?;
    let rest = &body[start + SURFACE_MARKER.len()..];
    let end = rest.find("-->")?;
    Some(rest[..end].trim().to_string())
}

/// Escape HTML comments to prevent Markdown injection or XSS in external report output.
pub fn escape_html_comment(content: &str) -> String {
    content.replace("-->", "==>")
}
