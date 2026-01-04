# Security Analysis: main.tf - AWS S3 bucket resources

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/dodo/main.tf`
- **検出パターン**: AWS S3 bucket resources

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `Other("AWS-S3-ENCRYPTION")`

## PAR Policy Analysis

### Principals (データ源)

- **Terraform provider with AWS credentials** - Untrusted

### Actions (セキュリティ制御)

- **No encryption configuration applied** - Missing

### Resources (操作対象)

- **S3 bucket 'backup' for data replication** - Critical

### Policy Violations

- **AWS-S3-ENCRYPTION**: Resource definition → unencrypted S3 bucket creation
  - Severity: warning | Confidence: 80%

## マッチしたソースコード

```hcl
resource "aws_s3_bucket" "backup" {
  bucket = "backup"

  versioning {
    enabled = true
  }
}
```

## 詳細解析

## S3バケット暗号化欠落 (backup)

### 脆弱性の詳細
AWS S3バケット `backup` (行84-90) がサーバーサイド暗号化設定を欠いています。

### PARフレームワーク分析
- **Principal**: Terraformプロバイダー（AWS認証情報）
- **Action**: 暗号化制御なし - `server_side_encryption_configuration`ブロックが存在しない
- **Resource**: S3バケット（データストレージ）
- **データフロー**: リソース定義 → 暗号化なしバケット作成

### 攻撃シナリオ (T1530)
1. 保存データの暗号化がないため、S3へのアクセスやバケット内データにアクセスした攻撃者が平文データを読取可能
2. コンプライアンス要件（HIPAA、PCI-DSS等）への違反
3. 継続的データ保護（CDP）とリカバリー戦略の不完全性

### 修正方法
```hcl
resource "aws_s3_bucket" "backup" {
  bucket = "backup"

  versioning {
    enabled = true
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "backup" {
  bucket = aws_s3_bucket.backup.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}
```

### リスク評価
- **重要度**: 高（機密データ保護の基本的な要件）
- **影響範囲**: レプリケーション先バケット（T1537対策不完全）

