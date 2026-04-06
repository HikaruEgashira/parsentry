//! Analysis response types.

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::vuln_type::VulnType;

/// The main response structure for security analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub scratchpad: String,
    #[serde(default)]
    pub analysis: String,
    #[serde(default)]
    pub poc: String,
    #[serde(default)]
    pub confidence_score: i32,
    #[serde(default)]
    pub vulnerability_types: Vec<VulnType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_source_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_source_code: Option<String>,
}

impl Response {
    /// Normalize confidence score (convert 1-10 scale to 1-100).
    #[must_use]
    pub fn normalize_confidence_score(score: i32) -> i32 {
        if score > 0 && score <= 10 {
            score * 10
        } else {
            score
        }
    }

    /// Clean up and validate the response data.
    pub fn sanitize(&mut self) {
        let mut unique_vulns = std::collections::HashSet::new();
        self.vulnerability_types
            .retain(|v| unique_vulns.insert(v.clone()));

        if self.vulnerability_types.is_empty() && self.confidence_score > 50 {
            self.confidence_score = 0;
        }
    }

    /// Check if this response indicates a vulnerability was found.
    #[must_use]
    pub fn has_vulnerability(&self) -> bool {
        !self.vulnerability_types.is_empty() && self.confidence_score > 0
    }

    /// Get severity level based on confidence score.
    #[must_use]
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
#[must_use]
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
            }
        },
        "required": ["scratchpad", "analysis", "poc", "confidence_score", "vulnerability_types"]
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

    #[test]
    fn test_has_vulnerability() {
        let mut response = Response::default();
        assert!(!response.has_vulnerability());

        response.vulnerability_types = vec![VulnType::SQLI];
        response.confidence_score = 80;
        assert!(response.has_vulnerability());
    }

    #[test]
    fn test_response_json_schema_returns_non_default() {
        let schema = response_json_schema();
        assert!(schema.is_object());
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].get("confidence_score").is_some());
        assert!(schema["properties"].get("vulnerability_types").is_some());
    }

    #[test]
    fn test_normalize_confidence_score_boundaries() {
        assert_eq!(Response::normalize_confidence_score(1), 10);
        assert_eq!(Response::normalize_confidence_score(11), 11);
        assert_eq!(Response::normalize_confidence_score(-5), -5);
    }

    #[test]
    fn test_severity_level_boundaries() {
        let mut r = Response::default();
        r.confidence_score = 90;
        assert_eq!(r.severity_level(), "critical");
        r.confidence_score = 100;
        assert_eq!(r.severity_level(), "critical");
        r.confidence_score = 89;
        assert_eq!(r.severity_level(), "high");
        r.confidence_score = 70;
        assert_eq!(r.severity_level(), "high");
        r.confidence_score = 69;
        assert_eq!(r.severity_level(), "medium");
        r.confidence_score = 50;
        assert_eq!(r.severity_level(), "medium");
        r.confidence_score = 49;
        assert_eq!(r.severity_level(), "low");
        r.confidence_score = 30;
        assert_eq!(r.severity_level(), "low");
        r.confidence_score = 29;
        assert_eq!(r.severity_level(), "info");
        r.confidence_score = 0;
        assert_eq!(r.severity_level(), "info");
    }

    #[test]
    fn test_normalize_confidence_score_boundary_zero() {
        // score=0 should NOT be multiplied (kills > → >= : 0 >= 0 would be true)
        assert_eq!(Response::normalize_confidence_score(0), 0);
        // score=1 should be multiplied
        assert_eq!(Response::normalize_confidence_score(1), 10);
        // score=11 should NOT be multiplied
        assert_eq!(Response::normalize_confidence_score(11), 11);
    }

    #[test]
    fn test_sanitize_empty_vulns_boundary_at_50() {
        // Kills > → >= at 50 in sanitize
        // score=50 with empty vulns: 50 > 50 is false, so NOT reset to 0
        let mut r = Response {
            confidence_score: 50,
            vulnerability_types: vec![],
            ..Default::default()
        };
        r.sanitize();
        assert_eq!(r.confidence_score, 50);

        // score=51 with empty vulns: 51 > 50 is true, reset to 0
        let mut r2 = Response {
            confidence_score: 51,
            vulnerability_types: vec![],
            ..Default::default()
        };
        r2.sanitize();
        assert_eq!(r2.confidence_score, 0);
    }
}
