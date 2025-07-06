# Security Analysis: work_info_controller.rb - Command line arguments

## ファイル情報

- **ファイルパス**: `repo/app/controllers/work_info_controller.rb`
- **検出パターン**: Command line arguments

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `IDOR`

## PAR Policy Analysis

### Principals (データ源)

- **params[:user_id]**: Untrusted
  - Context: HTTPリクエストパラメータ
  - Risk Factors: ユーザー制御可能, 未検証入力

### Actions (セキュリティ制御)

- **Admin存在チェック**: Insufficient
  - Function: 認可チェック
  - Weaknesses: 不十分な認可チェック
  - Bypass Vectors: 直接ID参照パラメータ挿入(IDOR)

### Resources (操作対象)

- **Userレコード**: Medium
  - Operation: DB Read
  - Protection: 存在チェック, 管理者除外チェック(不十分)

### Policy Violations

#### OWASP-A5: Broken Access Control: 適切な所有権/認可チェックが欠如している (Pattern: Command line arguments)

- **Path**: WorkInfoController#index
- **Severity**: high
- **Confidence**: 0.80

## マッチしたソースコード

```code
ApplicationController
```

## 詳細解析

WorkInfoController#indexアクションでは、params[:user_id]（信頼できないユーザー入力）がそのままDB参照に使用され、所有者検証やアクセス権限チェックが実装されていません。その結果、他ユーザーの情報を不正に参照できるIDOR脆弱性が存在します。

## PoC（概念実証コード）

```text
# 攻撃者が直接他ユーザーIDを指定して情報取得
GET /work_info?user_id=2 HTTP/1.1
Host: example.com
```

## 修復ガイダンス

### 認可機構

- **Required**: リソース所有者または適切な権限を持つユーザーのみアクセス許可を与える認可チェックを実装
- **Guidance**: PunditやCanCanCanなどの認可ライブラリを利用するか、current_user.idとparams[:user_id]の一致を明示的に検証してください。
- **Priority**: high

## 解析ノート

params[:user_id]がPrincipal、User.find_byがResourceへのDBアクセス、存在チェックのみでActionが不十分。IDORを検出。

