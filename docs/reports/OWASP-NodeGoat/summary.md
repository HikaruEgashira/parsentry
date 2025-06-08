# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [contributions-dao.js](contributions-dao.js.md) | IDOR | 🔴 高 | AC-USER-OWNERSHIP, AC-USER-OWNERSHIP |
| [memos.js](memos.js.md) | XSS | 🔴 高 | XSS-001 |
| [contributions.js](contributions.js.md) | RCE | 🔴 高 | JSINJ-01 |
| [server.js](server.js.md) | XSS | 🔴 高 | A6, A8, A3, A3 |
| [allocations-dao.js](allocations-dao.js.md) | SQLI | 🟠 中高 | A1_2_NoSQL_Injection |
| [bootstrap.js](bootstrap.js.md) | XSS | 🟠 中高 | BS-POPUP-01 |
| [morris-0.4.3.min.js](morris-0.4.3.min.js.md) | XSS | 🟠 中高 |  |
| [research.js](research.js.md) | SSRF, XSS | 🟠 中高 | SSRF-01, XSS-01 |
| [allocations.js](allocations.js.md) | IDOR | 🟠 中高 | C4 |
| [config.js](config.js.md) | LFI, RCE | 🟠 中高 | NODEJS_DYNAMIC_REQUIRE_UNSAFE |
| [benefits.js](benefits.js.md) | IDOR | 🟠 中高 | AUTH-001, INPUT-002 |
| [Gruntfile.js](Gruntfile.js.md) | RCE, AFO, IDOR, XSS, LFI, SQLI, SSRF, AFO, IDOR, XSS, LFI, SQLI, XSS, SSRF, LFI, SQLI, IDOR, XSS, SSRF, LFI, SQLI, IDOR, SSRF, XSS, SQLI, LFI, IDOR, SSRF, XSS | 🟠 中高 |  |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| AC-USER-OWNERSHIP | 2 | ユーザーは自身のリソースのみ操作可能であるべき |
| A8 | 1 | CSRF保護が有効化されていない |
| A1_2_NoSQL_Injection | 1 | NoSQLインジェクションを防止するため、ユーザー入力を動的クエリコードに埋め込んではいけない |
| A3 | 2 | セッションCookieにHttpOnly/secure属性が設定されていない |
| XSS-01 | 1 | 取得したレスポンスをエスケープせずにHTML出力（XSS） |
| INPUT-002 | 1 | userIdおよびbenefitStartDateの入力バリデーションが未実装 |
| SSRF-01 | 1 | 未検証のユーザー入力による任意のURLへのリクエスト実行（SSRF） |
| C4 | 1 | IDOR: ユーザ入力による直接オブジェクト参照 |
| A6 | 1 | HTTPSを強制せずHTTPのみで通信 |
| BS-POPUP-01 | 1 | ユーザー制御のHTMLコンテンツをサニタイズせずに挿入してはならない |
| NODEJS_DYNAMIC_REQUIRE_UNSAFE | 1 | 環境変数から直接ファイルパスを動的に読み込むことはパストラバーサル及び任意コード実行のリスクを生む |
| JSINJ-01 | 1 | 信頼できない入力をevalでそのまま評価している |
| XSS-001 | 1 | ユーザー入力をエスケープせずに HTML 出力に含めている（Stored XSS） |
| AUTH-001 | 1 | 管理者権限チェックが未実装で、未認可ユーザーがリソースを更新可能 |
