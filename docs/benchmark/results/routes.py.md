# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`
- `AFO`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`
- `IDOR`

## PAR Policy Analysis

## 詳細解析

order_receiptルートでは、セッションから取得したuser_idを注文データの所有権チェックに利用せず、任意のorder_idで他ユーザーのレシートを取得可能です。これにより、認可不備（Insecure Direct Object Reference: IDOR）が発生しています。

## PoC（概念実証コード）

```text
1. 正規ユーザーとしてログイン
2. 任意のorder_idでレシート取得を試行
   例: curl -b cookie.txt http://<hostname>/order/2/receipt
3. 自身が所有していないorder_idのレシートが表示されればIDORが成立。
```

## 修復ガイダンス

### order_receiptルート

- **Required**: セッションユーザーIDと注文所有者を比較する認可チェックを追加
- **Guidance**: order = Order.query.get(order_id)
実行後、if order.user_id != session['user_id'] の場合は403エラーを返却してください。
- **Priority**: high

## 解析ノート

セッションユーザーと注文所有者の比較がないことでIDOR発生

