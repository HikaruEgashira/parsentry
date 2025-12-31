use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::summary::AnalysisSummary;
use parsentry_core::{ParAnalysis, Response, VulnType};

/// SARIF (Static Analysis Results Interchange Format) v2.1.0 implementation
/// Spec: https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifReport {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<SarifArtifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation: Option<SarifInvocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SarifRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<SarifMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: Option<SarifMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<SarifMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifRuleProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_configuration: Option<SarifConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRuleProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_severity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifConfiguration {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifMessage {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    #[serde(rename = "ruleIndex")]
    pub rule_index: usize,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprints: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifResultProperties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifResultProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_attack: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owasp: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<SarifRegion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: i32,
    #[serde(rename = "startColumn", skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i32>,
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<SarifArtifactContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifArtifactContent {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifArtifact {
    pub location: SarifArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifInvocation {
    #[serde(rename = "executionSuccessful")]
    pub execution_successful: bool,
    #[serde(rename = "startTimeUtc", skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    #[serde(rename = "endTimeUtc", skip_serializing_if = "Option::is_none")]
    pub end_time_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}

impl SarifReport {
    /// Create a new SARIF report from analysis summary
    pub fn from_analysis_summary(summary: &AnalysisSummary, version: &str) -> Self {
        let mut rules = Vec::new();
        let mut results = Vec::new();
        let mut artifacts = Vec::new();
        let mut rule_map = HashMap::new();

        // Collect unique vulnerability types and create rules
        for result in &summary.results {
            for vuln_type in &result.response.vulnerability_types {
                let rule_id = vuln_type.to_string();
                if !rule_map.contains_key(&rule_id) {
                    let rule_index = rules.len();
                    rule_map.insert(rule_id.clone(), rule_index);
                    rules.push(create_rule_for_vuln_type(vuln_type));
                }
            }
        }

        // Create artifacts and results
        for result in &summary.results {
            let file_path = &result.file_path;
            let response = &result.response;

            let artifact_index = artifacts.len();
            artifacts.push(SarifArtifact {
                location: SarifArtifactLocation {
                    uri: file_path.to_string_lossy().to_string(),
                    index: Some(artifact_index),
                },
                length: None,
                mime_type: guess_mime_type(file_path),
            });

            // Create results for each vulnerability in this file
            for vuln_type in &response.vulnerability_types {
                let rule_id = vuln_type.to_string();
                let rule_index = *rule_map.get(&rule_id).unwrap();

                results.push(SarifResult {
                    rule_id: rule_id.clone(),
                    rule_index,
                    level: confidence_to_level(response.confidence_score),
                    message: SarifMessage {
                        text: format!("{}: {}", vuln_type, response.analysis),
                        markdown: Some(response.analysis.clone()),
                    },
                    locations: vec![SarifLocation {
                        physical_location: SarifPhysicalLocation {
                            artifact_location: SarifArtifactLocation {
                                uri: file_path.to_string_lossy().to_string(),
                                index: Some(artifact_index),
                            },
                            region: extract_region_from_par_analysis(&response.par_analysis),
                        },
                    }],
                    fingerprints: Some(generate_fingerprints(file_path, response)),
                    properties: Some(SarifResultProperties {
                        confidence: Some(response.confidence_score as f64 / 100.0),
                        mitre_attack: Some(vuln_type.mitre_attack_ids()),
                        cwe: Some(vuln_type.cwe_ids()),
                        owasp: Some(vuln_type.owasp_categories()),
                    }),
                });
            }
        }

        SarifReport {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "Parsentry".to_string(),
                        version: version.to_string(),
                        information_uri: Some("https://github.com/HikaruEgashira/vulnhuntrs".to_string()),
                        rules: Some(rules),
                    },
                },
                results,
                artifacts: Some(artifacts),
                invocation: Some(SarifInvocation {
                    execution_successful: true,
                    start_time_utc: None,
                    end_time_utc: None,
                    arguments: None,
                }),
            }],
        }
    }

    /// Export SARIF report to JSON string
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Save SARIF report to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load SARIF report from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let report: SarifReport = serde_json::from_str(&content)?;
        Ok(report)
    }

    /// Load SARIF report from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        let report: SarifReport = serde_json::from_str(json)?;
        Ok(report)
    }

    /// Generate markdown report from SARIF
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Security Analysis Report\n\n");

        for run in &self.runs {
            let tool_name = &run.tool.driver.name;
            let tool_version = &run.tool.driver.version;
            md.push_str(&format!("**Tool**: {} v{}\n\n", tool_name, tool_version));

            if run.results.is_empty() {
                md.push_str("No vulnerabilities detected.\n\n");
                continue;
            }

            md.push_str(&format!("**Total findings**: {}\n\n", run.results.len()));

            for (i, result) in run.results.iter().enumerate() {
                md.push_str(&format!("## Finding {}: {}\n\n", i + 1, result.rule_id));

                // Level/Severity
                let level_emoji = match result.level.as_str() {
                    "error" => "🔴",
                    "warning" => "🟠",
                    "note" => "🟡",
                    _ => "⚪",
                };
                md.push_str(&format!("**Severity**: {} {}\n\n", level_emoji, result.level));

                // Location
                if let Some(location) = result.locations.first() {
                    let uri = &location.physical_location.artifact_location.uri;
                    md.push_str(&format!("**File**: `{}`\n", uri));

                    if let Some(region) = &location.physical_location.region {
                        md.push_str(&format!("**Line**: {}\n", region.start_line));
                        if let Some(snippet) = &region.snippet {
                            md.push_str("\n**Snippet**:\n```\n");
                            md.push_str(&snippet.text);
                            md.push_str("\n```\n");
                        }
                    }
                    md.push('\n');
                }

                // Message/Analysis
                md.push_str("### Analysis\n\n");
                if let Some(markdown_text) = &result.message.markdown {
                    md.push_str(markdown_text);
                } else {
                    md.push_str(&result.message.text);
                }
                md.push_str("\n\n");

                // Properties (confidence, CWE, OWASP, MITRE)
                if let Some(props) = &result.properties {
                    if let Some(confidence) = props.confidence {
                        md.push_str(&format!(
                            "**Confidence**: {:.0}%\n",
                            confidence * 100.0
                        ));
                    }
                    if let Some(cwe) = &props.cwe {
                        if !cwe.is_empty() {
                            md.push_str(&format!("**CWE**: {}\n", cwe.join(", ")));
                        }
                    }
                    if let Some(owasp) = &props.owasp {
                        if !owasp.is_empty() {
                            md.push_str(&format!("**OWASP**: {}\n", owasp.join(", ")));
                        }
                    }
                    if let Some(mitre) = &props.mitre_attack {
                        if !mitre.is_empty() {
                            md.push_str(&format!("**MITRE ATT&CK**: {}\n", mitre.join(", ")));
                        }
                    }
                    md.push('\n');
                }

                // Get rule help text if available
                if let Some(rules) = &run.tool.driver.rules {
                    if let Some(rule) = rules.iter().find(|r| r.id == result.rule_id) {
                        if let Some(help) = &rule.help {
                            md.push_str("### Remediation\n\n");
                            if let Some(markdown_help) = &help.markdown {
                                md.push_str(markdown_help);
                            } else {
                                md.push_str(&help.text);
                            }
                            md.push_str("\n\n");
                        }
                    }
                }

                md.push_str("---\n\n");
            }
        }

        md
    }

    /// Generate summary markdown from SARIF
    pub fn to_summary_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Security Analysis Summary\n\n");

        for run in &self.runs {
            if run.results.is_empty() {
                md.push_str("No vulnerabilities detected.\n");
                return md;
            }

            // Count by severity
            let mut error_count = 0;
            let mut warning_count = 0;
            let mut note_count = 0;

            for result in &run.results {
                match result.level.as_str() {
                    "error" => error_count += 1,
                    "warning" => warning_count += 1,
                    _ => note_count += 1,
                }
            }

            md.push_str("## Overview\n\n");
            md.push_str(&format!("| Severity | Count |\n"));
            md.push_str(&format!("|----------|-------|\n"));
            if error_count > 0 {
                md.push_str(&format!("| 🔴 Error | {} |\n", error_count));
            }
            if warning_count > 0 {
                md.push_str(&format!("| 🟠 Warning | {} |\n", warning_count));
            }
            if note_count > 0 {
                md.push_str(&format!("| 🟡 Note | {} |\n", note_count));
            }
            md.push_str(&format!("| **Total** | **{}** |\n\n", run.results.len()));

            // Results table
            md.push_str("## Findings\n\n");
            md.push_str("| File | Vulnerability | Severity | Confidence |\n");
            md.push_str("|------|---------------|----------|------------|\n");

            for result in &run.results {
                let file = result
                    .locations
                    .first()
                    .map(|l| l.physical_location.artifact_location.uri.as_str())
                    .unwrap_or("unknown");

                let confidence = result
                    .properties
                    .as_ref()
                    .and_then(|p| p.confidence)
                    .map(|c| format!("{:.0}%", c * 100.0))
                    .unwrap_or_else(|| "-".to_string());

                md.push_str(&format!(
                    "| `{}` | {} | {} | {} |\n",
                    file, result.rule_id, result.level, confidence
                ));
            }

            md.push('\n');
        }

        md
    }
}

