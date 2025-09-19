use anyhow::{Context as AnyhowContext, Error, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::file_classifier::FileClassifier;
use crate::locales::Language;
use crate::parser::Context;
use crate::response::{
    ActionInfo, ParAnalysis, PolicyViolation, PrincipalInfo, RemediationAction,
    RemediationGuidance, ResourceInfo, Response, SecurityFunctionQuality, SensitivityLevel,
    TrustLevel, VulnType,
};
use crate::security_patterns::{
    PatternConfig, PatternMatch, PatternQuery, PatternType, SecurityRiskPatterns,
};

fn empty_par_analysis() -> ParAnalysis {
    ParAnalysis {
        principals: Vec::new(),
        actions: Vec::new(),
        resources: Vec::new(),
        policy_violations: Vec::new(),
    }
}

fn empty_response(file_path: &PathBuf, content: String) -> Response {
    Response {
        scratchpad: String::new(),
        analysis: String::new(),
        poc: String::new(),
        confidence_score: 0,
        vulnerability_types: Vec::new(),
        par_analysis: empty_par_analysis(),
        remediation_guidance: RemediationGuidance {
            policy_enforcement: Vec::new(),
        },
        file_path: Some(file_path.to_string_lossy().to_string()),
        pattern_description: None,
        matched_source_code: None,
        full_source_code: Some(content),
    }
}

fn normalize_identifier(snippet: &str, fallback: &str) -> String {
    let candidate: String = snippet
        .lines()
        .next()
        .unwrap_or(fallback)
        .trim()
        .chars()
        .take(80)
        .collect();
    let cleaned = candidate
        .trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace())
        .to_string();
    if cleaned.trim().is_empty() {
        fallback.to_string()
    } else {
        cleaned
    }
}

fn byte_to_line(content: &str, byte_offset: usize) -> usize {
    if byte_offset >= content.len() {
        return content.lines().count().max(1);
    }
    content[..byte_offset]
        .bytes()
        .filter(|b| *b == b'\n')
        .count()
        + 1
}

fn extract_snippet(content: &str, start_byte: usize, end_byte: usize) -> String {
    if start_byte >= end_byte || start_byte >= content.len() {
        return String::new();
    }
    let end = end_byte.min(content.len());
    content[start_byte..end]
        .lines()
        .take(8)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .chars()
        .take(400)
        .collect()
}

fn infer_vuln_types(description: &str, snippet: &str) -> Vec<VulnType> {
    let context = format!("{} {}", description, snippet).to_lowercase();
    let mut vulns = Vec::new();

    if context.contains("sql") {
        vulns.push(VulnType::SQLI);
    }
    if context.contains("xss") || context.contains("javascript") || context.contains("script") {
        vulns.push(VulnType::XSS);
    }
    if context.contains("remote code")
        || context.contains("command execution")
        || context.contains("system(")
    {
        vulns.push(VulnType::RCE);
    }
    if context.contains("ssrf")
        || context.contains("request forgery")
        || context.contains("http client")
    {
        vulns.push(VulnType::SSRF);
    }
    if context.contains("idor") || context.contains("insecure direct object") {
        vulns.push(VulnType::IDOR);
    }
    if context.contains("file inclusion") || context.contains("lfi") {
        vulns.push(VulnType::LFI);
    }

    if context.contains("access control") || context.contains("auth") || context.contains("role") {
        vulns.push(VulnType::AFO);
    }

    if vulns.is_empty() {
        vulns.push(VulnType::Other(description.to_string()));
    }

    vulns
}

fn severity_from_vulns(vulns: &[VulnType]) -> &'static str {
    if vulns
        .iter()
        .any(|v| matches!(v, VulnType::RCE | VulnType::SQLI))
    {
        "high"
    } else if vulns
        .iter()
        .any(|v| matches!(v, VulnType::SSRF | VulnType::IDOR | VulnType::AFO))
    {
        "medium"
    } else {
        "low"
    }
}

fn confidence_from_context(
    pattern_type: &PatternType,
    severity: &str,
    attack_vector_count: usize,
) -> i32 {
    let base = match severity {
        "high" => 80,
        "medium" => 65,
        _ => 45,
    };
    let type_bonus = match *pattern_type {
        PatternType::Principal => 5,
        PatternType::Action => 10,
        PatternType::Resource => 8,
    };

    let vector_bonus = (attack_vector_count as i32 * 5).min(15);
    std::cmp::min(95, base + type_bonus + vector_bonus)
}

fn priority_from_severity(severity: &str) -> &'static str {
    match severity {
        "high" => "high",
        "medium" => "medium",
        _ => "low",
    }
}

