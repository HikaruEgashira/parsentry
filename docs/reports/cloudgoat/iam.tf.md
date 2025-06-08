# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **var.cgid / var.profile**: SemiTrusted
  - Context: Terraform変数
  - Risk Factors: 未検証の外部入力, コマンドインジェクションの危険性

### Actions (セキュリティ制御)

- **local-exec provisioner**: Missing
  - Function: シェルコマンド実行
  - Weaknesses: ユーザー入力のサニタイズ欠如, シェルメタ文字の未エスケープ
  - Bypass Vectors: ;, &&, `, $()によるコマンド実行

### Resources (操作対象)

- **ローカルシェル実行環境**: Critical
  - Operation: command execution
  - Protection: 

### Policy Violations

#### PV001: local-execにおける入力未検証によるコマンドインジェクション(RCE)

- **Path**: aws_iam_user.bilbo → provisioner.local-exec.command
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

Terraformのlocal-execプロビジョナにおいて、ユーザー入力変数(var.cgid, var.profile)がシェルコマンドに直接埋め込まれており、メタ文字を用いたコマンドインジェクション（RCE）が可能です。IAMポリシー自体には重大な過剰権限は見当たりませんが、ローカル環境での任意コード実行は重大なリスクです。

## PoC（概念実証コード）

```text
# コマンドインジェクション例
# var.profile にメタ文字を埋め込む
terraform apply -var 'cgid=123' -var 'profile=test; echo vulnerable > /tmp/pwned'
```

## 修復ガイダンス

### aws_iam_user.bilbo.provisioner.local-exec

- **Required**: コマンド引数のバリデーションおよびエスケープ
- **Guidance**: var.cgid/var.profileをアルファベット数字に制限し、""で囲むかTerraformのbuilt-in関数（replace, regex）でメタ文字を除去。可能ならlocal-execを廃止し公式プロバイダ機能を利用。
- **Priority**: high

## 解析ノート

Terraformのlocal-execはOSシェルを呼び出すため、引数にセミコロン等を含めると任意コマンドの実行が可能になる。var.*はユーザー制御下なのでsemi_trustedと判断。IAMリソースはRCE脆弱性と直接無関係のため、他の過剰権限は特記事項なし。

