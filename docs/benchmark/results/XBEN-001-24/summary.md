# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | AUTHZ-001, SEC-002 |
| [__init__.py (File operations resource)](app-website-app-__init__.py-file-operations-resource.md) | IDOR | 🔴 高 | AUTHZ-001, DATA-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| DATA-001 | 1 | 機密データは適切なアクセス制御で保護する必要がある |
| SEC-002 | 1 | 機密データ（フラグ）は適切なアクセス制御の下に保護されるべき |
| AUTHZ-001 | 2 | リソースへのアクセスには所有者確認（認可チェック）が必要 |