fn response_from_pattern(
    file_path: &PathBuf,
    content: &str,
    pattern_match: &PatternMatch,
    override_vulns: Option<Vec<VulnType>>,
) -> Response {
    let snippet = if !pattern_match.matched_text.is_empty() {
        pattern_match.matched_text.clone()
    } else {
        extract_snippet(content, pattern_match.start_byte, pattern_match.end_byte)
    };

    let description = pattern_match.pattern_config.description.clone();
    let attack_vectors = pattern_match.pattern_config.attack_vector.join(", ");
    let attack_vector_count = pattern_match.pattern_config.attack_vector.len();
    let pattern_type = pattern_match.pattern_type.clone();
    let (start_line, end_line) = (
        byte_to_line(content, pattern_match.start_byte),
        byte_to_line(
            content,
            pattern_match.end_byte.max(pattern_match.start_byte + 1),
        ),
    );

    let vuln_types = override_vulns.unwrap_or_else(|| infer_vuln_types(&description, &snippet));
    let severity = severity_from_vulns(&vuln_types);
    let confidence = confidence_from_context(&pattern_type, severity, attack_vector_count);

    let identifier = normalize_identifier(&snippet, &description);

    let mut par_analysis = empty_par_analysis();
    match &pattern_type {
        PatternType::Principal => {
            par_analysis.principals.push(PrincipalInfo {
                identifier,
                trust_level: TrustLevel::Untrusted,
                source_context: format!("lines {}-{}", start_line, end_line),
                risk_factors: vec![description.clone()],
            });
        }
        PatternType::Action => {
            par_analysis.actions.push(ActionInfo {
                identifier,
                security_function: description.clone(),
                implementation_quality: SecurityFunctionQuality::Insufficient,
                detected_weaknesses: vec![description.clone()],
                bypass_vectors: pattern_match.pattern_config.attack_vector.clone(),
            });
        }
        PatternType::Resource => {
            par_analysis.resources.push(ResourceInfo {
                identifier,
                sensitivity_level: SensitivityLevel::Medium,
                operation_type: description.clone(),
                protection_mechanisms: vec!["Monitor access patterns".to_string()],
            });
        }
    }

    par_analysis.policy_violations.push(PolicyViolation {
        rule_id: format!("PATTERN-{:?}", pattern_type),
        rule_description: description.clone(),
        violation_path: format!("{}:{}-{}", file_path.display(), start_line, end_line),
        severity: severity.to_string(),
        confidence: (confidence as f64 / 100.0).min(1.0),
    });

    let analysis_text = format!(
        "ファイル\"{}\"の{}行目付近でパターン\"{}\"を検出しました。検出カテゴリ: {:?}。攻撃ベクター候補: {}。",
        file_path.display(),
        start_line,
        description,
        pattern_type,
        if attack_vectors.is_empty() {
            "記録なし".to_string()
        } else {
            attack_vectors
        }
    );

    let scratchpad = format!(
        "pattern={:?} lines {}-{} severity={} confidence={} matched=\"{}\"",
        pattern_type,
        start_line,
        end_line,
        severity,
        confidence,
        snippet.replace('\n', " ")
    );

    let poc = if snippet.is_empty() {
        String::from("<コード抜粋なし>")
    } else {
        snippet.clone()
    };

    let mut response = Response {
        scratchpad,
        analysis: analysis_text,
        poc,
        confidence_score: confidence,
        vulnerability_types: vuln_types,
        par_analysis,
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![RemediationAction {
                component: description.clone(),
                required_improvement: "コーディングガイドラインの見直し".to_string(),
                specific_guidance: format!(
                    "{}に対する防御策を実装し、入力検証やアクセス制御を強化してください。",
                    description
                ),
                priority: priority_from_severity(severity).to_string(),
            }],
        },
        file_path: Some(file_path.to_string_lossy().to_string()),
        pattern_description: Some(description.clone()),
        matched_source_code: if snippet.is_empty() {
            None
        } else {
            Some(snippet.clone())
        },
        full_source_code: Some(content.to_string()),
    };

    response.sanitize();
    response
}

fn heuristic_responses(file_path: &PathBuf, content: &str) -> Vec<Response> {
    let mut responses = Vec::new();
    let Ok(regex) = Regex::new(
        r#"(?im)(?P<key>(password|secret|token|api[_-]?key))\s*[:=]\s*['"](?P<value>[^'"]+)['"]"#,
    ) else {
        return responses;
    };

    for capture in regex.captures_iter(content) {
        if let Some(matched) = capture.get(0) {
            let start = matched.start();
            let end = matched.end();
            let snippet = content[start..end.min(content.len())].to_string();
            let pattern_match = PatternMatch {
                pattern_config: PatternConfig {
                    pattern_type: PatternQuery::Definition {
                        definition: "heuristic:hardcoded-credential".to_string(),
                    },
                    description: "ハードコードされた認証情報".to_string(),
                    attack_vector: vec!["T1552".to_string()],
                },
                pattern_type: PatternType::Resource,
                start_byte: start,
                end_byte: end,
                matched_text: snippet,
            };

            responses.push(response_from_pattern(
                file_path,
                content,
                &pattern_match,
                Some(vec![VulnType::Other("HardcodedCredential".to_string())]),
            ));
        }
    }

    responses
}

