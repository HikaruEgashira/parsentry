# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SSRF`
- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **req.query.url, req.query.symbol**: Untrusted
  - Context: HTTPリクエストのクエリパラメータ
  - Risk Factors: ユーザー制御可能, 未検証

### Actions (セキュリティ制御)

- **ConstructAndFetchURL**: Missing
  - Function: 入力バリデーション（欠如）
  - Weaknesses: 入力検証欠如, ホワイトリスト未実装
  - Bypass Vectors: 
- **WriteResponseBody**: Missing
  - Function: HTMLエスケープ（欠如）
  - Weaknesses: 出力エスケープ欠如
  - Bypass Vectors: 

### Resources (操作対象)

- **NetworkResource**: Medium
  - Operation: HTTP GET
  - Protection: 

### Policy Violations

#### SSRF-01: 未検証のユーザー入力による任意のURLへのリクエスト実行（SSRF）

- **Path**: ResearchHandler.displayResearch -> needle.get(url)
- **Severity**: high
- **Confidence**: 0.90

#### XSS-01: 取得したレスポンスをエスケープせずにHTML出力（XSS）

- **Path**: ResearchHandler.displayResearch -> res.write(newResponse.body)
- **Severity**: medium
- **Confidence**: 0.80

## 詳細解析

このコードでは、未検証のユーザー入力（req.query.url, req.query.symbol）を連結して外部・内部ネットワークリソースへのHTTPリクエスト（needle.get）を実行しており、SSR F脆弱性が存在します。さらに、取得したレスポンスボディをエスケープせずにそのままHTMLとして出力しているため、XSS攻撃を許容します。

## PoC（概念実証コード）

```text
// SSRFによるAWSメタデータ取得の例
// GET /research?url=http://169.254.169.254/latest/meta-data/&symbol=
// 攻撃者はインスタンスメタデータを窃取可能

const http = require('http');
http.get('http://target/app/research?url=http://169.254.169.254/latest/meta-data/&symbol=', res => {
  let data = '';
  res.on('data', chunk => data += chunk);
  res.on('end', () => console.log(data));
});
```

## 修復ガイダンス

### ResearchHandler.displayResearch

- **Required**: 入力されたURLとシンボルのバリデーション・ホワイトリスト化
- **Guidance**: 許可ドメイン一覧を用意し、URLオブジェクトを解析してホスト名が許可リストに含まれる場合のみリクエスト実行する。
- **Priority**: high

### ResearchHandler.displayResearch

- **Required**: 取得レスポンスのHTMLエスケープ
- **Guidance**: レスポンスボディを出力前にライブラリ（例：lodash.escape）でサニタイズし、XSSを防止する。
- **Priority**: high

## 解析ノート

1. req.query.urlとreq.query.symbolはuntrustedなユーザー入力。
2. これを連結してneedle.getで任意のURLへリクエスト -> SSRFリスク。
3. newResponse.bodyを出力時にエスケープしておらずXSSの懸念。
4. Actionに入力バリデーションもエスケープも実装されていないため、implementation_qualityはmissing。
5. Policy violationsとしてSSR F-01とXSS-01を検出。
6. Remediation: ドメインホワイトリストとHTMLエスケープの追加。

