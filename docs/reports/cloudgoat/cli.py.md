# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **dbtable引数**: Untrusted
  - Context: click CLI 引数として渡されるテーブル名
  - Risk Factors: ユーザ制御入力, 特殊文字挿入可能

### Actions (セキュリティ制御)

- **SQLクエリ文字列組み立て**: Insufficient
  - Function: ユーザ入力をSQL文字列に埋め込み
  - Weaknesses: 識別子への不十分なサニタイズ
  - Bypass Vectors: テーブル名に']'を挿入し角括弧引用を打ち破る

### Resources (操作対象)

- **SQLiteデータベース**: High
  - Operation: SQL実行
  - Protection: 

### Policy Violations

#### SQLI-001: SQLインジェクション：ユーザ制御のテーブル名を適切にサニタイズせずに生SQLへ組み込んでいる

- **Path**: rows -> ctx.invoke(query) -> f"select {columns} from [{dbtable}]"
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

cliコード内でユーザ入力（特にテーブル名やビュー名）をそのままSQLクエリ文字列に埋め込んで実行しており、SQLiteの識別子引用に用いられる角括弧「[」「]」を破る特殊文字（]）の挿入によりSQLインジェクションが可能です。たとえば `sqlite-utils rows my.db --dbtable "users]; DROP TABLE users;--"` とすると任意のSQLが実行されます。

## PoC（概念実証コード）

```text
# PoC: テーブル名にSQLを挿入しDROP TABLEを実行
$ sqlite-utils rows my.db "users]; DROP TABLE users;--"
```

## 修復ガイダンス

### rowsコマンド／query実装

- **Required**: ユーザ入力の識別子を安全にエスケープまたはホワイトリスト検証
- **Guidance**: sqlite_utils.Database.quote()を用いてテーブル名を引用するか、正規表現で英数字とアンダースコアのみ許可する
- **Priority**: high

## 解析ノート

ユーザ入力のテーブル名を角括弧で囲むだけでは不十分。']'で抜けられるためSQLインジェクション成立。関数rowsやsearchの内部で同様のパターンがある。identifier quoting関数の利用を推奨。

