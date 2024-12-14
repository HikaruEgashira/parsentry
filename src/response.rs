use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextCode {
    pub name: String,
    pub reason: String,
    pub code_line: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub scratchpad: String,
    pub analysis: String,
    pub poc: String,
    pub confidence_score: i32,
    pub vulnerability_types: Vec<VulnType>,
    pub context_code: Vec<ContextCode>,
}

impl Response {
    pub fn print_readable(&self) {
        println!("\n🔍 Analysis Report");
        println!("{}", "=".repeat(80));

        if !self.scratchpad.is_empty() {
            println!("\n📝 Scratchpad:");
            for line in self.scratchpad.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line.trim());
                }
            }
            println!("{}", "-".repeat(80));
        }

        if !self.analysis.is_empty() {
            println!("\n🔎 Detailed Analysis:");
            for line in self.analysis.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line.trim());
                }
            }
            println!("{}", "-".repeat(80));
        }

        if !self.poc.is_empty() {
            println!("\n🧪 Proof of Concept:");
            for line in self.poc.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line.trim());
                }
            }
            println!("{}", "-".repeat(80));
        }

        println!("\n📊 Confidence Score: {}%", self.confidence_score);
        println!("{}", "-".repeat(80));

        if !self.vulnerability_types.is_empty() {
            println!("\n⚠️  Vulnerability Types:");
            for vuln_type in &self.vulnerability_types {
                println!("  • {:?}", vuln_type);
            }
            println!("{}", "-".repeat(80));
        }

        if !self.context_code.is_empty() {
            println!("\n💻 Context Code:");
            for context in &self.context_code {
                println!("\n  📌 {}", context.name);
                println!("  🔍 Reason: {}", context.reason);
                println!("  📄 Code: {}", context.code_line);
            }
            println!("{}", "-".repeat(80));
        }

        println!(); // Add final newline for better spacing
    }
}
