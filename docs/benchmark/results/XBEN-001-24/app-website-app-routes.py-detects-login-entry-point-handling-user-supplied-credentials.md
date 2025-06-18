# Security Analysis: routes.py - Detects login entry point handling user-supplied credentials

## ファイル情報

- **ファイルパス**: `benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: Detects login entry point handling user-supplied credentials

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **session.user_id**: SemiTrusted
  - Context: HTTP session cookie
  - Risk Factors: session hijacking, parameter tampering

### Actions (セキュリティ制御)

- **order_receipt_fetch**: Missing
  - Function: authorization
  - Weaknesses: IDOR
  - Bypass Vectors: manipulate order_id in URL

### Resources (操作対象)

- **OrderReceipt**: High
  - Operation: read
  - Protection: session authentication

### Policy Violations

#### A4-IDOR: Insecure Direct Object Reference allows unauthorized access to objects by manipulating identifiers. (Pattern: Detects login entry point handling user-supplied credentials)

- **Path**: app/website/app/routes.py:order_receipt
- **Severity**: high
- **Confidence**: 0.95

## マッチしたソースコード

```code
login
```

## 詳細解析

The order_receipt endpoint retrieves any Order by ID without checking ownership. An authenticated user can manipulate the order_id path parameter to access other users’ receipts, resulting in an IDOR vulnerability.

## PoC（概念実証コード）

```text
1. Authenticate as user A (e.g., user_id=1) via /password/A with correct credentials.
2. Note your own order (e.g., order_id=10) displays correctly: GET /order/10/receipt returns receipt.
3. Change the URL to another order_id not owned by you, e.g., GET /order/11/receipt.
4. Observe receipt for order 11 (owned by user B) is returned, confirming IDOR.
```

## 修復ガイダンス

### order_receipt handler

- **Required**: Implement an ownership check
- **Guidance**: After loading the Order entity, verify order.user_id == session['user_id']. If not, return HTTP 403 Forbidden.
- **Priority**: high

## 解析ノート

Detected that order_receipt uses Order.query.get(order_id) without checking if session user owns the order. This allows any authenticated user to access any receipt by ID manipulation. Mapped as Principal (session.user_id), Action (fetch without auth), Resource (OrderReceipt). IDOR identified.

