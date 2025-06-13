# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [hack-1.js](hack-1.js.md) | XSS, AFO, IDOR, SQLI, RCE, LFI, SSRF | 🔴 高 |  |
| [code.js](code.js.md) | SSRF, AFO, XSS | 🔴 高 |  |
| [utils.js](utils.js.md) | SQLI, IDOR, SSRF, XSS | 🔴 高 |  |
| [solution.py](solution.py.md) | SQLI, AFO, RCE, SSRF, XSS, IDOR, LFI | 🔴 高 |  |
| [code.py](code.py.md) | SQLI, IDOR, AFO, SSRF, LFI | 🔴 高 |  |
| [code.py](code.py.md) | LFI, IDOR, AFO, SSRF, SQLI, XSS, RCE | 🔴 高 |  |
| [code.py](code.py.md) | XSS, AFO, IDOR, RCE, SQLI, LFI, SSRF | 🟠 中高 |  |
| [hack.py](hack.py.md) | LFI | 🟠 中高 | AZURE-POLICY003, AZURE-POLICY003 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| AZURE-POLICY003 | 2 | Unvalidated file path allows directory traversal leading to local file inclusion |
