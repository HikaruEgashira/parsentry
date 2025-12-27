//! Output parser for Claude Code responses.

use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize a string that may be null as an empty string.
fn null_to_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Response from Claude Code security analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClaudeCodeResponse {
    /// Analysis reasoning and notes.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub scratchpad: String,

    /// Comprehensive security assessment.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub analysis: String,

    /// Proof of concept code.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub poc: String,

    /// Confidence score (0-100).
    #[serde(default)]
    pub confidence_score: i32,

    /// Detected vulnerability types.
    #[serde(default)]
    pub vulnerability_types: Vec<String>,

    /// PAR (Principal-Action-Resource) analysis.
    #[serde(default)]
    pub par_analysis: ParAnalysis,

    /// Remediation guidance.
    #[serde(default)]
    pub remediation_guidance: RemediationGuidance,

    /// File path being analyzed.
    #[serde(default)]
    pub file_path: Option<String>,

    /// Pattern description if pattern-based analysis.
    #[serde(default)]
    pub pattern_description: Option<String>,

    /// Matched source code snippet.
    #[serde(default)]
    pub matched_source_code: Option<String>,
}

impl ClaudeCodeResponse {
    /// Normalize confidence score to multiples of 10.
    pub fn normalize_confidence_score(&mut self) {
        self.confidence_score = ((self.confidence_score + 5) / 10 * 10).clamp(0, 100);
    }

    /// Check if any vulnerabilities were detected.
    pub fn has_vulnerabilities(&self) -> bool {
        !self.vulnerability_types.is_empty() && self.confidence_score > 0
    }

    /// Get vulnerability info for each detected type.
    pub fn get_vulnerability_infos(&self) -> Vec<VulnerabilityInfo> {
        self.vulnerability_types
            .iter()
            .map(|vuln_type| VulnerabilityInfo {
                vuln_type: vuln_type.clone(),
                confidence: self.confidence_score,
                description: self.analysis.clone(),
                poc: self.poc.clone(),
            })
            .collect()
    }
}

/// PAR (Principal-Action-Resource) analysis structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParAnalysis {
    /// Identified principals (untrusted data sources).
    #[serde(default)]
    pub principals: Vec<PrincipalInfo>,

    /// Identified actions (security controls).
    #[serde(default)]
    pub actions: Vec<ActionInfo>,

    /// Identified resources (sensitive operations).
    #[serde(default)]
    pub resources: Vec<ResourceInfo>,

    /// Detected policy violations.
    #[serde(default)]
    pub policy_violations: Vec<PolicyViolation>,
}

/// Information about a principal (data source).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrincipalInfo {
    /// Identifier for the principal.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub identifier: String,

    /// Trust level: trusted, semi_trusted, or untrusted.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub trust_level: String,

    /// Source context description.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub source_context: String,

    /// Risk factors associated with this principal.
    #[serde(default)]
    pub risk_factors: Vec<String>,
}

/// Information about an action (security control).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionInfo {
    /// Identifier for the action.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub identifier: String,

    /// Security function description.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub security_function: String,

    /// Implementation quality: adequate, insufficient, missing, or bypassed.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub implementation_quality: String,

    /// Detected weaknesses in the action.
    #[serde(default)]
    pub detected_weaknesses: Vec<String>,

    /// Potential bypass vectors.
    #[serde(default)]
    pub bypass_vectors: Vec<String>,
}

/// Information about a resource (sensitive operation target).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// Identifier for the resource.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub identifier: String,

    /// Sensitivity level: low, medium, high, or critical.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub sensitivity_level: String,

    /// Type of operation performed.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub operation_type: String,

    /// Protection mechanisms in place.
    #[serde(default)]
    pub protection_mechanisms: Vec<String>,
}

/// Information about a policy violation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// Rule identifier.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub rule_id: String,

    /// Rule description.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub rule_description: String,

    /// Path showing the violation flow.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub violation_path: String,

    /// Severity level.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub severity: String,

    /// Confidence in this violation.
    #[serde(default)]
    pub confidence: f64,
}

/// Remediation guidance structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemediationGuidance {
    /// Policy enforcement recommendations.
    #[serde(default)]
    pub policy_enforcement: Vec<PolicyEnforcement>,
}

/// Policy enforcement recommendation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyEnforcement {
    /// Component that needs improvement.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub component: String,

    /// Required improvement description.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub required_improvement: String,

    /// Specific guidance for implementation.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub specific_guidance: String,

    /// Priority level.
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub priority: String,
}

/// Summary information about a detected vulnerability.
#[derive(Debug, Clone)]
pub struct VulnerabilityInfo {
    /// Type of vulnerability (e.g., XSS, SQLI, RCE).
    pub vuln_type: String,
    /// Confidence score.
    pub confidence: i32,
    /// Description of the vulnerability.
    pub description: String,
    /// Proof of concept code.
    pub poc: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "analysis": "SQL injection vulnerability detected",
            "confidence_score": 85,
            "vulnerability_types": ["SQLI"],
            "par_analysis": {
                "principals": [],
                "actions": [],
                "resources": [],
                "policy_violations": []
            }
        }"#;

        let response: ClaudeCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.confidence_score, 85);
        assert_eq!(response.vulnerability_types, vec!["SQLI"]);
    }

    #[test]
    fn test_response_with_null_values() {
        let json = r#"{
            "scratchpad": "test",
            "analysis": null,
            "poc": null,
            "confidence_score": 80,
            "vulnerability_types": ["XSS"],
            "par_analysis": {
                "principals": [{"identifier": null, "trust_level": "untrusted"}],
                "actions": [],
                "resources": [],
                "policy_violations": []
            }
        }"#;

        let response: ClaudeCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.scratchpad, "test");
        assert_eq!(response.analysis, "");
        assert_eq!(response.poc, "");
        assert_eq!(response.par_analysis.principals[0].identifier, "");
    }

    #[test]
    fn test_normalize_confidence_score() {
        let mut response = ClaudeCodeResponse {
            confidence_score: 87,
            ..Default::default()
        };
        response.normalize_confidence_score();
        assert_eq!(response.confidence_score, 90);
    }

    #[test]
    fn test_has_vulnerabilities() {
        let response = ClaudeCodeResponse {
            confidence_score: 80,
            vulnerability_types: vec!["XSS".to_string()],
            ..Default::default()
        };
        assert!(response.has_vulnerabilities());

        let empty_response = ClaudeCodeResponse::default();
        assert!(!empty_response.has_vulnerabilities());
    }
}
