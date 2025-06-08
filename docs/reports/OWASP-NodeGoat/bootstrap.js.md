# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **HTMLAttribute(data-content)**: Untrusted
  - Context: DOM要素のdata-content属性
  - Risk Factors: ユーザー制御入力, XSS注入の可能性

### Actions (セキュリティ制御)

- **popover.setContent**: Missing
  - Function: コンテンツ挿入
  - Weaknesses: サニタイズなしのhtml()呼び出し
  - Bypass Vectors: data-content属性によるスクリプトタグ挿入, オプションhtml:true設定

### Resources (操作対象)

- **popover inner HTML container**: Low
  - Operation: DOM innerHTML update
  - Protection: 

### Policy Violations

#### BS-POPUP-01: ユーザー制御のHTMLコンテンツをサニタイズせずに挿入してはならない

- **Path**: popover.setContent → html()
- **Severity**: medium
- **Confidence**: 0.80

## 詳細解析

BootstrapのPopover/Tooltip機能では、data-content属性やオプションで渡されたcontentをそのままhtml()メソッドで挿入しており、ユーザー制御の属性値をサニタイズせずにDOMへ注入するため、XSSのリスクがあります。

## PoC（概念実証コード）

```text
<button id="btn" data-toggle="popover" data-content="<img src=x onerror=alert('XSS')>">Click me</button>
<script>$('#btn').popover({html:true}).popover('show');</script>
```

## 修復ガイダンス

### popover.js

- **Required**: ユーザー制御のHTMLコンテンツを挿入前に必ずサニタイズする
- **Guidance**: DOMPurifyなどのライブラリでdata-contentをサニタイズするか、必要に応じてtext()を使用し、外部入力を直接html()に渡さない
- **Priority**: high

## 解析ノート

・Popover/Tooltipでhtml()を使い、data-contentやオプションcontentをそのまま挿入している点を確認
・外部制御可能な属性から生のHTMLを注入しておりXSSの典型パターン
・サニタイズ処理が存在しないため実装品質は'missing'
・低機密性のDOM操作だが、XSS攻撃により完全性/可用性に影響
・ポリシールール: 「ユーザー制御HTMLはサニタイズ必須」違反
・Remediation: サニタイズ or text()に変更、DOMPurify導入など

