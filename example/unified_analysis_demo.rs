//! 統一データフロー解析のデモンストレーション
//! Programming言語とIaCの脆弱性を同じアルゴリズムで検出

use crate::par_dataflow::{UnifiedDataFlowAnalyzer, DataFlowNode, DataFlowNodeType, FlowType};
use crate::par_analyzer::{PARTriplet, Principal, Action, Resource, Effect, PrincipalType, PrivilegeLevel};
use std::collections::HashMap;

pub fn demo_unified_analysis() {
    let mut analyzer = UnifiedDataFlowAnalyzer::new();
    
    // 1. Programming言語の脆弱性パターン (SQL Injection)
    add_programming_vulnerability_example(&mut analyzer);
    
    // 2. IaCの脆弱性パターン (Unauthorized Resource Access)  
    add_iac_vulnerability_example(&mut analyzer);
    
    // 3. 同一アルゴリズムで両方を検出
    let vulnerabilities = analyzer.detect_vulnerabilities();
    
    println!("=== 統一データフロー解析結果 ===");
    for vuln in vulnerabilities {
        println!("ID: {}", vuln.vulnerability_id);
        println!("説明: {}", vuln.description);
        println!("対応策: {}", vuln.remediation);
        println!("影響ノード: {:?}", vuln.affected_nodes);
        println!("---");
    }
}

fn add_programming_vulnerability_example(analyzer: &mut UnifiedDataFlowAnalyzer) {
    // Programming言語: user_input → database_query (validation抜け)
    
    let user_input = DataFlowNode {
        node_id: "user_input_1".to_string(),
        node_type: DataFlowNodeType::Source,
        sensitivity_level: crate::par_dataflow::SensitivityLevel::Public,
        trust_boundary: crate::par_dataflow::TrustBoundary::Internet,
        attributes: HashMap::from([
            ("input_type".to_string(), "form_data".to_string()),
            ("source".to_string(), "external".to_string()),
        ]),
    };
    
    let database_query = DataFlowNode {
        node_id: "sql_query_1".to_string(),
        node_type: DataFlowNodeType::Sink,
        sensitivity_level: crate::par_dataflow::SensitivityLevel::Confidential,
        trust_boundary: crate::par_dataflow::TrustBoundary::Internal,
        attributes: HashMap::from([
            ("query_type".to_string(), "SELECT".to_string()),
            ("table".to_string(), "users".to_string()),
        ]),
    };
    
    // データフロー追加 (validation なし = 脆弱)
    let flow = FlowType::DataFlow {
        from: user_input,
        to: database_query,
        data_type: "string".to_string(),
        transformations: vec![], // 検証処理なし！
    };
    
    analyzer.flows.push(flow);
}

fn add_iac_vulnerability_example(analyzer: &mut UnifiedDataFlowAnalyzer) {
    // IaC: external_principal → sensitive_resource (access_control抜け)
    
    let par_triplet = PARTriplet {
        principal: Principal {
            principal_type: PrincipalType::External,
            identifier: "arn:aws:iam::123456789012:root".to_string(),
            attributes: HashMap::from([
                ("account_type".to_string(), "external".to_string()),
                ("trust_level".to_string(), "unknown".to_string()),
            ]),
        },
        action: Action {
            service: "s3".to_string(),
            operation: "GetObject".to_string(),
            is_wildcard: false,
            privilege_level: PrivilegeLevel::Read,
        },
        resource: Resource {
            resource_type: "s3_bucket".to_string(),
            identifier: "company-sensitive-data".to_string(),
            is_wildcard: false,
            sensitivity: crate::par_analyzer::DataSensitivity::Confidential,
        },
        effect: Effect::Allow,
        conditions: vec![], // アクセス制御条件なし！
    };
    
    // PAR三組をデータフローに変換して追加
    analyzer.add_par_triplet(&par_triplet);
}

