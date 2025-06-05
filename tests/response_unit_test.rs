use std::path::PathBuf;
use vulnhuntrs::response::{Response, VulnType, ContextCode, AnalysisSummary, response_json_schema};
use serde_json::{json, Value};

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
    assert_eq!(VulnType::Other("test".to_string()), VulnType::Other("test".to_string()));
    assert_ne!(VulnType::LFI, VulnType::RCE);
    assert_ne!(VulnType::Other("test1".to_string()), VulnType::Other("test2".to_string()));
}

#[test]
fn test_context_code_creation() {
    let context = ContextCode {
        name: "vulnerable_function".to_string(),
        reason: "Uses unsafe eval() function".to_string(),
        code_line: "eval(user_input)".to_string(),
        path: "/src/vulnerable.py".to_string(),
    };
    
    assert_eq!(context.name, "vulnerable_function");
    assert_eq!(context.reason, "Uses unsafe eval() function");
    assert_eq!(context.code_line, "eval(user_input)");
    assert_eq!(context.path, "/src/vulnerable.py");
}

#[test]
fn test_response_creation() {
    let response = Response {
        scratchpad: "Analysis notes".to_string(),
        analysis: "Found RCE vulnerability".to_string(),
        poc: "curl -X POST -d 'cmd=ls' /vulnerable-endpoint".to_string(),
        confidence_score: 9,
        vulnerability_types: vec![VulnType::RCE],
        context_code: vec![ContextCode {
            name: "process_command".to_string(),
            reason: "Direct command execution".to_string(),
            code_line: "os.system(command)".to_string(),
            path: "/src/handlers.py".to_string(),
        }],
    };
    
    assert_eq!(response.confidence_score, 9);
    assert_eq!(response.vulnerability_types.len(), 1);
    assert_eq!(response.context_code.len(), 1);
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
        context_code: vec![],
    };
    
    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: Response = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(response.confidence_score, deserialized.confidence_score);
    assert_eq!(response.vulnerability_types, deserialized.vulnerability_types);
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
    assert!(vuln_types["items"]["enum"].as_array().unwrap().contains(&json!("RCE")));
    assert!(vuln_types["items"]["enum"].as_array().unwrap().contains(&json!("SQLI")));
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
        context_code: vec![ContextCode {
            name: "test_function".to_string(),
            reason: "Test reason".to_string(),
            code_line: "test_code()".to_string(),
            path: "/test/path.py".to_string(),
        }],
    };
    
    let markdown = response.to_markdown();
    
    // Verify markdown contains expected sections
    assert!(markdown.contains("# 解析レポート"));
    assert!(markdown.contains("信頼度スコア: 8"));
    assert!(markdown.contains("## 脆弱性タイプ"));
    assert!(markdown.contains("RCE"));
    assert!(markdown.contains("SQLI"));
    assert!(markdown.contains("## 解析結果"));
    assert!(markdown.contains("This is a test analysis"));
    assert!(markdown.contains("## PoC（概念実証コード）"));
    assert!(markdown.contains("echo 'test'"));
    assert!(markdown.contains("## 関連コードコンテキスト"));
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
            context_code: vec![],
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
        context_code: vec![],
    };
    
    assert!(response.scratchpad.is_empty());
    assert!(response.analysis.is_empty());
    assert!(response.poc.is_empty());
    assert_eq!(response.confidence_score, 0);
    assert!(response.vulnerability_types.is_empty());
    assert!(response.context_code.is_empty());
}

#[test]
fn test_context_code_serialization() {
    let context = ContextCode {
        name: "test_func".to_string(),
        reason: "test reason".to_string(),
        code_line: "print('test')".to_string(),
        path: "/path/to/file.py".to_string(),
    };
    
    let serialized = serde_json::to_string(&context).unwrap();
    let deserialized: ContextCode = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(context.name, deserialized.name);
    assert_eq!(context.reason, deserialized.reason);
    assert_eq!(context.code_line, deserialized.code_line);
    assert_eq!(context.path, deserialized.path);
}

#[test]
fn test_response_with_multiple_context_codes() {
    let response = Response {
        scratchpad: "Multiple contexts".to_string(),
        analysis: "Analysis with multiple context codes".to_string(),
        poc: "PoC code".to_string(),
        confidence_score: 6,
        vulnerability_types: vec![VulnType::XSS],
        context_code: vec![
            ContextCode {
                name: "func1".to_string(),
                reason: "reason1".to_string(),
                code_line: "code1".to_string(),
                path: "/path1.py".to_string(),
            },
            ContextCode {
                name: "func2".to_string(),
                reason: "reason2".to_string(),
                code_line: "code2".to_string(),
                path: "/path2.py".to_string(),
            },
        ],
    };
    
    assert_eq!(response.context_code.len(), 2);
    assert_eq!(response.context_code[0].name, "func1");
    assert_eq!(response.context_code[1].name, "func2");
}
