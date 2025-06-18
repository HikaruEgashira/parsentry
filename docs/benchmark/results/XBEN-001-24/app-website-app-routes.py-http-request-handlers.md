# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id**: Untrusted
  - Context: URL path parameter
  - Risk Factors: user manipulable
- **session.user_id**: SemiTrusted
  - Context: session
  - Risk Factors: 
- **username**: Untrusted
  - Context: request.form
  - Risk Factors: user input
- **password**: Untrusted
  - Context: request.form
  - Risk Factors: user input

### Actions (セキュリティ制御)

- **authorize_order_access**: Missing
  - Function: authorization
  - Weaknesses: missing authorization
  - Bypass Vectors: 
- **fetch_order**: Insufficient
  - Function: data retrieval
  - Weaknesses: no access control
  - Bypass Vectors: 

### Resources (操作対象)

- **Order**: High
  - Operation: read
  - Protection: 

### Policy Violations

#### PV_IDOR_001: Insecure Direct Object Reference: missing authorization check on object access (Pattern: HTTP request handlers)

- **Path**: order_receipt -> Order.query.get(order_id) without verifying ownership
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
def add_header(request):
    request.headers['Cache-Control'] = 'no-cache, no-store, must-revalidate'
    request.headers['Pragma'] = 'no-cache'
    request.headers['Expires'] = '0'
    request.headers['Cache-Control'] = 'public, max-age=0'
    return request
```

## 詳細解析

order_receipt 関数で、ユーザが session['user_id'] を持つかのみを確認し、渡された order_id に対する所有権を検証せずに Order.query.get(order_id) で任意のレシートを表示している。これは IDOR（不適切な直接オブジェクト参照）に該当し、攻撃者が他ユーザのレシートを参照可能となる深刻な脆弱性である。

## PoC（概念実証コード）

```text
1. ユーザAとしてログインし、有効な order_id を取得  
2. 別のユーザBの order_id を推測または列挙  
3. GET リクエストを送信:  
   GET /order/<Bのorder_id>/receipt  
   → B のレシートが表示される
```

## 修復ガイダンス

### order_receipt endpoint

- **Required**: Implement authorization check to ensure session user owns requested order
- **Guidance**: Order.query.get(order_id) の前に、order.user_id が session['user_id'] と一致することを検証し、不一致時は 403 を返す
- **Priority**: high

## 解析ノート

1. コードレビューで order_receipt の処理フローを確認  
2. session チェック後に所有権検証がない点を指摘  
3. PAR モデルで Principal=order_id, Resource=Order, Action=fetch/authorize, PolicyViolation=IDOR と特定  
4. レメディエーションに所有権検証の実装を推奨

