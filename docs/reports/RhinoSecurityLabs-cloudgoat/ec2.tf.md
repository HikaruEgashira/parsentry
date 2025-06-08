# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **var.rds-password**: SemiTrusted
  - Context: Terraform変数
  - Risk Factors: 非構造化データ, シェルメタ文字含む可能性
- **var.rds-username**: SemiTrusted
  - Context: Terraform変数
  - Risk Factors: 非構造化データ

### Actions (セキュリティ制御)

- **remote-exec inline mysql コマンド実行**: Missing
  - Function: シェルコマンド実行
  - Weaknesses: コマンドインジェクションの可能性, 引数による認証情報露出
  - Bypass Vectors: シェルメタ文字を含むパスワード/ユーザー名

### Resources (操作対象)

- **EC2 インスタンス (remote-exec)**: High
  - Operation: シェルコマンド実行
  - Protection: SSHキー認証, IMDSv2必須

### Policy Violations

#### CMD_INJECTION: ユーザー入力（terraform変数）をシェルコマンドへ直接埋め込み、シェルインジェクションを許可している

- **Path**: resource.aws_instance.cg-ec2-instance.provisioner.remote-exec.inline
- **Severity**: high
- **Confidence**: 0.80

#### CRED_EXP_POSIX: DBパスワードをmysql -p引数で平文渡ししており、プロセスリストに露出する

- **Path**: resource.aws_instance.cg-ec2-instance.provisioner.remote-exec.inline
- **Severity**: medium
- **Confidence**: 0.90

## 詳細解析

Terraformのremote-execでvar.rds-username/var.rds-passwordをエスケープせずにシェルコマンドへ直接埋め込んでいるため、シェルインジェクションによるRCE（リモートコード実行）や、パスワードがプロセスリストに平文で露出する脆弱性があります。

## PoC（概念実証コード）

```text
// Terraform変数に次を設定
// var.rds-password = "password'; touch /tmp/pwned; #"
// remote-exec実行時、以下のように解釈されて/tmp/pwnedが作成される
mysql -h host -u user -ppassword'; touch /tmp/pwned; # < /home/ubuntu/insert_data.sql
```

## 修復ガイダンス

### remote-exec

- **Required**: シェルインジェクション防止のため変数を安全に引用またはエスケープ
- **Guidance**: mysqlコマンドの引数は--defaults-fileを利用するか、Terraformによる変数展開前にシェルエスケープ処理を行い、シェルメタ文字を無害化してください。
- **Priority**: high

### credential_management

- **Required**: DBパスワードの安全な管理
- **Guidance**: Terraform変数に直接保持せず、AWS Secrets ManagerやSSMパラメータストアを利用し、provisionerからは環境変数や設定ファイルを介して読み込むように変更してください。
- **Priority**: medium

## 解析ノート

・remote-exec inline mysqlコマンドでterraform変数をそのまま埋め込んでいる
・var.rds-passwordがシェルメタ文字を含めば任意コマンド実行可能→RCE
・mysql -pに平文渡しされるためpsで閲覧可能
・マネージドシークレットや適切なエスケープ/引用で対策可能
・vulnerability_typesにRCEを設定
・confidence_scoreは85点と評価
・PoCとしてパスワードに"; touch /tmp/pwned; #"を含める例を記載
・remediation_guidanceにescapeとシークレット管理を提案

