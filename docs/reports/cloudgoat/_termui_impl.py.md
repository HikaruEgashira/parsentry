# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **環境変数(PAGER, VISUAL, EDITOR)**: Untrusted
  - Context: 環境変数
  - Risk Factors: ユーザー制御可能, シェル構文を埋め込める
- **open_urlのurl引数**: Untrusted
  - Context: 関数引数
  - Risk Factors: ユーザー入力可能, シェル文字列に挿入可能

### Actions (セキュリティ制御)

- **shell_execution**: Insufficient
  - Function: 外部コマンド実行
  - Weaknesses: 未検証の文字列をshell=Trueで実行
  - Bypass Vectors: 環境変数PAGERに悪意あるコマンド挿入, URL引数に`; rm -rf /`等のシェルインジェクション挿入

### Resources (操作対象)

- **OSコマンド実行**: Critical
  - Operation: command_execution
  - Protection: 

### Policy Violations

#### RCE1: 未検証の外部入力をshell=Trueやos.systemで実行し、RCEを引き起こす

- **Path**: _pipepager -> subprocess.Popen(cmd, shell=True), _tempfilepager/edit_file/open_url -> os.system/subprocess.Popen(shell=True)
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

このコードでは、環境変数(PAGER, VISUAL, EDITOR)やユーザーから渡されるURL文字列をそのままシェルコマンド(os.system, subprocess.Popen(shell=True))として実行しており、任意コマンド注入(RCE)のリスクがあります。入力の検証・サニタイズが一切行われておらず、shell=Trueでコマンドを起動しているため、攻撃者は環境変数やURLに悪意あるシェル構文を含めて任意コードを実行できます。

## PoC（概念実証コード）

```text
# POC: PAGER環境変数経由で任意コマンド
import os
os.environ['PAGER']="sh -c 'touch /tmp/pwned'"
from click._termui_impl import pager
pager(["dummy text\n"])
# 実行後 /tmp/pwned が作成される
```

## 修復ガイダンス

### _pipepager, _tempfilepager, edit_file, open_url

- **Required**: 外部入力(環境変数・引数)を直接シェル実行しない
- **Guidance**: subprocess.run([...], shell=False) 形式でコマンドを実行し、入力文字列をリスト化して渡す。環境変数の値はホワイトリスト検証またはエスケープ処理を行い、URLにはシェル構文が含まれないことを確認する。
- **Priority**: high

## 解析ノート

1. コード内で環境変数(PAGER, VISUAL, EDITOR)やopen_urlのurl引数を用いてos.system/subprocess.Popenをshell=Trueで呼び出している箇所を確認
2. これらは外部から制御可能でサニタイズがないためシェルインジェクション(RCE)の危険がある
3. PAR: Principals=環境変数, url引数(信頼なし); Action=shell実行(未検証); Resource=OSコマンド実行(critical)
4. policy_violations: RCEリスク
5. 改善: shell=False, 引数リスト化、入力ホワイトリスト化等を推奨する。

