# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 95**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

Terraformコード内に任意のファイル操作に該当するリソースやプロビジョニングロジックは見当たりません。全てAzureリソースの宣言的定義のみであり、ファイル読み書きやローカルファイルへのアクセスパスを受け付ける箇所は存在しないため、AFO脆弱性は未検出です。

## 解析ノート

1. コード全体を確認し、ファイルパス操作やローカルファイルアクセスを行うプロビジョナやモジュール呼び出しがないことを確認。
2. directory_traversal, symlink, race_conditionを誘発する入力も存在せず。
3. Terraform宣言のみでAFOに該当せず。

