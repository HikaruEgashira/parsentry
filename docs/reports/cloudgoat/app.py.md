# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`

## PAR Policy Analysis

## 詳細解析

アプリケーションのindex()エンドポイントにおいて、ユーザーから送信されたselected_dateをそのまま文字列フォーマットでSQLクエリに埋め込んで実行しているため、SQLインジェクション攻撃が可能です。psycopg2のパラメタライズドクエリではなく、Pythonの.format()を使用しており、入力検証やエスケープ処理が全く行われていません。

## PoC（概念実証コード）

```text
# 攻撃例: POSTリクエストで以下のpayloadを送信
# selected_date = "2023-10-01' OR '1'='1"
# これにより全レコードが取得可能
import requests
url = 'http://localhost:5000/'
data = {'selected_date': "2023-10-01' OR '1'='1"}
r = requests.post(url, data=data)
print(r.text)
```

## 修復ガイダンス

### index関数のSQL実行部分

- **Required**: パラメタライズドクエリへの変更
- **Guidance**: cur.execute("SELECT * FROM original_data WHERE order_date = %s", (selected_date,)) のようにプレースホルダを利用し、入力を直接埋め込まないでください。
- **Priority**: high

## 解析ノート

ユーザー入力(selected_date)を.format()で文字列結合したSQLに挿入→SQLI脆弱性。formatを使わず、必ずパラメタライズドクエリを利用すべき。

