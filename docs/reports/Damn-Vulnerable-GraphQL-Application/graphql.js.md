# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`
- `SSRF`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

このライブラリは任意の URL を検証せずに HTTP/HTTPS リクエストに直接使用しており、Node.js ブランチの __doRequest 関数で require('http') や require('https') を介してサーバーサイドでリクエストを実行します。GraphQLClient の利用者が URL をコントロールできる場合、内向きの管理用エンドポイントやメタデータサービス（例: 169.254.169.254）に対してリクエストを発生させる SSRF 攻撃が可能です。適切なホワイトリストやフィルタリングが実装されていないため、DNS rebinding や IP エンコードトリックなどのバイパスも容易です。

## PoC（概念実証コード）

```text
// PoC: 内部メタデータサービスへのリクエストを発生させる例
const GraphQLClient = require('graphql');
const client = new GraphQLClient('http://169.254.169.254/latest/meta-data/', {});
client.query('{ dummy }')
  .then(data => console.log(data))
  .catch(err => console.error(err));
```

## 解析ノート

- コード全体を確認し、__doRequest の Node.js ブランチが外部 URL をバリデーションせずに使用していることを確認
- GraphQLClientコンストラクタで任意の URL を this.url に格納している
- __request 関数経由で __doRequest をコールし、攻撃者制御下の URL に HTTP リクエスト発行が可能
- SSRF に対してホワイトリストやプロトコル制限が未実装
- PoC によりメタデータサービスへのアクセスを示した

