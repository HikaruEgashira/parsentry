# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SSRF`
- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **request.query['url']**: Untrusted
  - Context: HTTP GET クエリパラメータ
  - Risk Factors: 完全に攻撃者制御下, 未検証の外部入力

### Actions (セキュリティ制御)

- **needle.get(url)**: Missing
  - Function: 外部/内部 HTTP リクエスト取得
  - Weaknesses: 入力バリデーション不足, 内部ネットワークアクセス制限なし
  - Bypass Vectors: 169.254.169.254/latest/meta-data アクセス, DNS rebinding による内部 IP アクセス
- **response.write(url および body)**: Missing
  - Function: ユーザ入力の HTML 出力
  - Weaknesses: HTML エスケープ処理なし, 反射型 XSS 脆弱性
  - Bypass Vectors: <script>タグ挿入

### Resources (操作対象)

- **outbound_request**: Medium
  - Operation: network_request
  - Protection: 
- **http_response**: Low
  - Operation: html_output
  - Protection: 

### Policy Violations

#### SSRF-001: ユーザ制御パラメータ経由で検証なしに外部/内部リクエストを送信

- **Path**: app.get -> needle.get(url)
- **Severity**: high
- **Confidence**: 0.90

#### XSS-001: ユーザ入力を HTML にエスケープせず出力

- **Path**: response.write('<font>…' + url + '…')
- **Severity**: medium
- **Confidence**: 0.80

## 詳細解析

本アプリケーションでは、ユーザが提供する URL パラメータを検証・制限せずに needle.get でリクエストし、そのレスポンスをそのまま HTML に埋め込んで返却しています。このため、SSRF によりメタデータサービス（169.254.169.254 など）や内部ネットワークへのアクセスが可能になるほか、ユーザ入力をエスケープせずに <font> タグ内およびレスポンスボディに埋め込むことで反射型 XSS が発生します。

## PoC（概念実証コード）

```text
1) SSRF:
curl "http://vulnerable-app/?url=http://169.254.169.254/latest/meta-data/"

2) 反射型 XSS:
curl "http://vulnerable-app/?url=<script>alert('XSS')</script>&mime=html"
```

## 修復ガイダンス

### URLパラメータ検証

- **Required**: 許可リスト方式でホスト・プロトコルを制限
- **Guidance**: Node.js の URL モジュールでパースし、プライベート IP やメタデータサービスへのアクセスをブロック
- **Priority**: high

### HTML出力エスケープ

- **Required**: ユーザ入力をエスケープまたはテンプレートエンジン使用
- **Guidance**: 返却前に lodash.escape や Handlebars などのライブラリで HTML エスケープを実施
- **Priority**: medium

## 解析ノート

1. request.query['url'] を untrusted と特定
2. needle.get(url) による外部/内部アクセスを検出 -> SSRF
3. response.write で URL とボディをエスケープなしに埋め込み -> XSS
4. PAR モデルで principals, actions, resources を整理
5. ポリシー違反ルール定義と severity・confidence を付与

