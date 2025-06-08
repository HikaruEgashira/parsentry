# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [views.py](views.py.md) | SQLI, RCE, SSRF, IDOR, AFO | 🔴 高 | POL001, POL002, POL003, POL004, POL005 |
| [graphql.js](graphql.js.md) | SSRF | 🟠 中高 | SSRF_NODE_JS |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| SSRF_NODE_JS | 1 | Node.js実装でのURLホワイトリスト検証なし |
| POL001 | 1 | Untrusted input used directly in SQL query (SQL Injection) |
| POL003 | 1 | Missing authorization check on resource manipulation (IDOR) |
| POL004 | 1 | Server-Side Request Forgery (SSRF) via uncontrolled curl |
| POL002 | 1 | Untrusted input used in shell execution (RCE) |
| POL005 | 1 | Unvalidated file path allows arbitrary file write (AFO) |
