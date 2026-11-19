//! Acceptance of single findings reported by an agent.
//!
//! `parsentry report --surface <id> --finding <json>` validates one SARIF
//! result before it ever reaches `reports/<id>/result.sarif.json`, so a
//! malformed or over-confident finding is rejected at write time instead of
//! being silently normalized at merge time. Every accepted write also drops
//! an [`ACCEPTED_MARKER`] next to the SARIF; merge ignores surface results
//! without it.

use anyhow::{Context, Result};
use serde_json::Value;
use std::fmt;
use std::path::Path;

use crate::evidence::{EvidenceKind, STATIC_CONFIDENCE_CAP, Verdict};
use crate::merge::fingerprint;
use crate::sarif::*;

/// Marker file written next to `result.sarif.json` once a finding was accepted.
pub const ACCEPTED_MARKER: &str = ".accepted";

const RESULT_FILE: &str = "result.sarif.json";
const LEVELS: [&str; 4] = ["error", "warning", "note", "none"];

/// Why a finding was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum RejectReason {
    InvalidJson(String),
    MissingRuleId,
    InvalidLevel(String),
    MissingMessage,
    MissingLocations,
    CweNotArray,
    InvalidVerdict(String),
    InvalidEvidenceKind(String),
    MissingCounterevidence,
    ConfidenceOutOfRange(f64),
    ConfidenceAboveCap(f64),
    ConfirmedWithoutDynamicPoc,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(e) => write!(f, "finding is not a valid SARIF result: {e}"),
            Self::MissingRuleId => write!(f, "`ruleId` is required"),
            Self::InvalidLevel(l) => {
                write!(f, "`level` must be one of {LEVELS:?}, got {l:?}")
            }
            Self::MissingMessage => write!(f, "`message.text` is required"),
            Self::MissingLocations => write!(
                f,
                "`locations` must contain at least one `physicalLocation.artifactLocation.uri`"
            ),
            Self::CweNotArray => write!(f, "`properties.cwe` must be an array of strings"),
            Self::InvalidVerdict(v) => write!(
                f,
                "`properties.verdict` must be confirmed | ruled_out | indeterminate, got {v}"
            ),
            Self::InvalidEvidenceKind(v) => write!(
                f,
                "`properties.evidence_kind` must be dynamic_poc | static_trace | heuristic, got {v}"
            ),
            Self::MissingCounterevidence => {
                write!(f, "`properties.counterevidence` is required")
            }
            Self::ConfidenceOutOfRange(c) => {
                write!(f, "`properties.confidence` must be within 0.0-1.0, got {c}")
            }
            Self::ConfidenceAboveCap(c) => write!(
                f,
                "`properties.confidence` {c} exceeds {STATIC_CONFIDENCE_CAP} without dynamic_poc evidence"
            ),
            Self::ConfirmedWithoutDynamicPoc => {
                write!(
                    f,
                    "`confirmed` verdict requires `evidence_kind=dynamic_poc`"
                )
            }
        }
    }
}

impl std::error::Error for RejectReason {}

/// Outcome of [`accept_finding`].
#[derive(Debug, Clone, PartialEq)]
pub enum Acceptance {
    /// The finding was appended; carries its fingerprint.
    Accepted(String),
    /// A finding with the same fingerprint already exists; carries its id.
    Duplicate(String),
}

fn non_empty_str(v: Option<&Value>) -> bool {
    v.and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}

