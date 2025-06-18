# Security Analysis: routes.py - Detects functions that add security-related HTTP response headers

## ファイル情報

- **ファイルパス**: `benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: Detects functions that add security-related HTTP response headers

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **request.path.order_id**: Untrusted
  - Context: HTTPリクエストURL
  - Risk Factors: 改ざん可能
- **session.user_id**: SemiTrusted
  - Context: サーバセッション
  - Risk Factors: セッション乗っ取り

### Actions (セキュリティ制御)

- **Order.query.get(order_id)**: Missing
  - Function: authorization_check
  - Weaknesses: IDOR
  - Bypass Vectors: 

### Resources (操作対象)

- **Order(id=order_id)**: Critical
  - Operation: read
  - Protection: 

### Policy Violations

#### IDOR-001: リクエストしたユーザが対象リソースの所有者であるか検証していない (Pattern: Detects functions that add security-related HTTP response headers)

- **Path**: order_receipt
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
add_header
```

## 詳細解析

アプリケーションのorder_receiptルートにおいて、ユーザのセッションIDのみでOrder.query.get(order_id)を実行しており、リクエストパスのorder_idパラメータが信頼できない入力にもかかわらず、オーナーチェックや認可検証が欠落しているためIDORが発生しています。

## PoC（概念実証コード）

```text
1. 正常にログインし、自分の注文IDを確認
2. ブラウザで /order/<他ユーザの注文ID>/receipt にアクセス
3. 他ユーザのレシート情報が参照可能になる
```

## 修復ガイダンス

### order_receiptハンドラ

- **Required**: オーナーチェック（ユーザID照合）の実装
- **Guidance**: session['user_id'] と Order.user_id を比較し、一致しない場合は403を返却してください。
- **Priority**: high

## 解析ノート

- order_receiptでOrder.query.getを実行し認可チェックがない点を確認
- 攻撃ベクトルはURLパラメータ改ざんによるIDOR
- remediationとして所有者チェックを推奨

