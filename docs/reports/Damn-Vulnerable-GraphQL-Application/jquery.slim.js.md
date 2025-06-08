# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **ユーザー入力(htmlString, code)**: Untrusted
  - Context: $('selector').append(...), jQuery.parseHTML, jQuery.globalEval 呼び出し
  - Risk Factors: untrusted HTML/JS

### Actions (セキュリティ制御)

- **DOM 挿入／スクリプト実行**: Missing
  - Function: HTML/JS 挿入の検証・サニタイズ
  - Weaknesses: 入力サニタイズ欠如, スクリプトタグ無条件実行
  - Bypass Vectors: 任意の HTML/JS を挿入

### Resources (操作対象)

- **ブラウザ DOM／グローバルスクリプト評価**: Medium
  - Operation: innerHTML 挿入 / script eval
  - Protection: 

### Policy Violations

#### JSXSS: 不正な HTML/JS を無検証で挿入すると XSS が成立する

- **Path**: domManip → buildFragment → DOMEval / append / globalEval
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

jquery.slim.js 内の DOMEval や jQuery.globalEval、domManip→buildFragment などの経路で、ユーザー提供の HTML/JS 文字列を検証・サニタイズなしに innerHTML 挿入や script タグ生成・評価を行っているため、XSS が成立する可能性があります。

## PoC（概念実証コード）

```text
// XSS PoC
// untrusted HTML を挿入すると onerror が動作
jQuery('body').append("<img src=x onerror=alert('XSS')>");
// untrusted コードを評価すると任意実行可能
jQuery.globalEval("alert('XSS')");
```

## 修復ガイダンス

### DOM 操作・スクリプト評価部分

- **Required**: ユーザー提供 HTML/JS を受け取る前に厳格にエスケープまたはサニタイズ
- **Guidance**: DOM 挿入には text() を利用し、許可タグのみをホワイトリスト化した sanitizer を挟んでから innerHTML を操作してください。globalEval は廃止、または信頼済み経路のみ利用に制限してください。
- **Priority**: high

## 解析ノート

1. ユーザー提供の htmlString/code が principal (untrusted)
2. domManip→buildFragment→innerHTML 挿入、DOMEval/globalEval が action (missing サニタイズ)
3. リソースはブラウザ DOM とグローバルスクリプト実行環境 (sensitivity=medium)
4. 入力検証・エスケープなし → XSS 脆弱性(policy JSXSS)
5. PoC: $('body').append("<img onerror=alert>") / globalEval により alert 発火
6. 対策: sanitizer 挿入・text() 利用・globalEval 廃止

