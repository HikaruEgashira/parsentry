use vulnhuntrs::prompts::{
    SYS_PROMPT_TEMPLATE, 
    INITIAL_ANALYSIS_PROMPT_TEMPLATE,
    ANALYSIS_APPROACH_TEMPLATE,
    GUIDELINES_TEMPLATE,
    vuln_specific
};
use vulnhuntrs::response::VulnType;

#[test]
fn test_sys_prompt_template() {
    assert!(!SYS_PROMPT_TEMPLATE.is_empty());
    assert!(SYS_PROMPT_TEMPLATE.contains("セキュリティ研究者"));
    assert!(SYS_PROMPT_TEMPLATE.contains("脆弱性"));
    assert!(SYS_PROMPT_TEMPLATE.contains("入力値の検証"));
    assert!(SYS_PROMPT_TEMPLATE.contains("認証・認可"));
}

#[test]
fn test_initial_analysis_prompt_template() {
    assert!(!INITIAL_ANALYSIS_PROMPT_TEMPLATE.is_empty());
    assert!(INITIAL_ANALYSIS_PROMPT_TEMPLATE.contains("セキュリティ脆弱性"));
    assert!(INITIAL_ANALYSIS_PROMPT_TEMPLATE.contains("ユーザー入力"));
    assert!(INITIAL_ANALYSIS_PROMPT_TEMPLATE.contains("データベースクエリ"));
    assert!(INITIAL_ANALYSIS_PROMPT_TEMPLATE.contains("コマンド実行"));
}

#[test]
fn test_analysis_approach_template() {
    assert!(!ANALYSIS_APPROACH_TEMPLATE.is_empty());
    assert!(ANALYSIS_APPROACH_TEMPLATE.contains("手順"));
    assert!(ANALYSIS_APPROACH_TEMPLATE.contains("エントリポイント"));
    assert!(ANALYSIS_APPROACH_TEMPLATE.contains("データフロー"));
    assert!(ANALYSIS_APPROACH_TEMPLATE.contains("バイパス"));
    assert!(ANALYSIS_APPROACH_TEMPLATE.contains("影響を評価"));
}

#[test]
fn test_guidelines_template() {
    assert!(!GUIDELINES_TEMPLATE.is_empty());
    assert!(GUIDELINES_TEMPLATE.contains("ガイドライン"));
    assert!(GUIDELINES_TEMPLATE.contains("悪用可能"));
    assert!(GUIDELINES_TEMPLATE.contains("行番号"));
    assert!(GUIDELINES_TEMPLATE.contains("信頼度"));
    assert!(GUIDELINES_TEMPLATE.contains("日本語"));
}

#[test]
fn test_prompt_templates_are_non_empty() {
    let templates = [
        SYS_PROMPT_TEMPLATE,
        INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        ANALYSIS_APPROACH_TEMPLATE,
        GUIDELINES_TEMPLATE,
    ];
    
    for template in templates {
        assert!(!template.is_empty());
        assert!(template.len() > 10); // Reasonable minimum length
    }
}

#[test]
fn test_prompt_templates_contain_analysis_keywords() {
    let analysis_keywords = [
        "分析",
        "脆弱性",
        "セキュリティ",
        "コード",
    ];
    
    let templates = [
        SYS_PROMPT_TEMPLATE,
        INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        ANALYSIS_APPROACH_TEMPLATE,
        GUIDELINES_TEMPLATE,
    ];
    
    for template in templates {
        let contains_analysis_keywords = analysis_keywords
            .iter()
            .any(|keyword| template.contains(keyword));
        assert!(contains_analysis_keywords, "Template should contain analysis keywords");
    }
}

#[test]
fn test_vuln_specific_module_exists() {
    // Test that the vuln_specific module can be accessed
    // This verifies the module structure is correct
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    assert!(!vuln_info_map.is_empty());
}

#[test]
fn test_vuln_info_map_contains_common_types() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    
    // Test that common vulnerability types are present
    let common_types = [
        VulnType::RCE,
        VulnType::SQLI,
        VulnType::XSS,
        VulnType::LFI,
    ];
    
    for vuln_type in common_types {
        assert!(vuln_info_map.contains_key(&vuln_type), 
               "Should contain vulnerability type: {:?}", vuln_type);
    }
}

