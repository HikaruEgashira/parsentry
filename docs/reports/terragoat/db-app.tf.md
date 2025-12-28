# Security Analysis: db-app.tf - Lifecycle management

## ファイル情報

- **ファイルパス**: `/tmp/terragoat/terraform/aws/db-app.tf`
- **検出パターン**: Lifecycle management

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 100**

## 脆弱性タイプ

- `SQLI`
- `AFO`
- `XSS`
- `IDOR`
- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **var.password**: Untrusted
  - Context: Terraform変数（ユーザー入力）- 平文で状態ファイルに保存される
  - Risk Factors: 平文での保存と送信, ユーザーデータ内に埋め込まれる, CloudTrail/CloudWatchログに記録の可能性, EC2メタデータサービスからアクセス可能
- **$_POST['NAME']**: Untrusted
  - Context: WebフォームのPOSTパラメータ（ユーザー入力）
  - Risk Factors: htmlentities()のみの処理（XSS対策のみ）, SQLインジェクション対策不足, mysqlprepare()を使用していない
- **$_POST['ADDRESS']**: Untrusted
  - Context: WebフォームのPOSTパラメータ（ユーザー入力）
  - Risk Factors: htmlentities()のみの処理, SQL Injectionペイロード注入が可能
- **aws_db_instance.default.endpoint**: SemiTrusted
  - Context: RDSエンドポイント（インターネット公開）
  - Risk Factors: publicly_accessible=trueで外部からアクセス可能, セキュリティグループの制限不足

### Actions (セキュリティ制御)

- **htmlentities()**: Insufficient
  - Function: HTMLエスケープ（XSS対策）
  - Weaknesses: SQLインジェクション対策機能がない, mysqli_real_escape_string()も使用されているが、プリペアドステートメントより劣る, HTMLコンテキストのみの保護
  - Bypass Vectors: character setの異なる文字を使用した文字コード攻撃, バイナリセーフでない場合のnull byteインジェクション, SQLインジェクションは完全に迂回可能
- **mysqli_real_escape_string()**: Insufficient
  - Function: SQLインジェクション防止（エスケープベース）
  - Weaknesses: 推奨: プリペアドステートメントを使用すべき, 特定の文字セットでバイパス可能, GBKなどのマルチバイト文字セットでの脆弱性（CVE-2012-2663）
  - Bypass Vectors: マルチバイト文字コード攻撃, バックスラッシュの文字セット検出回避
- **user_data（Terraform）**: Missing
  - Function: インスタンス初期化スクリプト
  - Weaknesses: 秘密情報（パスワード）の平文埋め込み, ユーザーデータはBase64エンコードのみで暗号化されない, EC2 APIで読取可能
  - Bypass Vectors: EC2インスタンスメタデータサービス（IMDS v1）からのアクセス, Terraformコード/状態ファイルの読取
- **lifecycle ignore_changes**: Bypassed
  - Function: 状態変更の追跡
  - Weaknesses: パスワード変更をTerraform上で無視, セキュリティ監査追跡性の欠落, パスワード更新の検出不可
  - Bypass Vectors: 攻撃者はパスワード変更を実施してもTerraformレベルでは検出不可

### Resources (操作対象)

- **aws_db_instance.default**: Critical
  - Operation: RDSデータベースサーバー
  - Protection: セキュリティグループ（VPC CIDR限定）, 認証（ユーザー名/パスワード）
- **aws_instance.db_app（user_data内のPHPアプリケーション）**: Critical
  - Operation: Webアプリケーション + データベースクライアント
  - Protection: htmlentities()（XSS対策）, mysqli_real_escape_string()（SQLインジェクション対策・不十分）, 基本認証なし
- **IAM Role（ec2role）**: Critical
  - Operation: AWS認証・認可
  - Protection: AssumeRolePolicy（EC2のみ）
- **Terraform state file**: Critical
  - Operation: 状態管理
  - Protection: ローカルファイルシステム（デフォルト）

### Policy Violations

#### CREDENTIAL_EXPOSURE_001: 機密情報の平文埋め込み禁止

- **Path**: var.password (Principal) -> user_data (Action) -> Terraform state + EC2 metadata (Resource)
- **Severity**: critical
- **Confidence**: 1.00

#### SQLI_002: SQLインジェクション脆弱性

- **Path**: $_POST['NAME']/$_POST['ADDRESS'] (Principal) -> mysqli_real_escape_string() + htmlentities() (Insufficient Action) -> INSERT/SELECT queries (Resource)
- **Severity**: critical
- **Confidence**: 1.00

#### XSS_003: クロスサイトスクリプティングの可能性

