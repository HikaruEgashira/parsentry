//! Re-exports from parsentry-core and extensions for the main crate.

use std::io::{self, Write};

pub use parsentry_core::{Language as CoreLanguage, Response, VulnType, response_json_schema};

fn write_stdout(content: &str) {
    let mut out = io::stdout().lock();
    let _ = out.write_all(content.as_bytes()).and_then(|()| out.flush());
}

/// Extension trait for Response to add methods specific to the main parsentry crate.
pub trait ResponseExt {
    /// Print a human-readable report to stdout.
    fn print_readable(&self);
    /// Convert to markdown format.
    fn to_markdown(&self) -> String;
}

impl ResponseExt for Response {
    fn print_readable(&self) {
        let mut buf = String::new();
        buf.push_str("\n  Security Analysis Report\n");
        buf.push_str(&"=".repeat(80));
        buf.push('\n');

        let confidence_icon = match self.confidence_score {
            90..=100 => "critical",
            70..=89 => "high",
            50..=69 => "medium",
            30..=49 => "low",
            _ => "info",
        };
        buf.push_str(&format!(
            "\n  Confidence: {} ({})\n",
            self.confidence_score, confidence_icon
        ));

        if !self.vulnerability_types.is_empty() {
            buf.push_str("\n  Vulnerability types:\n");
            for vuln_type in &self.vulnerability_types {
                buf.push_str(&format!("  - {:?}\n", vuln_type));
            }
        }

        buf.push_str("\n  Analysis:\n");
        buf.push_str(&"-".repeat(80));
        buf.push('\n');
        buf.push_str(&self.analysis);
        buf.push('\n');

        if !self.poc.is_empty() {
            buf.push_str("\n  PoC:\n");
            buf.push_str(&"-".repeat(80));
            buf.push('\n');
            buf.push_str(&self.poc);
            buf.push('\n');
        }

        if !self.scratchpad.is_empty() {
            buf.push_str("\n  Notes:\n");
            buf.push_str(&"-".repeat(80));
            buf.push('\n');
            buf.push_str(&self.scratchpad);
            buf.push('\n');
        }

        buf.push('\n');
        write_stdout(&buf);
    }

    fn to_markdown(&self) -> String {
        parsentry_reports::to_markdown(self)
    }
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
        ..Default::default()
    }
}
