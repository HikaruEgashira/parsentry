//! PAR (Principal-Action-Resource) analysis types.

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParAnalysis {
    pub principals: Vec<PrincipalInfo>,
    pub actions: Vec<ActionInfo>,
    pub resources: Vec<ResourceInfo>,
    pub policy_violations: Vec<PolicyViolation>,
}

impl Default for ParAnalysis {
    fn default() -> Self {
        Self {
            principals: Vec::new(),
            actions: Vec::new(),
            resources: Vec::new(),
            policy_violations: Vec::new(),
        }
    }
}

impl ParAnalysis {
    /// Check if the analysis is empty (no findings).
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationGuidance {
    pub policy_enforcement: Vec<RemediationAction>,
}

impl Default for RemediationGuidance {
    fn default() -> Self {
        Self {
            policy_enforcement: Vec::new(),
        }
    }
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
}
