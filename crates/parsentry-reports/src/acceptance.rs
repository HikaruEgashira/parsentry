//! Acceptance gate for agent-reported findings.
//!
//! Agents submit one SARIF result at a time through `parsentry report`
//! instead of writing `result.sarif.json` directly. Each result is
//! validated, deduplicated by fingerprint and appended to the surface's
//! SARIF file, and the surface directory is marked with [`ACCEPTED_MARKER`].
//! Merging only reads surface results that carry the marker, so SARIF
//! written around this gate never reaches the report.

use std::fmt;
use std::path::Path;

use serde_json::Value;

use crate::evidence::{EvidenceKind, STATIC_CONFIDENCE_CAP, Verdict};
use crate::merge::fingerprint;
use crate::sarif::{SarifDriver, SarifReport, SarifResult, SarifRun, SarifTool};

/// File in a surface directory marking its SARIF as produced by the gate.
pub const ACCEPTED_MARKER: &str = ".accepted";

/// SARIF file of a surface directory.
pub const RESULT_FILE: &str = "result.sarif.json";

/// Why a submitted finding was not accepted.
#[derive(Debug, Clone, PartialEq)]
pub enum Rejection {
    /// The finding is not a JSON object or does not match the SARIF shape.
    Malformed(String),
    MissingRuleId,
    InvalidLevel(String),
    MissingMessage,
    MissingLocations,
    CweNotArray,
    InvalidVerdict(String),
    InvalidEvidenceKind(String),
    MissingCounterevidence,
    /// Confidence outside 0.0-1.0, or above the cap without `dynamic_poc`.
    ConfidenceOutOfRange(f64),
    /// A result with the same fingerprint was already accepted.
    Duplicate(String),
    Io(String),
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rejection::Malformed(e) => write!(f, "finding is not a valid SARIF result: {e}"),
            Rejection::MissingRuleId => write!(f, "`ruleId` must be a non-empty string"),
            Rejection::InvalidLevel(l) => {
                write!(f, "`level` must be error, warning or note (got {l})")
            }
            Rejection::MissingMessage => write!(f, "`message.text` must be a non-empty string"),
            Rejection::MissingLocations => write!(
                f,
                "`locations` must contain at least one physicalLocation.artifactLocation.uri"
            ),
            Rejection::CweNotArray => write!(f, "`properties.cwe` must be an array of strings"),
            Rejection::InvalidVerdict(v) => write!(
                f,
                "`properties.verdict` must be confirmed, ruled_out or indeterminate (got {v})"
            ),
            Rejection::InvalidEvidenceKind(k) => write!(
                f,
                "`properties.evidence_kind` must be dynamic_poc, static_trace or heuristic (got {k})"
            ),
            Rejection::MissingCounterevidence => {
                write!(f, "`properties.counterevidence` must be a non-empty string")
            }
            Rejection::ConfidenceOutOfRange(c) => write!(
                f,
                "`properties.confidence` {c} must be within 0.0-1.0 and at most \
                 {STATIC_CONFIDENCE_CAP} without dynamic_poc evidence"
            ),
            Rejection::Duplicate(id) => write!(f, "duplicate finding; already accepted as {id}"),
            Rejection::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Rejection {}

fn non_empty_str(v: Option<&Value>) -> bool {
    v.and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}

/// Validate one SARIF result submitted as JSON.
pub fn validate_finding(json: &str) -> Result<SarifResult, Rejection> {
    let value: Value =
        serde_json::from_str(json).map_err(|e| Rejection::Malformed(e.to_string()))?;
    let obj = value
        .as_object()
        .ok_or_else(|| Rejection::Malformed("expected a JSON object".into()))?;

    if !non_empty_str(obj.get("ruleId")) {
        return Err(Rejection::MissingRuleId);
    }
    match obj.get("level").and_then(Value::as_str) {
        Some("error" | "warning" | "note") => {}
        other => {
            return Err(Rejection::InvalidLevel(
                other.unwrap_or("<missing>").to_string(),
            ));
        }
    }
    if !non_empty_str(obj.get("message").and_then(|m| m.get("text"))) {
        return Err(Rejection::MissingMessage);
    }
    let has_location = obj
        .get("locations")
        .and_then(Value::as_array)
        .is_some_and(|locs| {
            !locs.is_empty()
                && locs
                    .iter()
                    .all(|l| non_empty_str(l.pointer("/physicalLocation/artifactLocation/uri")))
        });
    if !has_location {
        return Err(Rejection::MissingLocations);
    }

    let props = obj.get("properties");
    let prop = |key: &str| props.and_then(|p| p.get(key));
    if let Some(cwe) = prop("cwe")
        && !cwe
            .as_array()
            .is_some_and(|a| a.iter().all(Value::is_string))
    {
        return Err(Rejection::CweNotArray);
    }
    let verdict = prop("verdict").and_then(Value::as_str);
    if serde_json::from_value::<Verdict>(prop("verdict").cloned().unwrap_or(Value::Null)).is_err() {
        return Err(Rejection::InvalidVerdict(
            verdict.unwrap_or("<missing>").to_string(),
        ));
    }
    let kind = serde_json::from_value::<EvidenceKind>(
        prop("evidence_kind").cloned().unwrap_or(Value::Null),
    )
    .map_err(|_| {
        Rejection::InvalidEvidenceKind(
            prop("evidence_kind")
                .and_then(Value::as_str)
                .unwrap_or("<missing>")
                .to_string(),
        )
    })?;
    if !non_empty_str(prop("counterevidence")) {
        return Err(Rejection::MissingCounterevidence);
    }
    if let Some(c) = prop("confidence") {
        let c = c.as_f64().ok_or_else(|| {
            Rejection::Malformed("`properties.confidence` must be a number".into())
        })?;
        let cap = if kind.is_executed() {
            1.0
        } else {
            STATIC_CONFIDENCE_CAP
        };
        if !(0.0..=cap).contains(&c) {
            return Err(Rejection::ConfidenceOutOfRange(c));
        }
    }

    serde_json::from_value(value).map_err(|e| Rejection::Malformed(e.to_string()))
}

fn empty_report() -> SarifReport {
    SarifReport {
        schema: "https://json.schemastore.org/sarif-2.1.0.json".into(),
        version: "2.1.0".into(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "parsentry".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
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

/// Read the surface SARIF, starting from an empty report unless it was
/// accepted before (anything else was written around the gate).
fn load_accepted(surface_dir: &Path) -> Result<SarifReport, Rejection> {
    let path = surface_dir.join(RESULT_FILE);
    if !surface_dir.join(ACCEPTED_MARKER).exists() || !path.exists() {
        return Ok(empty_report());
    }
    let mut report = SarifReport::from_file(&path).map_err(|e| Rejection::Io(format!("{e:#}")))?;
    if report.runs.is_empty() {
        report.runs = empty_report().runs;
    }
    Ok(report)
}

fn store(surface_dir: &Path, report: &SarifReport) -> Result<(), Rejection> {
    let io = |e: std::io::Error| Rejection::Io(e.to_string());
    std::fs::create_dir_all(surface_dir).map_err(io)?;
    let json = report
        .to_json()
        .map_err(|e| Rejection::Io(format!("{e:#}")))?;
    let tmp = surface_dir.join(format!("{RESULT_FILE}.tmp"));
    std::fs::write(&tmp, json).map_err(io)?;
    std::fs::rename(&tmp, surface_dir.join(RESULT_FILE)).map_err(io)?;
    std::fs::write(surface_dir.join(ACCEPTED_MARKER), "").map_err(io)
}

/// Validate `json` and append it to the surface's SARIF.
///
/// Returns the fingerprint id of the accepted result. A result whose
/// fingerprint was already accepted is rejected with
/// [`Rejection::Duplicate`] carrying the existing id.
pub fn accept_finding(surface_dir: &Path, json: &str) -> Result<String, Rejection> {
    let mut result = validate_finding(json)?;
    let mut report = load_accepted(surface_dir)?;
    let id = fingerprint(&result);
    if report
        .runs
        .iter()
        .flat_map(|r| &r.results)
        .any(|r| fingerprint(r) == id)
    {
        return Err(Rejection::Duplicate(id));
    }
    result
        .fingerprints
        .get_or_insert_default()
        .entry("parsentry/v1".to_string())
        .or_insert_with(|| id.clone());
    report.runs[0].results.push(result);
    store(surface_dir, &report)?;
    Ok(id)
}

/// Mark a surface as analyzed, creating an empty SARIF if nothing was
/// accepted yet (a surface without findings).
pub fn complete_surface(surface_dir: &Path) -> Result<(), Rejection> {
    let report = load_accepted(surface_dir)?;
    store(surface_dir, &report)
}

/// Whether a surface's SARIF went through the acceptance gate.
pub fn is_accepted(surface_dir: &Path) -> bool {
    surface_dir.join(ACCEPTED_MARKER).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    fn finding() -> Value {
        json!({
            "ruleId": "sql-injection",
            "level": "error",
            "message": {"text": "user input reaches query"},
            "locations": [{"physicalLocation": {"artifactLocation": {"uri": "app.py"},
                           "region": {"startLine": 3}}}],
            "properties": {
                "confidence": 0.6,
                "verdict": "indeterminate",
                "evidence_kind": "static_trace",
                "counterevidence": "ORM may parameterize",
                "cwe": ["CWE-89"]
            }
        })
    }

    fn with(path: &str, v: Value) -> String {
        let mut f = finding();
        *f.pointer_mut(path).unwrap() = v;
        f.to_string()
    }

    fn without(parent: &str, key: &str) -> String {
        let mut f = finding();
        f.pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key);
        f.to_string()
    }

    fn reject(json: &str) -> Rejection {
        validate_finding(json).unwrap_err()
    }

    #[test]
    fn valid_finding_is_accepted() {
        assert!(validate_finding(&finding().to_string()).is_ok());
    }

    #[test]
    fn malformed_json_is_rejected() {
        assert!(matches!(
            validate_finding("{oops"),
            Err(Rejection::Malformed(_))
        ));
        assert!(matches!(
            validate_finding("[]"),
            Err(Rejection::Malformed(_))
        ));
    }

    #[test]
    fn rejects_missing_rule_id() {
        assert_eq!(reject(&without("", "ruleId")), Rejection::MissingRuleId);
        assert_eq!(
            reject(&with("/ruleId", json!(" "))),
            Rejection::MissingRuleId
        );
    }

    #[test]
    fn rejects_invalid_level() {
        assert_eq!(
            reject(&with("/level", json!("critical"))),
            Rejection::InvalidLevel("critical".into())
        );
    }

    #[test]
    fn rejects_missing_message_text() {
        assert_eq!(
            reject(&with("/message/text", json!(""))),
            Rejection::MissingMessage
        );
    }

    #[test]
    fn rejects_missing_locations() {
        assert_eq!(
            reject(&with("/locations", json!([]))),
            Rejection::MissingLocations
        );
        assert_eq!(
            reject(&with("/locations", json!([{"physicalLocation": {}}]))),
            Rejection::MissingLocations
        );
    }

    #[test]
    fn rejects_cwe_string() {
        assert_eq!(
            reject(&with("/properties/cwe", json!("CWE-89"))),
            Rejection::CweNotArray
        );
    }

    #[test]
    fn rejects_invalid_verdict() {
        assert_eq!(
            reject(&with("/properties/verdict", json!("likely"))),
            Rejection::InvalidVerdict("likely".into())
        );
        assert!(matches!(
            validate_finding(&without("/properties", "verdict")),
            Err(Rejection::InvalidVerdict(_))
        ));
    }

    #[test]
    fn rejects_invalid_evidence_kind() {
        assert_eq!(
            reject(&with("/properties/evidence_kind", json!("vibes"))),
            Rejection::InvalidEvidenceKind("vibes".into())
        );
    }

    #[test]
    fn rejects_missing_counterevidence() {
        assert_eq!(
            reject(&without("/properties", "counterevidence")),
            Rejection::MissingCounterevidence
        );
    }

    #[test]
    fn rejects_confidence_above_static_cap() {
        assert_eq!(
            reject(&with("/properties/confidence", json!(0.9))),
            Rejection::ConfidenceOutOfRange(0.9)
        );
        let mut f = finding();
        f["properties"]["evidence_kind"] = json!("dynamic_poc");
        f["properties"]["confidence"] = json!(0.9);
        assert!(validate_finding(&f.to_string()).is_ok());
        f["properties"]["confidence"] = json!(1.5);
        assert!(matches!(
            validate_finding(&f.to_string()),
            Err(Rejection::ConfidenceOutOfRange(_))
        ));
    }

    #[test]
    fn accept_appends_and_marks_surface() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("S-1");
        let id = accept_finding(&dir, &finding().to_string()).unwrap();
        assert!(is_accepted(&dir));
        let mut other = finding();
        other["ruleId"] = json!("xss");
        accept_finding(&dir, &other.to_string()).unwrap();
        let report = SarifReport::from_file(dir.join(RESULT_FILE)).unwrap();
        let results = &report.runs[0].results;
        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0].fingerprints.as_ref().unwrap()["parsentry/v1"],
            id
        );
    }

