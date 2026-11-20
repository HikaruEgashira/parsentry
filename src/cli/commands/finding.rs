//! `parsentry report`: accept a single finding into a surface's SARIF.

use std::path::Path;

use anyhow::{Context, Result, bail};
use parsentry_reports::{AcceptOutcome, accept_finding, validate_finding};

use super::common::{cache_dir_for, write_stdout};

/// Surface ids become directory names, so only allow a conservative set.
pub fn is_safe_surface_id(id: &str) -> bool {
    !id.is_empty()
        && id != "."
        && id != ".."
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Validate `finding` and append it to `<reports_dir>/<surface>/result.sarif.json`.
/// Returns the line to print (`accepted <fp>` / `duplicate <fp>`).
pub fn report_finding(reports_dir: &Path, surface: &str, finding: &str) -> Result<String> {
    if !is_safe_surface_id(surface) {
        bail!("invalid surface id: {surface:?} (allowed: A-Z a-z 0-9 - _ .)");
    }
    let raw: serde_json::Value =
        serde_json::from_str(finding).context("finding is not valid JSON")?;
    let result = validate_finding(&raw).map_err(|e| anyhow::anyhow!("rejected: {e}"))?;
    let surface_dir = reports_dir.join(surface);
    std::fs::create_dir_all(&surface_dir)
        .with_context(|| format!("creating {}", surface_dir.display()))?;
    Ok(match accept_finding(&surface_dir, result)? {
        AcceptOutcome::Accepted(fp) => format!("accepted {fp}"),
        AcceptOutcome::Duplicate(fp) => format!("duplicate {fp}"),
    })
}

pub fn run_report_command(target: &str, surface: &str, finding: &str) -> Result<()> {
    let reports_dir = cache_dir_for(target).join("reports");
    let line = report_finding(&reports_dir, surface, finding)?;
    write_stdout(&format!("{line}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const FINDING: &str = r#"{
        "ruleId": "sqli",
        "level": "error",
        "message": {"text": "SQL injection"},
        "locations": [{"physicalLocation": {"artifactLocation": {"uri": "app.py"}, "region": {"startLine": 3}}}],
        "properties": {"verdict": "indeterminate", "evidence_kind": "static_trace",
                       "counterevidence": "no PoC run", "confidence": 0.5, "cwe": ["CWE-89"]}
    }"#;

    #[test]
    fn accepted_then_duplicate() {
        let dir = TempDir::new().unwrap();
        let first = report_finding(dir.path(), "S-1", FINDING).unwrap();
        let fp = first.strip_prefix("accepted ").expect(&first);
        assert!(!fp.is_empty());
        assert!(dir.path().join("S-1/result.sarif.json").exists());
        let second = report_finding(dir.path(), "S-1", FINDING).unwrap();
        assert_eq!(second, format!("duplicate {fp}"));
    }

    #[test]
    fn rejects_invalid_finding_and_surface() {
        let dir = TempDir::new().unwrap();
        let bad = FINDING.replace(r#""level": "error""#, r#""level": "high""#);
        let err = report_finding(dir.path(), "S-1", &bad).unwrap_err();
        assert!(err.to_string().contains("rejected"), "{err}");
        assert!(report_finding(dir.path(), "S-1", "not json").is_err());
        for id in ["", "..", "../x", "a/b", "a b", "S;1"] {
            assert!(report_finding(dir.path(), id, FINDING).is_err(), "{id}");
        }
        assert!(!dir.path().join("S-1/result.sarif.json").exists());
    }
}
