//! Re-exports from parsentry-core and extensions for the main crate.

pub use parsentry_core::{
    response_json_schema, Language as CoreLanguage, Response, VulnType,
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
        println!("\n  Security Analysis Report");
        println!("{}", "=".repeat(80));

        let confidence_icon = match self.confidence_score {
            90..=100 => "critical",
            70..=89 => "high",
            50..=69 => "medium",
            30..=49 => "low",
            _ => "info",
        };
        println!(
            "\n  Confidence: {} ({})",
            self.confidence_score, confidence_icon
        );

        if !self.vulnerability_types.is_empty() {
            println!("\n  Vulnerability types:");
            for vuln_type in &self.vulnerability_types {
                println!("  - {:?}", vuln_type);
            }
        }

        println!("\n  Analysis:");
        println!("{}", "-".repeat(80));
        println!("{}", self.analysis);

        if !self.poc.is_empty() {
            println!("\n  PoC:");
            println!("{}", "-".repeat(80));
            println!("{}", self.poc);
        }

        if !self.scratchpad.is_empty() {
            println!("\n  Notes:");
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

    let mut response = Response {
        scratchpad: cc_response.scratchpad,
        analysis: cc_response.analysis,
        poc: cc_response.poc,
        confidence_score: Response::normalize_confidence_score(cc_response.confidence_score),
        vulnerability_types,
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
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    }
}
