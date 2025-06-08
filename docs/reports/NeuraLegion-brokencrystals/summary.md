# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [venobox.js](venobox.js.md) | XSS | 🟠 中高 | CWE-79 |
| [venobox.min.js](venobox.min.js.md) | XSS | 🟠 中高 | NO_UNSANITIZED_HTML, NO_ATTRIBUTE_ESCAPING |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| NO_UNSANITIZED_HTML | 1 | innerHTML等での未サニタイズ文字列埋め込みによるXSS |
| NO_ATTRIBUTE_ESCAPING | 1 | 動的生成HTMLにおける属性値未エスケープ |
| CWE-79 | 1 | 外部レスポンスをサニタイズせずにDOMに挿入しているため、クロスサイトスクリプティング（XSS）が発生する |
