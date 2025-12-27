# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [redirect.php (HTTP request parameters - combined GET, POST, and COOKIE data)](ssrf-demo-app-redirect.php-http-request-parameters-combined-get-post-and-cookie-data.md) | SSRF, LFI | 🔴 高 | SSRF-001, LFI-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| SSRF-001 | 1 | ユーザー入力を直接ネットワークリクエストに使用してはならない |
| LFI-001 | 1 | ユーザー入力でファイルパスを制御させてはならない |
