# Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SSRF`

## PAR Policy Analysis

### Principals (データ源)

- **$_GET['url']**: Untrusted
  - Context: HTTP GET parameter
  - Risk Factors: user_controllable_input

### Actions (セキュリティ制御)

- **file_get_contents**: Missing
  - Function: fetch content from user-provided URL
  - Weaknesses: unsanitized_input
  - Bypass Vectors: internal IP addresses, file:// scheme, gopher:// scheme

### Resources (操作対象)

- **Target resource specified by URL**: Medium
  - Operation: HTTP request / file retrieval
  - Protection: 

### Policy Violations

#### CWE-918: Server-Side Request Forgery vulnerability via unvalidated URL

- **Path**: $_GET['url'] → file_get_contents
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

本コードではHTTP GETパラメータ“url”を検証せずにfile_get_contentsで直接フェッチしており、任意の外部／内部リソースへのリクエストが可能です。SSRF攻撃により内部システム情報の漏洩や不正操作が発生する恐れがあります。

## PoC（概念実証コード）

```text
アクセス例:
http://example.com/redirect.php?url=http://169.254.169.254/latest/meta-data/
```

## 修復ガイダンス

### Input Validation

- **Required**: URLの厳格な検証と許可ドメイン／スキームのホワイトリスト化を実装
- **Guidance**: PHPのparse_url関数を用い、schemeをhttp/httpsに限定し、ホスト名やポートを事前定義したリストと照合する
- **Priority**: high

### Network Access Control

- **Required**: 内部ネットワークやメタデータサービスへのアクセスをブロック
- **Guidance**: ファイアウォールやネットワークACLでプライベートIPレンジおよびメタデータサービスのエンドポイントへのアウトバウンドリクエストを禁止する
- **Priority**: medium

## 解析ノート

Untrusted GET parameter 'url'; no input validation; SSRF via file_get_contents; suggests allowlist and network ACLs.

