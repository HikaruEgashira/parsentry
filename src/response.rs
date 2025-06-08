use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

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
            VulnType::RCE => vec!["CWE-77".to_string(), "CWE-78".to_string(), "CWE-94".to_string()],
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
                println!("  - {}: {:?} ({})", principal.identifier, principal.trust_level, principal.source_context);
            }
        }

        if !self.par_analysis.actions.is_empty() {
            println!("\n⚙ Actions (セキュリティ制御):");
            for action in &self.par_analysis.actions {
                println!("  - {}: {:?} ({})", action.identifier, action.implementation_quality, action.security_function);
            }
        }

        if !self.par_analysis.resources.is_empty() {
            println!("\n🗄 Resources (操作対象):");
            for resource in &self.par_analysis.resources {
                println!("  - {}: {:?} ({})", resource.identifier, resource.sensitivity_level, resource.operation_type);
            }
        }

        if !self.par_analysis.policy_violations.is_empty() {
            println!("\n❌ Policy Violations:");
            for violation in &self.par_analysis.policy_violations {
                println!("  - {}: {}", violation.rule_id, violation.rule_description);
                println!("    Path: {}", violation.violation_path);
                println!("    Severity: {} (Confidence: {:.2})", violation.severity, violation.confidence);
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
        let mut md = String::new();
        md.push_str("# PAR Security Analysis Report\n\n");

        let confidence_badge = match self.confidence_score {
            90..=100 => "![高信頼度](https://img.shields.io/badge/信頼度-高-red)",
            70..=89 => "![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange)",
            50..=69 => "![中信頼度](https://img.shields.io/badge/信頼度-中-yellow)",
            30..=49 => "![中低信頼度](https://img.shields.io/badge/信頼度-中低-green)",
            _ => "![低信頼度](https://img.shields.io/badge/信頼度-低-blue)",
        };
        md.push_str(&format!(
            "{} **信頼度スコア: {}**\n\n",
            confidence_badge, self.confidence_score
        ));

        if !self.vulnerability_types.is_empty() {
            md.push_str("## 脆弱性タイプ\n\n");
            for vuln_type in &self.vulnerability_types {
                md.push_str(&format!("- `{:?}`\n", vuln_type));
            }
            md.push('\n');
        }

        md.push_str("## PAR Policy Analysis\n\n");

        md.push_str("### Principals (データ源)\n\n");
        for principal in &self.par_analysis.principals {
            md.push_str(&format!("- **{}**: {:?}\n", principal.identifier, principal.trust_level));
            md.push_str(&format!("  - Context: {}\n", principal.source_context));
            md.push_str(&format!("  - Risk Factors: {}\n", principal.risk_factors.join(", ")));
        }
        md.push('\n');

        md.push_str("### Actions (セキュリティ制御)\n\n");
        for action in &self.par_analysis.actions {
            md.push_str(&format!("- **{}**: {:?}\n", action.identifier, action.implementation_quality));
            md.push_str(&format!("  - Function: {}\n", action.security_function));
            md.push_str(&format!("  - Weaknesses: {}\n", action.detected_weaknesses.join(", ")));
            md.push_str(&format!("  - Bypass Vectors: {}\n", action.bypass_vectors.join(", ")));
        }
        md.push('\n');

        md.push_str("### Resources (操作対象)\n\n");
        for resource in &self.par_analysis.resources {
            md.push_str(&format!("- **{}**: {:?}\n", resource.identifier, resource.sensitivity_level));
            md.push_str(&format!("  - Operation: {}\n", resource.operation_type));
            md.push_str(&format!("  - Protection: {}\n", resource.protection_mechanisms.join(", ")));
        }
        md.push('\n');

        if !self.par_analysis.policy_violations.is_empty() {
            md.push_str("### Policy Violations\n\n");
            for violation in &self.par_analysis.policy_violations {
                md.push_str(&format!("#### {}: {}\n\n", violation.rule_id, violation.rule_description));
                md.push_str(&format!("- **Path**: {}\n", violation.violation_path));
                md.push_str(&format!("- **Severity**: {}\n", violation.severity));
                md.push_str(&format!("- **Confidence**: {:.2}\n\n", violation.confidence));
            }
        }

        md.push_str("## 詳細解析\n\n");
        md.push_str(&self.analysis);
        md.push_str("\n\n");

        if !self.poc.is_empty() {
            md.push_str("## PoC（概念実証コード）\n\n");
            md.push_str("```text\n");
            md.push_str(&self.poc);
            md.push_str("\n```\n\n");
        }

        if !self.remediation_guidance.policy_enforcement.is_empty() {
            md.push_str("## 修復ガイダンス\n\n");
            for remediation in &self.remediation_guidance.policy_enforcement {
                md.push_str(&format!("### {}\n\n", remediation.component));
                md.push_str(&format!("- **Required**: {}\n", remediation.required_improvement));
                md.push_str(&format!("- **Guidance**: {}\n", remediation.specific_guidance));
                md.push_str(&format!("- **Priority**: {}\n\n", remediation.priority));
            }
        }

        if !self.scratchpad.is_empty() {
            md.push_str("## 解析ノート\n\n");
            md.push_str(&self.scratchpad);
            md.push_str("\n\n");
        }

        md
    }
}

