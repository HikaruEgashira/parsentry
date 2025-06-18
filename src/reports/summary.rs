use std::collections::HashMap;
use std::path::PathBuf;

use crate::response::{Response, VulnType};

#[derive(Debug, Clone)]
pub struct FileAnalysisResult {
    pub file_path: PathBuf,
    pub response: Response,
    pub output_filename: String, // The generated filename used for the actual markdown file
}

#[derive(Debug, Clone, Default)]
pub struct AnalysisSummary {
    pub results: Vec<FileAnalysisResult>,
}

impl AnalysisSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, file_path: PathBuf, response: Response, output_filename: String) {
        self.results.push(FileAnalysisResult {
            file_path,
            response,
            output_filename,
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

                // Create display name from filename + pattern if available
                let display_name = if let Some(pattern) = &result.response.pattern_description {
                    format!("{} ({})", 
                        result.file_path.file_name().unwrap_or_default().to_string_lossy(),
                        pattern)
                } else {
                    result.file_path.file_name().unwrap_or_default().to_string_lossy().to_string()
                };

                md.push_str(&format!(
                    "| [{}]({}) | {} | {} | {} |\n",
                    display_name,
                    result.output_filename,
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
                *violation_count
                    .entry(violation.rule_id.clone())
                    .or_insert(0) += 1;
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