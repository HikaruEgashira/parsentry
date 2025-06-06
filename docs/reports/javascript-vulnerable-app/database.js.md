# 解析レポート

![中低信頼度](https://img.shields.io/badge/信頼度-中低-green) **信頼度スコア: 40**

## 脆弱性タイプ

- `LFI`

## 解析結果

このコードでは、初期化時に以下のリスクがあるデフォルトデータを挿入しています。

1. 平文の管理者アカウント（admin/admin123）やゲストアカウントが作成されるため、初期設定のまま運用すると容易にログイン突破される。
2. ドキュメントテーブルにファイルパス（'/etc/passwd' や '../../etc/shadow'）が含まれており、これらをファイル提供エンドポイントで読み込むとLFI攻撃として機能する可能性がある。
3. system_configテーブルにDBパスワードやAPIシークレットを平文で保存しており、データベース内から容易に漏洩する。
4. APIトークンやプロファイルデータも固定値で挿入されるため、リプレイ攻撃や権限昇格が可能。

以上により、初期デプロイ直後の環境は認証認可が破綻し、LFIや権限の不正取得（AFO/IDOR的問題）を招きます。

## PoC（概念実証コード）

```text
1) 管理者ログインの証明: curl -X POST http://<host>/login -d 'username=admin&password=admin123'  
2) ファイル読み込みエンドポイント例: curl http://<host>/download?path=/etc/passwd →  /etc/passwd の内容が表示される

これにより、認証バイパスおよびLFIが実証できる。
```

## 関連コードコンテキスト

### 関数名: insertDefaultUsers
- 理由: 平文の管理者アカウントを初期挿入しており、認証が破綻するリスクがある
- パス: example/javascript-vulnerable-app/config/database.js
```rust
`INSERT OR IGNORE INTO users (username, password, email, role, api_key) VALUES ('admin', 'admin123', 'admin@example.com', 'admin', 'sk-js-1234567890abcdef')`
```

### 関数名: insertDefaultDocuments
- 理由: ファイルパスにシステムファイルを指定しており、LFI攻撃として利用される恐れがある
- パス: example/javascript-vulnerable-app/config/database.js
```rust
`INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) VALUES ('Secret Config', 'database_password=super_secret_123', 1, '/etc/passwd')`
```

### 関数名: insertDefaultDocuments
- 理由: 相対パスでシステムファイルを指定し、ディレクトリトラバーサルを誘発可能
- パス: example/javascript-vulnerable-app/config/database.js
```rust
`INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) VALUES ('User Data', 'Sensitive user information', 2, '../../etc/shadow')`
```

### 関数名: insertDefaultConfig
- 理由: 機密情報をデータベースに平文保存しており、漏洩リスクが高い
- パス: example/javascript-vulnerable-app/config/database.js
```rust
`INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) VALUES ('database_password', 'super_secret_db_pass', 'Main database password', 1)`
```

### 関数名: insertDefaultTokens
- 理由: 固定トークンを挿入しており、リプレイ攻撃や不正利用が可能
- パス: example/javascript-vulnerable-app/config/database.js
```rust
`INSERT OR IGNORE INTO api_tokens (token, user_id, permissions, expires_at) VALUES ('admin_token_2024_secret', 1, 'admin,read,write,delete', datetime('now', '+1 year'))`
```

## 解析ノート

コードを確認すると、ユーザー入力を扱う箇所は無いが、初期データとして固定の管理者アカウントやファイルパスが登録されている。これらは実運用向けとは言えず、放置すると悪用される。特にファイルパスのエントリをそのまま返すAPIがあるとLFIになる。データのサニタイズや環境変数管理が一切ない点も問題。

