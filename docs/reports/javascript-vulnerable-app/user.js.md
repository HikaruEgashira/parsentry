# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `SQLI`
- `IDOR`

## 解析結果

本コードはほとんど全てのDB操作でユーザー入力を直接文字列連結してSQLクエリを生成しており、SQLインジェクションのリスクが極めて高いです。また、認可チェックが一切なく任意のuserIdで更新・削除・閲覧が可能なためIDOR(不適切なアクセス制御)も存在します。具体的には：

1. searchUsers, authenticateUser, createUser, updateUser, deleteUser, getUserProfileなど、全てのクエリ生成箇所でパラメータをエスケープせずに連結している。これにより攻撃者は`' OR '1'='1`などをインジェクションして任意のデータを取得・操作可能。
2. updateUser/deleteUser/getUserProfileはユーザーIDを直接コントロールできるにも関わらず、認可チェックがないため他ユーザーのデータを改ざん・削除・閲覧できる。

影響範囲：データ漏洩、改ざん、権限昇格など

## PoC（概念実証コード）

```text
例: authenticateUser呼び出し時に username="' OR '1'='1" password="anything" を送信すると全ユーザーにマッチし認証バイパスできる。

// Node.js 例
await userService.authenticateUser("' OR '1'='1", "foo");
```

## 関連コードコンテキスト

### 関数名: searchUsers
- 理由: ユーザー入力を直接クエリ連結しており、LIKE句でSQLインジェクション可能
- パス: repo/services/user.js
```rust
query += ` AND username LIKE '%${username}%'`;
```

### 関数名: authenticateUser
- 理由: 認証処理でパラメータをバインドせずに文字列連結し、認証バイパスや情報取得が可能
- パス: repo/services/user.js
```rust
const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
```

### 関数名: updateUser
- 理由: userIdを直接使用し、認可チェックがなくIDORを許している
- パス: repo/services/user.js
```rust
const query = `UPDATE users SET ${setParts.join(', ')} WHERE id = ${userId}`;
```

### 関数名: deleteUser
- 理由: 同様に認可チェックなしで他ユーザーを削除可能
- パス: repo/services/user.js
```rust
const query = `DELETE FROM users WHERE id = ${userId}`;
```

### 関数名: getUserProfile
- 理由: 任意IDで他者のプロファイル閲覧が可能なIDOR
- パス: repo/services/user.js
```rust
const query = `SELECT u.*, up.profile_data, up.permissions ... WHERE u.id = ${userId}`;
```

## 解析ノート

Identified SQLi on nearly all methods and missing access control checks leading to IDOR.

