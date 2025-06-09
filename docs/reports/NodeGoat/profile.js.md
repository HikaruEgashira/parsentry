# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `IDOR`
- `AFO`
- `RCE`
- `LFI`
- `SSRF`
- `SQLI`

## PAR Policy Analysis

## 詳細解析

ProfileHandler.displayProfileでは、ユーザー入力（doc.firstName）をHTMLコンテキスト用にESAPI.encoder().encodeForHTMLでエンコードし、firstNameSafeStringに格納しています。しかし、同じ変数をURL属性のコンテキスト（リンクヘッダー）でも使用しているため、HTMLエンコードではURL属性内のクォート(\")や特殊文字がブラウザの属性デコード時に復元され、属性切断・スクリプト挿入が可能です。この不適切なコンテキストでのエンコーディング選択がXSS脆弱性を生じさせています。

## PoC（概念実証コード）

```text
1. ユーザーのfirstNameを以下のように設定してプロフィール更新:\n   firstName=\" onmouseover=alert(1) x=\"\n2. displayProfileを表示すると、リンクに挿入されたfirstNameSafeStringが属性デコードされ、以下のようなHTMLが出力される:\n   <a href="/profile?name=&quot; onmouseover=alert(1) x=&quot;">...<\/a>\n3. マウスオーバーでalert(1)が発火し、XSSが確認できる。
```

## 解析ノート

・displayProfileでHTML用エンコードのみ実施
・URLコンテキストで再利用のため属性切断可能
・ESAPI.encodeForURLを使うべき


