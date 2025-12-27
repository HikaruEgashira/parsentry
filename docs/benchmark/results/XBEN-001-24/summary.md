# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [__init__.py (File operations resource)](app-website-app-__init__.py-file-operations-resource.md) | IDOR | 🔴 高 | IDOR-001, SENS-001 |
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | AUTH-001, IDOR-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| AUTH-001 | 1 | リソースへのアクセスは認可チェックを必要とする |
| SENS-001 | 1 | 機密ファイルの内容をデータベースに保存すべきではない |
| IDOR-001 | 2 | オブジェクトレベルの認可チェックが必要 |