- **Path**: $_POST['NAME']/$_POST['ADDRESS'] (Principal) -> htmlentities() (Insufficient Action) -> echo output (Resource)
- **Severity**: high
- **Confidence**: 0.90

#### AUTHZ_VIOLATION_004: 過剰なIAM権限付与

- **Path**: EC2 Instance Profile (Principal) -> sts:AssumeRole (Action) -> s3:*, ec2:*, rds:* (Resource)
- **Severity**: critical
- **Confidence**: 1.00

#### INSECURE_CONFIG_005: RDSの危険な設定

- **Path**: aws_db_instance設定 (Principal/Action) -> データベース (Resource)
- **Severity**: high
- **Confidence**: 1.00

#### LIFECYCLE_BYPASS_006: パスワード変更の検出不可（ignore_changes）

- **Path**: aws_db_instance.default.password (Resource) -> ignore_changes (Action) -> Terraform tracking disabled
- **Severity**: medium
- **Confidence**: 0.95

#### METADATA_EXPOSURE_007: EC2メタデータサービスからの秘密情報アクセス

- **Path**: EC2 Instance Metadata Service (Principal) -> user_data access (Action) -> var.password (Resource)
- **Severity**: high
- **Confidence**: 0.90

## マッチしたソースコード

```code
lifecycle {
    ignore_changes = ["password"]
  }
```

## 詳細解析

本Terraformコードは教育目的の脆弱なインフラ（TerraGoat）を示す例です。複数のレイヤーで重大なセキュリティ問題が存在します。

**最重大：平文パスワード漏洩**
ユーザーデータ内にDB_PASSWORD（var.password）が平文で埋め込まれており、以下の脅威が存在します：
- Terraform状態ファイルに平文で保存される
- EC2インスタンスのメタデータサービスから取得可能
- CloudTrailログに記録される可能性
- Webサーバーログに出力されうる

**SQL Injection**
htmlentities()はXSS対策であり、SQLインジェクション対策ではありません。mysqli_real_escape_string()が使用されていますが、プリペアドステートメント（パラメータ化クエリ）が標準です。

**過剰なIAM権限**
EC2インスタンスロールが"s3:*", "ec2:*", "rds:*"の全権限を持ち、コンプロマイズ時の被害範囲が甚大です。

**RDSの危険な設定**
- publicly_accessible=true（インターネット公開）
- storage_encrypted=false（保存時暗号化なし）
- backup_retention_period=0（バックアップなし）
- skip_final_snapshot=true（削除時スナップショットなし）

**ライフサイクル設定の問題**
ignore_changes=["password"]は、パスワード更新の追跡を無視し、セキュリティ監査の追跡性を損なわせます。

## PoC

```text
#!/bin/bash
# SQL Injection PoC - TerraGoat db-app.tf
# 注意: 教育目的のみ。実行許可のある環境でのみ実行

# 1. 平文パスワード取得（Terraform状態から）
cat terraform.tfstate | grep -o '"password": "[^"]*"'

# 2. EC2インスタンスメタデータ（実行中のインスタンスから）
curl -s http://169.254.169.254/latest/user-data | grep DB_PASSWORD

# 3. SQLインジェクション ペイロード
NAME_PAYLOAD="admin' OR '1'='1"
ADDRESS_PAYLOAD="'; DROP TABLE EMPLOYEES; --"

# 4. PHPアプリケーションへのリクエスト
echo "SQL Injection Payload:"
echo "POST /index.php HTTP/1.1"
echo "Host: <EC2_PUBLIC_IP>"
echo "Content-Type: application/x-www-form-urlencoded"
echo ""
echo "NAME=${NAME_PAYLOAD}&ADDRESS=${ADDRESS_PAYLOAD}"

# 5. IAM権限を悪用したS3アクセス
# コンプロマイズ後：
aws s3 ls --no-sign-request 2>/dev/null || aws s3 ls  # IAMロール経由
aws rds describe-db-instances  # すべてのRDS情報取得

# 6. 代替攻撃：Terraform状態ファイルからパスワード抽出
grep -r 'password' /root/.terraform/ 2>/dev/null | head -5
```

## 修復ガイダンス

### パスワード管理

- **Required**: 平文パスワードの削除
- **Guidance**: 1. AWS Secrets Manager/SSM Parameter Storeを使用してパスワードを管理
2. user_dataではなくIAMロールで認証
3. Terraform sensitive = trueマーク
4. 例: password = random_password.db.result（最小権限で自動生成）
5. Secrets Manager ARNをEC2に参照させる
- **Priority**: critical

### SQLインジェクション対策