/// Validate one SARIF result given as JSON.
pub fn validate_finding(json: &str) -> std::result::Result<SarifResult, RejectReason> {
    let raw: Value =
        serde_json::from_str(json).map_err(|e| RejectReason::InvalidJson(e.to_string()))?;

    if !non_empty_str(raw.get("ruleId")) {
        return Err(RejectReason::MissingRuleId);
    }
    let level = raw.get("level").and_then(Value::as_str).unwrap_or_default();
    if !LEVELS.contains(&level) {
        return Err(RejectReason::InvalidLevel(level.to_string()));
    }
    if !non_empty_str(raw.pointer("/message/text")) {
        return Err(RejectReason::MissingMessage);
    }
    let has_location = raw
        .get("locations")
        .and_then(Value::as_array)
        .is_some_and(|locs| {
            !locs.is_empty()
                && locs
                    .iter()
                    .all(|l| non_empty_str(l.pointer("/physicalLocation/artifactLocation/uri")))
        });
    if !has_location {
        return Err(RejectReason::MissingLocations);
    }

    let props = raw.get("properties").cloned().unwrap_or(Value::Null);
    if let Some(cwe) = props.get("cwe")
        && !cwe
            .as_array()
            .is_some_and(|a| a.iter().all(Value::is_string))
    {
        return Err(RejectReason::CweNotArray);
    }
    let verdict_raw = props.get("verdict").cloned().unwrap_or(Value::Null);
    let verdict: Verdict = serde_json::from_value(verdict_raw.clone())
        .map_err(|_| RejectReason::InvalidVerdict(verdict_raw.to_string()))?;
    let kind_raw = props.get("evidence_kind").cloned().unwrap_or(Value::Null);
    let kind: EvidenceKind = serde_json::from_value(kind_raw.clone())
        .map_err(|_| RejectReason::InvalidEvidenceKind(kind_raw.to_string()))?;
    if !non_empty_str(props.get("counterevidence")) {
        return Err(RejectReason::MissingCounterevidence);
    }
    if let Some(c) = props.get("confidence").and_then(Value::as_f64) {
        if !(0.0..=1.0).contains(&c) {
            return Err(RejectReason::ConfidenceOutOfRange(c));
        }
        if !kind.is_executed() && c > STATIC_CONFIDENCE_CAP {
            return Err(RejectReason::ConfidenceAboveCap(c));
        }
    }
    if verdict == Verdict::Confirmed && !kind.is_executed() {
        return Err(RejectReason::ConfirmedWithoutDynamicPoc);
    }

    serde_json::from_value(raw).map_err(|e| RejectReason::InvalidJson(e.to_string()))
}

fn empty_report() -> SarifReport {
    SarifReport {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "parsentry".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    information_uri: None,
                    rules: None,
                },
            },
            results: vec![],
            artifacts: None,
            invocation: None,
        }],
    }
}

