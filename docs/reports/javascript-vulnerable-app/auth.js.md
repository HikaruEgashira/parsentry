# 解析レポート

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`
- `AFO`

## 解析結果

提示されたコードには多数の致命的なセキュリティ脆弱性があります。特にSQLインジェクション、認証弱体化、機密情報の漏洩を多く含んでいます。
1. ユーザー入力を直接SQL文字列に埋め込んでいるため、認証・登録・パスワードリセット・変更処理でSQLインジェクションが可能です。
2. パスワード・APIキー・クエリ文字列をログに平文出力・レスポンスに含めており、機密情報が漏洩します。
3. JWTトークンの生成・検証が脆弱で、トークン失効処理を行わないため、盗聴やリプレイ攻撃を受けやすい設計です。
4. パスワードリセットトークンはMD5＋タイムスタンプで予測可能、かつ応答に返却しているため攻撃者が容易に悪用できます。
5. パスワード変更で既存パスワード検証を実施せず、リセットトークン検証も甘いためアカウント乗っ取りが可能です。
6. 登録時にroleを任意に設定可能で、管理者権限作成まで許しており、認可設計が大きく欠落しています。

## PoC（概念実証コード）

```text
1) SQLインジェクション認証バイパス例:
   POST /login
   Content-Type: application/json
   {
     "username": "' OR '1'='1",
     "password": "any"
   }
2) パスワードリセットトークン予測:
   - 同一メールで2回リセットを行い、得られるトークンを比較。MD5のタイムスタンプ差分から生成規則を解析し他ユーザーをリセット。
3) 管理者権限作成:
   POST /register
   { "username":"attacker","password":"pass","email":"a@b.com","role":"admin" }
   => 管理者ユーザーが作成可能
```

## 関連コードコンテキスト

### 関数名: login SQLインジェクション
- 理由: ユーザー入力を直接埋め込んでおり、' OR '1'='1' 等で認証バイパス可能
- パス: routes/auth.js
```rust
const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
```

### 関数名: 機密情報のログ出力
- 理由: ユーザー名・パスワードを平文でログに出力し、ログから漏洩リスクが高い
- パス: routes/auth.js
```rust
console.log(`Login attempt: ${username}:${password} from ${req.ip}`);
```

### 関数名: エラーレスポンスにSQLクエリを含有
- 理由: 実行クエリを外部に晒し、攻撃者が構造把握や二次攻撃に利用可能
- パス: routes/auth.js
```rust
res.status(500).json({ error: `Authentication failed: ${err.message}`, query: query});
```

### 関数名: 予測可能なパスワードリセットトークン
- 理由: MD5＋タイムスタンプは予測可能で、総当りでリセット可能
- パス: routes/auth.js
```rust
const resetToken = crypto.createHash('md5').update(email + Date.now()).digest('hex');
```

### 関数名: リセットトークン保存SQLインジェクション
- 理由: emailフィールドをエスケープせずクエリに組み込み、任意実行可能
- パス: routes/auth.js
```rust
const query = `UPDATE users SET reset_token = '${resetToken}' WHERE email = '${email}'`;
```

### 関数名: 登録処理のSQLインジェクション
- 理由: あらゆるユーザー入力を直接埋め込み、管理者権限登録や任意クエリ実行が可能
- パス: routes/auth.js
```rust
const query = `INSERT INTO users (username, password, email, role) VALUES ('${username}', '${password}', '${email}', '${userRole}')`;
```

### 関数名: トークン失効処理未実装
- 理由: クライアントへログアウト成功を返すのみで、JWTは依然有効なまま
- パス: routes/auth.js
```rust
res.json({ message: 'Logged out successfully', warning: 'JWT token not invalidated...', token_hint: token ? token.substring(0, 20) + '...' : 'none' });
```

## 解析ノート

SQLIが主要。ログ・レスポンスに機密露出多数。JWT設計も認可機構も欠落。IDORはないがAFOに分類。

