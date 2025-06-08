use crate::response::VulnType;
use std::collections::HashMap;

pub const SYS_PROMPT_TEMPLATE: &str = r#"
あなたはセキュリティ研究者として、コードの脆弱性を分析します。特に以下に注目してください：
- 入力値の検証とサニタイズ
- 認証・認可
- データの取り扱いと漏洩
- コマンドインジェクションの可能性
- パストラバーサルの脆弱性
- その他セキュリティ上重要なパターン
"#;

pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
与えられたコードをPAR（Principal-Action-Resource）モデルに基づいて分析し、以下のJSON形式で結果を出力してください：

**Principal（誰が/データ源）**: データの起点となるエンティティや入力源
- ユーザー入力、API応答、ファイル読み込み、環境変数など
- 各Principalの信頼レベル（trusted/semi_trusted/untrusted）を評価

**Action（何を/検証・処理）**: データ処理や検証操作（脆弱性の可能性を含む）
- 入力検証、サニタイズ、認証・認可、暗号化など（バイパス可能性要注意）
- 実装品質（adequate/insufficient/missing/bypassed）を評価

**Resource（どこで/危険な操作）**: 機密性・完全性・可用性に影響する操作
- ファイル書き込み、コマンド実行、データベース更新、出力など
- 機密性レベル（low/medium/high/critical）を評価

各コード要素がどのPAR役割を持ち、適切なセキュリティポリシーが実装されているかを評価し、policy_violationsとして報告してください。
"#;

pub const ANALYSIS_APPROACH_TEMPLATE: &str = r#"
PARモデルに基づく分析手順：
1. **Principal識別**: 危険なデータ源（信頼できない入力）を特定
2. **Resource識別**: 機密性・完全性・可用性に影響する危険な操作を特定
3. **Action評価**: PrincipalからResourceへの経路で適切な検証・防御が実装されているかを評価
4. **ポリシー違反検出**: 不適切なPrincipal-Resource間の直接アクセスを検出
5. **PAR関係の文脈評価**: コード全体の文脈でPAR関係が適切かを判断
"#;

pub const GUIDELINES_TEMPLATE: &str = r#"
PARベースのセキュリティポリシー評価ガイドライン：

## 分析手順
1. **Principal評価**: 信頼できないデータ源を特定し、その危険性を評価
2. **Resource評価**: 機密性・完全性・可用性に影響する操作の危険性を評価
3. **Action評価**: Principal-Resource間の適切な防御策実装を評価
4. **ポリシー違反**: 危険なPrincipalが適切なActionなしでResourceに直接アクセスする場合を検出
5. **文脈考慮**: コード全体の文脈でPAR関係の適切性を判断
6. **宣言的判定**: 「このPrincipalにはこのActionが必要」といった宣言的ポリシーで評価

## 出力形式要件
必ずJSON形式で以下のstructureに従って出力してください。脆弱性が見つからない場合はconfidence_score=0、vulnerability_types=[]とし、空のPAR analysisを返してください：

{
  "scratchpad": "解析思考プロセス",
  "analysis": "詳細な脆弱性説明",
  "poc": "概念実証コード（見つからない場合は空文字列）",
  "confidence_score": 0-100の整数,
  "vulnerability_types": ["LFI","RCE","SSRF","AFO","SQLI","XSS","IDOR"] // 実際に検出された脆弱性のみ、重複なし,
  "par_analysis": {
    "principals": [{ // 実際に特定されたPrincipalのみ
      "identifier": "識別子",
      "trust_level": "trusted|semi_trusted|untrusted",
      "source_context": "コンテキスト説明",
      "risk_factors": ["リスク要因リスト"]
    }],
    "actions": [{ // 実際に特定されたActionのみ
      "identifier": "識別子", 
      "security_function": "セキュリティ機能説明",
      "implementation_quality": "adequate|insufficient|missing|bypassed",
      "detected_weaknesses": ["弱点リスト"],
      "bypass_vectors": ["バイパス手法リスト"]
    }],
    "resources": [{ // 実際に特定されたResourceのみ
      "identifier": "識別子",
      "sensitivity_level": "low|medium|high|critical", 
      "operation_type": "操作タイプ",
      "protection_mechanisms": ["保護メカニズムリスト"]
    }],
    "policy_violations": [{ // 実際に検出されたPolicy Violationのみ
      "rule_id": "ルールID",
      "rule_description": "ルール説明", 
      "violation_path": "違反パス",
      "severity": "重要度",
      "confidence": 0.0-1.0の浮動小数点
    }]
  },
  "remediation_guidance": {
    "policy_enforcement": [{ // 修復が必要な場合のみ
      "component": "コンポーネント名",
      "required_improvement": "必要な改善",
      "specific_guidance": "具体的なガイダンス", 
      "priority": "優先度"
    }]
  }
}

重要: 脆弱性が存在しない場合は、虚偽の検出を避けるため、confidence_score=0、vulnerability_types=[]、空のpar_analysisを返してください。

7. 必ず日本語で応答してください

注意: Actionパターン（バリデーション・処理）はバイパス可能性があり、実装不備が脆弱性の直接原因となります。
"#;

pub mod vuln_specific {
    use super::*;

    pub struct VulnTypeInfo {
        pub prompt: String,
        pub bypasses: Vec<String>,
    }

    pub fn get_vuln_specific_info() -> HashMap<VulnType, VulnTypeInfo> {
        let mut map = HashMap::new();

        map.insert(
            VulnType::LFI,
            VulnTypeInfo {
                prompt: "Analyze for Local File Inclusion vulnerabilities...".to_string(),
                bypasses: vec![
                    "Path traversal sequences(../../)".to_string(),
                    "URL encoding".to_string(),
                    "Null byte injection".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::RCE,
            VulnTypeInfo {
                prompt: "Analyze for Remote Code Execution vulnerabilities...".to_string(),
                bypasses: vec![
                    "Shell metacharacters for command injection".to_string(),
                    "Python execution vectors".to_string(),
                    "Deserialization attacks".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::SSRF,
            VulnTypeInfo {
                prompt: "Analyze for Server-Side Request Forgery vulnerabilities...".to_string(),
                bypasses: vec![
                    "DNS rebinding".to_string(),
                    "IP address encoding tricks".to_string(),
                    "Redirect chain".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::AFO,
            VulnTypeInfo {
                prompt: "Analyze for Arbitrary File Operation vulnerabilities...".to_string(),
                bypasses: vec![
                    "Directory traversal sequences".to_string(),
                    "Following symbolic links".to_string(),
                    "Race conditions".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::SQLI,
            VulnTypeInfo {
                prompt: "Analyze for SQL Injection vulnerabilities...".to_string(),
                bypasses: vec![
                    "UNION-based injection".to_string(),
                    "Boolean-based blind injection".to_string(),
                    "Time-based blind injection".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::XSS,
            VulnTypeInfo {
                prompt: "Analyze for Cross-Site Scripting vulnerabilities...".to_string(),
                bypasses: vec![
                    "HTML entity encoding bypass".to_string(),
                    "JavaScript template injection".to_string(),
                    "DOM-based XSS vectors".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::IDOR,
            VulnTypeInfo {
                prompt: "Analyze for Insecure Direct Object Reference vulnerabilities..."
                    .to_string(),
                bypasses: vec![
                    "Parameter tampering".to_string(),
                    "Horizontal privilege escalation".to_string(),
                    "Predictable resource paths".to_string(),
                ],
            },
        );

        map
    }
}
