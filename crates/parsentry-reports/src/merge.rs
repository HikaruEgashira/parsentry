//! Merge multiple per-surface SARIF files into a single SARIF report.
//!
//! Each surface analysis produces an independent SARIF file. This module
//! reads them from a directory, deduplicates rules, and merges all results
//! into a single run.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

use crate::sarif::*;

/// Merge all `*.sarif.json` files in `dir` into a single [`SarifReport`].
///
/// Rules are deduplicated by `id`. Results from all files are combined into
/// a single run. Each result's `ruleIndex` is rewritten to point at the
/// merged rule list.
pub fn merge_sarif_dir(dir: &Path) -> Result<SarifReport> {
    let mut sarif_files: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("cannot read directory: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .map(|n| n.to_string_lossy().ends_with(".sarif.json"))
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .collect();

    sarif_files.sort();

    if sarif_files.is_empty() {
        anyhow::bail!("no *.sarif.json files found in {}", dir.display());
    }

    let mut all_rules: Vec<SarifRule> = Vec::new();
    let mut rule_index_map: HashMap<String, usize> = HashMap::new();
    let mut all_results: Vec<SarifResult> = Vec::new();

    for path in &sarif_files {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read {}", path.display()))?;

        let report: SarifReport = serde_json::from_str(&content)
            .with_context(|| format!("invalid SARIF JSON in {}", path.display()))?;

        for run in report.runs {
            // Merge rules (deduplicate by id)
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

            // Rewrite result ruleIndex to point at merged rule list
            for mut result in run.results {
                if let Some(local_idx) = result.rule_index {
                    result.rule_index = local_to_merged.get(&local_idx).copied();
                } else if let Some(&idx) = rule_index_map.get(&result.rule_id) {
                    result.rule_index = Some(idx);
                }
                all_results.push(result);
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_sarif(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(name), content).unwrap();
    }

    #[test]
    fn merges_two_sarif_files() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        write_sarif(dir, "SURFACE-001.sarif.json", r#"{
            "$schema": "https://example.com/sarif",
            "version": "2.1.0",
            "runs": [{
                "tool": {"driver": {"name": "test", "version": "1.0", "rules": [
                    {"id": "SQLI", "name": "SQLInjection"}
                ]}},
                "results": [{"ruleId": "SQLI", "ruleIndex": 0, "level": "error", "message": {"text": "found sqli"}, "locations": []}]
            }]
        }"#);

        write_sarif(dir, "SURFACE-002.sarif.json", r#"{
            "$schema": "https://example.com/sarif",
            "version": "2.1.0",
            "runs": [{
                "tool": {"driver": {"name": "test", "version": "1.0", "rules": [
                    {"id": "XSS", "name": "CrossSiteScripting"},
                    {"id": "SQLI", "name": "SQLInjection"}
                ]}},
                "results": [
                    {"ruleId": "XSS", "ruleIndex": 0, "level": "warning", "message": {"text": "found xss"}, "locations": []},
                    {"ruleId": "SQLI", "ruleIndex": 1, "level": "error", "message": {"text": "another sqli"}, "locations": []}
                ]
            }]
        }"#);

        let merged = merge_sarif_dir(dir).unwrap();
        assert_eq!(merged.runs.len(), 1);

        let run = &merged.runs[0];
        let rules = run.tool.driver.rules.as_ref().unwrap();
        // SQLI deduplicated: only 2 unique rules
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].id, "SQLI");
        assert_eq!(rules[1].id, "XSS");

        // 3 total results
        assert_eq!(run.results.len(), 3);

        // ruleIndex rewritten correctly
        assert_eq!(run.results[0].rule_index, Some(0)); // SQLI -> 0
        assert_eq!(run.results[1].rule_index, Some(1)); // XSS -> 1
        assert_eq!(run.results[2].rule_index, Some(0)); // SQLI -> 0
    }

    #[test]
    fn errors_on_empty_dir() {
        let tmp = TempDir::new().unwrap();
        assert!(merge_sarif_dir(tmp.path()).is_err());
    }

    #[test]
    fn ignores_non_sarif_files() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        std::fs::write(dir.join("README.md"), "not sarif").unwrap();
        std::fs::write(dir.join("SURFACE-001.prompt.md"), "prompt").unwrap();
        assert!(merge_sarif_dir(dir).is_err()); // no .sarif.json files
    }
}
