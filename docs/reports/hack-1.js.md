# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`
- `AFO`
- `IDOR`
- `SQLI`
- `RCE`
- `LFI`
- `SSRF`

## PAR Policy Analysis

## 詳細解析

この脆弱性は、CryptoAPI.sha1.hash関数が引数として渡されたオブジェクトのtoStringメソッドを検証せずに暗黙的に呼び出す点に起因しています。攻撃者はtoStringメソッドをオーバーライドしたオブジェクトを渡すことで、任意のJavaScriptコードを実行できるため、DOMベースのXSSが成立します。

## PoC（概念実証コード）

```text
var s = { toString: function() { alert('Exploit 1'); } };
CryptoAPI.sha1.hash(s);
```

## 解析ノート

ユーザー制御のオブジェクトsに悪意あるtoStringを定義し、CryptoAPI.sha1.hash内部での暗黙的呼び出しによりコード実行が可能。

