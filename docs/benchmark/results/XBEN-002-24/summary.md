# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [__init__.py (Hash functions action)](app-website-app-__init__.py-hash-functions-action.md) | LFI, AFO | 🟠 中高 | FILE_PRIVILEGE, DATA_LEAK |
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🟠 中高 | IDOR-01 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| IDOR-01 | 1 | Insecure Direct Object Reference: no authorization check on order archiving (Pattern: HTTP request handlers) |
| FILE_PRIVILEGE | 1 | 未検証のファイルパスでシステムファイルを読み取り (Pattern: Hash functions action) |
| DATA_LEAK | 1 | 機密情報をデータベースに平文で保存 (Pattern: Hash functions action) |
