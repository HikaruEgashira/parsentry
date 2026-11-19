//! Triage write-back: record suppressions in SARIF `suppressions[]`.
//!
//! Suppressions are keyed by result fingerprint so they survive a rescan:
//! the report phase carries them from the previous merged SARIF into the
//! freshly merged one via [`carry_suppressions`].

use std::collections::HashMap;

use crate::merge::fingerprint;
use crate::sarif::{SarifReport, SarifResult, SarifSuppression};

fn find_mut<'a>(report: &'a mut SarifReport, fp: &str) -> Option<&'a mut SarifResult> {
    report
        .runs
        .iter_mut()
        .flat_map(|run| run.results.iter_mut())
        .find(|r| fingerprint(r) == fp)
}

/// Suppress the result with fingerprint `fp` as an accepted external
/// suppression. Returns `false` when no result matches.
pub fn suppress_result(report: &mut SarifReport, fp: &str, justification: &str) -> bool {
    let Some(result) = find_mut(report, fp) else {
        return false;
    };
    let justification = justification.trim();
    result.suppressions = Some(vec![SarifSuppression {
        kind: "external".to_string(),
        status: Some("accepted".to_string()),
        justification: (!justification.is_empty()).then(|| justification.to_string()),
    }]);
    true
}

/// Remove all suppressions from the result with fingerprint `fp`.
pub fn unsuppress_result(report: &mut SarifReport, fp: &str) -> bool {
    let Some(result) = find_mut(report, fp) else {
        return false;
    };
    result.suppressions = None;
    true
}

/// Copy suppressions from `previous` onto matching results in `merged`
/// that don't already carry their own.
pub fn carry_suppressions(merged: &mut SarifReport, previous: &SarifReport) {
    let suppressed: HashMap<String, &Vec<SarifSuppression>> = previous
        .runs
        .iter()
        .flat_map(|run| &run.results)
        .filter_map(|r| {
            r.suppressions
                .as_ref()
                .filter(|s| !s.is_empty())
                .map(|s| (fingerprint(r), s))
        })
        .collect();
    for result in merged.runs.iter_mut().flat_map(|run| run.results.iter_mut()) {
        if result.suppressions.is_none()
            && let Some(s) = suppressed.get(&fingerprint(result))
        {
            result.suppressions = Some((*s).clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report() -> SarifReport {
        serde_json::from_value(serde_json::json!({
            "$schema": "s", "version": "2.1.0",
            "runs": [{"tool": {"driver": {"name": "t", "version": "1"}}, "results": [
                {"ruleId": "SQLI", "level": "error", "message": {"text": "m"},
                 "fingerprints": {"parsentry/v1": "fp1"}},
                {"ruleId": "XSS", "level": "warning", "message": {"text": "m"},
                 "fingerprints": {"parsentry/v1": "fp2"}}
            ]}]
        }))
        .unwrap()
    }

    #[test]
    fn suppress_and_unsuppress() {
        let mut r = report();
        assert!(suppress_result(&mut r, "fp1", " false positive "));
        let s = r.runs[0].results[0].suppressions.as_ref().unwrap();
        assert_eq!(s[0].kind, "external");
        assert_eq!(s[0].status.as_deref(), Some("accepted"));
        assert_eq!(s[0].justification.as_deref(), Some("false positive"));
        assert!(!suppress_result(&mut r, "missing", ""));
        assert!(unsuppress_result(&mut r, "fp1"));
        assert!(r.runs[0].results[0].suppressions.is_none());
    }

    #[test]
    fn suppression_survives_rescan() {
        let mut previous = report();
        suppress_result(&mut previous, "fp2", "accepted risk");
        let mut rescanned = report();
        carry_suppressions(&mut rescanned, &previous);
        assert!(rescanned.runs[0].results[0].suppressions.is_none());
        let s = rescanned.runs[0].results[1].suppressions.as_ref().unwrap();
        assert_eq!(s[0].justification.as_deref(), Some("accepted risk"));
    }
}
