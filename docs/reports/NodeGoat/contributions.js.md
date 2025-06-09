# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **req.body.preTax / afterTax / roth**: Untrusted
  - Context: HTTPリクエストボディ
  - Risk Factors: ユーザー入力, インジェクションソース

### Actions (セキュリティ制御)

- **eval**: Insufficient
  - Function: 動的コード評価
  - Weaknesses: SSJS Injection
  - Bypass Vectors: 任意コードの渡し込みによるeval実行

### Resources (操作対象)

- **Node.js実行コンテキスト**: Critical
  - Operation: コード実行
  - Protection: 

### Policy Violations

#### SSJS-INSECURE-EVAL: Insecure use of eval() allows arbitrary server‐side code execution

- **Path**: app/routes/contributions.js: handleContributionsUpdate 内
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

handleContributionsUpdate関数内で、req.body.preTax／afterTax／rothの値を直接eval()に渡しており、SSJSインジェクションにより任意のサーバサイドコード実行（RCE）が可能です。ユーザー入力は数値チェック前に評価されるため、攻撃者はprocess.exit()やchild_process.exec()を呼び出すなど、サーバ制御やシステムコマンド実行を引き起こせます。

## PoC（概念実証コード）

```text
攻撃リクエスト例:
POST /contributions/update
Content-Type: application/json

{
  "preTax": "require('child_process').exec('id > /tmp/pwn')",
  "afterTax": "0",
  "roth": "0"
}

このリクエストにより、eval()がexec()を実行して任意コマンドが走る。
```

## 修復ガイダンス

### ContributionsHandler.handleContributionsUpdate

- **Required**: eval()の使用を廃止
- **Guidance**: parseIntやNumber()、JSON.parseなど安全なパース手法を用い、数値範囲検証後にDB更新処理を行う。
- **Priority**: high

## 解析ノート

ユーザー入力 -> eval に直接流し込まれているためSSJSインジェクション。RCE可能。対策はevalをparseInt等に置き換え、範囲チェックを行う。

