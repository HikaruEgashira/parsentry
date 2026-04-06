//! Re-exports from parsentry-core and extensions for the main crate.

pub use parsentry_core::{
    response_json_schema, ActionInfo, Language as CoreLanguage, ParAnalysis, PolicyViolation,
    PrincipalInfo, RemediationAction, RemediationGuidance, ResourceInfo, Response,
    SecurityFunctionQuality, SensitivityLevel, TrustLevel, VulnType,
};

/// Extension trait for Response to add methods specific to the main parsentry crate.
pub trait ResponseExt {
    /// Print a human-readable report to stdout.
    fn print_readable(&self);
    /// Convert to markdown format.
    fn to_markdown(&self) -> String;
}

impl ResponseExt for Response {
    fn print_readable(&self) {
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

    fn to_markdown(&self) -> String {
        parsentry_reports::to_markdown(self)
    }
}

/// Create a Response from ClaudeCodeResponse.
pub fn from_claude_code_response(
    cc_response: parsentry_executor::ClaudeCodeResponse,
    file_path: String,
) -> Response {
    let vulnerability_types: Vec<VulnType> = cc_response
        .vulnerability_types
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let par_analysis = ParAnalysis {
        principals: cc_response
            .par_analysis
            .principals
            .iter()
            .map(|p| PrincipalInfo {
                identifier: p.identifier.clone(),
                trust_level: match p.trust_level.as_str() {
                    "trusted" => TrustLevel::Trusted,
                    "semi_trusted" => TrustLevel::SemiTrusted,
                    _ => TrustLevel::Untrusted,
                },
                source_context: p.source_context.clone(),
                risk_factors: p.risk_factors.clone(),
            })
            .collect(),
        actions: cc_response
            .par_analysis
            .actions
            .iter()
            .map(|a| ActionInfo {
                identifier: a.identifier.clone(),
                security_function: a.security_function.clone(),
                implementation_quality: match a.implementation_quality.as_str() {
                    "adequate" => SecurityFunctionQuality::Adequate,
                    "insufficient" => SecurityFunctionQuality::Insufficient,
                    "missing" => SecurityFunctionQuality::Missing,
                    _ => SecurityFunctionQuality::Bypassed,
                },
                detected_weaknesses: a.detected_weaknesses.clone(),
                bypass_vectors: a.bypass_vectors.clone(),
            })
            .collect(),
        resources: cc_response
            .par_analysis
            .resources
            .iter()
            .map(|r| ResourceInfo {
                identifier: r.identifier.clone(),
                sensitivity_level: match r.sensitivity_level.as_str() {
                    "low" => SensitivityLevel::Low,
                    "medium" => SensitivityLevel::Medium,
                    "high" => SensitivityLevel::High,
                    _ => SensitivityLevel::Critical,
                },
                operation_type: r.operation_type.clone(),
                protection_mechanisms: r.protection_mechanisms.clone(),
            })
            .collect(),
        policy_violations: cc_response
            .par_analysis
            .policy_violations
            .iter()
            .map(|v| PolicyViolation {
                rule_id: v.rule_id.clone(),
                rule_description: v.rule_description.clone(),
                violation_path: v.violation_path.clone(),
                severity: v.severity.clone(),
                confidence: v.confidence,
            })
            .collect(),
    };

    let remediation_guidance = RemediationGuidance {
        policy_enforcement: cc_response
            .remediation_guidance
            .policy_enforcement
            .iter()
            .map(|p| RemediationAction {
                component: p.component.clone(),
                required_improvement: p.required_improvement.clone(),
                specific_guidance: p.specific_guidance.clone(),
                priority: p.priority.clone(),
            })
            .collect(),
    };

    let mut response = Response {
        scratchpad: cc_response.scratchpad,
        analysis: cc_response.analysis,
        poc: cc_response.poc,
        confidence_score: Response::normalize_confidence_score(cc_response.confidence_score),
        vulnerability_types,
        par_analysis,
        remediation_guidance,
        file_path: Some(file_path),
        pattern_description: cc_response.pattern_description,
        matched_source_code: cc_response.matched_source_code,
        full_source_code: None,
    };

    response.sanitize();
    response
}


/// Create a test response with default optional fields.
#[cfg(test)]
pub fn test_response(
    analysis: String,
    confidence_score: i32,
    vulnerability_types: Vec<VulnType>,
) -> Response {
    Response {
        scratchpad: "Test scratchpad".to_string(),
        analysis,
        poc: "Test PoC".to_string(),
        confidence_score,
        vulnerability_types,
        par_analysis: ParAnalysis::default(),
        remediation_guidance: RemediationGuidance::default(),
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    }
}
