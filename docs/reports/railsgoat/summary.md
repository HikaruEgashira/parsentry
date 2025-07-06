# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [password_resets_controller.rb (Validates the structure and integrity of password reset tokens)](password_resets_controller.rb-validates-the-structure-and-integrity-of-password-reset-tokens.md) | RCE | 🔴 高 | DSLRUBY001 |
| [work_info_controller.rb (Command line arguments)](work_info_controller.rb-command-line-arguments.md) | IDOR | 🟠 中高 | PAR001 |
| [work_info_controller.rb (Command line arguments)](work_info_controller.rb-command-line-arguments.md) | IDOR | 🟠 中高 | OWASP_A5 |
| [dashboard_controller.rb (Command line arguments)](dashboard_controller.rb-command-line-arguments.md) | AFO, RCE | 🟠 中高 | PAR_001 |
| [dashboard_controller.rb (Command line arguments)](dashboard_controller.rb-command-line-arguments.md) | RCE, AFO | 🟠 中高 | PR001 |
| [password_resets_controller.rb (Sends email via ActionMailer for password resets)](password_resets_controller.rb-sends-email-via-actionmailer-for-password-resets.md) | RCE | 🟠 中高 | DESERIALIZATION-01, AUTH-01, CRYPTO-01 |
| [work_info_controller.rb (Command line arguments)](work_info_controller.rb-command-line-arguments.md) | IDOR | 🟠 中高 | OWASP-A5 |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| PR001 | 1 | 動的メソッド呼び出しに対する入力検証・ホワイトリストが未実装 (Pattern: Command line arguments) |
| CRYPTO-01 | 1 | Weak cryptographic algorithm for token generation (Pattern: Sends email via ActionMailer for password resets) |
| PAR001 | 1 | リクエストパラメータを直接データベースクエリに使用し、リソース所有確認を行っていない (Pattern: Command line arguments) |
| OWASP-A5 | 1 | Broken Access Control: 適切な所有権/認可チェックが欠如している (Pattern: Command line arguments) |
| DSLRUBY001 | 1 | Untrusted input passed to unsafe deserialization (Pattern: Validates the structure and integrity of password reset tokens) |
| OWASP_A5 | 1 | Broken Access Control - Missing authorization check (Pattern: Command line arguments) |
| DESERIALIZATION-01 | 1 | Unsafe deserialization of untrusted data (Pattern: Sends email via ActionMailer for password resets) |
| PAR_001 | 1 | 未検証のパラメータによる動的メソッド呼び出し (Pattern: Command line arguments) |
| AUTH-01 | 1 | Missing authentication before sensitive operation (Pattern: Sends email via ActionMailer for password resets) |
