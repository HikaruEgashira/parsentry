# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **ブラウザコンソール(ユーザー入力)**: Untrusted
  - Context: エンドユーザーのブラウザコンソールからのスクリプト実行
  - Risk Factors: プロトタイプ汚染, 信頼されていない入力によるSetter上書き

### Actions (セキュリティ制御)

- **CryptoAPI.sha1.hash**: Insufficient
  - Function: SHA-1ハッシュ生成
  - Weaknesses: プロトタイプ汚染対策の欠如, Arrayへの直接代入操作
  - Bypass Vectors: Array.prototype.__defineSetter__によるインジェクション

### Resources (操作対象)

- **Arrayインスタンスのindex 0への代入**: Low
  - Operation: 配列要素書き込み
  - Protection: 

### Policy Violations

#### JS-PP-001: Array.prototypeへのSetter汚染を防止するべき

- **Path**: CryptoAPI.sha1.hash -> Array[0]書き込み
- **Severity**: medium
- **Confidence**: 0.90

## 詳細解析

CryptoAPI.sha1.hash関数内でArray[0]への代入が行われており、攻撃者がブラウザコンソールなどからArray.prototypeのsetterを上書きできるため、関数実行時に任意コード（アラート）が実行されます。これはクライアントサイドでのプロトタイプ汚染を利用したクロスサイトスクリプティング(XSS)に相当する脆弱性です。

## PoC（概念実証コード）

```text
Array.prototype.__defineSetter__("0", function() { alert('Exploit 3'); });
CryptoAPI.sha1.hash("abc");
```

## 修復ガイダンス

### CryptoAPI.sha1.hash

- **Required**: プロトタイプ汚染対策の実装
- **Guidance**: typed arrayやバッファ専用オブジェクトを使用し、Array.prototypeへの依存を排除する。Object.freezeでプロトタイプの変更を防止。
- **Priority**: high

## 解析ノート

1. コード内ではCryptoAPI.sha1.hash呼び出し時にArray[0]への書き込みが行われていることを確認
2. 攻撃シナリオとして攻撃者がコンソールからArray.prototypeのsetterを上書きできることを想定
3. 結果として任意コード実行(alert)が発生し、XSS相当の脆弱性となる
4. PARモデルでPrincipalはコンソール入力(untrusted)、Actionはhash関数(insufficient)、Resourceは配列書き込み(low)、ポリシー違反を報告
5. 対策としてプロトタイプ汚染防止とTypedArray利用を提案」,

