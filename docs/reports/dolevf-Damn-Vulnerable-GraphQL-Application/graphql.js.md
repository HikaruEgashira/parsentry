# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SSRF`

## PAR Policy Analysis

### Principals (データ源)

- **GraphQLClient.url**: SemiTrusted
  - Context: GraphQLClient生成時のurl引数
  - Risk Factors: 任意の外部URL, 攻撃者制御可能

### Actions (セキュリティ制御)

- **__doRequest (Node.jsパス)**: Missing
  - Function: 外部URLへのHTTPリクエスト発行
  - Weaknesses: URL検証欠如によるSSRFリスク
  - Bypass Vectors: 

### Resources (操作対象)

- **任意の内部/外部HTTPエンドポイント**: High
  - Operation: network_request
  - Protection: 

### Policy Violations

#### SSRF_NODE_JS: Node.js実装でのURLホワイトリスト検証なし

- **Path**: GraphQLClient.createSenderFunction -> __doRequest -> http(s).request
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

Node.js環境向けの__doRequest関数は、GraphQLClientに渡された任意のURLを検証なしでhttp/httpsモジュールに直接渡しリクエストを発行しています。このため、攻撃者が制御可能なurlパラメータを利用してメタデータサービスやイントラネット上の機密サービスへリクエストを飛ばすSSRFが発生します。

## PoC（概念実証コード）

```text
// PoC: 攻撃者が内部メタデータサービスを呼び出し
const { graphql } = require('repo/static/jquery/graphql.js');
const client = graphql('http://169.254.169.254/latest/meta-data', {});
client.query('{dummy}{dummy}').then(console.log).catch(console.error);

```

## 修復ガイダンス

### __doRequest

- **Required**: URLのホワイトリスト検証を実装
- **Guidance**: 許可されたドメイン・IPレンジのみ受け付けるか、URLのホスト部分を固定・検証し、意図しない内部リソースへのアクセスを防止してください。
- **Priority**: high

## 解析ノート

コード全体を読む -> Node.js版__doRequestでURL.parse(url)後に検証なしでhttp(s).request呼び出し -> GraphQLClient.urlがsemi_trustedな外部入力 -> SSRF可能 / PoC例でメタデータ取得試行
vulnerability_types:["SSRF"]