/// Validate `json` and append it to `surface_dir/result.sarif.json`.
///
/// A finding whose fingerprint already exists is not appended; the existing
/// fingerprint is returned instead. Both outcomes write [`ACCEPTED_MARKER`].
pub fn accept_finding(surface_dir: &Path, json: &str) -> Result<Acceptance> {
    let mut result = validate_finding(json)?;
    let fp = fingerprint(&result);
    result
        .fingerprints
        .get_or_insert_with(Default::default)
        .entry("parsentry/v1".to_string())
        .or_insert_with(|| fp.clone());

    std::fs::create_dir_all(surface_dir)
        .with_context(|| format!("cannot create {}", surface_dir.display()))?;
    let path = surface_dir.join(RESULT_FILE);
    let mut report = match std::fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str::<SarifReport>(&s)
            .with_context(|| format!("invalid SARIF JSON in {}", path.display()))?,
        Err(_) => empty_report(),
    };
    if report.runs.is_empty() {
        report.runs = empty_report().runs;
    }

    let duplicate = report
        .runs
        .iter()
        .flat_map(|r| &r.results)
        .any(|r| fingerprint(r) == fp);
    let outcome = if duplicate {
        Acceptance::Duplicate(fp)
    } else {
        report.runs[0].results.push(result);
        std::fs::write(&path, serde_json::to_string_pretty(&report)?)
            .with_context(|| format!("cannot write {}", path.display()))?;
        Acceptance::Accepted(fp)
    };
    std::fs::write(surface_dir.join(ACCEPTED_MARKER), "")?;
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn finding(patch: impl FnOnce(&mut Value)) -> String {
        let mut v = serde_json::json!({
            "ruleId": "SQLI",
            "level": "error",
            "message": {"text": "sql injection"},
            "locations": [{"physicalLocation": {"artifactLocation": {"uri": "app.py"}}}],
            "properties": {
                "confidence": 0.6,
                "verdict": "indeterminate",
                "evidence_kind": "static_trace",
                "counterevidence": "input may be sanitized upstream",
                "cwe": ["CWE-89"]
            }
        });
        patch(&mut v);
        v.to_string()
    }

    fn reject(patch: impl FnOnce(&mut Value)) -> RejectReason {
        validate_finding(&finding(patch)).unwrap_err()
    }

    #[test]
    fn valid_finding_passes() {
        assert!(validate_finding(&finding(|_| {})).is_ok());
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(matches!(
            validate_finding("{oops").unwrap_err(),
            RejectReason::InvalidJson(_)
        ));
    }

    #[test]
    fn rejects_missing_rule_id() {
        assert_eq!(
            reject(|v| v["ruleId"] = "".into()),
            RejectReason::MissingRuleId
        );
    }

    #[test]
    fn rejects_invalid_level() {
        assert_eq!(
            reject(|v| v["level"] = "critical".into()),
            RejectReason::InvalidLevel("critical".into())
        );
    }

    #[test]
    fn rejects_missing_message() {
        assert_eq!(
            reject(|v| v["message"] = serde_json::json!({})),
            RejectReason::MissingMessage
        );
    }

    #[test]
    fn rejects_missing_locations() {
        assert_eq!(
            reject(|v| v["locations"] = serde_json::json!([])),
            RejectReason::MissingLocations
        );
    }

    #[test]
    fn rejects_cwe_string() {
        assert_eq!(
            reject(|v| v["properties"]["cwe"] = "CWE-89".into()),
            RejectReason::CweNotArray
        );
    }

    #[test]
    fn rejects_invalid_verdict() {
        assert!(matches!(
            reject(|v| v["properties"]["verdict"] = "maybe".into()),
            RejectReason::InvalidVerdict(_)
        ));
    }

    #[test]
    fn rejects_invalid_evidence_kind() {
        assert!(matches!(
            reject(|v| v["properties"]["evidence_kind"] = "vibes".into()),
            RejectReason::InvalidEvidenceKind(_)
        ));
    }

    #[test]
    fn rejects_missing_counterevidence() {
        assert_eq!(
            reject(|v| {
                v["properties"]
                    .as_object_mut()
                    .unwrap()
                    .remove("counterevidence");
            }),
            RejectReason::MissingCounterevidence
        );
    }

    #[test]
    fn rejects_confidence_out_of_range() {
        assert_eq!(
            reject(|v| v["properties"]["confidence"] = 1.5.into()),
            RejectReason::ConfidenceOutOfRange(1.5)
        );
    }

    #[test]
    fn rejects_static_confidence_above_cap() {
        assert_eq!(
            reject(|v| v["properties"]["confidence"] = 0.9.into()),
            RejectReason::ConfidenceAboveCap(0.9)
        );
    }

    #[test]
    fn rejects_confirmed_without_dynamic_poc() {
        assert_eq!(
            reject(|v| v["properties"]["verdict"] = "confirmed".into()),
            RejectReason::ConfirmedWithoutDynamicPoc
        );
    }

    #[test]
    fn dynamic_poc_may_be_confirmed_with_high_confidence() {
        let json = finding(|v| {
            v["properties"]["verdict"] = "confirmed".into();
            v["properties"]["evidence_kind"] = "dynamic_poc".into();
            v["properties"]["confidence"] = 0.95.into();
        });
        assert!(validate_finding(&json).is_ok());
    }

    #[test]
    fn accept_appends_and_writes_marker() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("S-1");
        let a = accept_finding(&dir, &finding(|_| {})).unwrap();
        let b = accept_finding(
            &dir,
            &finding(|v| {
                v["locations"][0]["physicalLocation"]["artifactLocation"]["uri"] = "b.py".into()
            }),
        )
        .unwrap();
        assert!(matches!(a, Acceptance::Accepted(_)));
        assert!(matches!(b, Acceptance::Accepted(_)));
        assert!(dir.join(ACCEPTED_MARKER).exists());
        let report: SarifReport =
            serde_json::from_str(&std::fs::read_to_string(dir.join(RESULT_FILE)).unwrap()).unwrap();
        assert_eq!(report.runs[0].results.len(), 2);
    }

    #[test]
    fn accept_rejects_duplicate_fingerprint_with_existing_id() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("S-1");
        let Acceptance::Accepted(fp) = accept_finding(&dir, &finding(|_| {})).unwrap() else {
            panic!("first finding must be accepted");
        };
        assert_eq!(
            accept_finding(&dir, &finding(|_| {})).unwrap(),
            Acceptance::Duplicate(fp)
        );
        let report: SarifReport =
            serde_json::from_str(&std::fs::read_to_string(dir.join(RESULT_FILE)).unwrap()).unwrap();
        assert_eq!(report.runs[0].results.len(), 1);
    }

    #[test]
    fn rejected_finding_writes_nothing() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("S-1");
        assert!(accept_finding(&dir, &finding(|v| v["level"] = "x".into())).is_err());
        assert!(!dir.join(RESULT_FILE).exists());
        assert!(!dir.join(ACCEPTED_MARKER).exists());
    }
}
