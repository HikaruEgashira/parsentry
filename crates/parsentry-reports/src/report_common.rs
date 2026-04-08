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
                    let safe_snippet = snippet.text.replace("```", "` ` `");
                    body.push_str(&format!("\n```\n{}\n```\n\n", safe_snippet));
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
        let safe_fp = fp.replace("--", "\\-\\-");
        body.push_str(&format!("\n{FINGERPRINT_MARKER} {safe_fp} -->"));
    }

    body
}

pub fn parse_fingerprint_from_body(body: &str) -> Option<String> {
    let start = body.find(FINGERPRINT_MARKER)?;
    let rest = &body[start + FINGERPRINT_MARKER.len()..];
    let end = rest.find("-->")?;
    Some(rest[..end].trim().replace("\\-\\-", "--"))
}

pub fn parse_surface_from_body(body: &str) -> Option<String> {
    let start = body.find(SURFACE_MARKER)?;
    let rest = &body[start + SURFACE_MARKER.len()..];
    let end = rest.find("-->")?;
    Some(rest[..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_result(rule_id: &str, level: &str, uri: &str) -> SarifResult {
        SarifResult {
            rule_id: rule_id.to_string(),
            rule_index: None,
            level: level.to_string(),
            message: crate::sarif::SarifMessage {
                text: format!("{} found", rule_id),
                markdown: None,
            },
            locations: vec![crate::sarif::SarifLocation {
                physical_location: crate::sarif::SarifPhysicalLocation {
                    artifact_location: crate::sarif::SarifArtifactLocation {
                        uri: uri.to_string(),
                        index: None,
                    },
                    region: None,
                },
            }],
            fingerprints: None,
            baseline_state: None,
            suppressions: None,
            properties: None,
        }
    }

    fn make_result_with_fingerprint(rule_id: &str, level: &str, fp: &str) -> SarifResult {
        let mut r = make_result(rule_id, level, "test.py");
        let mut fps = HashMap::new();
        fps.insert("parsentry/v1".to_string(), fp.to_string());
        r.fingerprints = Some(fps);
        r
    }

    // --- level_passes ---

    #[test]
    fn test_level_passes_error_threshold() {
        assert!(level_passes("error", "error"));
        assert!(level_passes("error", "warning"));
        assert!(level_passes("error", "note"));
        assert!(!level_passes("warning", "error"));
    }

    #[test]
    fn test_level_passes_warning_threshold() {
        assert!(level_passes("warning", "warning"));
        assert!(level_passes("warning", "note"));
        assert!(!level_passes("note", "warning"));
    }

    #[test]
    fn test_level_passes_note_threshold() {
        assert!(level_passes("note", "note"));
        assert!(!level_passes("none", "note"));
    }

    #[test]
    fn test_level_passes_unknown_level() {
        assert!(level_passes("unknown", "none"));
        assert!(!level_passes("unknown", "note"));
    }

    #[test]
    fn test_level_passes_same_level() {
        assert!(level_passes("error", "error"));
        assert!(level_passes("warning", "warning"));
        assert!(level_passes("note", "note"));
    }

    // --- extract_fingerprint ---

    #[test]
    fn test_extract_fingerprint_present() {
        let result = make_result_with_fingerprint("SQLI", "error", "abc123");
        assert_eq!(extract_fingerprint(&result), Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_fingerprint_absent() {
        let result = make_result("SQLI", "error", "test.py");
        assert_eq!(extract_fingerprint(&result), None);
    }

    #[test]
    fn test_extract_fingerprint_wrong_key() {
        let mut result = make_result("SQLI", "error", "test.py");
        let mut fps = HashMap::new();
        fps.insert("other/v1".to_string(), "abc".to_string());
        result.fingerprints = Some(fps);
        assert_eq!(extract_fingerprint(&result), None);
    }

    // --- parse_fingerprint_from_body / parse_surface_from_body roundtrip ---

    #[test]
    fn test_parse_fingerprint_roundtrip() {
        let fp = "abc123def";
        let body = format!("some text\n{} {} -->\nmore text", FINGERPRINT_MARKER, fp);
        assert_eq!(parse_fingerprint_from_body(&body), Some(fp.to_string()));
    }

    #[test]
    fn test_parse_fingerprint_not_present() {
        assert_eq!(parse_fingerprint_from_body("no marker here"), None);
    }

    #[test]
    fn test_parse_fingerprint_with_dashes() {
        let fp = "abc--def--ghi";
        let escaped = fp.replace("--", "\\-\\-");
        let body = format!("{} {} -->", FINGERPRINT_MARKER, escaped);
        assert_eq!(parse_fingerprint_from_body(&body), Some(fp.to_string()));
    }

    #[test]
    fn test_parse_surface_roundtrip() {
        let surface = "api-endpoint";
        let body = format!("text\n{} {} -->\nmore", SURFACE_MARKER, surface);
        assert_eq!(parse_surface_from_body(&body), Some(surface.to_string()));
    }

    #[test]
    fn test_parse_surface_not_present() {
        assert_eq!(parse_surface_from_body("no marker here"), None);
    }

    // --- build_title ---

    #[test]
    fn test_build_title_basic() {
        let result = make_result("SQLI", "error", "src/app.py");
        let title = build_title(&result);
        assert!(title.contains("[ERROR]"));
        assert!(title.contains("SQLI"));
        assert!(title.contains("app.py"));
    }

    #[test]
    fn test_build_title_no_locations() {
        let result = SarifResult {
            rule_id: "XSS".to_string(),
            rule_index: None,
            level: "warning".to_string(),
            message: crate::sarif::SarifMessage {
                text: "test".to_string(),
                markdown: None,
            },
            locations: vec![],
            fingerprints: None,
            baseline_state: None,
            suppressions: None,
            properties: None,
        };
        let title = build_title(&result);
        assert!(title.contains("unknown"));
        assert!(title.contains("[WARNING]"));
    }

    #[test]
    fn test_build_title_extracts_filename() {
        let result = make_result("RCE", "error", "deep/nested/path/to/file.js");
        let title = build_title(&result);
        assert!(title.contains("file.js"));
    }

    // --- build_markdown_body ---

    #[test]
    fn test_build_markdown_body_basic() {
        let result = make_result("SQLI", "error", "test.py");
        let body = build_markdown_body(&result, None);
        assert!(body.contains("## SQLI"));
        assert!(body.contains("SQLI found"));
        assert!(body.contains("parsentry"));
    }

    #[test]
    fn test_build_markdown_body_with_fingerprint() {
        let result = make_result("SQLI", "error", "test.py");
        let body = build_markdown_body(&result, Some("fp123"));
        assert!(body.contains("fp123"));
        assert!(body.contains(FINGERPRINT_MARKER));
    }

    #[test]
    fn test_build_markdown_body_no_locations() {
        let result = SarifResult {
            rule_id: "XSS".to_string(),
            rule_index: None,
            level: "warning".to_string(),
            message: crate::sarif::SarifMessage {
                text: "xss test".to_string(),
                markdown: None,
            },
            locations: vec![],
            fingerprints: None,
            baseline_state: None,
            suppressions: None,
            properties: None,
        };
        let body = build_markdown_body(&result, None);
        assert!(!body.contains("## Locations"));
    }

    #[test]
    fn test_build_markdown_body_with_properties() {
        let mut result = make_result("SQLI", "error", "test.py");
        result.properties = Some(crate::sarif::SarifResultProperties {
            confidence: Some(85.0),
            cwe: Some(vec!["CWE-89".to_string()]),
            owasp: Some(vec!["A03".to_string()]),
            mitre_attack: Some(vec!["T1190".to_string()]),
            principal: None,
            action: None,
            resource: None,
            data_flow: None,
        });
        let body = build_markdown_body(&result, None);
        assert!(body.contains("## Classification"));
        assert!(body.contains("85%"));
        assert!(body.contains("CWE-89"));
        assert!(body.contains("A03"));
        assert!(body.contains("T1190"));
    }

    #[test]
    fn test_build_markdown_body_sanitizes_code_fences() {
        let mut result = make_result("SQLI", "error", "test.py");
        result.locations[0].physical_location.region = Some(crate::sarif::SarifRegion {
            start_line: 1,
            start_column: None,
            end_line: None,
            end_column: None,
            snippet: Some(crate::sarif::SarifArtifactContent {
                text: "some ``` code ``` here".to_string(),
            }),
        });
        let body = build_markdown_body(&result, None);
        // Should NOT contain triple backtick in snippet
        assert!(!body.contains("some ```"));
        assert!(body.contains("` ` `"));
    }

    #[test]
    fn test_build_markdown_body_fingerprint_dash_escape() {
        let result = make_result("SQLI", "error", "test.py");
        let body = build_markdown_body(&result, Some("abc--def"));
        assert!(body.contains("\\-\\-"));
    }

    // --- collect_results ---

    #[test]
    fn test_collect_results_filters_by_level() {
        let report = crate::sarif::SarifReport {
            schema: "".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![crate::sarif::SarifRun {
                tool: crate::sarif::SarifTool {
                    driver: crate::sarif::SarifDriver {
                        name: "test".to_string(),
                        version: "1.0".to_string(),
                        information_uri: None,
                        rules: None,
                    },
                },
                results: vec![
                    make_result("SQLI", "error", "a.py"),
                    make_result("XSS", "warning", "b.py"),
                    make_result("LFI", "note", "c.py"),
                ],
                artifacts: None,
                invocation: None,
            }],
        };
        let errors = collect_results(&report, "error");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_id, "SQLI");

        let warnings = collect_results(&report, "warning");
        assert_eq!(warnings.len(), 2);

        let all = collect_results(&report, "note");
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_collect_results_empty() {
        let report = crate::sarif::SarifReport {
            schema: "".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![],
        };
        let results = collect_results(&report, "error");
        assert!(results.is_empty());
    }

    // --- load_surface_reports ---

    #[test]
    fn test_load_surface_reports_from_directory() {
        let tmp = tempfile::TempDir::new().unwrap();
        let reports_dir = tmp.path();

        // Create surface directory with a SARIF result
        let surface_dir = reports_dir.join("api-endpoint");
        std::fs::create_dir_all(&surface_dir).unwrap();

        let sarif_content = serde_json::json!({
            "$schema": "",
            "version": "2.1.0",
            "runs": [{
                "tool": { "driver": { "name": "test", "version": "1.0" } },
                "results": [
                    { "ruleId": "SQLI", "level": "error", "message": { "text": "sql injection" }, "locations": [] }
                ]
            }]
        });
        std::fs::write(surface_dir.join("result.sarif.json"), sarif_content.to_string()).unwrap();

        let reports = load_surface_reports(reports_dir, "note").unwrap();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].surface_name, "api-endpoint");
        assert_eq!(reports[0].results.len(), 1);
    }

    #[test]
    fn test_load_surface_reports_skips_no_sarif() {
        let tmp = tempfile::TempDir::new().unwrap();
        let reports_dir = tmp.path();

        let surface_dir = reports_dir.join("no-results");
        std::fs::create_dir_all(&surface_dir).unwrap();
        // No result.sarif.json

        let reports = load_surface_reports(reports_dir, "note").unwrap();
        assert!(reports.is_empty());
    }

    #[test]
    fn test_load_surface_reports_filters_by_level() {
        let tmp = tempfile::TempDir::new().unwrap();
        let reports_dir = tmp.path();

        let surface_dir = reports_dir.join("surface1");
        std::fs::create_dir_all(&surface_dir).unwrap();

        let sarif_content = serde_json::json!({
            "$schema": "",
            "version": "2.1.0",
            "runs": [{
                "tool": { "driver": { "name": "test", "version": "1.0" } },
                "results": [
                    { "ruleId": "SQLI", "level": "note", "message": { "text": "low" }, "locations": [] },
                    { "ruleId": "XSS", "level": "error", "message": { "text": "high" }, "locations": [] }
                ]
            }]
        });
        std::fs::write(surface_dir.join("result.sarif.json"), sarif_content.to_string()).unwrap();

        // Only error
        let reports = load_surface_reports(reports_dir, "error").unwrap();
        assert_eq!(reports[0].results.len(), 1);
        assert_eq!(reports[0].results[0].rule_id, "XSS");

        // All (note threshold)
        let reports = load_surface_reports(reports_dir, "note").unwrap();
        assert_eq!(reports[0].results.len(), 2);
    }

    #[test]
    fn test_load_surface_reports_nonexistent_dir() {
        let result = load_surface_reports(std::path::Path::new("/nonexistent/path"), "note");
        assert!(result.is_err());
    }
}