#[test]
fn test_vuln_info_structure() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    
    for (vuln_type, vuln_info) in &vuln_info_map {
        // Each vulnerability type should have non-empty prompt
        assert!(!vuln_info.prompt.is_empty(), 
               "Prompt should not be empty for {:?}", vuln_type);
        
        // Prompt should be reasonably long
        assert!(vuln_info.prompt.len() > 20, 
               "Prompt should be substantial for {:?}", vuln_type);
        
        // Bypasses can be empty but if present should be non-empty strings
        for bypass in &vuln_info.bypasses {
            assert!(!bypass.is_empty(), 
                   "Bypass strings should not be empty for {:?}", vuln_type);
        }
    }
}

#[test]
fn test_rce_specific_prompt() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let rce_info = vuln_info_map.get(&VulnType::RCE);
    
    assert!(rce_info.is_some());
    let rce_info = rce_info.unwrap();
    
    // RCE prompt should contain relevant keywords (English or Japanese)
    assert!(rce_info.prompt.contains("コマンド") || 
           rce_info.prompt.contains("実行") ||
           rce_info.prompt.contains("Remote Code Execution") ||
           rce_info.prompt.contains("Code Execution"));
    assert!(!rce_info.prompt.is_empty());
}

#[test]
fn test_sqli_specific_prompt() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let sqli_info = vuln_info_map.get(&VulnType::SQLI);
    
    assert!(sqli_info.is_some());
    let sqli_info = sqli_info.unwrap();
    
    // SQL injection prompt should contain relevant keywords
    assert!(sqli_info.prompt.contains("SQL") || 
           sqli_info.prompt.contains("データベース") ||
           sqli_info.prompt.contains("クエリ"));
    assert!(!sqli_info.prompt.is_empty());
}

#[test]
fn test_xss_specific_prompt() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let xss_info = vuln_info_map.get(&VulnType::XSS);
    
    assert!(xss_info.is_some());
    let xss_info = xss_info.unwrap();
    
    // XSS prompt should contain relevant keywords (English or Japanese)
    assert!(xss_info.prompt.contains("XSS") || 
           xss_info.prompt.contains("スクリプト") ||
           xss_info.prompt.contains("HTML") ||
           xss_info.prompt.contains("Cross-Site Scripting"));
    assert!(!xss_info.prompt.is_empty());
}

#[test]
fn test_lfi_specific_prompt() {
    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let lfi_info = vuln_info_map.get(&VulnType::LFI);
    
    assert!(lfi_info.is_some());
    let lfi_info = lfi_info.unwrap();
    
    // LFI prompt should contain relevant keywords (English or Japanese)
    assert!(lfi_info.prompt.contains("ファイル") || 
           lfi_info.prompt.contains("パス") ||
           lfi_info.prompt.contains("インクルード") ||
           lfi_info.prompt.contains("Local File Inclusion") ||
           lfi_info.prompt.contains("File"));
    assert!(!lfi_info.prompt.is_empty());
}

#[test]
fn test_prompt_templates_formatting() {
    let templates = [
        SYS_PROMPT_TEMPLATE,
        INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        ANALYSIS_APPROACH_TEMPLATE,
        GUIDELINES_TEMPLATE,
    ];
    
    for template in templates {
        // Templates should contain meaningful content (allow leading/trailing whitespace)
        assert!(!template.trim().is_empty());
        
        // Templates should contain proper line breaks
        assert!(template.contains('\n'));
    }
}

#[test]
fn test_evaluator_prompt_template() {
    use vulnhuntrs::prompts::EVALUATOR_PROMPT_TEMPLATE;
    
    assert!(!EVALUATOR_PROMPT_TEMPLATE.is_empty());
    // The evaluator prompt should be for evaluation purposes
    assert!(EVALUATOR_PROMPT_TEMPLATE.len() > 50); // Should be substantial
}
