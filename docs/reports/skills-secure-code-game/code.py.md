# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **request.args['input']**: Untrusted
  - Context: FlaskリクエストのGETパラメータ
  - Risk Factors: ユーザー制御の未検証文字列, HTTPクエリパラメータ

### Actions (セキュリティ制御)

- **get_stock_info**: Insufficient
  - Function: ユーザー入力からSQL SELECT文を生成・実行
  - Weaknesses: 文字列結合／フォーマットによる動的SQL, パラメータ化クエリ不使用, executescriptの危険な使用
  - Bypass Vectors: 悪意ある引用符やセミコロンを含む入力でクエリ構造を変更
- **get_stock_price**: Insufficient
  - Function: ユーザー入力からSQL SELECT文を生成・実行
  - Weaknesses: 文字列結合による動的SQL, executescriptの危険な使用
  - Bypass Vectors: セミコロンを含む入力で複数文を実行
- **exec_multi_query**: Missing
  - Function: ユーザー入力スクリプトを分割してexecute／commit
  - Weaknesses: 入力検証なし, 直接executeによる複数クエリ実行
  - Bypass Vectors: セミコロン区切りで任意のSQL文を注入・実行
- **exec_user_script**: Missing
  - Function: ユーザー入力スクリプトをexecutescript／execute
  - Weaknesses: 入力検証なし, executescriptの危険な使用
  - Bypass Vectors: セミコロン区切りで任意のSQL文を注入・実行

### Resources (操作対象)

- **SQLiteデータベース(level-4.db)内のstocksテーブル**: Medium
  - Operation: SQL SELECT/UPDATE/DDL
  - Protection: 

### Policy Violations

#### SQLI-1: 動的SQLに未検証のユーザー入力を含みパラメータ化していない

- **Path**: DB_CRUD_ops.get_stock_info -> cur.execute(query)
- **Severity**: high
- **Confidence**: 0.90

#### SQLI-1: 動的SQLに未検証のユーザー入力を含みパラメータ化していない

- **Path**: DB_CRUD_ops.get_stock_price -> cur.execute/executescript(query)
- **Severity**: high
- **Confidence**: 0.90

#### SQLI-1: 動的SQLに未検証のユーザー入力を含みパラメータ化していない

- **Path**: DB_CRUD_ops.exec_multi_query -> cur.execute(query)
- **Severity**: high
- **Confidence**: 0.90

#### SQLI-1: 動的SQLに未検証のユーザー入力を含みパラメータ化していない

- **Path**: DB_CRUD_ops.exec_user_script -> cur.executescript/execute(query)
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このコードでは、FlaskのHTTPクエリパラメータ "input"（untrusted）をそのままSQLiteのSQL文に文字列連結／フォーマットしており、パラメータ化されたクエリを使っていないため、SQLインジェクションが可能です。特にget_stock_info、get_stock_price、exec_multi_query、exec_user_scriptの各メソッドが脆弱です。

## PoC（概念実証コード）

```text
# 悪意あるクエリパラメータ例
# http://<host>/?input=MSFT'; DROP TABLE stocks;--
# または exec_multi_query や exec_user_script で:
# input=SELECT+*+FROM+stocks;DROP+TABLE+stocks;--
```

## 修復ガイダンス

### DB_CRUD_ops.get_stock_info／get_stock_price／exec_multi_query／exec_user_script

- **Required**: パラメータ化クエリを使用し、文字列連結やexecutescriptを廃止する
- **Guidance**: 例: cur.execute('SELECT * FROM stocks WHERE symbol = ?', (stock_symbol,)) のようにプレースホルダを用いる
- **Priority**: high

### exec_multi_query／exec_user_script

- **Required**: ユーザー提供スクリプトの直接実行を禁止またはホワイトリスト化する
- **Guidance**: 許可されたクエリのみを明示的に定義し、それ以外は拒否する
- **Priority**: medium

## 解析ノート

1. Flaskのrequest.args\"input\"を直接全メソッドに渡している点を確認
2. get_stock_infoでformat、get_stock_priceで+連結、exec_multi_query／exec_user_scriptでexecutescriptやexecuteを直接使用
3. パラメータバインディング未使用のためSQLインジェクション確実に可能
4. 複数ステートメント実行も許可されており脅威度高
5. remediationはパラメータ化クエリ導入とexecutescript廃止を提案

