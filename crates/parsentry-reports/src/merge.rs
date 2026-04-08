//! SARIF merge with baseline comparison and triage preservation.
//!
//! Merges per-surface SARIF files into a single report. When a baseline
//! SARIF is provided, preserves triage state (suppressions) and annotates
//! each result with `baselineState` (new/unchanged/updated/absent).
//!
//! Results are matched across runs by fingerprint. If the agent didn't
//! generate fingerprints, they are computed from `ruleId + file URI`.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

use crate::sarif::*;

/// Maximum SARIF file size (10 MiB) to prevent OOM from malicious agents.
const MAX_SARIF_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Compute a stable fingerprint for a result.
///
/// Uses agent-provided `fingerprints["parsentry/v1"]` if available.
/// Otherwise falls back to `SHA256(ruleId + first location URI)`.
fn fingerprint(result: &SarifResult) -> String {
    // Use agent-provided fingerprint if available
    if let Some(ref fps) = result.fingerprints {
        if let Some(fp) = fps.get("parsentry/v1") {
            return fp.clone();
        }
        // Use any available fingerprint
        if let Some((_, fp)) = fps.iter().next() {
            return fp.clone();
        }
    }

    // Compute from ruleId + first location URI
    let uri = result
        .locations
        .first()
        .map(|l| l.physical_location.artifact_location.uri.as_str())
        .unwrap_or("");

    let mut hasher = Sha256::new();
    hasher.update(result.rule_id.as_bytes());
    hasher.update(b"\0");
    hasher.update(uri.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Ensure every result has a `fingerprints` map with `parsentry/v1`.
fn ensure_fingerprint(result: &mut SarifResult) {
    let fp = fingerprint(result);
    let map = result.fingerprints.get_or_insert_with(HashMap::new);
    map.entry("parsentry/v1".to_string()).or_insert(fp);
}

/// Merge all `*.sarif.json` files in `dir` into a single [`SarifReport`].
///
/// When `baseline` is provided:
/// - Results matched by fingerprint inherit suppressions from the baseline.
/// - Each result is annotated with `baselineState`:
///   - `"new"`: not in baseline
///   - `"unchanged"`: same ruleId + fingerprint exists in baseline
///   - `"absent"`: in baseline but not in current scan (appended with absent state)
pub fn merge_sarif_dir(dir: &Path, baseline: Option<&Path>) -> Result<SarifReport> {
    // Collect result.sarif.json from surface subdirectories, falling back to
    // flat *.sarif.json files in dir for backward compatibility.
    let mut sarif_files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("cannot read directory: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().map_or(false, |ft| ft.is_symlink()))
        .flat_map(|e| {
            let path = e.path();
            if path.is_dir() {
                let candidate = path.join("result.sarif.json");
                if candidate.exists() {
                    vec![candidate]
                } else {
                    vec![]
                }
            } else if path
                .file_name()
                .map(|n| n.to_string_lossy().ends_with(".sarif.json"))
                .unwrap_or(false)
            {
                vec![path]
            } else {
                vec![]
            }
        })
        .collect();

    sarif_files.sort();

    if sarif_files.is_empty() {
        anyhow::bail!("no sarif files found in {}", dir.display());
    }

    // Load baseline results indexed by fingerprint
    let baseline_map = if let Some(baseline_path) = baseline {
        load_baseline(baseline_path)?
    } else {
        HashMap::new()
    };

    let mut all_rules: Vec<SarifRule> = Vec::new();
    let mut rule_index_map: HashMap<String, usize> = HashMap::new();
    let mut all_results: Vec<SarifResult> = Vec::new();
    let mut seen_fingerprints: HashMap<String, usize> = HashMap::new();

    for path in &sarif_files {
        let meta = std::fs::metadata(path)
            .with_context(|| format!("cannot stat {}", path.display()))?;
        if meta.len() > MAX_SARIF_FILE_SIZE {
            anyhow::bail!(
                "SARIF file exceeds {}MiB limit: {} ({}B)",
                MAX_SARIF_FILE_SIZE / (1024 * 1024),
                path.display(),
                meta.len()
            );
        }
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read {}", path.display()))?;

        let report: SarifReport = serde_json::from_str(&content)
            .with_context(|| format!("invalid SARIF JSON in {}", path.display()))?;

        for run in report.runs {
            let local_rules = run.tool.driver.rules.unwrap_or_default();
            let mut local_to_merged: HashMap<usize, usize> = HashMap::new();

            for (local_idx, rule) in local_rules.into_iter().enumerate() {
                let merged_idx = if let Some(&existing) = rule_index_map.get(&rule.id) {
                    existing
                } else {
                    let idx = all_rules.len();
                    rule_index_map.insert(rule.id.clone(), idx);
                    all_rules.push(rule);
                    idx
                };
                local_to_merged.insert(local_idx, merged_idx);
            }

            for mut result in run.results {
                // Rewrite ruleIndex
                if let Some(local_idx) = result.rule_index {
                    result.rule_index = local_to_merged.get(&local_idx).copied();
                } else if let Some(&idx) = rule_index_map.get(&result.rule_id) {
                    result.rule_index = Some(idx);
                }

                // Ensure fingerprint exists
                ensure_fingerprint(&mut result);
                let fp = fingerprint(&result);

                // Deduplicate: skip if same fingerprint already seen
                if seen_fingerprints.contains_key(&fp) {
                    continue;
                }

                // Baseline comparison
                if let Some(baseline_result) = baseline_map.get(&fp) {
                    // Carry forward suppressions from baseline
                    if result.suppressions.is_none() {
                        result.suppressions = baseline_result.suppressions.clone();
                    }
                    result.baseline_state = Some("unchanged".to_string());
                } else if !baseline_map.is_empty() {
                    result.baseline_state = Some("new".to_string());
                }

                seen_fingerprints.insert(fp, all_results.len());
                all_results.push(result);
            }
        }
    }

    // Append absent results (in baseline but not in current scan)
    if !baseline_map.is_empty() {
        for (fp, baseline_result) in &baseline_map {
            if !seen_fingerprints.contains_key(fp) {
                let mut absent = baseline_result.clone();
                absent.baseline_state = Some("absent".to_string());

                // Ensure rule exists in merged rules
                if let Some(&idx) = rule_index_map.get(&absent.rule_id) {
                    absent.rule_index = Some(idx);
                } else {
                    let idx = all_rules.len();
                    rule_index_map.insert(absent.rule_id.clone(), idx);
                    all_rules.push(SarifRule {
                        id: absent.rule_id.clone(),
                        name: None,
                        short_description: None,
                        full_description: None,
                        help: None,
                        properties: None,
                        default_configuration: None,
                    });
                    absent.rule_index = Some(idx);
                }

                all_results.push(absent);
            }
        }
    }

    Ok(SarifReport {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "parsentry".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    information_uri: Some("https://github.com/HikaruEgashira/parsentry".to_string()),
                    rules: Some(all_rules),
                },
            },
            results: all_results,
            artifacts: None,
            invocation: None,
        }],
    })
}