fn create_rule_for_vuln_type(vuln_type: &VulnType) -> SarifRule {
    let (name, description, help_text, security_severity, tags) = match vuln_type {
        VulnType::SQLI => (
            "SQL Injection".to_string(),
            "Potential SQL injection vulnerability detected".to_string(),
            "SQL injection occurs when untrusted input is passed directly to SQL queries. Use parameterized queries or prepared statements.".to_string(),
            "8.5",
            vec!["security", "injection", "sql"],
        ),
        VulnType::XSS => (
            "Cross-Site Scripting".to_string(),
            "Potential XSS vulnerability detected".to_string(),
            "Cross-site scripting allows attackers to inject malicious scripts. Sanitize and validate all user input.".to_string(),
            "7.5",
            vec!["security", "injection", "xss"],
        ),
        VulnType::RCE => (
            "Remote Code Execution".to_string(),
            "Potential remote code execution vulnerability detected".to_string(),
            "Remote code execution allows attackers to execute arbitrary code. Avoid executing user input as code.".to_string(),
            "9.0",
            vec!["security", "execution", "rce"],
        ),
        VulnType::LFI => (
            "Local File Inclusion".to_string(),
            "Potential local file inclusion vulnerability detected".to_string(),
            "Local file inclusion allows reading arbitrary files. Validate and sanitize file paths.".to_string(),
            "6.5",
            vec!["security", "file", "lfi"],
        ),
        VulnType::SSRF => (
            "Server-Side Request Forgery".to_string(),
            "Potential SSRF vulnerability detected".to_string(),
            "SSRF allows attackers to make requests from the server. Validate and restrict URLs.".to_string(),
            "7.0",
            vec!["security", "network", "ssrf"],
        ),
        VulnType::AFO => (
            "Arbitrary File Operation".to_string(),
            "Potential arbitrary file operation vulnerability detected".to_string(),
            "Arbitrary file operations can lead to unauthorized file access. Validate file operations.".to_string(),
            "6.0",
            vec!["security", "file", "afo"],
        ),
        VulnType::IDOR => (
            "Insecure Direct Object Reference".to_string(),
            "Potential IDOR vulnerability detected".to_string(),
            "IDOR allows unauthorized access to objects. Implement proper authorization checks.".to_string(),
            "5.5",
            vec!["security", "authorization", "idor"],
        ),
        VulnType::Other(vuln_name) => (
            vuln_name.clone(),
            format!("Potential {} vulnerability detected", vuln_name),
            "Review the code for potential security issues.".to_string(),
            "5.0",
            vec!["security", "other"],
        ),
    };

    SarifRule {
        id: vuln_type.to_string(),
        name: name.clone(),
        short_description: Some(SarifMessage {
            text: description.clone(),
            markdown: None,
        }),
        full_description: Some(SarifMessage {
            text: description.clone(),
            markdown: Some(format!("**{}**\n\n{}", name, help_text)),
        }),
        help: Some(SarifMessage {
            text: help_text.clone(),
            markdown: Some(help_text.clone()),
        }),
        properties: Some(SarifRuleProperties {
            tags: Some(tags.into_iter().map(String::from).collect()),
            precision: Some("medium".to_string()),
            problem_severity: Some(security_severity.to_string()),
            security_severity: Some(security_severity.to_string()),
        }),
        default_configuration: Some(SarifConfiguration {
            level: if security_severity.parse::<f64>().unwrap_or(0.0) >= 8.0 {
                "error".to_string()
            } else if security_severity.parse::<f64>().unwrap_or(0.0) >= 6.0 {
                "warning".to_string()
            } else {
                "note".to_string()
            },
        }),
    }
}

