# Security Analysis: app.py - SQL execution resource

## ファイル情報

- **ファイルパス**: `repo/app.py`
- **検出パターン**: SQL execution resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 100**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

## マッチしたソースコード

```code
cursor.execute
```

## 詳細解析

## SQL Injection - LIKE句での脆弱性

### 脆弱性の詳細
ユーザーからの入力値`username`がLIKE句に直接埋め込まれており、SQL Injectionが可能です。

### PAR分析
- **Principal（主体）**: HTTPリクエストパラメータ `username` (request.args.get)
- **Action（制御）**: **なし** - 入力値の検証やサニタイゼーション処理が存在しない
- **Resource（リソース）**: SQLiteデータベースの`cursor.execute()`による直接実行

### 攻撃シナリオ
攻撃者は以下のようなペイロードを送信できます：
```
username=' OR '1'='1
username=%' UNION SELECT 1,2,3,4 --
```

### 推奨される修正方法
1. パラメータ化クエリ（プリペアドステートメント）を使用
2. 入力値の厳密な検証
3. SQLのホワイトリスト管理

### コード例（修正方法）
```python
# 脆弱なコード
query1 = f"SELECT id, username, email, role FROM users WHERE username LIKE '%{username}%'"
results1 = cursor.execute(query1).fetchall()

# 修正されたコード
query1 = "SELECT id, username, email, role FROM users WHERE username LIKE ?"
results1 = cursor.execute(query1, (f'%{username}%',)).fetchall()
```

### 関連CWE
- **CWE-89**: SQL Injection