#[derive(Debug, Clone)]
pub struct FileAnalysisResult {
    pub file_path: PathBuf,
    pub response: Response,
}

#[derive(Debug, Clone, Default)]
pub struct AnalysisSummary {
    pub results: Vec<FileAnalysisResult>,
}

impl AnalysisSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, file_path: PathBuf, response: Response) {
        self.results.push(FileAnalysisResult {
            file_path,
            response,
        });
    }

    pub fn sort_by_confidence(&mut self) {
        self.results.sort_by(|a, b| {
            b.response
                .confidence_score
                .cmp(&a.response.confidence_score)
        });
    }

    pub fn filter_by_min_confidence(&self, min_score: i32) -> Self {
        Self {
            results: self
                .results
                .iter()
                .filter(|r| r.response.confidence_score >= min_score)
                .cloned()
                .collect(),
        }
    }

    pub fn filter_by_vuln_types(&self, vuln_types: &[VulnType]) -> Self {
        Self {
            results: self
                .results
                .iter()
                .filter(|r| {
                    r.response
                        .vulnerability_types
                        .iter()
                        .any(|vt| vuln_types.contains(vt))
                })
                .cloned()
                .collect(),
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# PAR Security Analysis Summary Report\n\n");

        md.push_str("## 概要\n\n");
        md.push_str("| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |\n");
        md.push_str("|---------|------------|--------|------------------|\n");

        for result in &self.results {
            if result.response.confidence_score > 0 {
                let confidence_level = match result.response.confidence_score {
                    90..=100 => "🔴 高",
                    70..=89 => "🟠 中高",
                    50..=69 => "🟡 中",
                    30..=49 => "🟢 中低",
                    _ => "🔵 低",
                };

                let vuln_types = result
                    .response
                    .vulnerability_types
                    .iter()
                    .map(|vt| format!("{:?}", vt))
                    .collect::<Vec<_>>()
                    .join(", ");

                let violations = result
                    .response
                    .par_analysis
                    .policy_violations
                    .iter()
                    .map(|v| v.rule_id.clone())
                    .collect::<Vec<_>>()
                    .join(", ");

                md.push_str(&format!(
                    "| [{}]({}.md) | {} | {} | {} |\n",
                    result
                        .file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    result
                        .file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    vuln_types,
                    confidence_level,
                    violations
                ));
            }
        }

        md.push_str("\n## Policy Violation Analysis\n\n");
        
        let mut violation_count: HashMap<String, i32> = HashMap::new();
        for result in &self.results {
            for violation in &result.response.par_analysis.policy_violations {
                *violation_count.entry(violation.rule_id.clone()).or_insert(0) += 1;
            }
        }

        md.push_str("| Rule ID | 件数 | 説明 |\n");
        md.push_str("|---------|------|------|\n");

        for (rule_id, count) in violation_count.iter() {
            // Find the first occurrence to get the description
            let description = self
                .results
                .iter()
                .flat_map(|r| &r.response.par_analysis.policy_violations)
                .find(|v| v.rule_id == *rule_id)
                .map(|v| v.rule_description.clone())
                .unwrap_or_default();
                
            md.push_str(&format!("| {} | {} | {} |\n", rule_id, count, description));
        }

        md
    }
}