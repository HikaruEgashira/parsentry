# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [routes.py (HTTP request handlers)](app-website-app-routes.py-http-request-handlers.md) | IDOR | 🔴 高 | PV_IDOR_001 |
| [routes.py (Detects functions taking an order_id parameter from user input)](app-website-app-routes.py-detects-functions-taking-an-order-id-parameter-from-user-input.md) | IDOR | 🔴 高 | AC-1 |
| [__init__.py (Security utilities action)](app-website-app-__init__.py-security-utilities-action.md) | LFI | 🔴 高 | P-PRIV-01 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| PV_IDOR_001 | 1 | Insecure Direct Object Reference: missing authorization check on object access (Pattern: HTTP request handlers) |
| AC-1 | 1 | IDOR脆弱性 – 不正な直接オブジェクト参照 (Pattern: Detects functions taking an order_id parameter from user input) |
| P-PRIV-01 | 1 | Sensitive file reads must be protected by authorization and not exposed to application logic (Pattern: Security utilities action) |
