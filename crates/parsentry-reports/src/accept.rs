//! Per-finding acceptance gate.
//!
//! [`validate_finding`] checks a single raw SARIF result against the shape
//! the merge phase relies on and returns a typed [`RejectReason`] on failure.
//! [`accept_finding`] appends a valid finding to `<surface>/result.sarif.json`
//! and writes an `.accepted` marker, skipping findings whose fingerprint is
//! already present.

use std::fmt;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::evidence::{EvidenceKind, STATIC_CONFIDENCE_CAP, Verdict};
use crate::merge::fingerprint;
use crate::sarif::{SarifDriver, SarifReport, SarifResult, SarifRun, SarifTool};

/// SARIF result file inside a surface directory.
pub const RESULT_FILE: &str = "result.sarif.json";
/// Marker written after a finding has been accepted.
pub const ACCEPTED_MARKER: &str = ".accepted";

const LEVELS: [&str; 4] = ["error", "warning", "note", "none"];

/// Why a finding was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum RejectReason {
    NotAnObject,
    MissingRuleId,
    InvalidLevel(String),
    MissingMessageText,
    MissingLocations,
    CweNotArray,
    InvalidVerdict(String),
    InvalidEvidenceKind(String),
    /// `confirmed` requires `evidence_kind=dynamic_poc`.
    ConfirmedWithoutDynamicPoc,
    /// Non-confirmed verdicts must record the proof gap / control.
    MissingCounterevidence,
    /// Confidence above [`STATIC_CONFIDENCE_CAP`] without `dynamic_poc`.
    ConfidenceAboveCap(f64),
    InvalidConfidence,
    Malformed(String),
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnObject => write!(f, "finding must be a JSON object"),
            Self::MissingRuleId => write!(f, "ruleId must be a non-empty string"),
            Self::InvalidLevel(l) => write!(f, "invalid level: {l}"),
            Self::MissingMessageText => write!(f, "message.text must be a non-empty string"),
            Self::MissingLocations => write!(f, "locations must be a non-empty array"),
            Self::CweNotArray => write!(f, "properties.cwe must be an array of strings"),
            Self::InvalidVerdict(v) => write!(f, "invalid verdict: {v}"),
            Self::InvalidEvidenceKind(k) => write!(f, "invalid evidence_kind: {k}"),
            Self::ConfirmedWithoutDynamicPoc => {
                write!(f, "confirmed verdict requires evidence_kind=dynamic_poc")
            }
            Self::MissingCounterevidence => {
                write!(f, "counterevidence is required unless verdict is confirmed")
            }
            Self::ConfidenceAboveCap(c) => write!(
                f,
                "confidence {c} exceeds {STATIC_CONFIDENCE_CAP} without dynamic_poc"
            ),
            Self::InvalidConfidence => write!(f, "confidence must be a number in [0, 1]"),
            Self::Malformed(e) => write!(f, "malformed result: {e}"),
        }
    }
}

impl std::error::Error for RejectReason {}

/// Outcome of [`accept_finding`].
#[derive(Debug, Clone, PartialEq)]
pub enum AcceptOutcome {
    /// Appended; carries the finding's fingerprint.
    Accepted(String),
    /// A result with the same fingerprint already exists; carries its id.
    Duplicate(String),
}

fn non_empty_str(v: Option<&Value>) -> bool {
    v.and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}

