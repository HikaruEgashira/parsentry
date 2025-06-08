# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **HTTP Request Body `code`**: Untrusted
  - Context: ユーザからのPOSTリクエスト
  - Risk Factors: 未検証のシェルコマンド文字列

### Actions (セキュリティ制御)

- **keyチェック**: Insufficient
  - Function: 認証
  - Weaknesses: ハードコードされたシークレットキー, リプレイ防止機能なし
  - Bypass Vectors: キー漏洩
- **execSync**: Missing
  - Function: コマンド実行
  - Weaknesses: 入力未検証によるコマンドインジェクション
  - Bypass Vectors: 任意コマンド実行

### Resources (操作対象)

- **OSシェル**: Critical
  - Operation: コマンド実行
  - Protection: シークレットキー認証

### Policy Violations

#### RCE001: Untrusted input passed to execSync, leading to RCE

- **Path**: execSync(codeString)
- **Severity**: critical
- **Confidence**: 0.90

## 詳細解析

このNode.jsアプリケーションでは、POSTリクエストのボディに含まれるcodeパラメータを検証せずに直接execSyncに渡しており、リモートから任意のシェルコマンドを実行できるRCE脆弱性が存在します。

## PoC（概念実証コード）

```text
curl -X POST http://localhost:9000/ -H "Content-Type: application/json" -d '{"key":"mjhbwmepyskaup9knxve","code":"touch /tmp/pwned"}'
```

## 修復ガイダンス

### execSync使用箇所

- **Required**: ユーザ入力によるシェルコマンド実行を廃止
- **Guidance**: ホワイトリスト化したコマンドのみ実行するか、child_process.spawnで引数を分離し、入力検証を徹底してください。
- **Priority**: high

## 解析ノート

1. POST / にkeyとcodeを受信
2. key比較後、execSync(codeString)で直接シェル実行
3. codeは未検証・未制限のため任意コマンドが実行可能
4. RCE脆弱性として特定
5. PARモデルでPrincipal=untrusted入力、Action=execSync、Resource=OSシェルとして評価
6. ポリシー違反(RCE001)を報告
7. 改善策としてホワイトリスト化・入力検証・spawnへの移行を提案