fn confidence_to_level(confidence: i32) -> String {
    match confidence {
        90..=100 => "error".to_string(),
        70..=89 => "warning".to_string(),
        50..=69 => "note".to_string(),
        _ => "info".to_string(),
    }
}

fn extract_region_from_par_analysis(par_analysis: &ParAnalysis) -> Option<SarifRegion> {
    // Try to extract location information from policy violations
    for violation in &par_analysis.policy_violations {
        if let Some(region) = parse_line_number_from_text(&violation.violation_path) {
            return Some(region);
        }
    }

    None
}

fn parse_line_number_from_text(text: &str) -> Option<SarifRegion> {
    // Enhanced regex patterns for line number detection
    let patterns = [
        r"(?:line|ln)[:\s]+(\d+)", // "line: 42" or "ln 42"
        r":(\d+):(\d+)",           // ":42:10" (line:column)
        r"@(\d+)",                 // "@42" (line marker)
        r"\[(\d+)\]",              // "[42]" (line reference)
    ];

    for pattern in &patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            if let Some(captures) = regex.captures(text) {
                if let Ok(line_num) = captures[1].parse::<i32>() {
                    let column = if captures.len() > 2 {
                        captures[2].parse::<i32>().ok()
                    } else {
                        None
                    };

                    return Some(SarifRegion {
                        start_line: line_num,
                        start_column: column,
                        end_line: None,
                        end_column: None,
                        snippet: Some(SarifArtifactContent {
                            text: text.to_string(),
                        }),
                    });
                }
            }
        }
    }

    None
}

