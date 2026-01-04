# Security Analysis: gitea.py - SQL execution resource

## ファイル情報

- **ファイルパス**: `repo/gitea/giteacasc/gitea.py`
- **検出パターン**: SQL execution resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **user_input (token, name parameters)** - Untrusted

### Actions (セキュリティ制御)

- **no_validation_no_parameterized_query** - Bypassed

### Resources (操作対象)

- **sqlite3_database_insert_operation** - Critical

### Policy Violations

- **SQLI**: create_token(name, token) -> f-string -> con.execute(INSERT) -> access_token table
  - Severity: error | Confidence: 95%

## マッチしたソースコード

```python
con.execute
```

## 詳細解析

## SQLインジェクション脆弱性

### PARフレームワーク分析:

**Principal（主体）**: ユーザー入力
- `User.create_token()` メソッドの引数 `token` (ユーザー提供)
- `name` パラメータ（デフォルト値 'token'）
- `self.uid` (不正なアクセスの可能性がある)

**Action（制御）**: セキュリティ制御がない
- 文字列連結による直接的なSQL構築
- パラメータ化クエリなし
- 入力値の検証なし

**Resource（対象）**: SQLデータベース
- SQLite3 `/data/gitea/gitea.db` へのDML操作
- `access_token` テーブルへの挿入

### 脆弱性の詳細:

79-80行目:
```python
con.execute(f"INSERT INTO access_token VALUES ({token_id + 1}, {self.uid}, '{name}', '{token_hash}', '{salt}', "
            f"'{token_last_eight}', {now}, {now})")
```

このコードは以下の文字列パラメータをそのまま埋め込んでいます:
- `{name}`: シングルクォートで囲まれているがエスケープなし
- `{token_hash}`: シングルクォートで囲まれているがエスケープなし
- `{salt}`: シングルクォートで囲まれているがエスケープなし
- `{token_last_eight}`: シングルクォートで囲まれているがエスケープなし

### 攻撃シナリオ:

`name` パラメータに `test', (SELECT * FROM users), 'x` のような値を渡すと、以下のSQL文が生成されます:

```sql
INSERT INTO access_token VALUES (1, 100, 'test', (SELECT * FROM users), 'x', 'a1b2c3d4', 1234567890, 1234567890)
```

### 影響:

- データベーススキーマの改ざん
- 権限昇格
- 認証バイパス
- データ漏洩
- T1190: Exploit Public-Facing Application
- T1213: Data from Information Repositories

### 修正方法:

`execute()` の代わりに `execute()` にバインドパラメータを使用:
```python
con.execute(
    "INSERT INTO access_token VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    (token_id + 1, self.uid, name, token_hash, salt, token_last_eight, now, now)
)
```

