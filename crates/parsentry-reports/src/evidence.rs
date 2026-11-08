//! Verdict and evidence classification for findings.
//!
//! Every finding must end in one of three states (confirmed / ruled out /
//! indeterminate) and declare what kind of evidence backs it. A finding that
//! was never executed cannot be more than moderately confident, no matter
//! what the agent claims, so [`normalize`] caps the confidence of findings
//! without dynamic evidence.

use serde::{Deserialize, Serialize};

use crate::sarif::{SarifResult, SarifResultProperties};

/// Upper bound on `confidence` for findings that were not executed.
pub const STATIC_CONFIDENCE_CAP: f64 = 0.7;

/// Final state of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// Working PoC, or a complete reachable source-to-impact trace.
    Confirmed,
    /// A specific control at a specific location prevents the issue.
    RuledOut,
    /// Neither proven nor disproven; the proof gap is recorded in
    /// `counterevidence`. This is the default for unexecuted findings.
    #[default]
    Indeterminate,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Verdict::Confirmed => "confirmed",
            Verdict::RuledOut => "ruled_out",
            Verdict::Indeterminate => "indeterminate",
        }
    }
}

/// What kind of evidence backs a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// The issue was reproduced against a running target.
    DynamicPoc,
    /// A source-to-sink trace was read but never executed.
    StaticTrace,
    /// Pattern or configuration smell without a traced data flow.
    #[default]
    Heuristic,
}

impl EvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EvidenceKind::DynamicPoc => "dynamic_poc",
            EvidenceKind::StaticTrace => "static_trace",
            EvidenceKind::Heuristic => "heuristic",
        }
    }

    /// Whether this evidence allows confidence above [`STATIC_CONFIDENCE_CAP`].
    pub fn is_executed(self) -> bool {
        matches!(self, EvidenceKind::DynamicPoc)
    }
}

/// Fill in defaults and enforce the confidence cap on a single result.
///
/// Missing `verdict` becomes `indeterminate`, missing `evidence_kind`
/// becomes `heuristic`, and confidence of unexecuted findings is clamped to
/// [`STATIC_CONFIDENCE_CAP`]. Returns `true` if the confidence was clamped.
pub fn normalize(result: &mut SarifResult) -> bool {
    let props = result
        .properties
        .get_or_insert_with(SarifResultProperties::default);
    let verdict = props.verdict.get_or_insert_default();
    let kind = props.evidence_kind.get_or_insert_default();
    // A confirmed verdict without executed evidence is a claim, not a proof.
    if *verdict == Verdict::Confirmed && !kind.is_executed() {
        *verdict = Verdict::Indeterminate;
    }
    let kind = *kind;
    match props.confidence {
        Some(c) if !kind.is_executed() && c > STATIC_CONFIDENCE_CAP => {
            props.confidence = Some(STATIC_CONFIDENCE_CAP);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sarif::SarifMessage;

    fn result(props: Option<SarifResultProperties>) -> SarifResult {
        SarifResult {
            rule_id: "r".into(),
            rule_index: None,
            level: "error".into(),
            message: SarifMessage {
                text: "m".into(),
                markdown: None,
            },
            locations: vec![],
            fingerprints: None,
            baseline_state: None,
            suppressions: None,
            properties: props,
        }
    }

    #[test]
    fn missing_properties_default_to_indeterminate_heuristic() {
        let mut r = result(None);
        assert!(!normalize(&mut r));
        let p = r.properties.unwrap();
        assert_eq!(p.verdict, Some(Verdict::Indeterminate));
        assert_eq!(p.evidence_kind, Some(EvidenceKind::Heuristic));
    }

    #[test]
    fn static_confidence_is_capped() {
        let mut r = result(Some(SarifResultProperties {
            confidence: Some(0.95),
            evidence_kind: Some(EvidenceKind::StaticTrace),
            ..Default::default()
        }));
        assert!(normalize(&mut r));
        assert_eq!(
            r.properties.unwrap().confidence,
            Some(STATIC_CONFIDENCE_CAP)
        );
    }

    #[test]
    fn dynamic_confidence_is_untouched() {
        let mut r = result(Some(SarifResultProperties {
            confidence: Some(0.95),
            verdict: Some(Verdict::Confirmed),
            evidence_kind: Some(EvidenceKind::DynamicPoc),
            ..Default::default()
        }));
        assert!(!normalize(&mut r));
        let p = r.properties.unwrap();
        assert_eq!(p.confidence, Some(0.95));
        assert_eq!(p.verdict, Some(Verdict::Confirmed));
    }

    #[test]
    fn confirmed_without_execution_is_demoted() {
        let mut r = result(Some(SarifResultProperties {
            verdict: Some(Verdict::Confirmed),
            evidence_kind: Some(EvidenceKind::StaticTrace),
            ..Default::default()
        }));
        normalize(&mut r);
        assert_eq!(r.properties.unwrap().verdict, Some(Verdict::Indeterminate));
    }

    #[test]
    fn serde_uses_snake_case() {
        assert_eq!(
            serde_json::to_string(&Verdict::RuledOut).unwrap(),
            "\"ruled_out\""
        );
        assert_eq!(
            serde_json::from_str::<EvidenceKind>("\"dynamic_poc\"").unwrap(),
            EvidenceKind::DynamicPoc
        );
    }
}
