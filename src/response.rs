use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum VulnType {
    LFI,
    RCE,
    SSRF,
    AFO,
    SQLI,
    XSS,
    IDOR,
    Other(String),
}

impl std::fmt::Display for VulnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulnType::LFI => write!(f, "LFI"),
            VulnType::RCE => write!(f, "RCE"),
            VulnType::SSRF => write!(f, "SSRF"),
            VulnType::AFO => write!(f, "AFO"),
            VulnType::SQLI => write!(f, "SQLI"),
            VulnType::XSS => write!(f, "XSS"),
            VulnType::IDOR => write!(f, "IDOR"),
            VulnType::Other(name) => write!(f, "{}", name),
        }
    }
}

impl VulnType {
    /// Get CWE (Common Weakness Enumeration) IDs for this vulnerability type
    pub fn cwe_ids(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["CWE-89".to_string()],
            VulnType::XSS => vec!["CWE-79".to_string(), "CWE-80".to_string()],
            VulnType::RCE => vec![
                "CWE-77".to_string(),
                "CWE-78".to_string(),
                "CWE-94".to_string(),
            ],
            VulnType::LFI => vec!["CWE-22".to_string(), "CWE-98".to_string()],
            VulnType::SSRF => vec!["CWE-918".to_string()],
            VulnType::AFO => vec!["CWE-22".to_string(), "CWE-73".to_string()],
            VulnType::IDOR => vec!["CWE-639".to_string(), "CWE-284".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Get MITRE ATT&CK technique IDs for this vulnerability type
    pub fn mitre_attack_ids(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["T1190".to_string()], // Exploit Public-Facing Application
            VulnType::XSS => vec!["T1190".to_string(), "T1185".to_string()], // Browser Session Hijacking
            VulnType::RCE => vec!["T1190".to_string(), "T1059".to_string()], // Command and Scripting Interpreter
            VulnType::LFI => vec!["T1083".to_string()], // File and Directory Discovery
            VulnType::SSRF => vec!["T1090".to_string()], // Connection Proxy
            VulnType::AFO => vec!["T1083".to_string(), "T1005".to_string()], // Data from Local System
            VulnType::IDOR => vec!["T1190".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Get OWASP Top 10 category for this vulnerability type
    pub fn owasp_categories(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["A03:2021-Injection".to_string()],
            VulnType::XSS => vec!["A03:2021-Injection".to_string()],
            VulnType::RCE => vec!["A03:2021-Injection".to_string()],
            VulnType::LFI => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::SSRF => vec!["A10:2021-Server-Side Request Forgery".to_string()],
            VulnType::AFO => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::IDOR => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::Other(_) => vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustLevel {
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "semi_trusted")]
    SemiTrusted,
    #[serde(rename = "untrusted")]
    Untrusted,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalInfo {
    pub identifier: String,
    pub trust_level: TrustLevel,
    pub source_context: String,
    pub risk_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    pub identifier: String,
    pub security_function: String,
    pub implementation_quality: SecurityFunctionQuality,
    pub detected_weaknesses: Vec<String>,
    pub bypass_vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub identifier: String,
    pub sensitivity_level: SensitivityLevel,
    pub operation_type: String,
    pub protection_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub rule_id: String,
    pub rule_description: String,
    pub violation_path: String,
    pub severity: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParAnalysis {
    pub principals: Vec<PrincipalInfo>,
    pub actions: Vec<ActionInfo>,
    pub resources: Vec<ResourceInfo>,
    pub policy_violations: Vec<PolicyViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    pub component: String,
    pub required_improvement: String,
    pub specific_guidance: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationGuidance {
    pub policy_enforcement: Vec<RemediationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub scratchpad: String,
    pub analysis: String,
    pub poc: String,
    pub confidence_score: i32,
    pub vulnerability_types: Vec<VulnType>,
    pub par_analysis: ParAnalysis,
    pub remediation_guidance: RemediationGuidance,
    // Report enhancement fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_source_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_source_code: Option<String>,
}

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

impl Response {
    pub fn normalize_confidence_score(score: i32) -> i32 {
        if score > 0 && score <= 10 {
            score * 10
        } else {
            score
        }
    }

    /// Clean up and validate the response data
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
        if self.par_analysis.principals.is_empty()
            && self.par_analysis.actions.is_empty()
            && self.par_analysis.resources.is_empty()
            && self.par_analysis.policy_violations.is_empty()
            && self.confidence_score > 30
        {
            self.confidence_score = std::cmp::min(self.confidence_score, 30);
        }
    }

    pub fn print_readable(&self) {
        println!("\n📝 PAR Security Analysis Report");
        println!("{}", "=".repeat(80));

        let confidence_icon = match self.confidence_score {
            90..=100 => "🔴 高",
            70..=89 => "🟠 中高",
            50..=69 => "🟡 中",
            30..=49 => "🟢 中低",
            _ => "🔵 低",
        };
        println!(
            "\n🎯 信頼度スコア: {} ({})",
            self.confidence_score, confidence_icon
        );

        if !self.vulnerability_types.is_empty() {
            println!("\n⚠ 検出された脆弱性タイプ:");
            for vuln_type in &self.vulnerability_types {
                println!("  - {:?}", vuln_type);
            }
        }

        println!("\n🔍 PAR Policy Analysis:");
        println!("{}", "-".repeat(80));

        if !self.par_analysis.principals.is_empty() {
            println!("\n🧑 Principals (データ源):");
            for principal in &self.par_analysis.principals {
                println!(
                    "  - {}: {:?} ({})",
                    principal.identifier, principal.trust_level, principal.source_context
                );
            }
        }

        if !self.par_analysis.actions.is_empty() {
            println!("\n⚙ Actions (セキュリティ制御):");
            for action in &self.par_analysis.actions {
                println!(
                    "  - {}: {:?} ({})",
                    action.identifier, action.implementation_quality, action.security_function
                );
            }
        }

        if !self.par_analysis.resources.is_empty() {
            println!("\n🗄 Resources (操作対象):");
            for resource in &self.par_analysis.resources {
                println!(
                    "  - {}: {:?} ({})",
                    resource.identifier, resource.sensitivity_level, resource.operation_type
                );
            }
        }

        if !self.par_analysis.policy_violations.is_empty() {
            println!("\n❌ Policy Violations:");
            for violation in &self.par_analysis.policy_violations {
                println!("  - {}: {}", violation.rule_id, violation.rule_description);
                println!("    Path: {}", violation.violation_path);
                println!(
                    "    Severity: {} (Confidence: {:.2})",
                    violation.severity, violation.confidence
                );
            }
        }

        println!("\n📊 詳細解析:");
        println!("{}", "-".repeat(80));
        println!("{}", self.analysis);

        if !self.poc.is_empty() {
            println!("\n🔨 PoC(概念実証コード):");
            println!("{}", "-".repeat(80));
            println!("{}", self.poc);
        }

        if !self.remediation_guidance.policy_enforcement.is_empty() {
            println!("\n🔧 修復ガイダンス:");
            println!("{}", "-".repeat(80));
            for remediation in &self.remediation_guidance.policy_enforcement {
                println!("Component: {}", remediation.component);
                println!("Required: {}", remediation.required_improvement);
                println!("Guidance: {}", remediation.specific_guidance);
                println!("Priority: {}", remediation.priority);
                println!();
            }
        }

        if !self.scratchpad.is_empty() {
            println!("\n📓 解析ノート:");
            println!("{}", "-".repeat(80));
            println!("{}", self.scratchpad);
        }

        println!();
    }

    pub fn to_markdown(&self) -> String {
        crate::reports::markdown::to_markdown(self)
    }
}

#[cfg(test)]
impl Response {
    /// Create a test response with default optional fields
    pub fn test_response(
        analysis: String,
        confidence_score: i32,
        vulnerability_types: Vec<VulnType>,
    ) -> Self {
        Response {
            scratchpad: "Test scratchpad".to_string(),
            analysis,
            poc: "Test PoC".to_string(),
            confidence_score,
            vulnerability_types,
            par_analysis: ParAnalysis {
                principals: vec![],
                actions: vec![],
                resources: vec![],
                policy_violations: vec![],
            },
            remediation_guidance: RemediationGuidance {
                policy_enforcement: vec![],
            },
            file_path: None,
            pattern_description: None,
            matched_source_code: None,
            full_source_code: None,
        }
    }
}