/// Load baseline SARIF and index results by fingerprint.
fn load_baseline(path: &Path) -> Result<HashMap<String, SarifResult>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read baseline: {}", path.display()))?;

    let report: SarifReport = serde_json::from_str(&content)
        .with_context(|| format!("invalid baseline SARIF: {}", path.display()))?;

    let mut map = HashMap::new();
    for run in report.runs {
        for mut result in run.results {
            ensure_fingerprint(&mut result);
            let fp = fingerprint(&result);
            map.insert(fp, result);
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_sarif(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(name), content).unwrap();
    }

    fn minimal_sarif(rule_id: &str, uri: &str, message: &str) -> String {
        format!(
            r#"{{
            "$schema": "https://example.com/sarif",
            "version": "2.1.0",
            "runs": [{{
                "tool": {{"driver": {{"name": "test", "version": "1.0", "rules": [
                    {{"id": "{rule_id}"}}
                ]}}}},
                "results": [{{"ruleId": "{rule_id}", "ruleIndex": 0, "level": "error",
                    "message": {{"text": "{message}"}},
                    "locations": [{{"physicalLocation": {{"artifactLocation": {{"uri": "{uri}"}}}}}}]
                }}]
            }}]
        }}"#
        )
    }

    #[test]
    fn merges_two_files_with_dedup() {
        let tmp = TempDir::new().unwrap();
        write_sarif(
            tmp.path(),
            "S1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli1"),
        );
        write_sarif(
            tmp.path(),
            "S2.sarif.json",
            &minimal_sarif("XSS", "web.py", "xss1"),
        );

        let merged = merge_sarif_dir(tmp.path(), None).unwrap();
        let run = &merged.runs[0];
        assert_eq!(run.results.len(), 2);
        assert_eq!(run.tool.driver.rules.as_ref().unwrap().len(), 2);
        // No baseline = no baselineState
        assert!(run.results[0].baseline_state.is_none());
    }

    #[test]
    fn baseline_marks_new_and_unchanged() {
        let tmp = TempDir::new().unwrap();
        let baseline_dir = TempDir::new().unwrap();

        // Baseline has SQLI in app.py
        write_sarif(
            baseline_dir.path(),
            "old.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli"),
        );
        let baseline = merge_sarif_dir(baseline_dir.path(), None).unwrap();
        let baseline_path = tmp.path().join("baseline.sarif.json");
        std::fs::write(&baseline_path, serde_json::to_string(&baseline).unwrap()).unwrap();

        // Current scan has SQLI in app.py (unchanged) + XSS in web.py (new)
        write_sarif(
            tmp.path(),
            "S1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli"),
        );
        write_sarif(
            tmp.path(),
            "S2.sarif.json",
            &minimal_sarif("XSS", "web.py", "xss"),
        );

        let merged = merge_sarif_dir(tmp.path(), Some(&baseline_path)).unwrap();
        let results = &merged.runs[0].results;

        let sqli = results.iter().find(|r| r.rule_id == "SQLI").unwrap();
        assert_eq!(sqli.baseline_state.as_deref(), Some("unchanged"));

        let xss = results.iter().find(|r| r.rule_id == "XSS").unwrap();
        assert_eq!(xss.baseline_state.as_deref(), Some("new"));
    }

    #[test]
    fn baseline_marks_absent() {
        let scan_dir = TempDir::new().unwrap();
        let baseline_dir = TempDir::new().unwrap();

        // Baseline has SQLI + XSS
        write_sarif(
            baseline_dir.path(),
            "old1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli"),
        );
        write_sarif(
            baseline_dir.path(),
            "old2.sarif.json",
            &minimal_sarif("XSS", "web.py", "xss"),
        );
        let baseline = merge_sarif_dir(baseline_dir.path(), None).unwrap();
        let baseline_path = baseline_dir.path().join("baseline.sarif.json");
        std::fs::write(&baseline_path, serde_json::to_string(&baseline).unwrap()).unwrap();

        // Current scan only has SQLI (XSS was fixed)
        write_sarif(
            scan_dir.path(),
            "S1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli"),
        );

        let merged = merge_sarif_dir(scan_dir.path(), Some(&baseline_path)).unwrap();
        let results = &merged.runs[0].results;
        assert_eq!(results.len(), 2);

        let xss = results.iter().find(|r| r.rule_id == "XSS").unwrap();
        assert_eq!(xss.baseline_state.as_deref(), Some("absent"));
    }

    #[test]
    fn preserves_suppressions_from_baseline() {
        let tmp = TempDir::new().unwrap();

        // Build baseline with suppression
        let baseline_json = r#"{
            "$schema": "https://example.com/sarif",
            "version": "2.1.0",
            "runs": [{
                "tool": {"driver": {"name": "test", "version": "1.0", "rules": [{"id": "SQLI"}]}},
                "results": [{
                    "ruleId": "SQLI", "ruleIndex": 0, "level": "error",
                    "message": {"text": "sqli"},
                    "locations": [{"physicalLocation": {"artifactLocation": {"uri": "app.py"}}}],
                    "suppressions": [{"kind": "external", "status": "accepted", "justification": "false positive"}]
                }]
            }]
        }"#;
        let baseline_path = tmp.path().join("baseline.sarif.json");
        std::fs::write(&baseline_path, baseline_json).unwrap();

        // Current scan finds same issue (no suppression in new result)
        write_sarif(
            tmp.path(),
            "S1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli"),
        );

        let merged = merge_sarif_dir(tmp.path(), Some(&baseline_path)).unwrap();
        let result = &merged.runs[0].results[0];

        // Suppression carried forward from baseline
        let suppressions = result.suppressions.as_ref().unwrap();
        assert_eq!(suppressions.len(), 1);
        assert_eq!(suppressions[0].status.as_deref(), Some("accepted"));
        assert_eq!(
            suppressions[0].justification.as_deref(),
            Some("false positive")
        );
    }

    #[test]
    fn fingerprint_deduplicates_same_result() {
        let tmp = TempDir::new().unwrap();
        // Two surfaces report the same SQLI in app.py
        write_sarif(
            tmp.path(),
            "S1.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli found"),
        );
        write_sarif(
            tmp.path(),
            "S2.sarif.json",
            &minimal_sarif("SQLI", "app.py", "sqli found again"),
        );

        let merged = merge_sarif_dir(tmp.path(), None).unwrap();
        // Same ruleId + URI = same fingerprint = deduplicated
        assert_eq!(merged.runs[0].results.len(), 1);
    }

    #[test]
    fn errors_on_empty_dir() {
        let tmp = TempDir::new().unwrap();
        assert!(merge_sarif_dir(tmp.path(), None).is_err());
    }
}
