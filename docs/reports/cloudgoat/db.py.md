# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **order_byパラメータ**: Untrusted
  - Context: Table.searchメソッド引数
  - Risk Factors: 外部入力の未検証利用

### Actions (セキュリティ制御)

- **Table.search_sql**: Insufficient
  - Function: SQLクエリ文字列生成
  - Weaknesses: 動的文字列連結による未エスケープのORDER BY句
  - Bypass Vectors: order_by="1; DROP TABLE users; --"

### Resources (操作対象)

- **SQLiteデータベース**: Medium
  - Operation: クエリ実行
  - Protection: パラメータ化クエリ（q引数）

### Policy Violations

#### SQLI001: ORDER BY句の動的フォーマットでSQLインジェクションを許容している

- **Path**: Table.search_sql -> db.execute
- **Severity**: high
- **Confidence**: 0.85

## 詳細解析

Table.search_sqlメソッドでは、order_by引数を直接SQL文字列のフォーマットに埋め込んでいるため、悪意あるユーザ入力を利用したSQLインジェクションが可能です。クエリ本文(q)はパラメータ化されているものの、ORDER BY句に渡されるorder_byは非パラメータ化のまま文字列連結されており、任意のSQLステートメントを注入・実行される危険があります。

## PoC（概念実証コード）

```text
# POC: order_byに悪意あるSQLを挿入
from sqlite_utils import Database

db = Database(":memory:")
db["users"].insert_all([{"id":1,"name":"Alice"}])
# 悪意あるorder_byでDROP TABLEを実行
try:
    list(db["users"].search("Alice", order_by="id; DROP TABLE users; --"))
except Exception as e:
    print("エラー発生、DROP TABLEが実行された可能性:", e)
print("テーブル一覧:", db.table_names())
```

## 修復ガイダンス

### Table.search_sql

- **Required**: order_byの値を直接文字列連結せずにホワイトリスト検証またはエスケープ
- **Guidance**: order_byとして許可するカラム名リストを事前に定義し、外部入力をそのリストと照合した上で埋め込む。もしくは識別子引用機能を用いて安全にエスケープ処理を行う。
- **Priority**: high

## 解析ノート

・Table.search_sql内でorder_byを直接.format()に渡している点を発見
・qはパラメータ化されているが、order_byは未検証のままSQLに埋め込まれている
・これにより不正なORDER BY句からSQLインジェクションが可能
・poCコードにてorder_by経由でDROP TABLEを実行する例を示した　
・対策としてホワイトリスト検証や識別子エスケープの導入が必要
・confidence_score=85, vulnerability_types=["SQLI"]で報告

