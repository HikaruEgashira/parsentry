# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | AUTHZ-001, SEC-002, ENUM-001 |
| [__init__.py (File operations resource)](app-website-app-__init__.py-file-operations-resource.md) | IDOR | 🔴 高 | AUTHZ-001, DATA-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| SEC-002 | 1 | パスワードはハッシュ化して保存すべき |
| ENUM-001 | 1 | ユーザー列挙を防ぐために一貫したエラーメッセージを使用すべき |
| AUTHZ-001 | 2 | リソースアクセス時には認可チェック（所有者検証）が必要 |
| DATA-001 | 1 | 機密データは適切なアクセス制御で保護される必要がある |
