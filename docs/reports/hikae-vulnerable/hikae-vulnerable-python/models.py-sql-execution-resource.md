# Security Analysis: models.py - SQL execution resource

## ファイル情報

- **ファイルパス**: `repo/models.py`
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

## authenticate_user()におけるSQL Injection

**Principal**: ユーザー入力（`username`, `password`パラメータ）

**Action**: セキュリティ制御なし - パラメータ化クエリを使用していない

**Resource**: SQLデータベース（cursor.execute()）

**脆弱性の詳細**:
92行目でユーザー提供の`username`と`password`が文字列フォーマットでSQL文に直接挿入されている

攻撃者は以下のようなペイロードで認証を回避可能：
- username: `admin' --`
- password: `anything`
- admin のパスワード検証がバイパスされる

**推奨修正**:
パラメータ化クエリを使用してください

