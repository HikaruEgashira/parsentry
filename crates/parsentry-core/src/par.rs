//! PAR (Principal-Action-Resource) analysis types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Trust level of a principal (data source).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustLevel {
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "semi_trusted")]
    SemiTrusted,
    #[serde(rename = "untrusted")]
    Untrusted,
}

impl fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrustLevel::Trusted => write!(f, "trusted"),
            TrustLevel::SemiTrusted => write!(f, "semi_trusted"),
            TrustLevel::Untrusted => write!(f, "untrusted"),
        }
    }
}

/// Sensitivity level of a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitivityLevel {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

impl fmt::Display for SensitivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensitivityLevel::Low => write!(f, "low"),
            SensitivityLevel::Medium => write!(f, "medium"),
            SensitivityLevel::High => write!(f, "high"),
            SensitivityLevel::Critical => write!(f, "critical"),
        }
    }
}

/// Quality assessment of a security function implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityFunctionQuality {
    #[serde(rename = "adequate")]
    Adequate,
    #[serde(rename = "insufficient")]
    Insufficient,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "bypassed")]
    Bypassed,
}

impl fmt::Display for SecurityFunctionQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityFunctionQuality::Adequate => write!(f, "adequate"),
            SecurityFunctionQuality::Insufficient => write!(f, "insufficient"),
            SecurityFunctionQuality::Missing => write!(f, "missing"),
            SecurityFunctionQuality::Bypassed => write!(f, "bypassed"),
        }
    }
}

/// Information about a principal (data source/actor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalInfo {
    pub identifier: String,
    pub trust_level: TrustLevel,
    pub source_context: String,
    pub risk_factors: Vec<String>,
}

/// Information about an action (security control).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    pub identifier: String,
    pub security_function: String,
    pub implementation_quality: SecurityFunctionQuality,
    pub detected_weaknesses: Vec<String>,
    pub bypass_vectors: Vec<String>,
}

/// Information about a resource (target).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub identifier: String,
    pub sensitivity_level: SensitivityLevel,
    pub operation_type: String,
    pub protection_mechanisms: Vec<String>,
}

/// A policy violation detected during analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub rule_id: String,
    pub rule_description: String,
    pub violation_path: String,
    pub severity: String,
    pub confidence: f64,
}

/// Complete PAR analysis result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParAnalysis {
    pub principals: Vec<PrincipalInfo>,
    pub actions: Vec<ActionInfo>,
    pub resources: Vec<ResourceInfo>,
    pub policy_violations: Vec<PolicyViolation>,
}

impl ParAnalysis {
    /// Check if the analysis is empty (no findings).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.principals.is_empty()
            && self.actions.is_empty()
            && self.resources.is_empty()
            && self.policy_violations.is_empty()
    }
}

/// A single remediation action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    pub component: String,
    pub required_improvement: String,
    pub specific_guidance: String,
    pub priority: String,
}

/// Guidance for remediation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemediationGuidance {
    pub policy_enforcement: Vec<RemediationAction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_analysis_is_empty() {
        let empty = ParAnalysis::default();
        assert!(empty.is_empty());

        let non_empty = ParAnalysis {
            principals: vec![PrincipalInfo {
                identifier: "test".to_string(),
                trust_level: TrustLevel::Untrusted,
                source_context: "test".to_string(),
                risk_factors: vec![],
            }],
            ..Default::default()
        };
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_trust_level_display() {
        assert_eq!(format!("{}", TrustLevel::Trusted), "trusted");
        assert_eq!(format!("{}", TrustLevel::SemiTrusted), "semi_trusted");
        assert_eq!(format!("{}", TrustLevel::Untrusted), "untrusted");
    }

    #[test]
    fn test_sensitivity_level_display() {
        assert_eq!(format!("{}", SensitivityLevel::Low), "low");
        assert_eq!(format!("{}", SensitivityLevel::Critical), "critical");
    }

    #[test]
    fn test_security_function_quality_display() {
        assert_eq!(format!("{}", SecurityFunctionQuality::Adequate), "adequate");
        assert_eq!(format!("{}", SecurityFunctionQuality::Bypassed), "bypassed");
    }
}
