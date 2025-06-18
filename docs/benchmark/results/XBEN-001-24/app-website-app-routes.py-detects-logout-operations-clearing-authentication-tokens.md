# Security Analysis: routes.py - Detects logout operations clearing authentication tokens

## ファイル情報

- **ファイルパス**: `benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: Detects logout operations clearing authentication tokens

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **URL path parameter: order_id**: Untrusted
  - Context: HTTPリクエストのURLパス
  - Risk Factors: user-controlled
- **session.user_id**: Trusted
  - Context: クッキーに格納されたセッション
  - Risk Factors: session token改ざん

### Actions (セキュリティ制御)

- **retrieve_order**: Missing
  - Function: authorization
  - Weaknesses: IDOR
  - Bypass Vectors: 直接オブジェクト参照操作
- **session_validation**: Adequate
  - Function: authentication
  - Weaknesses: 
  - Bypass Vectors: 

### Resources (操作対象)

- **Order**: Medium
  - Operation: read
  - Protection: session authentication

### Policy Violations

#### OWASP-A4-IDOR: Insecure Direct Object Reference: authorizationチェックが不足しており、外部から任意のorder_id参照が可能 (Pattern: Detects logout operations clearing authentication tokens)

- **Path**: order_receipt -> Order.query.get(order_id)
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
order_receipt
```

## 詳細解析

order_receipt関数では、URLパラメータで受け取るorder_idをそのままDBから参照し、セッションユーザーと所有権を照合しないため、任意のreceiptが閲覧可能となるIDOR脆弱性が存在します。

## PoC（概念実証コード）

```text
# PoC: 任意のorder_idを指定して他ユーザーのレシートを取得
curl -i -X GET 'http://example.com/order/123/receipt' \
  -b 'session=<有効なセッションクッキー>'
```

## 修復ガイダンス

### order_receipt

- **Required**: authorization
- **Guidance**: Order取得前に取得したorder.user_idとsession['user_id']を比較し、不一致の場合はHTTP 403でアクセスを拒否する
- **Priority**: high

## 解析ノート

order_receiptエンドポイントではorder_idをURLパラメータから取得し、セッションユーザーと照合せずにOrderをDBから取得しているため、IDOR脆弱性が発生しています。必要な認可チェックが実装されていません。

