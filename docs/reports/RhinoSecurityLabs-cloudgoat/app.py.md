# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **request.form["selected_date"]**: Untrusted
  - Context: HTTP POSTパラメータ
  - Risk Factors: ユーザ入力, 未検証データ

### Actions (セキュリティ制御)

- **index()の動的SQL構築**: Insufficient
  - Function: SQLクエリの生成と実行
  - Weaknesses: SQLインジェクションの脆弱性
  - Bypass Vectors: ',"; OR 1=1--

### Resources (操作対象)

- **original_dataテーブル**: High
  - Operation: SQL SELECT/DML
  - Protection: 

### Policy Violations

#### SQLI_DYNAMIC_QUERY: ユーザ入力を直接SQL文に埋め込み動的に実行している

- **Path**: app.py:index() -> cur.execute("select * from original_data where order_date='{}'")
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

index()関数内で、ユーザから取得したselected_dateを直接文字列連結でSQLクエリに埋め込んで実行しており、SQLインジェクション攻撃を許しています。パラメータ化されたクエリを使用せず、入力値の検証・エスケープ処理も不十分です。

## PoC（概念実証コード）

```text
// Proof of Concept: SQLインジェクションによるテーブル削除
import requests
url = 'http://<host>/'
data = {'selected_date': "2023-10-01'; DROP TABLE original_data;--"}
resp = requests.post(url, data=data)
print(resp.text)
```

## 修復ガイダンス

### index()内SQL実行

- **Required**: パラメータ化クエリへの変更
- **Guidance**: psycopg2のプレースホルダー（cur.execute(query, [param])）を利用し、文字列連結を排除してください。
- **Priority**: high

## 解析ノート

1. index()内でrequest.form['selected_date']を取得
2. .format()でSQL文字列に埋め込む実装を確認
3. パラメータ化クエリ不使用によるSQLIリスクを特定
4. PRINCIPAL: untrustedなPOSTパラメータ
5. ACTION: 動的SQL生成、実装quality=insufficient
6. RESOURCE: original_dataテーブルへのSQL実行
7. ルール違反としてSQLI検出
8. 改善策としてパラメータ化クエリを推奨

