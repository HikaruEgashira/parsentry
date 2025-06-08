# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [main.py](main.py.md) | SQLI | 🔴 高 |  |
| [index.js](index.js.md) | RCE | 🔴 高 | RCE001 |
| [app.py](app.py.md) | SQLI | 🔴 高 | SQLI-001 |
| [app.js](app.js.md) | SSRF, XSS | 🔴 高 | PV-SSRF-001, PV-XSS-001 |
| [_termui_impl.py](_termui_impl.py.md) | RCE, IDOR, SSRF | 🔴 高 |  |
| [rebuild.py](rebuild.py.md) | LFI, RCE, SSRF, AFO, SQLI, XSS, IDOR | 🔴 高 |  |
| [cli.py](cli.py.md) | SQLI | 🔴 高 | SQLI-001 |
| [iam.tf](iam.tf.md) | RCE | 🔴 高 | PV001 |
| [resource_cleaning.sh](resource_cleaning.sh.md) | RCE | 🔴 高 | SHELL_INJECTION |
| [app.py](app.py.md) | SQLI | 🔴 高 |  |
| [main.go](main.go.md) | RCE, SSRF | 🔴 高 | CMD_INJECTION |
| [tfstate.py](tfstate.py.md) | LFI | 🟠 中高 | LFI-001 |
| [db.py](db.py.md) | SQLI, IDOR, SSRF, LFI, RCE, XSS, AFO | 🟠 中高 |  |
| [db.py](db.py.md) | SQLI | 🟠 中高 | SQLI001 |
| [_termui_impl.py](_termui_impl.py.md) | RCE | 🟠 中高 | RCE1 |
| [s3.tf](s3.tf.md) | AFO | 🟠 中高 | AWS.S3.PublicWrite |
| [ETL_JOB.py](ETL_JOB.py.md) | SSRF | 🟠 中高 |  |
| [s3.tf](s3.tf.md) | AFO, IDOR, RCE | 🟠 中高 |  |
| [datediff.py](datediff.py.md) | LFI | 🟠 中高 | POL001 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| PV-SSRF-001 | 1 | 未検証のユーザー指定URLから内部リソースへのリクエスト(SSRF) |
| PV001 | 1 | local-execにおける入力未検証によるコマンドインジェクション(RCE) |
| RCE1 | 1 | 未検証の外部入力をshell=Trueやos.systemで実行し、RCEを引き起こす |
| SQLI-001 | 2 | Untrusted input used in SQL query without sanitization |
| PV-XSS-001 | 1 | ユーザー入力をHTMLエスケープせずに返却(反射型XSS) |
| CMD_INJECTION | 1 | ユーザ入力を直接シェルコマンドに渡すとRCEを招く |
| SQLI001 | 1 | ORDER BY句の動的フォーマットでSQLインジェクションを許容している |
| POL001 | 1 | 信頼できない入力をファイルパスとして直接使用している |
| RCE001 | 1 | Untrusted input passed to execSync, leading to RCE |
| AWS.S3.PublicWrite | 1 | S3バケットがパブリックにPutObjectを許可している |
| LFI-001 | 1 | パス検証なしでのファイル読み取りによりLFIが発生 |
| SHELL_INJECTION | 1 | 未検証の外部入力がシェルコマンドに直接渡されている |
