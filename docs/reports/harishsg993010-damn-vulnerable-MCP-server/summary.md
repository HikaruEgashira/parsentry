# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [server_sse.py](server_sse.py.md) | RCE, AFO, LFI | 🔴 高 |  |
| [server.py](server.py.md) | RCE | 🔴 高 | CCE001, CCE001, CCE001, CCE001 |
| [server.py](server.py.md) | LFI, IDOR | 🔴 高 | EXCESSIVE_PERMISSION_SCOPE |
| [server_sse.py](server_sse.py.md) | LFI, AFO | 🔴 高 | PRIV-001, VALIDATION-002 |
| [server.py](server.py.md) | RCE, LFI, IDOR, AFO, SSRF, SQLI, XSS | 🔴 高 |  |
| [server.py](server.py.md) | RCE, LFI | 🔴 高 | MCP-RCE-001, MCP-RCE-002, MCP-LFI-001 |
| [server_sse.py](server_sse.py.md) | RCE | 🔴 高 | PY-EVAL-001 |
| [server.py](server.py.md) | AFO | 🔴 高 | NO_SANITIZE_01 |
| [server.py](server.py.md) | IDOR | 🟠 中高 | APP-IDO-001 |
| [utils.py](utils.py.md) | LFI | 🟠 中高 |  |
| [server_sse.py](server_sse.py.md) | LFI | 🟠 中高 | P1, P2 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| MCP-RCE-001 | 1 | 未検証ユーザ入力によるPythonコード実行（RCE） |
| VALIDATION-002 | 1 | パス検証不足: ディレクトリトラバーサル対策なし |
| CCE001 | 4 | ユーザー入力をエスケープせずにshell=Trueで渡している |
| PRIV-001 | 1 | 過剰な権限: ユーザからの任意ファイル操作を認可せず実行可能 |
| P1 | 1 | 未検証のユーザ入力によるファイルアクセス（パストラバーサル）を許可 |
| MCP-LFI-001 | 1 | 任意パスのファイル読み取りによる機密情報漏えい（LFI） |
| P2 | 1 | 認可チェックなしでシステム情報を公開 |
| EXCESSIVE_PERMISSION_SCOPE | 1 | ツールが意図しないプライベートリソースにアクセス可能 |
| MCP-RCE-002 | 1 | 未検証ユーザ入力によるシェルコマンド実行（OSコマンドインジェクション） |
| APP-IDO-001 | 1 | 認証・認可なしでの直接オブジェクト参照(IDOR)を禁止する |
| PY-EVAL-001 | 1 | Unsanitized use of eval leading to RCE |
| NO_SANITIZE_01 | 1 | ユーザー入力は必ずサニタイズまたは検証を行う必要がある |
