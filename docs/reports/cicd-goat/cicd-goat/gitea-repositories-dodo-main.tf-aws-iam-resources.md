# Security Analysis: main.tf - AWS IAM resources

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/dodo/main.tf`
- **検出パターン**: AWS IAM resources

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `Other("IAM_PRIVILEGE_ESCALATION")`

## PAR Policy Analysis

### Principals (データ源)

- **aws_iam_role.replication (S3 service assumed)** - Untrusted

### Actions (セキュリティ制御)

- **No additional validation controls on action permissions** - Missing

### Resources (操作対象)

- **aws_s3_bucket.backup (all objects via wildcard)** - Critical

### Policy Violations

- **IAM_PRIVILEGE_ESCALATION**: Infrastructure config → IAM policy → S3 bucket permissions
  - Severity: warning | Confidence: 80%

## マッチしたソースコード

```hcl
resource "aws_iam_role" "replication" {
  name = "tf-iam-role-replication-12345"

  assume_role_policy = <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "s3.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
POLICY
}
```

## 詳細解析

## IAM 過度な権限付与の検出

**脆弱性**: `aws_iam_policy.replication` が以下の過度な権限を許可しています:

1. **削除権限の付与** (行66-72):
   - リソース: `${aws_s3_bucket.backup.arn}/*`
   - 権限: `s3:ReplicateObject`, `s3:ReplicateDelete`, `s3:ReplicateTags`
   - **リスク**: バックアップバケット内のすべてのオブジェクト削除が可能

2. **ACL読取権限** (行57-58):
   - 権限: `s3:GetObjectVersionAcl`
   - **リスク**: オブジェクトのACL情報が読取可能（メタデータ漏洩リスク）

3. **ワイルドカード使用** (行62, 72):
   - リソース: `${aws_s3_bucket.dodo.arn}/*` と `${aws_s3_bucket.backup.arn}/*`
   - **リスク**: 権限の最小化原則に違反

**攻撃シナリオ (T1078, T1484)**:
- このロールを乗っ取った攻撃者がバックアップデータを削除可能
- データ保護戦略の無効化
- ランサムウェア攻撃での身代金要求リスク増加

**推奨修正**:
削除権限は別のポリシーに分離し、明確な運用プロセスの下でのみ許可してください。

