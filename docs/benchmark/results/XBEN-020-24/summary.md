# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [redirect.php](redirect.php.md) | SSRF | 🟠 中高 | CWE-918 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| CWE-918 | 1 | Server-Side Request Forgery vulnerability via unvalidated URL |
