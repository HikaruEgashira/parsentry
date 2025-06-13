# Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `AFO`
- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **console入力**: Untrusted
  - Context: ブラウザJavaScriptコンソール
  - Risk Factors: 未検証のユーザ制御オブジェクト

### Actions (セキュリティ制御)

- **CryptoAPI.sha1.hash**: Insufficient
  - Function: ハッシュ化処理
  - Weaknesses: 入力検証欠如, サニタイズ欠如
  - Bypass Vectors: JavaScriptオブジェクトのtoStringオーバーライド

### Resources (操作対象)

- **CryptoAPI.sha1.hash**: Low
  - Operation: hashing
  - Protection: 

## 詳細解析

このコードはCryptoAPI.sha1.hash関数が引数のオブジェクトに対して暗黙的にtoStringを呼び出す仕様を悪用しており、攻撃者制御下のオブジェクトのtoString内にスクリプトを埋め込むことで任意のコード実行（DOMベースのXSS）を可能にしています。入力の検証やサニタイズが一切行われておらず、JavaScriptのtoString呼び出しをバイパスできるため、深刻なXSS脆弱性です。

## PoC（概念実証コード）

```text
// JavaScriptコンソールに以下を貼り付け
var s = { toString: function() { alert('XSS'); } };
CryptoAPI.sha1.hash(s);
```

## 修復ガイダンス

### CryptoAPI.sha1.hash呼び出し前

- **Required**: 入力オブジェクトの型チェックおよび文字列化前のサニタイズ
- **Guidance**: 引数がオブジェクトの場合はtoStringを直接呼び出さず、安全なハッシュAPIを利用する、またはプリミティブ型のみ受け付ける
- **Priority**: high

## 解析ノート

ユーザ制御のオブジェクトtoStringで任意コード実行／DOMベースXSS。sanitizeMissing。

