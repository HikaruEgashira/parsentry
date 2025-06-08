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
与えられたコードをPAR（Principal-Action-Resource）モデルに基づいて分析してください：

**Principal（誰が/データ源）**: データの起点となるエンティティや入力源
- ユーザー入力、API応答、ファイル読み込み、環境変数など

**Action（何を/検証・処理）**: データ処理や検証操作（脆弱性の可能性を含む）
- 入力検証、サニタイズ、認証・認可、暗号化など（バイパス可能性要注意）

**Resource（どこで/危険な操作）**: 機密性・完全性・可用性に影響する操作
- ファイル書き込み、コマンド実行、データベース更新、出力など

各コード要素がどのPAR役割を持ち、適切なセキュリティポリシーが実装されているかを評価してください。
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

1. **Principal評価**: 信頼できないデータ源を特定し、その危険性を評価
2. **Resource評価**: 機密性・完全性・可用性に影響する操作の危険性を評価
3. **Action評価**: Principal-Resource間の適切な防御策実装を評価
4. **ポリシー違反**: 危険なPrincipalが適切なActionなしでResourceに直接アクセスする場合を検出
5. **文脈考慮**: コード全体の文脈でPAR関係の適切性を判断
6. **宣言的判定**: 「このPrincipalにはこのActionが必要」といった宣言的ポリシーで評価
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