/// Validate a raw SARIF result and return it typed.
pub fn validate_finding(raw: &Value) -> Result<SarifResult, RejectReason> {
    let obj = raw.as_object().ok_or(RejectReason::NotAnObject)?;

    if !non_empty_str(obj.get("ruleId")) {
        return Err(RejectReason::MissingRuleId);
    }
    match obj.get("level").and_then(Value::as_str) {
        Some(l) if LEVELS.contains(&l) => {}
        other => {
            return Err(RejectReason::InvalidLevel(
                other.map_or_else(|| "<missing>".into(), str::to_string),
            ));
        }
    }
    if !non_empty_str(obj.get("message").and_then(|m| m.get("text"))) {
        return Err(RejectReason::MissingMessageText);
    }
    if !obj
        .get("locations")
        .and_then(Value::as_array)
        .is_some_and(|a| !a.is_empty())
    {
        return Err(RejectReason::MissingLocations);
    }

    let props = obj.get("properties").and_then(Value::as_object);
    let get = |k: &str| props.and_then(|p| p.get(k)).filter(|v| !v.is_null());

    if let Some(cwe) = get("cwe")
        && !cwe
            .as_array()
            .is_some_and(|a| a.iter().all(Value::is_string))
    {
        return Err(RejectReason::CweNotArray);
    }
    let verdict = match get("verdict") {
        None => Verdict::default(),
        Some(v) => serde_json::from_value::<Verdict>(v.clone())
            .map_err(|_| RejectReason::InvalidVerdict(v.to_string()))?,
    };
    let kind = match get("evidence_kind") {
        None => EvidenceKind::default(),
        Some(v) => serde_json::from_value::<EvidenceKind>(v.clone())
            .map_err(|_| RejectReason::InvalidEvidenceKind(v.to_string()))?,
    };
    if verdict == Verdict::Confirmed && !kind.is_executed() {
        return Err(RejectReason::ConfirmedWithoutDynamicPoc);
    }
    if verdict != Verdict::Confirmed && !non_empty_str(get("counterevidence")) {
        return Err(RejectReason::MissingCounterevidence);
    }
    if let Some(c) = get("confidence") {
        let c = c
            .as_f64()
            .filter(|c| (0.0..=1.0).contains(c))
            .ok_or(RejectReason::InvalidConfidence)?;
        if !kind.is_executed() && c > STATIC_CONFIDENCE_CAP {
            return Err(RejectReason::ConfidenceAboveCap(c));
        }
    }

    serde_json::from_value(raw.clone()).map_err(|e| RejectReason::Malformed(e.to_string()))
}

fn empty_report() -> SarifReport {
    SarifReport {
        schema: "https://json.schemastore.org/sarif-2.1.0.json".into(),
        version: "2.1.0".into(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "Parsentry".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                    information_uri: None,
                    rules: None,
                },
            },
            results: Vec::new(),
            artifacts: None,
            invocation: None,
        }],
    }
}

