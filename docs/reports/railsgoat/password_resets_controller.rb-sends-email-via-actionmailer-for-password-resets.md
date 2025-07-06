# Security Analysis: password_resets_controller.rb - Sends email via ActionMailer for password resets

## ファイル情報

- **ファイルパス**: `repo/app/controllers/password_resets_controller.rb`
- **検出パターン**: Sends email via ActionMailer for password resets

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **params[:user]**: Untrusted
  - Context: HTTP Request Parameter
  - Risk Factors: deserialization, unsanitized
- **params[:token]**: Untrusted
  - Context: HTTP Request Parameter
  - Risk Factors: token forgery
- **params[:email]**: Untrusted
  - Context: HTTP Request Parameter
  - Risk Factors: user-controlled

### Actions (セキュリティ制御)

- **PasswordResetsController#reset_password**: Missing
  - Function: password reset
  - Weaknesses: no authentication check, unsafe deserialization
  - Bypass Vectors: deserialization payload injection
- **generate_token**: Insufficient
  - Function: token generation
  - Weaknesses: weak hash, no expiry
  - Bypass Vectors: brute-force MD5 preimage
- **is_valid?**: Insufficient
  - Function: token validation
  - Weaknesses: weak hash, no expiry
  - Bypass Vectors: MD5 collision attack

### Resources (操作対象)

- **Marshal.load**: Critical
  - Operation: object deserialization
  - Protection: 
- **User.save!**: High
  - Operation: database write
  - Protection: ActiveRecord validations
- **UserMailer.forgot_password.deliver**: Medium
  - Operation: email send
  - Protection: 

### Policy Violations

#### DESERIALIZATION-01: Unsafe deserialization of untrusted data (Pattern: Sends email via ActionMailer for password resets)

- **Path**: PasswordResetsController#reset_password -> Marshal.load(Base64.decode64(params[:user]))
- **Severity**: critical
- **Confidence**: 0.90

#### AUTH-01: Missing authentication before sensitive operation (Pattern: Sends email via ActionMailer for password resets)

- **Path**: skip_before_action :authenticated in PasswordResetsController
- **Severity**: high
- **Confidence**: 0.80

#### CRYPTO-01: Weak cryptographic algorithm for token generation (Pattern: Sends email via ActionMailer for password resets)

- **Path**: PasswordResetsController#generate_token uses Digest::MD5
- **Severity**: medium
- **Confidence**: 0.90

## マッチしたソースコード

```code
def password_reset_mailer(user)
    token = generate_token(user.id, user.email)
    UserMailer.forgot_password(user.email, token).deliver
  end
```

## 詳細解析

PasswordResetsControllerではskip_before_actionによる認証回避のまま、ユーザーから渡されたBase64化＆Marshal.loadデータをデシリアライズしており、unsafe deserializationによるRCEリスクがあります。さらにMD5によるトークン生成は予測可能・期限設定なしで強度不足です。

## PoC（概念実証コード）

```text
悪意あるペイロードを含むリクエスト例:
curl -X POST https://example.com/password_resets/reset_password \
  -d "user=$(echo 'BASTION_PAYLOAD' | base64)" \
  -d "password=newpass" -d "confirm_password=newpass"
# BASTION_PAYLOADには任意のRubyオブジェクト攻撃コードをMarshal.dumpでシリアライズ
```

## 修復ガイダンス

### PasswordResetsController#reset_password

- **Required**: ユーザー入力をMarshal.loadせず、安全なパース方式 (JSON.parse + strong parameters) を使用
- **Guidance**: サーバー側での直接的なデシリアライズをやめ、許可されたフィールドのみ受け付ける
- **Priority**: high

### generate_token

- **Required**: 予測不可能な期限付きトークンを利用
- **Guidance**: ActiveSupport::MessageVerifierやSecureRandom.urlsafe_base64とexpiryの組み合わせを使用
- **Priority**: medium

### PasswordResetsController認証

- **Required**: 認証チェックを必須化
- **Guidance**: skip_before_actionを削除し、トークン検証メソッドで権限チェックを行う
- **Priority**: high

## 解析ノート

· skip_before_actionにより認証チェック回避
· params[:user]をBase64→Marshal.loadでデシリアライズ → RCEリスク
· generate_tokenでMD5ハッシュ利用 → 予測可能かつ有効期限なし

