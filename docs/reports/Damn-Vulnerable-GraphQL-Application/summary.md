# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [helpers.py](helpers.py.md) | RCE, AFO, LFI | 🔴 高 | RCE-001, AFO-002, LFI-003 |
| [views.py](views.py.md) | RCE, SSRF, AFO, SSRF, SQLI, XSS, IDOR, LFI | 🔴 高 |  |
| [jquery.slim.js](jquery.slim.js.md) | XSS | 🔴 高 | JSXSS |
| [jquery.js](jquery.js.md) | XSS | 🟠 中高 | XSS001 |
| [bootstrap.js](bootstrap.js.md) | XSS | 🟠 中高 | XSS.VULN.UNSAFE_HTML |
| [graphql.js](graphql.js.md) | SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF, SSRF | 🟠 中高 |  |
| [jquery.min.js](jquery.min.js.md) | XSS | 🟠 中高 | JQ-001, JQ-002, JQ-003 |
| [jquery.slim.min.js](jquery.slim.min.js.md) | XSS | 🟠 中高 | XSS-UNSAFE-DOM |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| XSS001 | 1 | ユーザー入力をサニタイズせずinnerHTML/globalEvalに渡すことでスクリプトが実行される |
| LFI-003 | 1 | filename を検証せずファイルパス連結して書き込みしている |
| JQ-002 | 1 | JSONP コールバック名を検証せずに動的に URL に埋め込んでいる |
| RCE-001 | 1 | 外部入力を検証せずOSコマンドを直接実行している |
| XSS.VULN.UNSAFE_HTML | 1 | untrusted inputをinnerHTMLへ挿入してはいけない |
| JQ-001 | 1 | 信頼できない入力を直接 innerHTML や script タグに渡している |
| JQ-003 | 1 | globalEval で未検証文字列を eval 相当のコード実行している |
| AFO-002 | 1 | JWT 署名検証を無効化してデコードしている |
| XSS-UNSAFE-DOM | 1 | Untrusted input を innerHTML に直接割り当てている |
| JSXSS | 1 | 不正な HTML/JS を無検証で挿入すると XSS が成立する |
