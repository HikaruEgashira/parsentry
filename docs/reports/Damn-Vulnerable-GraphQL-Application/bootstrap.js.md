# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **EndUserInput**: Untrusted
  - Context: data-htmlツールチップ/ポップオーバーのcontent
  - Risk Factors: HTML/JSコード含有可能, 外部制御可能

### Actions (セキュリティ制御)

- **Tooltip.setElementContent**: Insufficient
  - Function: 入力サニタイズ/エスケープ
  - Weaknesses: サニタイズ未実施時に直接html()を使用, ホワイトリスト実装が任意
  - Bypass Vectors: html:true, sanitize:falseを設定, ホワイトリストにない属性やスタイル挿入

### Resources (操作対象)

- **DOM via innerHTML**: Medium
  - Operation: client-side DOM挿入
  - Protection: sanitizeHtml (任意)

### Policy Violations

#### XSS.VULN.UNSAFE_HTML: untrusted inputをinnerHTMLへ挿入してはいけない

- **Path**: Tooltip.show -> setElementContent -> jQuery.html
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

BootstrapのsanitizeHtml関数とTooltip/Popoverコンポーネントにおいて、untrustedなユーザ入力をinnerHTML経由でDOMに挿入する際、htmlオプションが有効かつsanitizeがfalseの場合にサニタイズ処理をバイパスできるため、XSS攻撃を許してしまいます。

## PoC（概念実証コード）

```text
<button id="btn" data-toggle="tooltip" data-html="true" title="<img src=x onerror=alert(1)>">Hover me</button>
<script>$('#btn').tooltip({html:true, sanitize:false}).tooltip('show');</script>
```

## 修復ガイダンス

### Tooltip/Popover

- **Required**: html:trueかつsanitize:falseの組み合わせ禁止
- **Guidance**: sanitizeオプションをtrue固定、もしくはhtmlを許可しない設定とし、必ずsanitizeHtml経由でホワイトリスト検証を行う
- **Priority**: high

## 解析ノート

Bootstrap4のTooltip/Popoverでは、htmlオプションとsanitizeオプションを組み合わせることで任意のHTMLがDOMに挿入可能となる状況を確認。sanitize:false時はjQuery.html()に直接渡される。これがXSSの発生点となる。対策として必ずsanitize=trueとし、sanitizeHtmlでホワイトリスト検証を強制する必要がある。

