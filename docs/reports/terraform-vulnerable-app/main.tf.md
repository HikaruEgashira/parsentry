# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `AFO`

## 解析結果

• aws_lambda_function にて機密情報（DB_PASSWORD, API_KEY）が平文で環境変数に設定されているため、ソース管理やログ、メトリクス経由で漏洩するリスクがあります。DEBUG_MODE が true である点も情報漏洩を拡大させます。
• aws_ebs_volume で encrypted = false が指定されており、データ保護が不十分です。EC2 インスタンスまたは権限を持つアクターによりボリュームをスナップショット化され、データが取得されるリスクがあります。
• aws_cloudtrail で enable_logging = false が指定されており、操作ログが記録されないためインシデント発生時の追跡・監査が不能になります。
• aws_lb で enable_deletion_protection = false としているため、誤操作または権限昇格攻撃時にロードバランサーが削除され、サービス停止につながる可能性があります。

## PoC（概念実証コード）

```text
1. Terraform を適用後、Lambda の環境変数を AWS コンソールや CLI で参照し、DB_PASSWORD/API_KEY を取得可能。
2. EBS ボリュームをスナップショット化する権限を持つ IAM ユーザーであれば、暗号化なしボリュームからデータを抽出可能。
3. CloudTrail ログが記録されないため、上記操作の痕跡は残らず、検知・追跡が困難になる。
```

## 関連コードコンテキスト

### 関数名: aws_lambda_function.data_processor
- 理由: 機密情報を平文で環境変数に設定しており、ソース管理やログ経由で漏洩リスクがある
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
DB_PASSWORD = var.admin_password
```

### 関数名: aws_lambda_function.data_processor
- 理由: 機密キーを平文で設定し、漏洩リスクを増大させている
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
API_KEY     = var.api_key
```

### 関数名: aws_lambda_function.data_processor
- 理由: デバッグモード有効により、内部情報がログに出力される可能性がある
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
DEBUG_MODE  = "true"
```

### 関数名: aws_ebs_volume.app_data
- 理由: EBS ボリュームが暗号化されておらず、データの機密性が保護されていない
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
encrypted         = false
```

### 関数名: aws_cloudtrail.audit
- 理由: CloudTrail が無効化されており、操作ログの監査が不可能となっている
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
enable_logging = false
```

### 関数名: aws_lb.main
- 理由: 削除保護が無効化されており、誤操作や攻撃でリソースが削除され得る
- パス: example/terraform-vulnerable-app/modules/compute/main.tf
```rust
enable_deletion_protection = false
```

## 解析ノート

Terraform コード内のセキュリティリスクを洗い出し
- 環境変数に平文シークレット
- EBS 未暗号化
- CloudTrail ログ無効
- LB 削除保護無効

