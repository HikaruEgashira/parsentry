# PAR Security Analysis Report

![中信頼度](https://img.shields.io/badge/信頼度-中-yellow) **信頼度スコア: 60**

## 脆弱性タイプ

- `XSS`
- `SQLI`
- `RCE`
- `LFI`
- `SSRF`
- `AFO`
- `IDOR`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

このPHPコードでは、ユーザ入力（POSTのNAME/ADDRESS）を`htmlentities`でエンティティ化した後、SQLインジェクション防止のために`mysqli_real_escape_string`を用いてDBに格納しています。しかし、DBから取得したデータをHTML出力時に再度エンコードせず、そのまま`echo`で出力しているため、万一エンコード済みでないデータや二重エンコードを回避して直接`<script>`等をDBに挿入された場合に、ストアドXSSが発生する可能性が残ります。

## PoC（概念実証コード）

```text
1. フォームに以下を入力し送信:
   NAME: <script>alert('XSS')</script>
   ADDRESS: test
2. 登録後、表示されるテーブル欄にスクリプトが実行されることを確認。
```

## 解析ノート

1. POSTのNAME/ADDRESSを`htmlentities`でエンティティ化しDB格納
2. 格納後、出力時にHTMLエスケープを行わず生出力
3. ストアドXSSの可能性を発見

