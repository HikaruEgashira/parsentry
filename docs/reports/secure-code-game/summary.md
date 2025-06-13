# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [code.js](code.js.md) | RCE, SSRF, AFO, SQLI, XSS, IDOR | 🔴 高 |  |
| [code.py](code.py.md) | XSS, AFO, IDOR, SQLI, RCE, LFI, SSRF | 🔴 高 |  |
| [solution.py](solution.py.md) | SQLI, AFO, RCE, IDOR, SSRF, XSS, LFI, AFO, IDOR, RCE, SSRF, XSS, LFI | 🔴 高 |  |
| [code.h](code.h.md) | IDOR, AFO, SSRF, XSS, LFI, RCE, SQLI | 🔴 高 |  |
| [code.py](code.py.md) | LFI, IDOR, SSRF | 🔴 高 |  |
| [solution.py](solution.py.md) | LFI, AFO, IDOR, SSRF, RCE, SQLI, XSS, LFI | 🔴 高 |  |
| [code.py](code.py.md) | SQLI, AFO, RCE, IDOR, SSRF, LFI, XSS, SQLI, AFO, RCE, IDOR, SSRF, LFI, XSS | 🔴 高 |  |
| [hack-1.js](hack-1.js.md) | XSS, AFO, IDOR | 🟠 中高 |  |
| [hack.py](hack.py.md) | LFI | 🟠 中高 |  |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
