# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **HTTP Request Body Parameter 'code'**: Untrusted
  - Context: readBody(req)によるPOSTボディ
  - Risk Factors: 任意コマンド挿入, サニタイズなし

### Actions (セキュリティ制御)

- **execSync(codeString)**: Missing
  - Function: OSコマンド実行
  - Weaknesses: 入力検証欠如, 直接的なコマンド実行
  - Bypass Vectors: 

### Resources (操作対象)

- **Shell Command Execution via child_process.execSync**: Critical
  - Operation: コマンド実行
  - Protection: 

### Policy Violations

#### RCE001: 未検証のユーザ入力を直接コマンド実行に用いている

- **Path**: index.js: execSync(codeString)
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

POSTリクエストのbodyパラメータ“code”がそのままexecSyncに渡され、OSコマンドとして実行されているため、任意のコマンド実行（RCE）が可能です。入力検証・サニタイズが一切行われていない点が重大な脆弱性です。

## PoC（概念実証コード）

```text
curl -X POST http://localhost:9000/ -H 'Content-Type: application/json' -d '{"key":"mjhbwmepyskaup9knxve","code":"id"}'
```

## 修復ガイダンス

### execSync呼び出し

- **Required**: ユーザ入力の妥当性検証と制限
- **Guidance**: 許可されたコマンドのみ実行するホワイトリスト方式を採用し、子プロセス呼び出しには引数分離（execFile等）を用いる
- **Priority**: 高

## 解析ノート

1. POSTリクエストのkey, codeパラメータ取得を確認
2. key照合後、code文字列をそのままexecSyncに渡していることを特定
3. サニタイズやバリデーション処理がないため、任意のシステムコマンドが実行可能
4. これによりRCEの脆弱性が確定
5. execSync呼び出し周辺を保護する必要性を判断

