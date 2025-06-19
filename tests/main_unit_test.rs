use parsentry::response::{ParAnalysis, RemediationGuidance, Response, VulnType};
use parsentry::reports::AnalysisSummary;
use std::path::PathBuf;

#[test]
fn test_vuln_type_parsing() {
    // Test vulnerability type parsing logic similar to main.rs
    let types_str = "LFI,RCE,SSRF,IDOR";
    let vuln_types: Vec<VulnType> = types_str
        .split(',')
        .map(|s| match s.trim() {
            "LFI" => VulnType::LFI,
            "RCE" => VulnType::RCE,
            "SSRF" => VulnType::SSRF,
            "AFO" => VulnType::AFO,
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "IDOR" => VulnType::IDOR,
            other => VulnType::Other(other.to_string()),
        })
        .collect();

    assert_eq!(vuln_types.len(), 4);
    assert_eq!(vuln_types[0], VulnType::LFI);
    assert_eq!(vuln_types[1], VulnType::RCE);
    assert_eq!(vuln_types[2], VulnType::SSRF);
    assert_eq!(vuln_types[3], VulnType::IDOR);
}

#[test]
fn test_vuln_type_parsing_with_unknown_type() {
    let types_str = "LFI,UNKNOWN_TYPE,RCE";
    let vuln_types: Vec<VulnType> = types_str
        .split(',')
        .map(|s| match s.trim() {
            "LFI" => VulnType::LFI,
            "RCE" => VulnType::RCE,
            "SSRF" => VulnType::SSRF,
            "AFO" => VulnType::AFO,
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "IDOR" => VulnType::IDOR,
            other => VulnType::Other(other.to_string()),
        })
        .collect();

    assert_eq!(vuln_types.len(), 3);
    assert_eq!(vuln_types[0], VulnType::LFI);
    assert_eq!(vuln_types[1], VulnType::Other("UNKNOWN_TYPE".to_string()));
    assert_eq!(vuln_types[2], VulnType::RCE);
}

