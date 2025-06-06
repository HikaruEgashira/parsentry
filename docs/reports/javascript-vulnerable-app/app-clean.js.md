# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`

## 解析結果

以下のコードは、ユーザーから送信されたSQLクエリとパラメータをそのままDBアクセス層(userRepository.executeQuery)に渡しており、サニタイズやプリペアドステートメントが適切に適用されていません。そのため、悪意あるユーザーが任意のSQLを実行するSQLインジェクション(SQLI)攻撃が可能です。

## PoC（概念実証コード）

```text
POST /api/v1/db/query HTTP/1.1
Host: localhost:3000
Content-Type: application/json

{
  "query": "SELECT * FROM users; DROP TABLE users; --",
  "params": []
}
```

## 関連コードコンテキスト

### 関数名: Direct Database Access Endpoint
- 理由: ユーザー制御の'db/query'エンドポイントが入力検証なしに任意のSQLクエリを実行しているため
- パス: example/javascript-vulnerable-app/app-clean.js
```rust
this.app.post('/api/v1/db/query', async (req, res) => { ... const result = await this.userRepository.executeQuery(query, params); ... });
```

## 解析ノート

コード中の'/api/v1/db/query'に注目。req.body.queryをそのままexecuteQueryに渡し、プリペアドステートメントやエスケープ処理がないため、SQLIが成立する。

