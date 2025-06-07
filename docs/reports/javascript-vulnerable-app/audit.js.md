# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `SQLI`
- `IDOR`

## 解析結果

本サービスでは、ユーザー入力（userId、action、details、ipAddress、userAgent、filters、criteria、searchTerm）がそのままSQLクエリ文字列へ埋め込まれており、プリペアドステートメントやサニタイズ処理が一切行われていません。これにより以下の問題が発生します。

1. SQLインジェクション（SQLI）
   - logAction、getUserLogs、getAllLogs、deleteLogs、searchLogs の各メソッドで入力値を直接連結しており、悪意あるSQL文を注入可能。

2. IDOR（Insecure Direct Object Reference）
   - 認可処理が存在せず、任意のuserIdやcriteriaを指定することで他ユーザーの監査ログ参照・削除が可能。

特にsearchLogsにおけるsearchTermやdeleteLogsのcriteria.olderThan等での未検証パラメータは深刻です。パラメータ化クエリ（プリペアドステートメント）や入力バリデーションを必須とし、認可チェックを導入すべきです。

## PoC（概念実証コード）

```text
// SQLインジェクションによる全レコード取得例
const maliciousUserId = "1 OR 1=1";
auditService.getUserLogs(maliciousUserId).then(logs => console.log(logs));

// DROP TABLE攻撃例
const maliciousSearch = "'; DROP TABLE audit_logs; --";
auditService.searchLogs(maliciousSearch).catch(err => console.error(err));
```

## 関連コードコンテキスト

### 関数名: logAction
- 理由: ユーザー入力を直接SQLに連結しており、SQLインジェクションのリスクがある
- パス: repo/services/audit.js
```rust
const query = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent, timestamp) VALUES (${userId}, '${action}', '${details}', '${ipAddress}', '${userAgent}', datetime('now'))`;
```

### 関数名: getUserLogs
- 理由: userIdとlimitが未検証のまま埋め込まれており、SQLインジェクションおよびIDORを招く
- パス: repo/services/audit.js
```rust
const query = `SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC LIMIT ${limit}`;
```

### 関数名: getAllLogs
- 理由: filters.action等の任意パラメータを未エスケープで連結し、SQLインジェクションを許す。また認可チェックがないためIDORになる
- パス: repo/services/audit.js
```rust
if (filters.action) { query += ` AND action = '${filters.action}'`; }
```

### 関数名: deleteLogs
- 理由: criteria.userId等を直接SQLに連結し、SQLインジェクションのほか、認可なしでログ削除が可能（IDOR）
- パス: repo/services/audit.js
```rust
let query = 'DELETE FROM audit_logs WHERE 1=1'; if (criteria.userId) { query += ` AND user_id = ${criteria.userId}`; }
```

### 関数名: searchLogs
- 理由: searchTerm がそのままLIKE句に埋め込まれ、SQLインジェクションを許す
- パス: repo/services/audit.js
```rust
const query = `SELECT * FROM audit_logs WHERE details LIKE '%${searchTerm}%' OR action LIKE '%${searchTerm}%' OR ip_address LIKE '%${searchTerm}%' ORDER BY timestamp DESC`;
```

## 解析ノート

ユーザー入力が全メソッドで未検証／未エスケープのまま文字列連結
→ プリペアドステートメント未使用 → SQLI確定
認可機構なし → 他ユーザー操作可能 → IDOR
対象メソッド: logAction, getUserLogs, getAllLogs, deleteLogs, searchLogs


