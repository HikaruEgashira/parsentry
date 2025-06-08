# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `LFI`
- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **process.env.NODE_ENV**: Untrusted
  - Context: 環境変数
  - Risk Factors: 制御されていない入力, 環境変数改ざんの可能性

### Actions (セキュリティ制御)

- **require(path.resolve(__dirname + "/../config/env/" + finalEnv.toLowerCase() + ".js"))**: Missing
  - Function: 動的モジュール読み込み
  - Weaknesses: パストラバーサル, 動的コード実行
  - Bypass Vectors: finalEnvに'../../../../path/to/malicious.js'を設定

### Resources (操作対象)

- **任意ファイル読み込み及びコード実行**: High
  - Operation: ファイル読み込み & モジュール実行
  - Protection: 

### Policy Violations

#### NODEJS_DYNAMIC_REQUIRE_UNSAFE: 環境変数から直接ファイルパスを動的に読み込むことはパストラバーサル及び任意コード実行のリスクを生む

- **Path**: require(path.resolve(__dirname + "/../config/env/" + finalEnv.toLowerCase() + ".js"))
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

環境変数NODE_ENVを検証せずに動的パス構築しrequireしているため、パストラバーサルや任意コード実行のリスクがある。攻撃者がNODE_ENVに"../../../../path/to/malicious.js"のような値を設定すると、意図しないファイルをモジュールとして読み込み実行できる。

## PoC（概念実証コード）

```text
// 攻撃例: NODE_ENVにパストラバーサルパスを設定して起動
// 実行コマンド:
//   NODE_ENV=../../../../etc/passwd node app.js

// または攻撃者が作成したmalicious.jsを読み込ませる:
//   NODE_ENV=../../../../attacker/malicious node app.js

// config.js内のrequireで任意のJSが実行される。
```

## 修復ガイダンス

### config.jsのdynamic require処理

- **Required**: finalEnvのホワイトリスト検証を実装
- **Guidance**: NODE_ENVを['development','production','test']など事前定義された値に限定し、その他はデフォルト('development')にフォールバックする
- **Priority**: high

## 解析ノート

- process.env.NODE_ENVに基づき動的にファイルをrequire
- finalEnv.toLowerCase() だがパストラバーサル防止なし
- path.resolveで正規化後にもディレクトリ外への移動を許可
- requireはJS実行を伴うためRCEのリスク
- 未検証環境変数を信頼せずホワイトリスト化が必要

