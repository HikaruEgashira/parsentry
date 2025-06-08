# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SSRF`
- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **request.query['url']**: Untrusted
  - Context: HTTPクエリパラメータ
  - Risk Factors: ユーザー制御可能, 内部ネットワークアクセスの可能性

### Actions (セキュリティ制御)

- **needle.get(url) 呼び出し**: Missing
  - Function: 外部HTTPリクエスト
  - Weaknesses: URLバリデーション欠如, プライベート/メタデータIPへのアクセス制限なし
  - Bypass Vectors: 
- **response.write(url, response1.body)**: Missing
  - Function: HTML出力エンコーディング
  - Weaknesses: ユーザー入力をエスケープなしでHTML返却
  - Bypass Vectors: 内部IPアドレス, 169.254.169.254

### Resources (操作対象)

- **任意URLのHTTPアクセス**: High
  - Operation: ネットワークリクエスト
  - Protection: 
- **HTMLレスポンス出力**: Medium
  - Operation: コンテンツ返却
  - Protection: 

### Policy Violations

#### PV-SSRF-001: 未検証のユーザー指定URLから内部リソースへのリクエスト(SSRF)

- **Path**: app.js → needle.get(request.query['url'])
- **Severity**: high
- **Confidence**: 0.90

#### PV-XSS-001: ユーザー入力をHTMLエスケープせずに返却(反射型XSS)

- **Path**: app.js → response.write(...)
- **Severity**: medium
- **Confidence**: 0.80

## 詳細解析

このコードでは、ユーザーが提供するURLをバリデーションせずに直接needle.get()でリクエストしているため、SSRFが発生します。また、リクエストしたURLとレスポンス本文をエスケープせずにHTMLとして返しているため、反射型XSSの脆弱性も存在します。

## PoC（概念実証コード）

```text
1. SSRFテスト例:
   http://<host>/?url=http://169.254.169.254/latest/meta-data/
2. 反射型XSSテスト例:
   http://<host>/?url=http://example.com"><script>alert(1)</script>
```

## 修復ガイダンス

### SSRF保護

- **Required**: 許可リストベースのURL検証を実装
- **Guidance**: URLパース時にスキーム(httpsのみ許可)とホストを検証し、プライベートIP/メタデータサービスへのアクセスをブロックしてください。
- **Priority**: high

### 出力エンコーディング

- **Required**: ユーザー入力を適切にエスケープ
- **Guidance**: テンプレートエンジンやライブラリを用いてHTMLエンコーディングを適用し、直接書き出しを避けてください。
- **Priority**: medium

## 解析ノート

1. request.query['url'] を入力源として識別 → untrusted
2. needle.get(url) によるネットワークリクエストはSSRFリスク → バリデーションなし
3. レスポンスとURLを直接HTMLに返却 → 反射型XSSリスク
4. 脆弱性タイプ: SSRF, XSS
5. PoC作成: メタデータ取得とスクリプト挿入
6. 改善: allowlist検証, HTMLエスケープ実装」