    #[test]
    fn duplicate_fingerprint_returns_existing_id() {
        let tmp = TempDir::new().unwrap();
        let id = accept_finding(tmp.path(), &finding().to_string()).unwrap();
        let mut again = finding();
        again["message"]["text"] = json!("reworded");
        assert_eq!(
            accept_finding(tmp.path(), &again.to_string()),
            Err(Rejection::Duplicate(id))
        );
        let report = SarifReport::from_file(tmp.path().join(RESULT_FILE)).unwrap();
        assert_eq!(report.runs[0].results.len(), 1);
    }

    #[test]
    fn rejected_finding_leaves_surface_untouched() {
        let tmp = TempDir::new().unwrap();
        assert!(accept_finding(tmp.path(), "{}").is_err());
        assert!(!is_accepted(tmp.path()));
        assert!(!tmp.path().join(RESULT_FILE).exists());
    }

    #[test]
    fn unaccepted_sarif_is_replaced_not_extended() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join(RESULT_FILE),
            r#"{"$schema":"s","version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","version":"1"}},
              "results":[{"ruleId":"X","level":"error","message":{"text":"raw"}}]}]}"#,
        )
        .unwrap();
        accept_finding(tmp.path(), &finding().to_string()).unwrap();
        let report = SarifReport::from_file(tmp.path().join(RESULT_FILE)).unwrap();
        assert_eq!(report.runs[0].results.len(), 1);
        assert_eq!(report.runs[0].results[0].rule_id, "sql-injection");
    }

    #[test]
    fn complete_surface_writes_empty_accepted_sarif() {
        let tmp = TempDir::new().unwrap();
        complete_surface(tmp.path()).unwrap();
        assert!(is_accepted(tmp.path()));
        let report = SarifReport::from_file(tmp.path().join(RESULT_FILE)).unwrap();
        assert!(report.runs[0].results.is_empty());
    }
}