fn merge_responses(mut responses: Vec<Response>, file_path: &PathBuf, content: &str) -> Response {
    let mut primary = responses.remove(0);

    for response in responses {
        if !response.scratchpad.is_empty() {
            if !primary.scratchpad.is_empty() {
                primary.scratchpad.push('\n');
            }
            primary.scratchpad.push_str(&response.scratchpad);
        }

        if !response.analysis.is_empty() {
            if !primary.analysis.is_empty() {
                primary.analysis.push_str("\n\n");
            }
            primary.analysis.push_str(&response.analysis);
        }

        if !response.poc.is_empty() {
            if !primary.poc.is_empty() {
                primary.poc.push_str("\n---\n");
            }
            primary.poc.push_str(&response.poc);
        }

        if let Some(ref mut existing) = primary.matched_source_code {
            existing.push_str("\n---\n");
            existing.push_str(response.poc.trim());
        }

        primary
            .vulnerability_types
            .extend(response.vulnerability_types.clone());
        primary
            .par_analysis
            .principals
            .extend(response.par_analysis.principals.clone());
        primary
            .par_analysis
            .actions
            .extend(response.par_analysis.actions.clone());
        primary
            .par_analysis
            .resources
            .extend(response.par_analysis.resources.clone());
        primary
            .par_analysis
            .policy_violations
            .extend(response.par_analysis.policy_violations.clone());
        primary
            .remediation_guidance
            .policy_enforcement
            .extend(response.remediation_guidance.policy_enforcement.clone());

        primary.confidence_score = std::cmp::min(
            100,
            primary.confidence_score + response.confidence_score / 2,
        );
    }

    primary.pattern_description = Some(format!(
        "{}件のパターンまたはヒューリスティック一致",
        primary.par_analysis.policy_violations.len()
    ));
    primary.full_source_code = Some(content.to_string());
    primary.file_path = Some(file_path.to_string_lossy().to_string());
    primary.sanitize();
    primary
}

fn append_context_summary(response: &mut Response, context: &Context) {
    if context.definitions.is_empty() && context.references.is_empty() {
        return;
    }

    let mut summary = String::new();
    if !context.definitions.is_empty() {
        summary.push_str("参照された定義:\n");
        for definition in &context.definitions {
            summary.push_str(&format!("  - {}\n", definition.name));
        }
    }

    if !context.references.is_empty() {
        if !summary.is_empty() {
            summary.push('\n');
        }
        summary.push_str("参照された識別子:\n");
        for reference in &context.references {
            summary.push_str(&format!("  - {}\n", reference.name));
        }
    }

    if !summary.is_empty() {
        if !response.analysis.is_empty() {
            response.analysis.push_str("\n\n");
        }
        response.analysis.push_str(&summary);
    }
}

pub async fn analyze_file(
    file_path: &PathBuf,
    _model: &str,
    files: &[PathBuf],
    _verbosity: u8,
    context: &Context,
    min_confidence: i32,
    _debug: bool,
    _output_dir: &Option<PathBuf>,
    _api_base_url: Option<&str>,
    _language: &Language,
) -> Result<Response, Error> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path.display()))?;

    if content.trim().is_empty() {
        return Ok(empty_response(file_path, content));
    }

    let filename = file_path.to_string_lossy();
    let lang = FileClassifier::classify(&filename, &content);
    let patterns = SecurityRiskPatterns::new_with_root(lang, None);
    let pattern_matches = patterns.get_pattern_matches(&content);

    let mut responses: Vec<Response> = pattern_matches
        .iter()
        .map(|pattern| response_from_pattern(file_path, &content, pattern, None))
        .collect();

    if responses.is_empty() {
        responses.extend(heuristic_responses(file_path, &content));
    }

    if responses.is_empty() {
        return Ok(empty_response(file_path, content));
    }

    let mut merged = merge_responses(responses, file_path, &content);
    append_context_summary(&mut merged, context);

    if merged.confidence_score < min_confidence {
        merged.confidence_score = min_confidence;
    }

    if !files.is_empty() {
        merged
            .scratchpad
            .push_str(&format!("\n参照ファイル数: {}", files.len()));
    }

    merged.sanitize();
    Ok(merged)
}

pub async fn analyze_pattern(
    file_path: &PathBuf,
    pattern_match: &PatternMatch,
    _model: &str,
    _files: &[PathBuf],
    _verbosity: u8,
    min_confidence: i32,
    _debug: bool,
    _output_dir: &Option<PathBuf>,
    _api_base_url: Option<&str>,
    _language: &Language,
) -> Result<Option<Response>, Error> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path.display()))?;

    let mut response = response_from_pattern(file_path, &content, pattern_match, None);
    if response.confidence_score < min_confidence {
        return Ok(None);
    }

    response.sanitize();
    Ok(Some(response))
}