/// Append a validated finding to `<surface>/result.sarif.json` and write
/// the `.accepted` marker.
///
/// If a result with the same fingerprint already exists, nothing is written
/// and [`AcceptOutcome::Duplicate`] is returned with the existing id.
pub fn accept_finding(surface_dir: &Path, mut finding: SarifResult) -> Result<AcceptOutcome> {
    let path = surface_dir.join(RESULT_FILE);
    let mut report = if path.exists() {
        SarifReport::from_file(&path).with_context(|| format!("reading {}", path.display()))?
    } else {
        empty_report()
    };
    if report.runs.is_empty() {
        report.runs = empty_report().runs;
    }

    let fp = fingerprint(&finding);
    if let Some(existing) = report
        .runs
        .iter()
        .flat_map(|r| &r.results)
        .find(|r| fingerprint(r) == fp)
    {
        return Ok(AcceptOutcome::Duplicate(fingerprint(existing)));
    }

    finding
        .fingerprints
        .get_or_insert_default()
        .entry("parsentry/v1".to_string())
        .or_insert_with(|| fp.clone());
    report.runs[0].results.push(finding);

    std::fs::create_dir_all(surface_dir)?;
    report.save_to_file(&path)?;
    std::fs::write(surface_dir.join(ACCEPTED_MARKER), format!("{fp}\n"))?;
    Ok(AcceptOutcome::Accepted(fp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid() -> Value {
        json!({
            "ruleId": "sqli",
            "level": "error",
            "message": {"text": "SQL injection"},
            "locations": [{"physicalLocation": {"artifactLocation": {"uri": "src/db.rs"}}}],
            "properties": {
                "cwe": ["CWE-89"],
                "verdict": "indeterminate",
                "evidence_kind": "static_trace",
                "counterevidence": "not executed",
                "confidence": 0.6
            }
        })
    }

    fn reject(mutate: impl FnOnce(&mut Value)) -> RejectReason {
        let mut v = valid();
        mutate(&mut v);
        validate_finding(&v).unwrap_err()
    }

    #[test]
    fn valid_finding_passes() {
        assert!(validate_finding(&valid()).is_ok());
    }

    #[test]
    fn rejects_non_object() {
        assert_eq!(
            validate_finding(&json!([])).unwrap_err(),
            RejectReason::NotAnObject
        );
    }

    #[test]
    fn rejects_missing_rule_id() {
        assert_eq!(
            reject(|v| v["ruleId"] = json!("")),
            RejectReason::MissingRuleId
        );
    }

    #[test]
    fn rejects_invalid_level() {
        assert_eq!(
            reject(|v| v["level"] = json!("critical")),
            RejectReason::InvalidLevel("critical".into())
        );
    }

    #[test]
    fn rejects_missing_message_text() {
        assert_eq!(
            reject(|v| v["message"] = json!({})),
            RejectReason::MissingMessageText
        );
    }

    #[test]
    fn rejects_missing_locations() {
        assert_eq!(
            reject(|v| v["locations"] = json!([])),
            RejectReason::MissingLocations
        );
    }

    #[test]
    fn rejects_cwe_string() {
        assert_eq!(
            reject(|v| v["properties"]["cwe"] = json!("CWE-89")),
            RejectReason::CweNotArray
        );
    }

    #[test]
    fn rejects_invalid_verdict() {
        assert!(matches!(
            reject(|v| v["properties"]["verdict"] = json!("maybe")),
            RejectReason::InvalidVerdict(_)
        ));
    }

    #[test]
    fn rejects_invalid_evidence_kind() {
        assert!(matches!(
            reject(|v| v["properties"]["evidence_kind"] = json!("vibes")),
            RejectReason::InvalidEvidenceKind(_)
        ));
    }

    #[test]
    fn rejects_confirmed_without_dynamic_poc() {
        assert_eq!(
            reject(|v| v["properties"]["verdict"] = json!("confirmed")),
            RejectReason::ConfirmedWithoutDynamicPoc
        );
    }

    #[test]
    fn rejects_missing_counterevidence() {
        assert_eq!(
            reject(|v| v["properties"]["counterevidence"] = json!(" ")),
            RejectReason::MissingCounterevidence
        );
    }

    #[test]
    fn rejects_confidence_above_cap() {
        assert_eq!(
            reject(|v| v["properties"]["confidence"] = json!(0.9)),
            RejectReason::ConfidenceAboveCap(0.9)
        );
    }

    #[test]
    fn rejects_invalid_confidence() {
        assert_eq!(
            reject(|v| v["properties"]["confidence"] = json!("high")),
            RejectReason::InvalidConfidence
        );
    }

    #[test]
    fn rejects_malformed() {
        assert!(matches!(
            reject(|v| v["locations"] = json!([{"bogus": 1}])),
            RejectReason::Malformed(_)
        ));
    }

    #[test]
    fn dynamic_poc_allows_high_confidence() {
        let mut v = valid();
        v["properties"]["verdict"] = json!("confirmed");
        v["properties"]["evidence_kind"] = json!("dynamic_poc");
        v["properties"]["confidence"] = json!(0.95);
        assert!(validate_finding(&v).is_ok());
    }

    #[test]
    fn accept_appends_and_writes_marker() {
        let dir = tempfile::tempdir().unwrap();
        let f = validate_finding(&valid()).unwrap();
        let AcceptOutcome::Accepted(fp) = accept_finding(dir.path(), f).unwrap() else {
            panic!("expected Accepted");
        };
        let mut other = valid();
        other["ruleId"] = json!("xss");
        let f2 = validate_finding(&other).unwrap();
        assert!(matches!(
            accept_finding(dir.path(), f2).unwrap(),
            AcceptOutcome::Accepted(_)
        ));

        let report = SarifReport::from_file(dir.path().join(RESULT_FILE)).unwrap();
        assert_eq!(report.runs[0].results.len(), 2);
        let marker = std::fs::read_to_string(dir.path().join(ACCEPTED_MARKER)).unwrap();
        assert!(!marker.trim().is_empty());
        assert_eq!(
            report.runs[0].results[0].fingerprints.as_ref().unwrap()["parsentry/v1"],
            fp
        );
    }

    #[test]
    fn duplicate_fingerprint_is_not_appended() {
        let dir = tempfile::tempdir().unwrap();
        let f = validate_finding(&valid()).unwrap();
        let AcceptOutcome::Accepted(fp) = accept_finding(dir.path(), f.clone()).unwrap() else {
            panic!("expected Accepted");
        };
        assert_eq!(
            accept_finding(dir.path(), f).unwrap(),
            AcceptOutcome::Duplicate(fp)
        );
        let report = SarifReport::from_file(dir.path().join(RESULT_FILE)).unwrap();
        assert_eq!(report.runs[0].results.len(), 1);
    }
}
