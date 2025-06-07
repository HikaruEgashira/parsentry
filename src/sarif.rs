use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::response::{AnalysisSummary, Response, VulnType};

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
    pub fn from_analysis_summary(summary: &AnalysisSummary) -> Self {
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
                        text: format!("{}: {}", vuln_type.to_string(), response.analysis),
                        markdown: Some(response.analysis.clone()),
                    },
                    locations: vec![SarifLocation {
                        physical_location: SarifPhysicalLocation {
                            artifact_location: SarifArtifactLocation {
                                uri: file_path.to_string_lossy().to_string(),
                                index: Some(artifact_index),
                            },
                            region: extract_region_from_context(&response.context_code),
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
                        version: env!("CARGO_PKG_VERSION").to_string(),
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

fn extract_region_from_context(context_code: &[crate::response::ContextCode]) -> Option<SarifRegion> {
    // Use line number from context if available
    for context in context_code {
        if let Some(line_num) = context.line_number {
            return Some(SarifRegion {
                start_line: line_num,
                start_column: context.column_number,
                end_line: None,
                end_column: None,
                snippet: Some(SarifArtifactContent {
                    text: context.code_line.clone(),
                }),
            });
        }
    }
    
    // Fallback to parsing location strings for line numbers
    for context in context_code {
        if let Some(region) = parse_line_number_from_text(&context.code_line) {
            return Some(region);
        }
    }
    
    None
}

fn parse_line_number_from_text(text: &str) -> Option<SarifRegion> {
    // Enhanced regex patterns for line number detection
    let patterns = [
        r"(?:line|ln)[:\s]+(\d+)",  // "line: 42" or "ln 42"
        r":(\d+):(\d+)",            // ":42:10" (line:column)
        r"@(\d+)",                  // "@42" (line marker)
        r"\[(\d+)\]",               // "[42]" (line reference)
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

    let mut hasher = DefaultHasher::new();
    format!("{}:{}", file_path.display(), response.analysis).hash(&mut hasher);
    let fingerprint = format!("{:x}", hasher.finish());

    [
        ("parsentry/v1".to_string(), fingerprint),
        ("vulnerability/type".to_string(), response.vulnerability_types.first().unwrap_or(&VulnType::Other("unknown".to_string())).to_string()),
    ]
    .into_iter()
    .collect()
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
    use crate::response::{Response, VulnType};
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
            context_code: vec![crate::response::ContextCode {
                name: "test_function".to_string(),
                reason: "Contains SQL query".to_string(),
                code_line: "SELECT * FROM users WHERE id = ?".to_string(),
                path: "test.py".to_string(),
                line_number: Some(42),
                column_number: Some(10),
            }],
        };
        
        summary.add_result(PathBuf::from("test.py"), response);
        
        let sarif = SarifReport::from_analysis_summary(&summary);
        
        assert_eq!(sarif.version, "2.1.0");
        assert_eq!(sarif.runs.len(), 1);
        assert_eq!(sarif.runs[0].results.len(), 2); // Two vulnerabilities
    }

    #[test]
    fn test_sarif_serialization() {
        let summary = AnalysisSummary::new();
        let sarif = SarifReport::from_analysis_summary(&summary);
        
        let json = sarif.to_json().unwrap();
        assert!(json.contains("\"version\": \"2.1.0\""));
        assert!(json.contains("Parsentry"));
    }

    #[test]
    fn test_sarif_file_export() {
        let dir = tempdir().unwrap();
        let sarif_path = dir.path().join("test.sarif");
        
        let summary = AnalysisSummary::new();
        let sarif = SarifReport::from_analysis_summary(&summary);
        
        sarif.save_to_file(&sarif_path).unwrap();
        assert!(sarif_path.exists());
        
        let content = std::fs::read_to_string(&sarif_path).unwrap();
        assert!(content.contains("Parsentry"));
    }

    #[test]
    fn test_vulnerability_mappings() {
        // Test CWE mappings
        assert_eq!(VulnType::SQLI.cwe_ids(), vec!["CWE-89"]);
        assert_eq!(VulnType::XSS.cwe_ids(), vec!["CWE-79", "CWE-80"]);
        assert_eq!(VulnType::RCE.cwe_ids(), vec!["CWE-77", "CWE-78", "CWE-94"]);
        
        // Test MITRE ATT&CK mappings
        assert_eq!(VulnType::SQLI.mitre_attack_ids(), vec!["T1190"]);
        assert_eq!(VulnType::XSS.mitre_attack_ids(), vec!["T1190", "T1185"]);
        assert_eq!(VulnType::RCE.mitre_attack_ids(), vec!["T1190", "T1059"]);
        
        // Test OWASP mappings
        assert_eq!(VulnType::SQLI.owasp_categories(), vec!["A03:2021-Injection"]);
        assert_eq!(VulnType::SSRF.owasp_categories(), vec!["A10:2021-Server-Side Request Forgery"]);
        assert_eq!(VulnType::IDOR.owasp_categories(), vec!["A01:2021-Broken Access Control"]);
    }

    #[test]
    fn test_region_extraction_from_context() {
        let context_with_line = vec![crate::response::ContextCode {
            name: "test_func".to_string(),
            reason: "test".to_string(),
            code_line: "vulnerable code".to_string(),
            path: "test.py".to_string(),
            line_number: Some(42),
            column_number: Some(10),
        }];
        
        let region = extract_region_from_context(&context_with_line);
        assert!(region.is_some());
        
        let region = region.unwrap();
        assert_eq!(region.start_line, 42);
        assert_eq!(region.start_column, Some(10));
        assert!(region.snippet.is_some());
    }

    #[test]
    fn test_parse_line_number_from_text() {
        // Test line:column format
        let region = parse_line_number_from_text("error at :42:10");
        assert!(region.is_some());
        let region = region.unwrap();
        assert_eq!(region.start_line, 42);
        assert_eq!(region.start_column, Some(10));
        
        // Test line marker format
        let region = parse_line_number_from_text("function @25 is vulnerable");
        assert!(region.is_some());
        let region = region.unwrap();
        assert_eq!(region.start_line, 25);
        assert_eq!(region.start_column, None);
        
        // Test line reference format
        let region = parse_line_number_from_text("vulnerability found [100]");
        assert!(region.is_some());
        let region = region.unwrap();
        assert_eq!(region.start_line, 100);
    }

    #[test]
    fn test_sarif_with_enhanced_properties() {
        let mut summary = AnalysisSummary::new();
        
        let response = Response {
            scratchpad: "Enhanced test".to_string(),
            analysis: "SQL injection vulnerability found".to_string(),
            poc: "SELECT * FROM users WHERE id = ? -- user_input injection".to_string(),
            confidence_score: 95,
            vulnerability_types: vec![VulnType::SQLI],
            context_code: vec![crate::response::ContextCode {
                name: "get_user".to_string(),
                reason: "Direct string concatenation in SQL query".to_string(),
                code_line: "query = \"SELECT * FROM users WHERE id = '\" + user_id + \"'\"".to_string(),
                path: "user_service.py".to_string(),
                line_number: Some(156),
                column_number: Some(8),
            }],
        };
        
        summary.add_result(PathBuf::from("user_service.py"), response);
        let sarif = SarifReport::from_analysis_summary(&summary);
        
        // Verify SARIF structure
        assert_eq!(sarif.runs.len(), 1);
        assert_eq!(sarif.runs[0].results.len(), 1);
        
        let result = &sarif.runs[0].results[0];
        
        // Verify properties include proper mappings
        assert!(result.properties.is_some());
        let props = result.properties.as_ref().unwrap();
        
        assert!(props.cwe.is_some());
        assert_eq!(props.cwe.as_ref().unwrap(), &vec!["CWE-89"]);
        
        assert!(props.mitre_attack.is_some());
        assert_eq!(props.mitre_attack.as_ref().unwrap(), &vec!["T1190"]);
        
        assert!(props.owasp.is_some());
        assert_eq!(props.owasp.as_ref().unwrap(), &vec!["A03:2021-Injection"]);
        
        // Verify region information
        assert!(result.locations[0].physical_location.region.is_some());
        let region = result.locations[0].physical_location.region.as_ref().unwrap();
        assert_eq!(region.start_line, 156);
        assert_eq!(region.start_column, Some(8));
        assert!(region.snippet.is_some());
    }
}
