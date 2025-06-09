# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`
- `AFO`
- `SSRF`
- `LFI`
- `RCE`
- `SQLI`
- `XSS`

## PAR Policy Analysis

## 詳細解析

このコードでは、認可チェックを行わずにリクエストパラメータから直接userIdを受け取り、そのままデータベース照会に利用しているため、攻撃者が任意のuserIdを指定すると他ユーザの機密データを取得できるIDOR脆弱性が存在します。

## PoC（概念実証コード）

```text
セッションにログインした状態で、別ユーザのIDを指定してリクエストを送信する例：

curl 'https://example.com/allocations/2?threshold=0' \
  -H 'Cookie: connect.sid=<有効なセッションID>'
```

## 解析ノート

- displayAllocations内でuserIdをセッションではなくreq.paramsから取得
- 認可チェック（自ユーザかどうかの検証）が存在しない
- IDOR脆弱性に該当
- Context Codeとして該当行を抜粋

