# LLMレスポンススキーマ

このドキュメントでは、Parsentryでセキュリティ脆弱性をレポートする際にLLMが従わなければならない構造化出力形式を定義します。

## 概要

Parsentryは以下を保証するためにLLMレスポンスの厳密なJSONスキーマを強制します：
- 一貫した脆弱性レポート
- 自動解析と処理
- 実用的なセキュリティ発見
- CI/CDパイプラインとの統合

## スキーマ定義

### 完全なJSON構造

```json
{
  "scratchpad": "string - 解析思考プロセスと推論",
  "analysis": "string - 詳細な脆弱性説明",
  "poc": "string - 概念実証悪用コード",
  "confidence_score": "integer - 信頼度レベル (0-10)",
  "vulnerability_types": ["脆弱性タイプ文字列の配列"],
  "context_code": [
    {
      "name": "string - 関数/クラス/メソッド名",
      "reason": "string - このコードが脆弱な理由",
      "code_line": "string - 実際の脆弱なコード"
    }
  ]
}
```

### フィールド仕様

#### 1. Scratchpad（必須）
- **型**: 文字列
- **目的**: LLMの解析プロセスを捕捉
- **内容**: 段階的推論、データフロー解析、セキュリティ考慮事項
- **例**:
```json
"scratchpad": "1. リクエストパラメータ'id'からユーザー入力を識別\n2. SQLクエリ構築へのフローを追跡\n3. パラメータ化が見つからない\n4. 直接文字列連結を確認\n5. SQLインジェクション脆弱性を確認"
```

#### 2. Analysis（必須）
- **型**: 文字列
- **目的**: 包括的な脆弱性説明
- **内容**: 
  - 根本原因分析
  - 攻撃ベクター
  - 潜在的影響
  - ビジネスリスク評価
- **例**:
```json
"analysis": "get_user関数のSQLインジェクション脆弱性。'id'パラメータからのユーザー入力がサニタイゼーションなしにSQLクエリに直接連結されています。攻撃者は悪意のあるSQLを注入して機密データを抽出、レコードを変更、またはデータベースコマンドを実行できます。影響：データベース全体の侵害が可能。"
```

#### 3. 概念実証（必須）
- **型**: 文字列
- **目的**: 悪用可能性を実証
- **内容**: 動作する悪用コードまたは明確な悪用手順
- **形式**: コードスニペット、curlコマンド、または段階的指示
- **例**:
```json
"poc": "curl 'http://example.com/user?id=1 OR 1=1--' \n# 全ユーザーを返す\ncurl 'http://example.com/user?id=1 UNION SELECT password FROM admins--'\n# 管理者パスワードを抽出"
```

#### 4. 信頼度スコア（必須）
- **型**: 整数 (0-10)
- **目的**: 解析の確実性を示す
- **スコアリングガイド**:
  - 0-3: 低信頼度、検証が必要
  - 4-6: 中程度の信頼度、脆弱である可能性が高い
  - 7-9: 高信頼度、脆弱性確認済み
  - 10: 確実、動作するPoCあり
- **例**: `8`

#### 5. 脆弱性タイプ（必須）
- **型**: 文字列の配列
- **目的**: 発見を分類
- **許可される値**:
  - `"LFI"` - ローカルファイルインクルージョン
  - `"RCE"` - リモートコード実行
  - `"SSRF"` - サーバーサイドリクエストフォージェリ
  - `"AFO"` - 任意ファイル操作
  - `"SQLI"` - SQLインジェクション
  - `"XSS"` - クロスサイトスクリプティング
  - `"IDOR"` - 安全でない直接オブジェクト参照
- **例**: `["SQLI", "IDOR"]`

#### 6. コンテキストコード（必須）
- **型**: オブジェクトの配列
- **目的**: 発見を特定のコードにリンク
- **オブジェクト構造**:
  - `name`: 脆弱なコンポーネントの識別子
  - `reason`: 脆弱性関連性の説明
  - `code_line`: 実際の脆弱なコードスニペット
- **例**:
```json
"context_code": [
  {
    "name": "get_user",
    "reason": "サニタイズされていない入力でSQLクエリを構築",
    "code_line": "query = \"SELECT * FROM users WHERE id = \" + request.params.id"
  },
  {
    "name": "execute_query",
    "reason": "脆弱なクエリを実行",
    "code_line": "results = db.execute(query)"
  }
]
```

## Complete Example Response

```json
{
  "scratchpad": "Analyzing Flask route /api/user/<id>. User input 'id' flows directly into SQL query construction via string formatting. No input validation or parameterized queries used. Database appears to be MySQL based on syntax.",
  "analysis": "Critical SQL Injection vulnerability in user lookup endpoint. The 'id' parameter is inserted directly into SQL query using string formatting, allowing attackers to inject arbitrary SQL. This could lead to unauthorized data access, data manipulation, or complete database takeover.",
  "poc": "# Extract all users:\ncurl 'http://localhost:5000/api/user/1%20OR%201=1'\n\n# Extract passwords:\ncurl 'http://localhost:5000/api/user/1%20UNION%20SELECT%20username,password%20FROM%20users--'",
  "confidence_score": 9,
  "vulnerability_types": ["SQLI"],
  "context_code": [
    {
      "name": "get_user",
      "reason": "Vulnerable SQL query construction",
      "code_line": "cursor.execute(f\"SELECT * FROM users WHERE id = {user_id}\")"
    }
  ]
}
```

## Validation Rules

### Required Fields
All six top-level fields must be present in every response.

### Type Constraints
- Strings must be non-empty
- `confidence_score` must be integer 0-10
- `vulnerability_types` must contain valid enum values
- `context_code` must have at least one entry if vulnerability found

### Content Guidelines
- `scratchpad`: Include actual analysis steps
- `analysis`: Be specific about impact and risk
- `poc`: Provide executable/testable code
- `context_code`: Reference actual line numbers when possible

## Error Handling

### Common Parsing Errors

1. **Missing Required Field**
```json
{
  "analysis": "...",
  "poc": "..."
  // Missing other required fields
}
```

2. **Invalid Vulnerability Type**
```json
{
  "vulnerability_types": ["SQLi"]  // Should be "SQLI"
}
```

3. **Wrong Type**
```json
{
  "confidence_score": "high"  // Should be integer
}
```

### Schema Validation

The schema is enforced by:
1. JSON parsing in `parse_json_response()`
2. Serde deserialization to `Response` struct
3. Type validation via Rust's type system

## Best Practices

### For Clear Analysis
- Start scratchpad with input identification
- Trace data flow systematically
- Note validation attempts (or lack thereof)
- Consider all attack vectors

### For Effective PoCs
- Make exploits copy-pasteable
- Include expected output
- Cover multiple attack scenarios
- Test edge cases

### For Accurate Context
- Include function signatures
- Show variable declarations
- Highlight data flow paths
- Reference line numbers

## Integration Notes

### CI/CD Pipeline
- Parse JSON response programmatically
- Fail builds on high confidence scores
- Generate reports from structured data
- Track vulnerability trends

### Security Tools
- Export to standard formats (SARIF, etc.)
- Integrate with issue trackers
- Feed into vulnerability databases
- Support automated remediation

## Future Extensions

Potential schema enhancements:
- CVSS scoring fields
- Remediation suggestions
- Reference links
- Affected versions
- Patch recommendations
