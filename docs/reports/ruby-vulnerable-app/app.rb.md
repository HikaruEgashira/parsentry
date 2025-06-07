# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `XSS`

## 解析結果

アプリケーションはSinatraを用い、GET /me エンドポイントで受け取った username パラメータを検証やサニタイズなしに文字列連結でSQLiteのクエリに組み込んでいます。その結果、SQLインジェクション攻撃が可能です。また、同じ入力値をHTMLのフォーム（value属性）や<pre>タグ内にエスケープせず直接埋め込んでいるため、反射型XSSも発生し得ます。認証・認可の仕組みもないため、誰でも自由に脆弱性を悪用できます。

## PoC（概念実証コード）

```text
以下のように OR 条件を注入すると、全ユーザーデータを取得できます。

curl "http://localhost:4567/me?username=admin'%20OR%20'1'%3D'1"

上記を実行すると、Results にすべてのレコードが表示されます。
```

## 関連コードコンテキスト

### 関数名: /me エンドポイント内のクエリ組み立て
- 理由: ユーザー入力をそのまま連結しており、SQLインジェクションを許可している
- パス: repo/app.rb
```rust
query = "SELECT * FROM users WHERE username = '#{username}'"
```

### 関数名: HTMLフォームの入力表示
- 理由: ユーザー入力をエスケープせずに埋め込んでおり、反射型XSSを誘発し得る
- パス: repo/app.rb
```rust
<input type="text" name="username" value="#{username}">
```

## 解析ノート

GET /me の username パラメータ → 検証/サニタイズなし → SQL文字列連結 → SQLI発生。さらに同値をHTMLに直接埋め込みエスケープ省略 → 反射型XSS発生の可能性。

