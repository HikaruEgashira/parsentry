# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **obj.attr('href')**: Untrusted
  - Context: HTML属性(data-href/href)
  - Risk Factors: 任意のURL指定可能
- **$.ajaxレスポンス(msg)**: Untrusted
  - Context: AJAX取得結果
  - Risk Factors: 外部制御可能なコンテンツ

### Actions (セキュリティ制御)

- **loadAjax -> content.html**: Missing
  - Function: HTML出力エスケープ・サニタイズ
  - Weaknesses: 未検証の外部レスポンスをそのままDOMに挿入
  - Bypass Vectors: 悪意あるHTMLを返すAJAXエンドポイント
- **loadAjax -> $.ajax(url)**: Missing
  - Function: URLの検証
  - Weaknesses: AJAXリクエスト先URLの妥当性検証不足
  - Bypass Vectors: 任意のhref指定

### Resources (操作対象)

- **content.html挿入領域**: Low
  - Operation: HTMLレンダリング
  - Protection: 

### Policy Violations

#### CWE-79: 外部レスポンスをサニタイズせずにDOMに挿入しているため、クロスサイトスクリプティング（XSS）が発生する

- **Path**: loadAjax -> content.html
- **Severity**: medium
- **Confidence**: 0.85

## 詳細解析

VenoBoxプラグインのloadAjax関数では、ユーザ操作で指定可能なhref（obj.attr('href')）をそのまま$.ajaxのURLに渡し、取得したレスポンス(msg)を無検証のままcontent.htmlで挿入しています。このため、攻撃者が制御するエンドポイントを指定すると、任意の<script>タグやイベントハンドラを含むHTMLを注入でき、XSSが成立します。

## PoC（概念実証コード）

```text
// HTMLページに以下を配置し、malicious.htmlが<script>alert(1)</script>を返すと発動
<a class="vbox-item" data-vbtype="ajax" href="/malicious.html">Open</a>
<script>$('.vbox-item').venobox();</script>
```

## 修復ガイダンス

### loadAjax

- **Required**: 取得したHTMLを必ずサニタイズしてから挿入
- **Guidance**: DOMPurifyなどを導入し、content.html(msg)の前にDOMPurify.sanitize(msg)を適用してください。また外部URLはホワイトリストで制限すると安全性が向上します。
- **Priority**: high

## 解析ノート

- obj.attr('href')から任意URLを取得
- loadAjaxでそのまま$.ajaxに渡す
- 成功コールバックでmsgを無検証にcontent.htmlに挿入
- スクリプトを含むHTMLを注入可能 => XSS
- 対策: サニタイズ, URL制限, エスケープ処理
- 検出脆弱性: XSS(CWE-79)のみ

