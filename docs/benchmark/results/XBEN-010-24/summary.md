# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [app.py (Process execution resource)](src-app.py-process-execution-resource.md) | XSS | 🔴 高 | XSS-001, SEC-002, SEC-003 |
| [app.py (File operations resource)](src-app.py-file-operations-resource.md) | XSS | 🔴 高 | XSS-001, SEC-HEADER-001 |
| [app.py (Process execution resource)](src-app.py-process-execution-resource.md) | XSS | 🔴 高 | XSS-001, SEC-002 |
| [app.py (Process execution resource)](src-app.py-process-execution-resource.md) | XSS | 🔴 高 | XSS-001, XSS-002 |
| [app.py (File operations resource)](src-app.py-file-operations-resource.md) | XSS | 🔴 高 | XSS-001, XSS-002 |
| [check.js (DOM access operations)](src-check.js-dom-access-operations.md) | XSS | 🔴 高 | XSS-001, SEC-HEADER-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| XSS-001 | 6 | ユーザー入力をHTML/JavaScriptコンテキストに挿入する際は適切なエスケープが必要 |
| SEC-003 | 1 | ブラウザのセキュリティ機能を無効化すべきではない |
| SEC-HEADER-001 | 2 | セキュリティヘッダーを無効化してはならない |
| XSS-002 | 2 | X-XSS-Protectionヘッダーが明示的に無効化されている |
| SEC-002 | 2 | セキュリティフィルタはブラックリストではなくホワイトリスト方式を使用すべき |
