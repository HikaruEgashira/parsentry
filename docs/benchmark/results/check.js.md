# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SSRF`
- `AFO`

## PAR Policy Analysis

## 詳細解析

このコードでは、外部入力（system.stdin.readLine()で取得したinput）がまったく検証されずに直接page.open()に渡されています。そのため、攻撃者は任意のURL（たとえば内部ネットワークやメタデータサービスのエンドポイント）を指定でき、サーバーサイドでリクエストが実行されるSSRF脆弱性が存在します。

## PoC（概念実証コード）

```text
# SSRF攻撃の例: 内部メタデータサービスへのアクセス
echo "http://169.254.169.254/latest/meta-data/" | phantomjs check.js
```

## 修復ガイダンス

### Input検証

- **Required**: 入力されたURLの検証・ホワイトリスト化
- **Guidance**: 許可されたドメインやIPアドレスのみを受け付けるようにし、リダイレクトチェーンやIPエンコーディングのバイパスも考慮してください。
- **Priority**: 高

### リクエスト制御

- **Required**: 内部向けリクエスト防止
- **Guidance**: プライベートIPレンジやAWSメタデータサービスへのリクエストをブロックするフィルタを導入してください。
- **Priority**: 中

## 解析ノート

- untrusted input: system.stdin.readLine()
- no validation before page.open
- SSRF allows internal network access
- propose echo to metadata service
- remediation: whitelist, block internal IP ranges

