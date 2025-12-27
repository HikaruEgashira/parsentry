# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | IDOR-001 |
| [__init__.py (File operations resource)](app-website-app-__init__.py-file-operations-resource.md) | IDOR, LFI | 🔴 高 | AUTHZ-001, SENS-001, CRED-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| AUTHZ-001 | 1 | リソースアクセス時に所有者検証が必要 |
| CRED-001 | 1 | デフォルト認証情報を本番環境で使用すべきではない |
| IDOR-001 | 1 | ユーザーは自身のリソースにのみアクセスできるべき |
| SENS-001 | 1 | 機密ファイルの内容をデータベースに永続化すべきではない |