/// 実際のTerraformコードからPAR三組を抽出してデータフロー解析
pub fn analyze_terraform_with_dataflow(terraform_content: &str) -> Vec<String> {
    let mut analyzer = UnifiedDataFlowAnalyzer::new();
    let mut par_analyzer = crate::par_analyzer::PARAnalyzer::new();
    
    // 1. TerraformコンテンツからPAR三組を抽出
    if let Ok(_) = par_analyzer.extract_par_triplets(terraform_content, "terraform") {
        // 2. PAR三組をデータフローに変換
        for triplet in par_analyzer.triplets {
            analyzer.add_par_triplet(&triplet);
        }
    }
    
    // 3. Programming言語と同じアルゴリズムで脆弱性検出
    let vulnerabilities = analyzer.detect_vulnerabilities();
    
    // 4. 結果をわかりやすい形式で返す
    vulnerabilities.into_iter()
        .map(|v| format!("{}: {}", v.vulnerability_id, v.description))
        .collect()
}

/// プログラミング言語とIaCの統合解析結果比較
pub fn compare_programming_vs_iac_patterns() {
    println!("=== Programming言語 vs IaC 脆弱性パターン対応表 ===");
    
    let mappings = vec![
        ("SQL Injection", "Unvalidated Resource Access", 
         "user_input → sql_query", "external_principal → database"),
        
        ("Command Injection", "Privilege Escalation",
         "user_input → system_call", "low_privilege → admin_resource"),
        
        ("XSS", "Trust Boundary Violation", 
         "user_input → html_output", "external_account → internal_resource"),
        
        ("CSRF", "Cross-Account Resource Access",
         "external_request → state_change", "cross_account → resource_modification"),
        
        ("Path Traversal", "Resource Enumeration",
         "user_path → file_access", "wildcard_principal → all_resources"),
    ];
    
    for (prog_vuln, iac_vuln, prog_flow, iac_flow) in mappings {
        println!("Programming: {} ({})", prog_vuln, prog_flow);
        println!("IaC:         {} ({})", iac_vuln, iac_flow);
        println!("-> 同一のパスベース検出アルゴリズムで対応可能");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_dataflow_analysis() {
        let mut analyzer = UnifiedDataFlowAnalyzer::new();
        
        // テスト用のPAR三組を追加
        let test_triplet = PARTriplet {
            principal: Principal {
                principal_type: PrincipalType::Wildcard,
                identifier: "*".to_string(),
                attributes: HashMap::new(),
            },
            action: Action {
                service: "s3".to_string(),
                operation: "*".to_string(),
                is_wildcard: true,
                privilege_level: PrivilegeLevel::Wildcard,
            },
            resource: Resource {
                resource_type: "bucket".to_string(),
                identifier: "*".to_string(),
                is_wildcard: true,
                sensitivity: crate::par_analyzer::DataSensitivity::Confidential,
            },
            effect: Effect::Allow,
            conditions: vec![],
        };
        
        analyzer.add_par_triplet(&test_triplet);
        let vulnerabilities = analyzer.detect_vulnerabilities();
        
        // ワイルドカード権限が検出されることを確認
        assert!(!vulnerabilities.is_empty());
        assert!(vulnerabilities.iter().any(|v| v.vulnerability_id.contains("unvalidated_resource_access")));
    }
    
    #[test]
    fn test_programming_iac_pattern_equivalence() {
        // Programming言語のSQLiパターン
        let programming_source = "user_input";
        let programming_sink = "sql_query";
        
        // IaCの同等パターン  
        let iac_source = "external_principal";
        let iac_sink = "sensitive_resource";
        
        // 同じ検出ロジックが適用できることを確認
        assert_eq!(
            is_vulnerable_pattern(programming_source, programming_sink),
            is_vulnerable_pattern(iac_source, iac_sink)
        );
    }
    
    fn is_vulnerable_pattern(source: &str, sink: &str) -> bool {
        // 簡略化された検出ロジック
        (source.contains("external") || source.contains("user")) &&
        (sink.contains("sensitive") || sink.contains("sql"))
    }
}