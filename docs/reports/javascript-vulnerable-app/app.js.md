# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `XSS`
- `RCE`
- `IDOR`

## 解析結果

本アプリケーションでは、ユーザー入力を適切に検証・サニタイズせずに直接組み込むコードが散見され、以下の重大な脆弱性を含みます。

1. SQLインジェクション（/login, /sqli）
   - ユーザー名・パスワードや検索クエリをテンプレート文字列で直接組み込み。攻撃者が"' OR '1'='1"等を注入可能。

2. クロスサイトスクリプティング（/xss）
   - `name`や`comment`パラメータをエスケープせずにHTMLに出力。ストアド／リフレクトXSSを誘発。

3. コマンドインジェクション（/cmdi）
   - `cmd`と`args`を結合して`execSync`で実行。セミコロン区切りで任意コマンド実行が可能。

4. IDOR（/logs）
   - `user_id`パラメータをそのままクエリに利用し、他ユーザーの監査ログを参照可能。認可チェックが欠如。

各脆弱性は単独でも深刻ですが、組み合わせによりシステム完全乗っ取りや機密情報漏えいを招きます。修正にはプリペアドステートメント、適切なサニタイズ／エスケープ、認可チェックの導入が必須です。

## PoC（概念実証コード）

```text
1. SQLiでログインバイパス:
   GET /login?username=' OR '1'='1&password=dummy

2. /sqliで全ユーザー表示:
   GET /sqli?username=' OR '1'='1&order=id

3. XSS反射型攻撃:
   GET /xss?name=<script>alert(1)</script>&comment=<img src=x onerror=alert(2)>

4. コマンドインジェクション:
   GET /cmdi?cmd=ls;cat%20/etc/passwd&args=

5. IDORによる他ユーザー監査ログ取得:
   GET /logs?user_id=2
```

## 関連コードコンテキスト

### 関数名: Login SQL Query
- 理由: ユーザー入力を直接組み込み、認証バイパス可能なSQLインジェクション
- パス: app.js
```rust
const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`
```

### 関数名: Dynamic SQL Query in /sqli
- 理由: 検索パラメータと並び順を直結して組み立てるSQLインジェクション
- パス: app.js
```rust
const query = `SELECT * FROM users WHERE username LIKE '%${username}%' ORDER BY ${order || 'id'}`
```

### 関数名: XSS in /xss Endpoint
- 理由: コメントをエスケープせずにHTMLに出力する反射型XSS
- パス: app.js
```rust
${comment ? `<div>Comment: ${comment}</div>` : ''}
```

### 関数名: Command Injection in /cmdi
- 理由: 攻撃者制御下のコマンドを`execSync`で実行するコマンドインジェクション
- パス: app.js
```rust
const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
```

### 関数名: IDOR in /logs
- 理由: ユーザー認可チェックなしに任意のuser_idで他ユーザーのログ参照が可能
- パス: app.js
```rust
db.all(`SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC LIMIT 50`, ...)
```

## 解析ノート

 - 認証とデータベースアクセス部分でテンプレート文字列による直結が多用
 - HTMLレスポンスにエスケープなしで挿入する箇所が存在
 - execSyncで動的コマンド実行
 - sessionやJWT部分も秘密鍵がハードコーディングされているが、本質は入力バリデーション不足
 - 上記4種類の脆弱性を確認

