use crate::response::{Response, VulnType};
use anyhow::{Error, Result};

#[derive(Debug)]
pub struct EvaluationResult {
    pub score: f32,
    pub feedback: String,
    pub correct_vulns_found: Vec<VulnType>,
    pub missed_vulns: Vec<VulnType>,
    pub false_positives: Vec<VulnType>,
}

impl EvaluationResult {
    pub fn print_readable(&self) {
        println!("\n📋 Evaluation Report");
        println!("{}", "=".repeat(80));

        println!("\n🎯 Overall Score: {:.1}%", self.score);
        println!("{}", "-".repeat(80));

        println!("\n✅ Correctly Identified Vulnerabilities:");
        for vuln in &self.correct_vulns_found {
            println!("  • {:?}", vuln);
        }
        println!("{}", "-".repeat(80));

        if !self.missed_vulns.is_empty() {
            println!("\n❌ Missed Vulnerabilities:");
            for vuln in &self.missed_vulns {
                println!("  • {:?}", vuln);
            }
            println!("{}", "-".repeat(80));
        }

        if !self.false_positives.is_empty() {
            println!("\n⚠️  False Positives:");
            for vuln in &self.false_positives {
                println!("  • {:?}", vuln);
            }
            println!("{}", "-".repeat(80));
        }

        println!("\n💭 Feedback:");
        for line in self.feedback.lines() {
            if !line.trim().is_empty() {
                println!("  {}", line.trim());
            }
        }
        println!("{}", "-".repeat(80));

        println!(); // Add final newline for better spacing
    }
}

pub fn evaluate_python_vulnerable_app(response: &Response) -> Result<EvaluationResult, Error> {
    // Known vulnerabilities in the Python example app
    let known_vulns = vec![VulnType::SQLI, VulnType::XSS, VulnType::RCE];
    
    let mut correct_vulns = Vec::new();
    let mut false_positives = Vec::new();
    let mut feedback = String::new();
    
    // Check found vulnerabilities
    for vuln in &response.vulnerability_types {
        if known_vulns.contains(vuln) {
            correct_vulns.push(vuln.clone());
        } else {
            false_positives.push(vuln.clone());
        }
    }

    // Calculate missed vulnerabilities
    let mut missed_vulns = Vec::new();
    for vuln in &known_vulns {
        if !correct_vulns.contains(vuln) {
            missed_vulns.push(vuln.clone());
        }
    }

    // Calculate base score based on correct findings vs total vulnerabilities
    let base_score = (correct_vulns.len() as f32 / known_vulns.len() as f32) * 100.0;
    
    // Penalize for false positives
    let false_positive_penalty = (false_positives.len() as f32 * 10.0);
    let mut final_score = base_score - false_positive_penalty;
    if final_score < 0.0 {
        final_score = 0.0;
    }

    // Generate feedback
    feedback.push_str(&format!(
        "Found {} out of {} known vulnerabilities.\n",
        correct_vulns.len(),
        known_vulns.len()
    ));

    if !false_positives.is_empty() {
        feedback.push_str(&format!(
            "Reported {} false positive(s). Each false positive results in a 10% score penalty.\n",
            false_positives.len()
        ));
    }

    // Evaluate quality of analysis
    if !response.analysis.is_empty() {
        let analysis_quality = evaluate_analysis_quality(response);
        feedback.push_str(&format!("\nAnalysis Quality:\n{}", analysis_quality));
    }

    // Evaluate PoC quality
    if !response.poc.is_empty() {
        let poc_quality = evaluate_poc_quality(response);
        feedback.push_str(&format!("\nPoC Quality:\n{}", poc_quality));
    }

    Ok(EvaluationResult {
        score: final_score,
        feedback,
        correct_vulns_found: correct_vulns,
        missed_vulns,
        false_positives,
    })
}

fn evaluate_analysis_quality(response: &Response) -> String {
    let mut feedback = String::new();
    
    // Check if analysis includes key elements
    if response.analysis.contains("impact") || response.analysis.contains("Impact") {
        feedback.push_str("✓ Analysis includes impact assessment\n");
    } else {
        feedback.push_str("✗ Analysis should include impact assessment\n");
    }

    if response.analysis.contains("mitigat") {
        feedback.push_str("✓ Analysis includes mitigation suggestions\n");
    } else {
        feedback.push_str("✗ Analysis should include mitigation suggestions\n");
    }

    if response.analysis.contains("root cause") || response.analysis.contains("caused by") {
        feedback.push_str("✓ Analysis includes root cause explanation\n");
    } else {
        feedback.push_str("✗ Analysis should include root cause explanation\n");
    }

    feedback
}

fn evaluate_poc_quality(response: &Response) -> String {
    let mut feedback = String::new();
    
    // Check if PoC includes key elements
    if response.poc.contains("curl") || response.poc.contains("http") {
        feedback.push_str("✓ PoC includes concrete example request\n");
    } else {
        feedback.push_str("✗ PoC should include concrete example request\n");
    }

    if response.poc.contains("Expected result") || response.poc.contains("expected output") {
        feedback.push_str("✓ PoC includes expected results\n");
    } else {
        feedback.push_str("✗ PoC should include expected results\n");
    }

    if response.poc.contains("Steps") || response.poc.contains("1.") {
        feedback.push_str("✓ PoC includes step-by-step instructions\n");
    } else {
        feedback.push_str("✗ PoC should include step-by-step instructions\n");
    }

    feedback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluation_perfect_report() {
        let response = Response {
            scratchpad: String::from("Analysis notes..."),
            analysis: String::from(
                "Impact: Critical. Root cause: Unsanitized input. Mitigation: Use parameterized queries."
            ),
            poc: String::from(
                "Steps:\n1. Send curl request\nExpected result: SQL injection successful"
            ),
            confidence_score: 95,
            vulnerability_types: vec![VulnType::SQLI, VulnType::XSS, VulnType::RCE],
            context_code: vec![],
        };

        let result = evaluate_python_vulnerable_app(&response).unwrap();
        assert_eq!(result.score, 100.0);
        assert_eq!(result.correct_vulns_found.len(), 3);
        assert_eq!(result.missed_vulns.len(), 0);
        assert_eq!(result.false_positives.len(), 0);
    }

    #[test]
    fn test_evaluation_missing_vulns() {
        let response = Response {
            scratchpad: String::new(),
            analysis: String::from("Basic analysis"),
            poc: String::new(),
            confidence_score: 80,
            vulnerability_types: vec![VulnType::SQLI],
            context_code: vec![],
        };

        let result = evaluate_python_vulnerable_app(&response).unwrap();
        assert_eq!(result.score, 33.333336); // Found 1 out of 3
        assert_eq!(result.correct_vulns_found.len(), 1);
        assert_eq!(result.missed_vulns.len(), 2);
    }

    #[test]
    fn test_evaluation_false_positives() {
        let response = Response {
            scratchpad: String::new(),
            analysis: String::from("Basic analysis"),
            poc: String::new(),
            confidence_score: 80,
            vulnerability_types: vec![VulnType::SQLI, VulnType::SSRF],
            context_code: vec![],
        };

        let result = evaluate_python_vulnerable_app(&response).unwrap();
        assert!(result.score < 33.333336); // Base score for 1 correct - penalty for false positive
        assert_eq!(result.false_positives.len(), 1);
    }
}