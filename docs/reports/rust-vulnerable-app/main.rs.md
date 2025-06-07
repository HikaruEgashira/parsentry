# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `RCE`
- `SSRF`
- `AFO`
- `IDOR`

## 解析結果

本アプリケーションは多数のユーザー制御入力を危険なAPIや外部コマンド、ファイル操作、DBクエリに直接渡しており、入力検証・エスケープ・認可チェックがほぼ存在しません。主な問題点は以下の通りです：
1. SQLインジェクション（/sqli, /api/documents/search）
2. コマンドインジェクション（/cmdi, /api/exec/command）
3. パストラバーサル・任意ファイル読み書き（/file, /api/file/upload, /static, /api/files/list）
4. SSRF（/api/ssrf/fetch）
5. 無認可アクセス（IDOR: /api/user/{id}, /api/logs/{id}）
6. LDAPインジェクション（/api/ldap/search）
7. XML/ YAML/JSONの安全でないパース（XXE, YAMLデシアライズ, 独自プレファレンスデシアライズ）
8. ハードコーディングされたシークレット、ログに平文パスワード出力

## PoC（概念実証コード）

```text
1. SQLI: GET /sqli?username=admin'%20--&order=id  
2. コマンドインジェクション: GET /cmdi?hostname=localhost;whoami&count=1  
3. LFI: GET /file?name=../../etc/passwd  
4. SSRF: POST /api/ssrf/fetch  body: {"url":"http://169.254.169.254/latest/meta-data/"}  
5. ファイルアップロードパストラバーサル: マルチパートでfilename: “../app.rs”  
6. IDOR: GET /api/user/2 （他人のデータ取得）
```

## 関連コードコンテキスト

### 関数名: sql_injection
- 理由: クエリ文字列を直接組み立てており、usernameパラメータが未エスケープ
- パス: src/main.rs
```rust
format!("SELECT * FROM users WHERE username = '{}'", username)
```

### 関数名: command_injection
- 理由: ユーザー入力hostnameをシェルコマンドに渡し、任意コマンド実行可能
- パス: src/main.rs
```rust
let command1 = format!("ping -c {} {}", count, hostname)
```

### 関数名: file_read
- 理由: filenameパラメータをバリデーションせずファイル読み込みに利用（パストラバーサル可）
- パス: src/main.rs
```rust
read_file_content(&filename)
```

### 関数名: ssrf_fetch
- 理由: ユーザー指定URLを検証なくリクエストすることでSSRFを誘発
- パス: src/main.rs
```rust
reqwest::get(&req.url).await
```

### 関数名: upload_file
- 理由: Content-Dispositionヘッダのfilenameをそのままファイルパスに使用し、パストラバーサルや上書き可能
- パス: src/main.rs
```rust
let filepath = format!("/tmp/{}", filename)
```

### 関数名: get_user
- 理由: 認可チェックなしで任意ユーザー情報を取得可能（IDOR）
- パス: src/main.rs
```rust
db.get_user_by_id(&user_id)
```

## 解析ノート

入力箇所→危険API→攻撃手法→影響の四段階で特定。上記6点を代表例として選定。

