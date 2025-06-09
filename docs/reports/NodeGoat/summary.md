# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [contributions-dao.js](contributions-dao.js.md) | IDOR | 🔴 高 |  |
| [research.js](research.js.md) | SSRF, IDOR, AFO, XSS | 🔴 高 |  |
| [allocations.js](allocations.js.md) | IDOR, AFO, SSRF, LFI, RCE, SQLI, XSS | 🔴 高 |  |
| [Gruntfile.js](Gruntfile.js.md) | RCE, AFO, IDOR, SSRF, LFI, SQLI, XSS | 🟠 中高 |  |
| [bootstrap-tour.js](bootstrap-tour.js.md) | XSS, AFO, IDOR, LFI, RCE, SSRF, SQLI | 🟠 中高 |  |
| [allocations-dao.js](allocations-dao.js.md) | SQLI | 🟠 中高 | CWE-743 |
| [benefits.js](benefits.js.md) | IDOR | 🟠 中高 | OWASP-A01-IDOR |
| [contributions.js](contributions.js.md) | RCE | 🟠 中高 | SSJS-INSECURE-EVAL |
| [profile.js](profile.js.md) | XSS, IDOR, AFO, RCE, LFI, SSRF, SQLI | 🟠 中高 |  |
| [memos.js](memos.js.md) | XSS, AFO, IDOR, RCE, LFI, SSRF, SQLI | 🟠 中高 |  |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| OWASP-A01-IDOR | 1 | 非管理者ユーザーによる任意レコード更新を防ぐ認可チェックがない |
| SSJS-INSECURE-EVAL | 1 | Insecure use of eval() allows arbitrary server‐side code execution |
| CWE-743 | 1 | Improper neutralization of input in MongoDB $where – NoSQL injection |
