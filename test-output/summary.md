# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [vulnerable.py (Detects definitions of functions named 'unsafe_query', which construct SQL queries via string interpolation)](vulnerable.py-detects-definitions-of-functions-named-unsafe-query-which-construct-sql-queries-via-string-interpolation.md) | SQLI | 🔴 高 | SQLI_001 |
| [vulnerable.py (Detects definitions of functions named 'execute_query', which execute raw SQL statements)](vulnerable.py-detects-definitions-of-functions-named-execute-query-which-execute-raw-sql-statements.md) | SQLI | 🟠 中高 | CWE-89 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| SQLI_001 | 1 | 信頼できない入力が検証なしで直接SQLクエリに埋め込まれているため、SQLインジェクションが発生する可能性があります。 (Pattern: Detects definitions of functions named 'unsafe_query', which construct SQL queries via string interpolation) |
| CWE-89 | 1 | ユーザー入力が未検証のままSQL文に埋め込まれている (Pattern: Detects definitions of functions named 'execute_query', which execute raw SQL statements) |
