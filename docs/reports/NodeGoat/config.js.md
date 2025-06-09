# PAR Security Analysis Report

![中信頼度](https://img.shields.io/badge/信頼度-中-yellow) **信頼度スコア: 60**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **process.env.NODE_ENV**: SemiTrusted
  - Context: Node.js environment variable
  - Risk Factors: modifiable by environment, influences file path

### Actions (セキュリティ制御)

- **requireDynamicPath**: Missing
  - Function: module inclusion
  - Weaknesses: lack of input validation
  - Bypass Vectors: path traversal

### Resources (操作対象)

- **config file**: Medium
  - Operation: read and execute
  - Protection: 

### Policy Violations

#### NODE001: Unvalidated dynamic module loading can lead to arbitrary code execution

- **Path**: config/config.js: 動的require箇所
- **Severity**: MEDIUM
- **Confidence**: 0.80

## 詳細解析

このコードでは、process.env.NODE_ENVの値を検証せずに動的にモジュールのパスを構築しており、ホワイトリストによる制限や入力サニタイズが実装されていません。悪意ある環境変数を設定することで、想定外のファイルを読み込ませ、任意コード実行や情報漏洩を引き起こす可能性があります。

## PoC（概念実証コード）

```text
# 悪意ある環境変数をセットして想定外のファイルを読み込ませる例
export NODE_ENV="../../../../malicious"  # config/env/../../../../malicious.js を読み込む
node index.js
```

## 修復ガイダンス

### config loader

- **Required**: Validate and whitelist environment names
- **Guidance**: finalEnv の値を ['development','production','test'] など許可されたリストに限定し、リスト外の場合は起動を中止する
- **Priority**: high

## 解析ノート

- Identify source: process.env.NODE_ENV から値を取得
- No validation: finalEnv の検証が未実装
- Dynamic require: path.resolve + finalEnv を用いてモジュールロード
- Risk: LFI/任意コード実行
- Map to PAR: Principal=env var, Action=dynamic require, Resource=file read+exec
- Policy violation: unvalidated dynamic include

