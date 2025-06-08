# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [compile.js](compile.js.md) | AFO | 🔴 高 | FILE_WRITE_NO_AUTHZ |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| FILE_WRITE_NO_AUTHZ | 1 | ファイル書き込みは認可・パス検証を要する |
