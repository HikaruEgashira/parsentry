# PAR Security Analysis Summary Report

## 概要

| ファイル | 脆弱性タイプ | 信頼度 | Policy Violations |
|---------|------------|--------|------------------|
| [s3.tf](s3.tf.md) | IDOR | 中高 | S3_PUBLIC, S3_ENCRYPTION, S3_LOGGING, S3_VERSIONING |
| [db-app.tf](db-app.tf.md) | SQLI, AFO, XSS, IDOR, RCE | 高 | CREDENTIAL_EXPOSURE, SQLI, XSS, AUTHZ_VIOLATION, INSECURE_CONFIG |

## Policy Violation Analysis

| Rule ID | 件数 | 説明 |
|---------|------|------|
| CREDENTIAL_EXPOSURE_001 | 1 | 機密情報の平文埋め込み禁止 |
| SQLI_002 | 1 | SQLインジェクション脆弱性 |
| XSS_003 | 1 | クロスサイトスクリプティングの可能性 |
| AUTHZ_VIOLATION_004 | 1 | 過剰なIAM権限付与 |
| INSECURE_CONFIG_005 | 1 | RDSの危険な設定 |
| LIFECYCLE_BYPASS_006 | 1 | パスワード変更の検出不可 |
| METADATA_EXPOSURE_007 | 1 | EC2メタデータサービスからの秘密情報アクセス |
| S3_PUBLIC | 1 | S3バケットがパブリックアクセスを許可してはいけない |
| S3_ENCRYPTION | 1 | S3バケットはサーバーサイド暗号化を有効にする必要がある |
| S3_LOGGING | 1 | S3バケットはアクセスログ取得を有効にする必要がある |
| S3_VERSIONING | 1 | S3バケットはバージョニングを有効にする必要がある |

## 検出パターン

- **AWS S3 bucket resources**: S3バケットリソースの設定ミス検出
- **AWS IAM resources**: IAM権限の過剰付与検出
- **AWS RDS database resources**: RDSセキュリティ設定の問題検出
- **Lifecycle management**: Terraform lifecycle設定の監査追跡性問題検出

## 解析環境

- **対象リポジトリ**: [bridgecrewio/terragoat](https://github.com/bridgecrewio/terragoat)
- **解析プロバイダ**: Claude Code
- **検出言語**: Terraform (HCL)
