//! Triage write-back: record SARIF suppressions (§3.35) on merged results and
//! carry them across rescans by fingerprint.

use std::collections::HashMap;

use crate::merge::fingerprint;
use crate::sarif::{SarifReport, SarifResult, SarifSuppression};

fn results_mut(report: &mut SarifReport) -> impl Iterator<Item = &mut SarifResult> {
    report.runs.iter_mut().flat_map(|r| r.results.iter_mut())
}

/// Suppress every result with fingerprint `fp` as an accepted external
/// suppression. Returns the number of results updated.
pub fn suppress_result(report: &mut SarifReport, fp: &str, justification: &str) -> usize {
    let mut updated = 0;
    for result in results_mut(report).filter(|r| fingerprint(r) == fp) {
        let justification = Some(justification.trim().to_string()).filter(|j| !j.is_empty());
        result.suppressions = Some(vec![SarifSuppression {
            kind: "external".to_string(),
            status: Some("accepted".to_string()),
            justification,
        }]);
        updated += 1;
    }
    updated
}

/// Remove the suppressions of every result with fingerprint `fp`.
/// Returns the number of results that were suppressed before.
pub fn unsuppress_result(report: &mut SarifReport, fp: &str) -> usize {
    results_mut(report)
        .filter(|r| fingerprint(r) == fp)
        .filter_map(|r| r.suppressions.take())
        .count()
}

/// Copy suppressions from `previous` onto results in `current` with the same
/// fingerprint that carry none of their own. Returns the number carried.
pub fn carry_suppressions(previous: &SarifReport, current: &mut SarifReport) -> usize {
    let known: HashMap<String, &Vec<SarifSuppression>> = previous
        .runs
        .iter()
        .flat_map(|r| &r.results)
        .filter_map(|r| r.suppressions.as_ref().map(|s| (fingerprint(r), s)))
        .collect();
    let mut carried = 0;
    for result in results_mut(current).filter(|r| r.suppressions.is_none()) {
        if let Some(s) = known.get(&fingerprint(result)) {
            result.suppressions = Some((*s).clone());
            carried += 1;
        }
    }
    carried
}

/// Whether a result has an accepted (or status-less) suppression.
pub fn is_suppressed(result: &SarifResult) -> bool {
    result.suppressions.iter().flatten().any(|s| {
        s.status
            .as_deref()
            .is_none_or(|status| status == "accepted")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(fps: &[&str]) -> SarifReport {
        let results: Vec<String> = fps
            .iter()
            .map(|fp| {
                format!(
                    r#"{{"ruleId":"r","level":"error","message":{{"text":"m"}},"locations":[],"fingerprints":{{"parsentry/v1":"{fp}"}}}}"#
                )
            })
            .collect();
        serde_json::from_str(&format!(
            r#"{{"$schema":"s","version":"2.1.0","runs":[{{"tool":{{"driver":{{"name":"t","version":"1"}}}},"results":[{}]}}]}}"#,
            results.join(",")
        ))
        .unwrap()
    }

    fn result(report: &SarifReport, i: usize) -> &SarifResult {
        &report.runs[0].results[i]
    }

    #[test]
    fn suppress_marks_only_matching_result() {
        let mut r = report(&["a", "b"]);
        assert_eq!(suppress_result(&mut r, "a", "  false positive "), 1);
        let s = &result(&r, 0).suppressions.as_ref().unwrap()[0];
        assert_eq!(s.kind, "external");
        assert_eq!(s.status.as_deref(), Some("accepted"));
        assert_eq!(s.justification.as_deref(), Some("false positive"));
        assert!(is_suppressed(result(&r, 0)));
        assert!(result(&r, 1).suppressions.is_none());
        assert!(!is_suppressed(result(&r, 1)));
        assert_eq!(suppress_result(&mut r, "zzz", "x"), 0);
    }

    #[test]
    fn suppress_with_blank_justification_omits_it() {
        let mut r = report(&["a"]);
        suppress_result(&mut r, "a", "   ");
        assert!(
            result(&r, 0).suppressions.as_ref().unwrap()[0]
                .justification
                .is_none()
        );
    }

    #[test]
    fn unsuppress_removes_and_counts() {
        let mut r = report(&["a", "b"]);
        suppress_result(&mut r, "a", "x");
        assert_eq!(unsuppress_result(&mut r, "b"), 0);
        assert_eq!(unsuppress_result(&mut r, "a"), 1);
        assert!(result(&r, 0).suppressions.is_none());
        assert_eq!(unsuppress_result(&mut r, "a"), 0);
    }

    #[test]
    fn carry_copies_by_fingerprint_without_overwriting() {
        let mut prev = report(&["a", "b"]);
        suppress_result(&mut prev, "a", "old a");
        suppress_result(&mut prev, "b", "old b");
        let mut cur = report(&["a", "b", "c"]);
        suppress_result(&mut cur, "b", "new b");
        assert_eq!(carry_suppressions(&prev, &mut cur), 1);
        let just = |i| {
            result(&cur, i)
                .suppressions
                .as_ref()
                .map(|s| s[0].justification.clone().unwrap())
        };
        assert_eq!(just(0).as_deref(), Some("old a"));
        assert_eq!(just(1).as_deref(), Some("new b"));
        assert_eq!(just(2), None);
    }

    #[test]
    fn is_suppressed_respects_status() {
        let mut r = report(&["a"]);
        let mut set = |status: Option<&str>| {
            r.runs[0].results[0].suppressions = Some(vec![SarifSuppression {
                kind: "external".into(),
                status: status.map(String::from),
                justification: None,
            }]);
            is_suppressed(&r.runs[0].results[0])
        };
        assert!(set(None));
        assert!(set(Some("accepted")));
        assert!(!set(Some("rejected")));
        assert!(!set(Some("underReview")));
    }
}