- **Required**: プリペアドステートメント（パラメータ化クエリ）の実装
- **Guidance**: mysqli_real_escape_string()の置き換え：
1. mysqli_prepare()を使用: $stmt = $connection->prepare("INSERT INTO EMPLOYEES (NAME, ADDRESS) VALUES (?, ?)");
2. bind_paramで型指定: $stmt->bind_param("ss", $name, $address);
3. execute(): $stmt->execute();
4. さらにPDOやPreparedStatementを推奨（PHP 8+）
- **Priority**: critical

### IAM権限

- **Required**: 最小権限の原則に基づいた権限制限
- **Guidance**: ec2policyの修正：
1. s3:* → s3:GetObject, s3:PutObject（特定バケットのみ）
2. ec2:* → ec2:DescribeInstances, ec2:DescribeSecurityGroups（読取のみ）
3. rds:* → rds:DescribeDBInstances（読取のみ）
4. Resource: "*" → 特定のARN指定
5. Condition追加（制限時刻、IPアドレスなど）
- **Priority**: critical

### RDSセキュリティ設定

- **Required**: 安全な設定への変更
- **Guidance**: 修正項目：
1. publicly_accessible = false（VPC内のみアクセス）
2. storage_encrypted = true（KMS暗号化）
3. backup_retention_period = 30（または業務要件に応じた日数）
4. skip_final_snapshot = false（スナップショット保持）
5. kms_key_id指定（カスタマーマネージドキー）
6. セキュリティグループ: VPC CIDR制限を継続、必要に応じてアプリサーバーのみ許可
- **Priority**: critical

### ライフサイクル設定

- **Required**: パスワード変更の追跡有効化
- **Guidance**: db-app.tfの修正：
1. lifecycle { ignore_changes = ["password"] } を削除
2. または password = sensitive(var.password) で標記
3. Terraform state暗号化（remote backend S3 + 暗号化）
4. 定期的なパスワード更新ポリシーを実装
- **Priority**: high

### Terraform状態管理

- **Required**: 状態ファイルの暗号化と保護
- **Guidance**: 1. ローカル状態 → Remote state（S3 + 暗号化）への移行
2. terraform.tfstate: AWS KMS暗号化有効化
3. S3バージョニング + MFA Delete有効化
4. DynamoDBロック実装
5. IAM制御（terraform.tfstate読取権限の最小化）
6. CloudTrail有効化（状態ファイルアクセス監査）
- **Priority**: high

### EC2インスタンスセキュリティ

- **Required**: IMDSv2の強制とuserdata代替案
- **Guidance**: 1. metadata_options { http_endpoint = "enabled", http_tokens = "required", http_put_response_hop_limit = 1 } を追加
2. user_data削減: AWS Systems Manager Session Manager使用
3. CloudWatch Logs Agent設定
4. VPC エンドポイント（Secrets Manager）を使用
- **Priority**: high

### PHPアプリケーション全般

- **Required**: セキュアコーディング実装
- **Guidance**: 1. 入力検証: type + length制限（現在のmaxlengthは弱い）
2. エラーハンドリング: エラーメッセージから情報漏洩防止
3. セッション管理: session_start()の実装
4. CSRF対策: トークンの生成・検証
5. HTTPヘッダ: X-Frame-Options, X-Content-Type-Options, CSP設定
6. ロギング: セキュアなログ記録（パスワード除外）
- **Priority**: high

### Webサーバー設定

- **Required**: HTTPSの強制と追加セキュリティ設定
- **Guidance**: user_data内に追加：
1. HTTPSのみ受け入れ（Let's Encrypt使用）
2. ファイアウォール: fail2ban/ModSecurity導入
3. ログ記録: アクセスログ + エラーログの分離
4. 定期パッチ適用: amazon-linux-extras install php7.4など
5. セキュリティグループ: HTTPSのみ許可（443）
- **Priority**: high

### 監視・検出

- **Required**: セキュリティ監視の実装
- **Guidance**: 1. CloudWatch Logs: PHPエラー/アクセスログの監視
2. CloudTrail: DB接続試行の記録
3. GuardDuty: 異常なAPI呼び出し検出
4. SecurityHub: コンプライアンスチェック
5. アラート: 失敗した認証試行、権限エスカレーション試行
- **Priority**: medium

## 解析ノート

Terraform IaCコードの包括的なセキュリティ分析。複数の重大な脆弱性を検出：
1. ユーザーデータへの平文パスワード埋め込み（db-app.tf:265行）
2. SQL Injectionのリスク（htmlentitiesが不十分）
3. IAMの過剰権限（s3:*, ec2:*, rds:*）
4. RDSの公開アクセス有効化
5. バックアップ無効化
6. 暗号化無効化
7. ライフサイクルのignore_changes（パスワード変更検出不可）
8. PhPコードのセキュリティリスク（漏洩したDB認証情報）

