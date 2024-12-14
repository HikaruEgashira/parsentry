use crate::response::{Response, VulnType};
use crate::llm::LLM;
use crate::prompts::EVALUATOR_PROMPT_TEMPLATE;
use anyhow::{Error, Result};
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
struct LLMEvaluation {
    score: f32,
    correct_vulns: Vec<String>,
    missed_vulns: Vec<String>,
    false_positives: Vec<String>,
    feedback: String,
}

pub async fn evaluate_python_vulnerable_app(response: &Response, llm: &dyn LLM) -> Result<EvaluationResult, Error> {
    // Format the report for evaluation
    let report = format!(
        "Identified Vulnerabilities: {:?}\n\nAnalysis:\n{}\n\nProof of Concept:\n{}",
        response.vulnerability_types,
        response.analysis,
        response.poc
    );

    // Get LLM evaluation
    let prompt = EVALUATOR_PROMPT_TEMPLATE.replace("{report}", &report);
    let eval_response = llm.chat(&prompt).await?;
    
    // Parse LLM response as JSON
    let eval: LLMEvaluation = serde_json::from_str(&eval_response)?;
    
    // Convert string vulnerability types to VulnType enum
    let correct_vulns = eval.correct_vulns.iter()
        .map(|v| match v.as_str() {
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "RCE" => VulnType::RCE,
            _ => VulnType::Other(v.clone()),
        })
        .collect();

    let missed_vulns = eval.missed_vulns.iter()
        .map(|v| match v.as_str() {
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "RCE" => VulnType::RCE,
            _ => VulnType::Other(v.clone()),
        })
        .collect();

    let false_positives = eval.false_positives.iter()
        .map(|v| match v.as_str() {
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "RCE" => VulnType::RCE,
            _ => VulnType::Other(v.clone()),
        })
        .collect();

    Ok(EvaluationResult {
        score: eval.score,
        feedback: eval.feedback,
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
    use async_trait::async_trait;
    use tokio;

    struct MockLLM;

    #[async_trait]
    impl LLM for MockLLM {
        async fn chat(&self, _prompt: &str) -> Result<String> {
            Ok(r#"{
                "score": 85.0,
                "correct_vulns": ["SQLI", "XSS"],
                "missed_vulns": ["RCE"],
                "false_positives": ["SSRF"],
                "feedback": "Good analysis but missed some vulnerabilities"
            }"#.to_string())
        }
    }

    #[tokio::test]
    async fn test_evaluation_perfect_report() {
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

        let llm = MockLLM;
        let result = evaluate_python_vulnerable_app(&response, &llm).await.unwrap();
        
        assert_eq!(result.score, 85.0);
        assert_eq!(result.correct_vulns_found.len(), 2);
        assert_eq!(result.missed_vulns.len(), 1);
        assert_eq!(result.false_positives.len(), 1);
        assert!(!result.feedback.is_empty());
    }
}
