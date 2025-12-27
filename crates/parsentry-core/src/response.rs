//! Analysis response types.

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::par::{ParAnalysis, RemediationGuidance};
use crate::vuln_type::VulnType;

/// The main response structure for security analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub scratchpad: String,
    pub analysis: String,
    pub poc: String,
    pub confidence_score: i32,
    pub vulnerability_types: Vec<VulnType>,
    pub par_analysis: ParAnalysis,
    pub remediation_guidance: RemediationGuidance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_source_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_source_code: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: 0,
            vulnerability_types: Vec::new(),
            par_analysis: ParAnalysis::default(),
            remediation_guidance: RemediationGuidance::default(),
            file_path: None,
            pattern_description: None,
            matched_source_code: None,
            full_source_code: None,
        }
    }
}

impl Response {
    /// Normalize confidence score (convert 1-10 scale to 1-100).
    pub fn normalize_confidence_score(score: i32) -> i32 {
        if score > 0 && score <= 10 {
            score * 10
        } else {
            score
        }
    }

    /// Clean up and validate the response data.
    pub fn sanitize(&mut self) {
        // Remove duplicate vulnerability types
        let mut unique_vulns = std::collections::HashSet::new();
        self.vulnerability_types
            .retain(|v| unique_vulns.insert(v.clone()));

        // If no vulnerability types and high confidence, reset confidence
        if self.vulnerability_types.is_empty() && self.confidence_score > 50 {
            self.confidence_score = 0;
        }

        // If PAR analysis is empty but high confidence, adjust confidence
        if self.par_analysis.is_empty() && self.confidence_score > 30 {
            self.confidence_score = std::cmp::min(self.confidence_score, 30);
        }
    }

    /// Check if this response indicates a vulnerability was found.
    pub fn has_vulnerability(&self) -> bool {
        !self.vulnerability_types.is_empty() && self.confidence_score > 0
    }

    /// Get severity level based on confidence score.
    pub fn severity_level(&self) -> &'static str {
        match self.confidence_score {
            90..=100 => "critical",
            70..=89 => "high",
            50..=69 => "medium",
            30..=49 => "low",
            _ => "info",
        }
    }
}

/// Generate JSON schema for the response structure.
pub fn response_json_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "scratchpad": { "type": "string" },
            "analysis": { "type": "string" },
            "poc": { "type": "string" },
            "confidence_score": { "type": "integer" },
            "vulnerability_types": {
                "type": "array",
                "items": {
                    "type": "string",
                    "enum": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"]
                }
            },
            "par_analysis": {
                "type": "object",
                "properties": {
                    "principals": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "identifier": { "type": "string" },
                                "trust_level": { "type": "string", "enum": ["trusted", "semi_trusted", "untrusted"] },
                                "source_context": { "type": "string" },
                                "risk_factors": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["identifier", "trust_level", "source_context", "risk_factors"]
                        }
                    },
                    "actions": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "identifier": { "type": "string" },
                                "security_function": { "type": "string" },
                                "implementation_quality": { "type": "string", "enum": ["adequate", "insufficient", "missing", "bypassed"] },
                                "detected_weaknesses": { "type": "array", "items": { "type": "string" } },
                                "bypass_vectors": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["identifier", "security_function", "implementation_quality", "detected_weaknesses", "bypass_vectors"]
                        }
                    },
                    "resources": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "identifier": { "type": "string" },
                                "sensitivity_level": { "type": "string", "enum": ["low", "medium", "high", "critical"] },
                                "operation_type": { "type": "string" },
                                "protection_mechanisms": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["identifier", "sensitivity_level", "operation_type", "protection_mechanisms"]
                        }
                    },
                    "policy_violations": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "rule_id": { "type": "string" },
                                "rule_description": { "type": "string" },
                                "violation_path": { "type": "string" },
                                "severity": { "type": "string" },
                                "confidence": { "type": "number" }
                            },
                            "required": ["rule_id", "rule_description", "violation_path", "severity", "confidence"]
                        }
                    }
                },
                "required": ["principals", "actions", "resources", "policy_violations"]
            },
            "remediation_guidance": {
                "type": "object",
                "properties": {
                    "policy_enforcement": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "component": { "type": "string" },
                                "required_improvement": { "type": "string" },
                                "specific_guidance": { "type": "string" },
                                "priority": { "type": "string" }
                            },
                            "required": ["component", "required_improvement", "specific_guidance", "priority"]
                        }
                    }
                },
                "required": ["policy_enforcement"]
            }
        },
        "required": ["scratchpad", "analysis", "poc", "confidence_score", "vulnerability_types", "par_analysis", "remediation_guidance"]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_confidence_score() {
        assert_eq!(Response::normalize_confidence_score(5), 50);
        assert_eq!(Response::normalize_confidence_score(10), 100);
        assert_eq!(Response::normalize_confidence_score(50), 50);
        assert_eq!(Response::normalize_confidence_score(0), 0);
    }

    #[test]
    fn test_severity_level() {
        let mut response = Response::default();

        response.confidence_score = 95;
        assert_eq!(response.severity_level(), "critical");

        response.confidence_score = 75;
        assert_eq!(response.severity_level(), "high");

        response.confidence_score = 55;
        assert_eq!(response.severity_level(), "medium");

        response.confidence_score = 35;
        assert_eq!(response.severity_level(), "low");

        response.confidence_score = 10;
        assert_eq!(response.severity_level(), "info");
    }

    #[test]
    fn test_sanitize() {
        let mut response = Response {
            confidence_score: 80,
            vulnerability_types: vec![VulnType::SQLI, VulnType::SQLI],
            ..Default::default()
        };
        response.sanitize();
        assert_eq!(response.vulnerability_types.len(), 1);
    }
}
