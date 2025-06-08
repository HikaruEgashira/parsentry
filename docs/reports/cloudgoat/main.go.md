# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `SSRF`

## PAR Policy Analysis

### Principals (データ源)

- **HTTPクエリパラメータ 'url'**: Untrusted
  - Context: r.URL.Query().Get("url")
  - Risk Factors: 未検証ユーザ入力, コマンドインジェクションリスク

### Actions (セキュリティ制御)

- **Shellコマンド実行 via exec.Command**: Insufficient
  - Function: コマンド実行
  - Weaknesses: 入力検証無しでシェルへ直渡し, シェルの-cオプション使用による文字列結合
  - Bypass Vectors: セミコロンによるコマンド分割, &&や||を用いた連鎖コマンド実行

### Resources (操作対象)

- **OS Shell Execution**: Critical
  - Operation: exec.Command
  - Protection: 

### Policy Violations

#### CMD_INJECTION: ユーザ入力を直接シェルコマンドに渡すとRCEを招く

- **Path**: handelGetRequest -> exec.Command("/bin/sh", "-c", ...)
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

ユーザが提供するクエリパラメータ「url」を検証せずに直接シェルコマンド（exec.Command("/bin/sh", "-c", "curl "+cmd)）に渡しているため、コマンドインジェクション（RCE）および任意ホストへのリクエスト実行（SSRF）が可能です。

## PoC（概念実証コード）

```text
GET /?url=example.com;id HTTP/1.1
```

## 修復ガイダンス

### handelGetRequest

- **Required**: ユーザ入力の厳格なバリデーションおよびエスケープ実装
- **Guidance**: exec.Commandでシェルラッパーを避け、引数配列でcurlバイナリを直接呼び出すか、URLホワイトリスト検証を行う
- **Priority**: high

## 解析ノート

コード読解: demo1でr.URL.Query().Get("url")によりユーザ入力を取得->handelGetRequestでexec.Command("/bin/sh","-c","curl "+cmd)を実行->サニタイズ無しの文字列連結でコマンドインジェクション(APによりRCE)および任意ホストへのリクエスト実行(SSRF)
PRINCIPAL: HTTPパラメータ 'url' は untrusted
ACTION: exec.Commandシェル実行は implementation_quality insufficient
RESOURCE: Shellコマンド実行は critical
ポリシー違反: CMD_INJECTION
修復: 引数分離 or ホワイトリスト検証

