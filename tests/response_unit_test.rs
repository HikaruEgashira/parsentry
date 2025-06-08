use serde_json::json;
use parsentry::response::{
    AnalysisSummary, Response, VulnType, response_json_schema, ParAnalysis, RemediationGuidance,
};

#[test]
fn test_vuln_type_serialization() {
    let vuln_types = vec![
        VulnType::LFI,
        VulnType::RCE,
        VulnType::SSRF,
        VulnType::AFO,
        VulnType::SQLI,
        VulnType::XSS,
        VulnType::IDOR,
        VulnType::Other("Custom".to_string()),
    ];

    let serialized = serde_json::to_string(&vuln_types).unwrap();
    let deserialized: Vec<VulnType> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(vuln_types, deserialized);
}

#[test]
fn test_vuln_type_equality() {
    assert_eq!(VulnType::LFI, VulnType::LFI);
    assert_eq!(
        VulnType::Other("test".to_string()),
        VulnType::Other("test".to_string())
    );
    assert_ne!(VulnType::LFI, VulnType::RCE);
    assert_ne!(
        VulnType::Other("test1".to_string()),
        VulnType::Other("test2".to_string())
    );
}

// Note: ContextCode struct no longer exists - test removed
// #[test]
// fn test_context_code_creation() {
//     // ContextCode struct has been removed from the Response struct
// }

#[test]
fn test_response_creation() {
    let response = Response {
        scratchpad: "Analysis notes".to_string(),
        analysis: "Found RCE vulnerability".to_string(),
        poc: "curl -X POST -d 'cmd=ls' /vulnerable-endpoint".to_string(),
        confidence_score: 9,
        vulnerability_types: vec![VulnType::RCE],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
    };

    assert_eq!(response.confidence_score, 9);
    assert_eq!(response.vulnerability_types.len(), 1);
    // Note: context_code field no longer exists
    assert!(response.analysis.contains("RCE"));
}

#[test]
fn test_response_serialization() {
    let response = Response {
        scratchpad: "Test scratchpad".to_string(),
        analysis: "Test analysis".to_string(),
        poc: "Test PoC".to_string(),
        confidence_score: 7,
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
    };

    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: Response = serde_json::from_str(&serialized).unwrap();

    assert_eq!(response.confidence_score, deserialized.confidence_score);
    assert_eq!(
        response.vulnerability_types,
        deserialized.vulnerability_types
    );
}

#[test]
fn test_response_json_schema() {
    let schema = response_json_schema();

    // Verify schema structure
    assert_eq!(schema["type"], "object");

    let properties = &schema["properties"];
    assert!(properties["scratchpad"]["type"] == "string");
    assert!(properties["analysis"]["type"] == "string");
    assert!(properties["poc"]["type"] == "string");
    assert!(properties["confidence_score"]["type"] == "integer");

    // Check vulnerability types array schema
    let vuln_types = &properties["vulnerability_types"];
    assert_eq!(vuln_types["type"], "array");
    assert!(
        vuln_types["items"]["enum"]
            .as_array()
            .unwrap()
            .contains(&json!("RCE"))
    );
    assert!(
        vuln_types["items"]["enum"]
            .as_array()
            .unwrap()
            .contains(&json!("SQLI"))
    );
}

#[test]
fn test_analysis_summary_default() {
    let summary = AnalysisSummary::default();
    assert_eq!(summary.results.len(), 0);
}

#[test]
fn test_analysis_summary_new() {
    let summary = AnalysisSummary::new();
    assert_eq!(summary.results.len(), 0);
}

#[test]
fn test_markdown_generation() {
    let response = Response {
        scratchpad: "Test scratchpad".to_string(),
        analysis: "This is a test analysis with **bold** text".to_string(),
        poc: "echo 'test'".to_string(),
        confidence_score: 8,
        vulnerability_types: vec![VulnType::RCE, VulnType::SQLI],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
    };

    let markdown = response.to_markdown();

    // Verify markdown contains expected sections
    assert!(markdown.contains("# PAR Security Analysis Report"));
    assert!(markdown.contains("信頼度スコア: 8"));
    assert!(markdown.contains("## 脆弱性タイプ"));
    assert!(markdown.contains("RCE"));
    assert!(markdown.contains("SQLI"));
    assert!(markdown.contains("## 詳細解析"));
    assert!(markdown.contains("This is a test analysis"));
    assert!(markdown.contains("## PoC（概念実証コード）"));
    assert!(markdown.contains("echo 'test'"));
    // Note: context_code related sections no longer exist
}

#[test]
fn test_confidence_score_validation() {
    // Test various confidence scores
    let scores = [0, 1, 5, 10, -1, 15];

    for score in scores {
        let response = Response {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: score,
            vulnerability_types: vec![],
            par_analysis: ParAnalysis {
                principals: vec![],
                actions: vec![],
                resources: vec![],
                policy_violations: vec![],
            },
            remediation_guidance: RemediationGuidance {
                policy_enforcement: vec![],
            },
        };

        // Confidence score should be stored as-is (validation is handled elsewhere)
        assert_eq!(response.confidence_score, score);
    }
}

#[test]
fn test_empty_response() {
    let response = Response {
        scratchpad: String::new(),
        analysis: String::new(),
        poc: String::new(),
        confidence_score: 0,
        vulnerability_types: vec![],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
    };

    assert!(response.scratchpad.is_empty());
    assert!(response.analysis.is_empty());
    assert!(response.poc.is_empty());
    assert_eq!(response.confidence_score, 0);
    assert!(response.vulnerability_types.is_empty());
    // Note: context_code field no longer exists
}

// Note: ContextCode struct no longer exists - test removed
// #[test]
// fn test_context_code_serialization() {
//     // ContextCode struct has been removed from the Response struct
// }

// Note: ContextCode struct no longer exists - test removed
// #[test]
// fn test_response_with_multiple_context_codes() {
//     // ContextCode struct has been removed from the Response struct
//     // This test functionality has been replaced by PAR analysis
// }
