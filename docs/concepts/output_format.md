# 出力形式

Parsentry はプロンプトを stdout に出力し、外部エージェントが分析結果を SARIF v2.1.0 形式でキャッシュに書き込みます。

## 設計思想

### なぜ SARIF か

- **標準規格**: OASIS 標準の静的解析結果交換フォーマット
- **ツール連携**: GitHub Code Scanning、VS Code 等との統合
- **構造化**: 脆弱性の位置、深刻度、メッセージを機械可読な形式で表現

### データフロー

```
parsentry scan → prompt.md (外部エージェントへ)
                    ↓
外部エージェント → result.sarif.json (SARIF v2.1.0)
                    ↓
parsentry generate → 統合 PDF レポート
```

## キャッシュディレクトリ構造

```
~/Library/Caches/parsentry/<target>/
├── model.json                              — 脅威モデル (ThreatModel)
├── reports/
│   ├── SURFACE-001/
│   │   ├── prompt.md                       — 攻撃面分析プロンプト
│   │   └── result.sarif.json               — SARIF v2.1.0 分析結果
│   ├── SURFACE-002/
│   │   ├── prompt.md
│   │   └── result.sarif.json
│   └── ...
```

キャッシュパスは `PARSENTRY_CACHE_DIR` 環境変数で上書き可能です。

## model.json — ThreatModel

外部エージェントが `parsentry model` のプロンプトに応答して書き込む JSON ファイルです。

```json
{
  "repository": "owner/repo",
  "generated_at": "2026-04-09T12:00:00Z",
  "app_type": "web_application",
  "summary": "Web application with REST API",
  "surfaces": [
    {
      "id": "SURFACE-001",
      "kind": "endpoint",
      "identifier": "POST /api/users",
      "locations": ["src/routes/users.py"],
      "description": "User creation endpoint with input handling"
    }
  ]
}
```

### AttackSurface フィールド

| フィールド | 型 | 説明 |
|-----------|-----|------|
| `id` | string | 一意識別子（SURFACE-NNN） |
| `kind` | string | 自由形式の種別（endpoint, db_table, public_api, iac_resource 等） |
| `identifier` | string | 人間可読な識別子 |
| `locations` | string[] | 関連ファイルパス |
| `description` | string | 分析対象の説明 |

## prompt.md — 攻撃面分析プロンプト

`parsentry scan` が各 AttackSurface ごとに生成する Markdown ファイルです。外部エージェントのサブエージェントが読み込みます。

### 内容

1. **システムプロンプト**: セキュリティ分析者の役割定義
2. **攻撃面メタデータ**: ID、kind、identifier、locations
3. **ソースコード**: locations に対応するファイルの内容
4. **出力指示**: SARIF v2.1.0 形式での結果書き込み先パス

## result.sarif.json — SARIF v2.1.0

外部エージェントが各攻撃面の分析結果として書き込む SARIF ファイルです。

### 基本構造

```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "parsentry-surface-analyzer",
          "version": "1.0.0"
        }
      },
      "results": [
        {
          "ruleId": "SQL_INJECTION",
          "level": "error",
          "message": {
            "text": "SQL injection vulnerability in user query"
          },
          "locations": [
            {
              "physicalLocation": {
                "artifactLocation": {
                  "uri": "src/routes/users.py"
                },
                "region": {
                  "startLine": 42
                }
              }
            }
          ]
        }
      ]
    }
  ]
}
```

### 主要フィールド

| フィールド | 説明 |
|-----------|------|
| `ruleId` | 検出ルールの識別子（SQL_INJECTION, XSS, RCE 等） |
| `level` | 深刻度（error, warning, note） |
| `message.text` | 脆弱性の説明 |
| `locations` | 該当コードの位置（ファイルパス + 行番号） |

## レポート生成

```
parsentry generate owner/repo [-o report.pdf]
```

全 surface の `result.sarif.json` をマージし、PDF レポートを生成します。内部的には `parsentry merge`（非公開コマンド）で SARIF ファイルを統合してから PDF をレンダリングします。
