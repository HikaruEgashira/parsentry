# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [code.js](code.js.md) | RCE, LFI, SSRF, AFO, SQLI, XSS, IDOR | 🔴 高 |  |
| [utils.js](utils.js.md) | SQLI, IDOR | 🔴 高 | DB001 |
| [solution.c](solution.c.md) | AFO | 🔴 高 | ARR-02 |
| [code.py](code.py.md) | SQLI | 🔴 高 | SQLI-1, SQLI-1, SQLI-1, SQLI-1 |
| [hack.py](hack.py.md) | LFI | 🔴 高 | PT-01, PT-01 |
| [hack.c](hack.c.md) | AFO | 🟠 中高 | AUTH-01 |
| [code.py](code.py.md) | LFI | 🟠 中高 | FILE_TRAVERSAL, UNVALIDATED_FILE_PATH |
| [hack-3.js](hack-3.js.md) | XSS | 🟠 中高 | JS-PP-001 |
| [code.py](code.py.md) | XSS, AFO, IDOR, LFI, RCE, SSRF, SQLI | 🟠 中高 |  |
| [code.h](code.h.md) | IDOR | 🟠 中高 | AUTHZ-001, AUTHZ-001, AUTHZ-001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| JS-PP-001 | 1 | Array.prototypeへのSetter汚染を防止するべき |
| AUTHZ-001 | 3 | ユーザーは自身のリソースのみ操作可能であるべき |
| AUTH-01 | 1 | 未認可のユーザーが保護された管理者フラグを変更できている |
| ARR-02 | 1 | 配列インデックスは0以上かつ上限未満を厳格に検証する必要がある |
| FILE_TRAVERSAL | 1 | 不適切なファイルパス検証によるディレクトリトラバーサル |
| DB001 | 1 | 未検証SQLクエリの実行によりデータベースから任意データが漏洩する |
| PT-01 | 2 | ユーザー制御のパスにより任意ファイル読み取りが可能 |
| SQLI-1 | 4 | 動的SQLに未検証のユーザー入力を含みパラメータ化していない |
| UNVALIDATED_FILE_PATH | 1 | 入力パス未検証による任意ファイル読み込み |
