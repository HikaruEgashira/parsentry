use std::collections::HashMap;

pub fn get_messages() -> HashMap<&'static str, &'static str> {
    let mut messages = HashMap::new();

    // Error messages
    messages.insert("error_clone_failed", "クローン先ディレクトリの削除に失敗");
    messages.insert("cloning_repo", "GitHubリポジトリをクローン中");
    messages.insert("analysis_target", "解析対象");
    messages.insert("context_collection_failed", "コンテキスト収集に失敗");
    messages.insert("analyzing_file", "ファイルを解析中");
    messages.insert("analysis_completed", "解析完了");
    messages.insert("error_directory_creation", "ディレクトリの作成に失敗");
    messages.insert("error_no_write_permission", "書き込み権限がありません");
    messages.insert("error_test_file_deletion", "テストファイルの削除に失敗");
    messages.insert(
        "error_no_file_creation_permission",
        "ファイル作成権限がありません",
    );
    messages.insert(
        "error_output_dir_check",
        "❌ 出力ディレクトリのチェックに失敗",
    );
    messages.insert(
        "relevant_files_detected",
        "関連するソースファイルを検出しました",
    );
    messages.insert(
        "security_pattern_files_detected",
        "セキュリティパターン該当ファイルを検出しました",
    );
    messages.insert("parse_add_failed", "ファイルのパース追加に失敗");
    messages.insert("analysis_failed", "解析に失敗");
    messages.insert(
        "markdown_report_output_failed",
        "Markdownレポート出力に失敗",
    );
    messages.insert("markdown_report_output", "Markdownレポートを出力");
    messages.insert("summary_report_output_failed", "サマリーレポート出力に失敗");
    messages.insert("summary_report_output", "サマリーレポートを出力");
    messages.insert(
        "summary_report_needs_output_dir",
        "サマリーレポートを出力するには --output-dir オプションが必要です",
    );
    messages.insert("sarif_report_output_failed", "SARIFレポート出力に失敗");
    messages.insert("sarif_report_output", "SARIFレポートを出力");
    messages.insert("sarif_output_failed", "SARIF出力に失敗");
    messages.insert(
        "github_repo_clone_failed",
        "GitHubリポジトリのクローンに失敗",
    );
    messages.insert(
        "custom_pattern_generation_start",
        "カスタムパターン生成モードを開始します",
    );
    messages.insert("pattern_generation_completed", "パターン生成が完了しました");

    messages
}

pub const SYS_PROMPT_TEMPLATE: &str = r#"
あなたはセキュリティ研究者として、コードの脆弱性を分析します。特に以下に注目してください：
- 入力値の検証とサニタイズ
- 認証・認可
- データの取り扱いと漏洩
- コマンドインジェクションの可能性
- パストラバーサルの脆弱性
- タイミング攻撃やレースコンディション
- その他セキュリティ上重要なパターン
"#;

pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
与えられたコード（関数定義または関数呼び出し）をPAR（Principal-Action-Resource）モデルに基づいて分析し、以下のどれに分類されるかを一つ決めてください：

**Principal（信頼できないソース）**: 攻撃者が制御可能なデータを提供する関数
- ユーザー入力を取得する関数（request.params, request.body等）
- 外部データを取得する関数（API応答、ファイル読み込み等）

**Action（セキュリティ処理）**: セキュリティ処理を行う関数
- バリデーション、サニタイズ、認証・認可処理を行う関数
- 入力検証、データ変換、権限チェック、暗号化等を行う関数

**Resource（攻撃対象）**: 攻撃対象となるリソースを操作する関数
- ファイルシステム、データベース、システムコマンド、DOM、ネットワークを操作する関数

## 関数定義の分類例:

**Resource例:**
```python
def get_user_data(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return execute_query(query)
```
→ この関数は最終的にデータベースを操作するため「Resource」に分類

**Action例:**
```python
def validate_user_id(user_id):
    if not isinstance(user_id, int):
        raise ValueError("Invalid user ID")
    return user_id
```
→ この関数は入力検証を行うため「Action」に分類

## 関数呼び出しの分類例:

**Principal例:**
```javascript
request.params.get('user_id')
```
→ この呼び出しは信頼できないユーザー入力を取得するため「Principal」

**Action例:**
```python
validate_email(user_email)
```
→ この呼び出しはバリデーション処理を実行するため「Action」

**Resource例:**
```python
db.query("SELECT * FROM users")
```
→ この呼び出しはデータベースを操作するため「Resource」

```javascript
document.getElementById('output').innerHTML = content
```
→ この呼び出しはDOMを操作するため「Resource」

**重要:** 関数呼び出しの場合は、呼び出される関数の性質で分類します。引数の内容ではなく、どの関数を呼んでいるかが決定要因です。
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
6. **宣言的判定**: 「このPrincipalはこのActionが可能」といった宣言的ポリシーで評価
7. 必ず日本語で応答してください

重要: 
- 脆弱性が存在しない場合は、confidence_score=0、vulnerability_types=[]、空のpar_analysisを返してください。
- Actionパターン（バリデーション・処理）はバイパス可能性があり、実装不備が脆弱性の直接原因となります。
"#;

pub const EVALUATOR_PROMPT_TEMPLATE: &str = r#"あなたは、脆弱性分析レポートを評価するセキュリティ専門家です。
このレポートは、SQLインジェクション（SQLI）、クロスサイトスクリプティング（XSS）、リモートコード実行（RCE）の脆弱性が含まれていることが知られているPythonウェブアプリケーションの脆弱性を特定することを目的としています。

以下の観点からレポートを評価してください：
1. 正しく特定された脆弱性（SQLI、XSS、RCE）
2. 誤検知（存在しない脆弱性が報告されている場合）
3. 分析の質（影響評価、根本原因の説明、緩和策の提案）
4. 検証用コードの質（明確な手順、例となるリクエスト、期待される結果）

評価対象のレポート：
{report}
"#;

pub const RESPONSE_LANGUAGE_INSTRUCTION: &str = "必ず日本語で応答してください";
