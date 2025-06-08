# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **req.body.preTax / afterTax / roth**: Untrusted
  - Context: HTTPリクエストボディ
  - Risk Factors: ユーザー制御可能, コード挿入の危険
- **req.session.userId**: Trusted
  - Context: サーバーセッションデータ
  - Risk Factors: 

### Actions (セキュリティ制御)

- **eval(req.body.*)**: Missing
  - Function: 動的コード評価
  - Weaknesses: SSJS注入, 不適切な入力検証
  - Bypass Vectors: "); require('child_process').exec('id'); x=(", __proto__.toString(), process.env.PATH

### Resources (操作対象)

- **SSJS実行環境**: Critical
  - Operation: コード実行
  - Protection: 

### Policy Violations

#### JSINJ-01: 信頼できない入力をevalでそのまま評価している

- **Path**: ContributionsHandler.handleContributionsUpdate -> eval(req.body.preTax)
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

handleContributionsUpdate関数内で、ユーザー入力(req.body.preTax, afterTax, roth)をeval()で直接評価しており、SSJS注入によるリモートコード実行(RCE)の脆弱性が発生しています。evalによって不正なJavaScriptが実行可能となり、攻撃者はシェルコマンド実行や環境変数の窃取などを行えます。

## PoC（概念実証コード）

```text
// リクエストボディに以下を含める
{
  "preTax": "1); require('child_process').exec('id', function(err,stdout){console.log(stdout)}); x=(",
  "afterTax": "0",
  "roth": "0"
}
```

## 修復ガイダンス

### handleContributionsUpdate

- **Required**: evalの使用を廃止し、安全なパーサーを利用
- **Guidance**: parseInt(req.body.preTax, 10)やNumber(req.body.preTax)を用いて数値変換し、不正文字の除去と範囲チェックを行ってください
- **Priority**: high

## 解析ノート

コードを読み込み、evalの使用箇所を確認
→ req.bodyからの入力を直接evalしている部分を確認
→ 不特定入力がコードとして実行可能なためSSJSインジェクションの脆弱性と判定
→ PARモデルでPrincipal: req.body(信頼できない)、Action: eval(動的評価・未実装)、Resource: SSJS実行環境(クリティカル)
→ ポリシー違反ルールを定義
→ 修正案としてeval廃止と安全なパーサー導入を提案

