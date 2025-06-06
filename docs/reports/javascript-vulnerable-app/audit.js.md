# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `IDOR`

## 解析結果

このコードは、主に動的に組み立てたSQL文をそのまま実行している箇所でSQLインジェクションの脆弱性が多発しており、さらに認可チェックが一切ないためIDOR（不正アクセス）も発生しています。

1. logAction／getAllLogs／deleteLogs／searchLogsではユーザー入力（userId／action／details／filters／searchTerm）を文字列連結でSQLに埋め込んでおり、SQLインジェクション攻撃が可能。
2. getUserLogs／deleteLogsには認可チェックがなく、ユーザーIDを指定するだけで他者の監査ログを取得・削除できるIDOR。
3. exportLogsのCSV出力ではデータに“=”、“+”などを含められるとCSVインジェクションの一種が発生する可能性がある（ただし本分類ではXSS扱いはしません）。

影響：機密データの漏洩、改ざん、データベース破壊、全ログ削除など重大。

推奨対策：プリペアドステートメントの利用、入力バリデーション／サニタイズ、認可チェックの実装。

## PoC（概念実証コード）

```text
```javascript
const AuditService = require('./services/audit');
const audit = new AuditService();

// SQLインジェクション PoC: 全レコード削除＆テーブル破壊
audit.getAllLogs({ action: "'; DROP TABLE audit_logs; --" })
  .then(console.log)
  .catch(console.error);

// IDOR PoC: 他ユーザーID=999のログ取得
audit.getUserLogs(999)
  .then(logs => console.log('他ユーザーのログ:', logs))
  .catch(console.error);
```
```

## 関連コードコンテキスト

### 関数名: logAction
- 理由: ユーザー入力をそのままVALUES句に埋め込んでおり、SQLインジェクション可能
- パス: services/audit.js
```rust
const query = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent, timestamp) VALUES (${userId}, '${action}', '${details}', '${ipAddress}', '${userAgent}', datetime('now'))`;
```

### 関数名: getUserLogs
- 理由: 認可チェックなしに任意のuserIdで他人のログ取得(IDOR)
- パス: services/audit.js
```rust
const query = `SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC LIMIT ${limit}`;
```

### 関数名: getAllLogs
- 理由: 動的にWHERE句を文字連結しており、SQLインジェクション可能
- パス: services/audit.js
```rust
if (filters.action) { query += ` AND action = '${filters.action}'`; }
```

### 関数名: deleteLogs
- 理由: 条件にユーザー入力を直接埋め込み、SQLインジェクション＋IDOR
- パス: services/audit.js
```rust
let query = 'DELETE FROM audit_logs WHERE 1=1'; if (criteria.userId) { query += ` AND user_id = ${criteria.userId}`; }
```

## 解析ノート

内部メモ:
- 全関数で文字列連結による動的SQLが多発
- 認可チェック（Role,ユーザーID照合）が欠如
- exportLogsのCSV生成も要注意(セル先頭に=はExcel式として評価)