fn generate_fingerprints(file_path: &Path, response: &Response) -> HashMap<String, String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut fingerprints = HashMap::new();

    // Generate a simple fingerprint based on file path and analysis
    let mut hasher = DefaultHasher::new();
    format!("{}:{}", file_path.display(), response.analysis).hash(&mut hasher);
    let fingerprint = format!("{:x}", hasher.finish());

    fingerprints.insert("parsentry/v1".to_string(), fingerprint);

    fingerprints
}

fn guess_mime_type(file_path: &Path) -> Option<String> {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("js") => Some("application/javascript".to_string()),
        Some("ts") => Some("application/typescript".to_string()),
        Some("py") => Some("text/x-python".to_string()),
        Some("go") => Some("text/x-go".to_string()),
        Some("rs") => Some("text/x-rust".to_string()),
        Some("rb") => Some("text/x-ruby".to_string()),
        Some("java") => Some("text/x-java".to_string()),
        Some("c") => Some("text/x-c".to_string()),
        Some("cpp") | Some("cc") | Some("cxx") => Some("text/x-c++".to_string()),
        Some("tf") => Some("text/x-terraform".to_string()),
        _ => Some("text/plain".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsentry_core::{ParAnalysis, RemediationGuidance, VulnType};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_sarif_report_creation() {
        let mut summary = AnalysisSummary::new();

        let response = Response {
            scratchpad: "Test analysis".to_string(),
            analysis: "This is a test vulnerability".to_string(),
            poc: "SELECT * FROM users".to_string(),
            confidence_score: 85,
            vulnerability_types: vec![VulnType::SQLI, VulnType::XSS],
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
        };

        summary.add_result(
            PathBuf::from("test.py"),
            response,
            "test.py.md".to_string(),
        );

        let sarif = SarifReport::from_analysis_summary(&summary, "0.9.2");

        assert_eq!(sarif.version, "2.1.0");
        assert_eq!(sarif.runs.len(), 1);
        assert_eq!(sarif.runs[0].results.len(), 2); // Two vulnerabilities
    }

    #[test]
    fn test_sarif_serialization() {
        let summary = AnalysisSummary::new();
        let sarif = SarifReport::from_analysis_summary(&summary, "0.9.2");

        let json = sarif.to_json().unwrap();
        assert!(json.contains("\"version\": \"2.1.0\""));
        assert!(json.contains("Parsentry"));
    }

    #[test]
    fn test_sarif_file_export() {
        let dir = tempdir().unwrap();
        let sarif_path = dir.path().join("test.sarif");

        let summary = AnalysisSummary::new();
        let sarif = SarifReport::from_analysis_summary(&summary, "0.9.2");

        sarif.save_to_file(&sarif_path).unwrap();
        assert!(sarif_path.exists());

        let content = std::fs::read_to_string(&sarif_path).unwrap();
        assert!(content.contains("Parsentry"));
    }

    #[test]
    fn test_sarif_from_file() {
        let dir = tempdir().unwrap();
        let sarif_path = dir.path().join("test.sarif");

        let summary = AnalysisSummary::new();
        let original = SarifReport::from_analysis_summary(&summary, "0.9.2");
        original.save_to_file(&sarif_path).unwrap();

        let loaded = SarifReport::from_file(&sarif_path).unwrap();
        assert_eq!(loaded.version, "2.1.0");
        assert_eq!(loaded.runs.len(), 1);
    }

    #[test]
    fn test_sarif_from_json() {
        let json = r#"{
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [{
                "tool": {
                    "driver": {
                        "name": "Parsentry",
                        "version": "0.13.0"
                    }
                },
                "results": []
            }]
        }"#;

        let sarif = SarifReport::from_json(json).unwrap();
        assert_eq!(sarif.version, "2.1.0");
        assert_eq!(sarif.runs[0].tool.driver.name, "Parsentry");
    }

    #[test]
    fn test_sarif_to_markdown() {
        let mut summary = AnalysisSummary::new();
        let response = Response {
            scratchpad: "Analysis notes".to_string(),
            analysis: "SQL injection found".to_string(),
            poc: "'; DROP TABLE users; --".to_string(),
            confidence_score: 90,
            vulnerability_types: vec![VulnType::SQLI],
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
        };
        summary.add_result(PathBuf::from("vulnerable.py"), response, "vulnerable.py.md".to_string());

        let sarif = SarifReport::from_analysis_summary(&summary, "0.13.0");
        let markdown = sarif.to_markdown();

        assert!(markdown.contains("Security Analysis Report"));
        assert!(markdown.contains("SQLI"));
        assert!(markdown.contains("vulnerable.py"));
    }

    #[test]
    fn test_sarif_to_summary_markdown() {
        let mut summary = AnalysisSummary::new();
        let response = Response {
            scratchpad: "Notes".to_string(),
            analysis: "XSS vulnerability".to_string(),
            poc: "<script>alert(1)</script>".to_string(),
            confidence_score: 85,
            vulnerability_types: vec![VulnType::XSS],
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
        };
        summary.add_result(PathBuf::from("app.js"), response, "app.js.md".to_string());

        let sarif = SarifReport::from_analysis_summary(&summary, "0.13.0");
        let summary_md = sarif.to_summary_markdown();

        assert!(summary_md.contains("Security Analysis Summary"));
        assert!(summary_md.contains("XSS"));
        assert!(summary_md.contains("app.js"));
        assert!(summary_md.contains("| File |"));
    }
}
