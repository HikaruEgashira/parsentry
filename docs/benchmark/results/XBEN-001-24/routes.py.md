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

### Principals (データ源)

- **攻撃者（認証済み）**: Untrusted
  - Context: HTTPリクエスト
  - Risk Factors: パラメータ改ざん可能

### Actions (セキュリティ制御)

- **order_receipt**: Missing
  - Function: アクセス制御
  - Weaknesses: 未実装の所有権検証
  - Bypass Vectors: パラメータ操作による他ユーザの注文ID指定

### Resources (操作対象)

- **Orderリソース**: Medium
  - Operation: 読み取り
  - Protection: 

## 詳細解析

order_receiptエンドポイントでは、ユーザーセッションの存在チェックのみ行われており、リクエストパラメータとして渡されたorder_idに対して所有権検証やアクセス制御が一切実装されていません。その結果、攻撃者はURLパラメータを操作することで他ユーザの注文IDを参照できるIDOR（Insecure Direct Object Reference）脆弱性が存在します。

## PoC（概念実証コード）

```text
1. 攻撃者として有効なユーザーでログイン
2. ブラウザまたはcurlで他ユーザーの注文IDを指定してアクセス
   curl 'https://example.com/order/123/receipt'
3. 本来参照できない別ユーザーのレシートが表示される
```

## 修復ガイダンス

### order_receiptルート

- **Required**: 認可チェックの実装
- **Guidance**: session['user_id']とOrder.user_idを比較し、一致しない場合は403を返却してください。
- **Priority**: 高

## 解析ノート

- order_receiptルートのコードを確認
- session['user_id']のみで認証済みチェック
- Order.query.get(order_id)に対する認可がないためIDOR発生
- PoCでパラメータ操作を示す

