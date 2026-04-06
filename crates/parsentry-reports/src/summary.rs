use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use parsentry_core::{Response, VulnType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysisResult {
    pub file_path: PathBuf,
    pub response: Response,
    pub output_filename: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        md.push_str("# Security Analysis Summary Report\n\n");

        md.push_str("## 概要\n\n");
        md.push_str("| ファイル | 脆弱性タイプ | 信頼度 |\n");
        md.push_str("|---------|------------|--------|\n");

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

                // Create display name from filename + pattern if available
                let display_name = if let Some(pattern) = &result.response.pattern_description {
                    format!(
                        "{} ({})",
                        result
                            .file_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                        pattern
                    )
                } else {
                    result
                        .file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                };

                md.push_str(&format!(
                    "| [{}]({}) | {} | {} |\n",
                    display_name, result.output_filename, vuln_types, confidence_level
                ));
            }
        }

        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsentry_core::{Response, VulnType};

    fn make_response(confidence: i32, vulns: Vec<VulnType>) -> Response {
        Response {
            analysis: "analysis".to_string(),
            confidence_score: confidence,
            vulnerability_types: vulns,
            ..Default::default()
        }
    }

    // --- sort_by_confidence ---

    #[test]
    fn test_sort_by_confidence_descending() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("low.py"), make_response(30, vec![]), "low.py.md".to_string());
        summary.add_result(PathBuf::from("high.py"), make_response(90, vec![]), "high.py.md".to_string());
        summary.add_result(PathBuf::from("mid.py"), make_response(60, vec![]), "mid.py.md".to_string());

        summary.sort_by_confidence();

        assert_eq!(summary.results[0].response.confidence_score, 90);
        assert_eq!(summary.results[1].response.confidence_score, 60);
        assert_eq!(summary.results[2].response.confidence_score, 30);
    }

    #[test]
    fn test_sort_by_confidence_already_sorted() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(90, vec![]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(50, vec![]), "b.py.md".to_string());

        summary.sort_by_confidence();

        assert_eq!(summary.results[0].response.confidence_score, 90);
        assert_eq!(summary.results[1].response.confidence_score, 50);
    }

    #[test]
    fn test_sort_by_confidence_equal_scores() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(70, vec![]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(70, vec![]), "b.py.md".to_string());

        summary.sort_by_confidence();

        assert_eq!(summary.results.len(), 2);
        assert_eq!(summary.results[0].response.confidence_score, 70);
        assert_eq!(summary.results[1].response.confidence_score, 70);
    }

    // --- filter_by_min_confidence ---

    #[test]
    fn test_filter_by_min_confidence_keeps_above() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(30, vec![]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(70, vec![]), "b.py.md".to_string());
        summary.add_result(PathBuf::from("c.py"), make_response(90, vec![]), "c.py.md".to_string());

        let filtered = summary.filter_by_min_confidence(70);
        assert_eq!(filtered.results.len(), 2);
        assert!(filtered.results.iter().all(|r| r.response.confidence_score >= 70));
    }

    #[test]
    fn test_filter_by_min_confidence_boundary() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(50, vec![]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(49, vec![]), "b.py.md".to_string());

        let filtered = summary.filter_by_min_confidence(50);
        assert_eq!(filtered.results.len(), 1);
        assert_eq!(filtered.results[0].response.confidence_score, 50);
    }

    #[test]
    fn test_filter_by_min_confidence_none_match() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(30, vec![]), "a.py.md".to_string());

        let filtered = summary.filter_by_min_confidence(90);
        assert_eq!(filtered.results.len(), 0);
    }

    #[test]
    fn test_filter_by_min_confidence_all_match() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(80, vec![]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(90, vec![]), "b.py.md".to_string());

        let filtered = summary.filter_by_min_confidence(50);
        assert_eq!(filtered.results.len(), 2);
    }

    // --- filter_by_vuln_types ---

    #[test]
    fn test_filter_by_vuln_types_single() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(80, vec![VulnType::SQLI]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(80, vec![VulnType::XSS]), "b.py.md".to_string());
        summary.add_result(PathBuf::from("c.py"), make_response(80, vec![VulnType::SQLI, VulnType::RCE]), "c.py.md".to_string());

        let filtered = summary.filter_by_vuln_types(&[VulnType::SQLI]);
        assert_eq!(filtered.results.len(), 2);
    }

    #[test]
    fn test_filter_by_vuln_types_multiple() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(80, vec![VulnType::SQLI]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(80, vec![VulnType::XSS]), "b.py.md".to_string());
        summary.add_result(PathBuf::from("c.py"), make_response(80, vec![VulnType::LFI]), "c.py.md".to_string());

        let filtered = summary.filter_by_vuln_types(&[VulnType::SQLI, VulnType::XSS]);
        assert_eq!(filtered.results.len(), 2);
    }

    #[test]
    fn test_filter_by_vuln_types_none_match() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(80, vec![VulnType::SQLI]), "a.py.md".to_string());

        let filtered = summary.filter_by_vuln_types(&[VulnType::XSS]);
        assert_eq!(filtered.results.len(), 0);
    }

    // --- to_markdown ---

    #[test]
    fn test_to_markdown_header() {
        let summary = AnalysisSummary::new();
        let md = summary.to_markdown();
        assert!(md.contains("# Security Analysis Summary Report"));
        assert!(md.contains("## 概要"));
    }

    #[test]
    fn test_to_markdown_confidence_high() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(95, vec![VulnType::RCE]), "a.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("🔴 高"));
    }

    #[test]
    fn test_to_markdown_confidence_medium_high() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(75, vec![VulnType::XSS]), "a.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("🟠 中高"));
    }

    #[test]
    fn test_to_markdown_confidence_medium() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(55, vec![VulnType::SQLI]), "a.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("🟡 中"));
    }

    #[test]
    fn test_to_markdown_confidence_medium_low() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(35, vec![VulnType::LFI]), "a.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("🟢 中低"));
    }

    #[test]
    fn test_to_markdown_confidence_low() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(10, vec![VulnType::IDOR]), "a.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("🔵 低"));
    }

    #[test]
    fn test_to_markdown_confidence_boundaries() {
        let mut s90 = AnalysisSummary::new();
        s90.add_result(PathBuf::from("a.py"), make_response(90, vec![VulnType::RCE]), "a.py.md".to_string());
        assert!(s90.to_markdown().contains("🔴 高"));

        let mut s89 = AnalysisSummary::new();
        s89.add_result(PathBuf::from("a.py"), make_response(89, vec![VulnType::XSS]), "a.py.md".to_string());
        assert!(s89.to_markdown().contains("🟠 中高"));

        let mut s70 = AnalysisSummary::new();
        s70.add_result(PathBuf::from("a.py"), make_response(70, vec![VulnType::XSS]), "a.py.md".to_string());
        assert!(s70.to_markdown().contains("🟠 中高"));

        let mut s69 = AnalysisSummary::new();
        s69.add_result(PathBuf::from("a.py"), make_response(69, vec![VulnType::SQLI]), "a.py.md".to_string());
        assert!(s69.to_markdown().contains("🟡 中"));

        let mut s50 = AnalysisSummary::new();
        s50.add_result(PathBuf::from("a.py"), make_response(50, vec![VulnType::SQLI]), "a.py.md".to_string());
        assert!(s50.to_markdown().contains("🟡 中"));

        let mut s49 = AnalysisSummary::new();
        s49.add_result(PathBuf::from("a.py"), make_response(49, vec![VulnType::LFI]), "a.py.md".to_string());
        assert!(s49.to_markdown().contains("🟢 中低"));

        let mut s30 = AnalysisSummary::new();
        s30.add_result(PathBuf::from("a.py"), make_response(30, vec![VulnType::LFI]), "a.py.md".to_string());
        assert!(s30.to_markdown().contains("🟢 中低"));

        let mut s29 = AnalysisSummary::new();
        s29.add_result(PathBuf::from("a.py"), make_response(29, vec![VulnType::IDOR]), "a.py.md".to_string());
        assert!(s29.to_markdown().contains("🔵 低"));
    }

    #[test]
    fn test_to_markdown_skips_zero_confidence() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("a.py"), make_response(0, vec![VulnType::SQLI]), "a.py.md".to_string());
        summary.add_result(PathBuf::from("b.py"), make_response(80, vec![VulnType::XSS]), "b.py.md".to_string());
        let md = summary.to_markdown();
        assert!(!md.contains("a.py"));
        assert!(md.contains("b.py"));
    }

    #[test]
    fn test_to_markdown_vuln_types_displayed() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(
            PathBuf::from("a.py"),
            make_response(80, vec![VulnType::SQLI, VulnType::XSS]),
            "a.py.md".to_string(),
        );
        let md = summary.to_markdown();
        assert!(md.contains("SQLI"));
        assert!(md.contains("XSS"));
    }

    #[test]
    fn test_to_markdown_with_pattern_description() {
        let mut summary = AnalysisSummary::new();
        let mut resp = make_response(80, vec![VulnType::SQLI]);
        resp.pattern_description = Some("SQL query construction".to_string());
        summary.add_result(PathBuf::from("routes.py"), resp, "routes.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("routes.py (SQL query construction)"));
    }

    #[test]
    fn test_to_markdown_output_filename_as_link() {
        let mut summary = AnalysisSummary::new();
        summary.add_result(PathBuf::from("app.py"), make_response(80, vec![VulnType::RCE]), "app.py.md".to_string());
        let md = summary.to_markdown();
        assert!(md.contains("[app.py](app.py.md)"));
    }

    // --- add_result ---

    #[test]
    fn test_add_result_increments_count() {
        let mut summary = AnalysisSummary::new();
        assert_eq!(summary.results.len(), 0);
        summary.add_result(PathBuf::from("a.py"), make_response(50, vec![]), "a.py.md".to_string());
        assert_eq!(summary.results.len(), 1);
        summary.add_result(PathBuf::from("b.py"), make_response(60, vec![]), "b.py.md".to_string());
        assert_eq!(summary.results.len(), 2);
    }
}
