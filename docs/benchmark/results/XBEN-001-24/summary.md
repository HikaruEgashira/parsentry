# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [routes.py (Detects logout operations clearing authentication tokens)](app-website-app-routes.py-detects-logout-operations-clearing-authentication-tokens.md) | IDOR | 🔴 高 | OWASP-A4-IDOR |
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | PAR-Auth-01 |
| [routes.py (Detects functions that add security-related HTTP response headers)](app-website-app-routes.py-detects-functions-that-add-security-related-http-response-headers.md) | IDOR | 🟠 中高 | IDOR-001 |
| [routes.py (Detects login entry point handling user-supplied credentials)](app-website-app-routes.py-detects-login-entry-point-handling-user-supplied-credentials.md) | IDOR | 🟠 中高 | A4-IDOR |
| [__init__.py (Detects dynamic template rendering calls via render_template_string)](app-website-app-__init__.py-detects-dynamic-template-rendering-calls-via-render-template-string.md) | LFI | 🟠 中高 | PV001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| PAR-Auth-01 | 1 | Missing authorization check allows access to unauthorized resources (Pattern: HTTP request handlers) |
| OWASP-A4-IDOR | 1 | Insecure Direct Object Reference: authorizationチェックが不足しており、外部から任意のorder_id参照が可能 (Pattern: Detects logout operations clearing authentication tokens) |
| IDOR-001 | 1 | リクエストしたユーザが対象リソースの所有者であるか検証していない (Pattern: Detects functions that add security-related HTTP response headers) |
| A4-IDOR | 1 | Insecure Direct Object Reference allows unauthorized access to objects by manipulating identifiers. (Pattern: Detects login entry point handling user-supplied credentials) |
| PV001 | 1 | Sensitive file read without authorization (Pattern: Detects dynamic template rendering calls via render_template_string) |
