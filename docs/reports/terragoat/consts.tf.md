# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

このTerraformコードにはファイルの読み書きを行うリソースやモジュール呼び出しが存在せず、任意ファイル操作（AFO）に該当する箇所はありません。

## PoC（概念実証コード）

```text
概念実証なし
```

## 解析ノート

- Terraform定義のみで、ファイル操作リソースが存在しない。
- 変数・ローカル値の定義のみで、外部入力からファイルパスを生成・参照するコードがない。
- よってAFOのリスクは検出されない。

