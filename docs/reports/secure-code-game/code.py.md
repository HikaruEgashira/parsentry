# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SQLI`
- `AFO`
- `RCE`
- `IDOR`
- `SSRF`
- `LFI`
- `XSS`
- `SQLI`
- `AFO`
- `RCE`
- `IDOR`
- `SSRF`
- `LFI`
- `XSS`

## PAR Policy Analysis

## 詳細解析

The application exposes multiple SQL injection points by concatenating or directly executing user-controlled input without proper parameterization or sanitization. 

• get_stock_price: If the input contains a semicolon, executescript() is called, allowing arbitrary statements.
• update_stock_price: The symbol is inserted via string formatting without validation.
• exec_multi_query: Splits on semicolons and executes each fragment with cur.execute(), allowing multi-statement injection.
• exec_user_script: Uses executescript() when semicolons are present, permitting arbitrary SQL.

## PoC（概念実証コード）

```text
from code import DB_CRUD_ops
# Example: drop the stocks table via get_stock_price
print(DB_CRUD_ops().get_stock_price("MSFT'; DROP TABLE stocks; --"))
```

## 解析ノート

Found SQL injection in multiple methods: get_stock_price, update_stock_price, exec_multi_query, exec_user_script; all use unsanitized input in SQL.

