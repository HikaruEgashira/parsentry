# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **$1 (ユーザー名), $2 (プロファイル名)**: Untrusted
  - Context: スクリプト引数
  - Risk Factors: 未検証のユーザー入力, シェルメタ文字を混入可能

### Actions (セキュリティ制御)

- **aws CLI 呼び出しに unquoted な $1, $2 を展開**: Missing
  - Function: 入力の検証/サニタイズ
  - Weaknesses: コマンドインジェクション
  - Bypass Vectors: セミコロンによるコマンド連結, バッククオートや$()によるコマンド実行

### Resources (操作対象)

- **IAM detach-user-policy 操作**: Critical
  - Operation: IAM 権限操作
  - Protection: AWS 認証情報, IAM ポリシー

### Policy Violations

#### SHELL_INJECTION: 未検証の外部入力がシェルコマンドに直接渡されている

- **Path**: resource_cleaning.sh:3,5
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

このスクリプトでは、引数($1: ユーザー名, $2: プロファイル名)を一切検証せずにシェルコマンドに展開しており、コマンドインジェクションが可能です。特にセミコロンやバッククオートを含む文字列を渡すことで任意のコマンドが実行され、最悪の場合EC2インスタンスやLambda環境の完全侵害につながる恐れがあります。

## PoC（概念実証コード）

```text
./resource_cleaning.sh 'victim; echo pwn > /tmp/pwn; #' default
```

## 修復ガイダンス

### resource_cleaning.sh

- **Required**: 外部入力の検証と変数の適切な引用
- **Guidance**: --user-name や --profile に渡す変数を"$1"のようにダブルクオートで囲み、入力値をホワイトリストで検証・正規表現で文字種を制限する
- **Priority**: high

## 解析ノート

1. スクリプトは引数を検証せずbashに展開している 2. $1, $2 にセミコロン等を含められコマンドインジェクション可能 3. IAM detach-user-policy は高感度操作 4. 適切なサニタイズ・引用が実装されておらず脆弱性と判定