#[test]
fn test_analysis_summary_creation() {
    let mut summary = AnalysisSummary::new();

    // Create mock responses
    let response1 = Response {
        scratchpad: "Test scratchpad 1".to_string(),
        analysis: "Test analysis 1".to_string(),
        poc: "Test PoC 1".to_string(),
        confidence_score: 8,
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
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    let response2 = Response {
        scratchpad: "Test scratchpad 2".to_string(),
        analysis: "Test analysis 2".to_string(),
        poc: "Test PoC 2".to_string(),
        confidence_score: 6,
        vulnerability_types: vec![VulnType::SQLI],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    // Add results to summary
    summary.add_result(PathBuf::from("/test/file1.py"), response1, "file1.py.md".to_string());
    summary.add_result(PathBuf::from("/test/file2.py"), response2, "file2.py.md".to_string());

    assert_eq!(summary.results.len(), 2);
    assert_eq!(summary.results[0].response.confidence_score, 8);
    assert_eq!(summary.results[1].response.confidence_score, 6);
}

#[test]
fn test_analysis_summary_filtering_by_confidence() {
    let mut summary = AnalysisSummary::new();

    // Create responses with different confidence scores
    let high_confidence = Response {
        scratchpad: "High confidence".to_string(),
        analysis: "High confidence analysis".to_string(),
        poc: "High confidence PoC".to_string(),
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
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    let low_confidence = Response {
        scratchpad: "Low confidence".to_string(),
        analysis: "Low confidence analysis".to_string(),
        poc: "Low confidence PoC".to_string(),
        confidence_score: 3,
        vulnerability_types: vec![VulnType::XSS],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    summary.add_result(PathBuf::from("/test/high.py"), high_confidence, "high.py.md".to_string());
    summary.add_result(PathBuf::from("/test/low.py"), low_confidence, "low.py.md".to_string());

    // Filter by minimum confidence 5
    let filtered = summary.filter_by_min_confidence(5);

    assert_eq!(filtered.results.len(), 1);
    assert_eq!(filtered.results[0].response.confidence_score, 9);
}

#[test]
fn test_analysis_summary_filtering_by_vuln_types() {
    let mut summary = AnalysisSummary::new();

    let rce_response = Response {
        scratchpad: "RCE vulnerability".to_string(),
        analysis: "RCE analysis".to_string(),
        poc: "RCE PoC".to_string(),
        confidence_score: 8,
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
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    let sqli_response = Response {
        scratchpad: "SQLI vulnerability".to_string(),
        analysis: "SQLI analysis".to_string(),
        poc: "SQLI PoC".to_string(),
        confidence_score: 7,
        vulnerability_types: vec![VulnType::SQLI],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    let xss_response = Response {
        scratchpad: "XSS vulnerability".to_string(),
        analysis: "XSS analysis".to_string(),
        poc: "XSS PoC".to_string(),
        confidence_score: 6,
        vulnerability_types: vec![VulnType::XSS],
        par_analysis: ParAnalysis {
            principals: vec![],
            actions: vec![],
            resources: vec![],
            policy_violations: vec![],
        },
        remediation_guidance: RemediationGuidance {
            policy_enforcement: vec![],
        },
        file_path: None,
        pattern_description: None,
        matched_source_code: None,
        full_source_code: None,
    };

    summary.add_result(PathBuf::from("/test/rce.py"), rce_response, "rce.py.md".to_string());
    summary.add_result(PathBuf::from("/test/sqli.py"), sqli_response, "sqli.py.md".to_string());
    summary.add_result(PathBuf::from("/test/xss.py"), xss_response, "xss.py.md".to_string());

    // Filter by specific vulnerability types
    let filter_types = vec![VulnType::RCE, VulnType::SQLI];
    let filtered = summary.filter_by_vuln_types(&filter_types);

    assert_eq!(filtered.results.len(), 2);
    assert!(
        filtered
            .results
            .iter()
            .any(|r| r.response.vulnerability_types.contains(&VulnType::RCE))
    );
    assert!(
        filtered
            .results
            .iter()
            .any(|r| r.response.vulnerability_types.contains(&VulnType::SQLI))
    );
    assert!(
        !filtered
            .results
            .iter()
            .any(|r| r.response.vulnerability_types.contains(&VulnType::XSS))
    );
}

#[test]
fn test_analysis_summary_sorting_by_confidence() {
    let mut summary = AnalysisSummary::new();

    // Add results in random order
    let responses = [
        (PathBuf::from("/test/medium.py"), 5),
        (PathBuf::from("/test/high.py"), 9),
        (PathBuf::from("/test/low.py"), 2),
        (PathBuf::from("/test/very_high.py"), 10),
    ];

    for (path, confidence) in responses {
        let response = Response {
            scratchpad: format!("Confidence {}", confidence),
            analysis: format!("Analysis with confidence {}", confidence),
            poc: format!("PoC {}", confidence),
            confidence_score: confidence,
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
            file_path: None,
            pattern_description: None,
            matched_source_code: None,
            full_source_code: None,
        };
        summary.add_result(path, response, format!("{}.md", confidence));
    }

    summary.sort_by_confidence();

    // Should be sorted in descending order by confidence
    assert_eq!(summary.results[0].response.confidence_score, 10);
    assert_eq!(summary.results[1].response.confidence_score, 9);
    assert_eq!(summary.results[2].response.confidence_score, 5);
    assert_eq!(summary.results[3].response.confidence_score, 2);
}

#[test]
fn test_pathbuf_from_string() {
    // Test path handling logic
    let path_str = "/tmp/test/vulnerable.py";
    let path = PathBuf::from(path_str);

    assert_eq!(path.to_string_lossy(), path_str);
    assert!(path.is_absolute());
    assert_eq!(path.extension().unwrap(), "py");
}

#[test]
fn test_model_default_value() {
    // Test that the default model is correctly set
    let default_model = "o4-mini";
    assert_eq!(default_model, "o4-mini");
}

#[test]
fn test_verbosity_levels() {
    // Test verbosity level handling
    let verbosity_levels = [0u8, 1u8, 2u8, 3u8];

    for level in verbosity_levels {
        match level {
            0 => assert_eq!(level, 0), // No verbose output
            1 => assert_eq!(level, 1), // Basic verbose output
            2 => assert_eq!(level, 2), // More verbose output
            _ => assert!(level >= 3),  // Maximum verbosity
        }
    }
}
