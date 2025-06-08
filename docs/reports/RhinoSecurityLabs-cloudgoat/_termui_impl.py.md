# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **os.environ['PAGER']**: Untrusted
  - Context: 環境変数
  - Risk Factors: ユーザ制御, 不検証
- **os.environ['EDITOR']/os.environ['VISUAL']**: Untrusted
  - Context: 環境変数
  - Risk Factors: ユーザ制御, 不検証

### Actions (セキュリティ制御)

- **subprocess.Popen(cmd, shell=True)**: Insufficient
  - Function: 外部プロセス呼び出し
  - Weaknesses: シェルインジェクション
  - Bypass Vectors: PAGERにセミコロン区切りで任意コマンドを埋め込む
- **os.system(cmd)**: Insufficient
  - Function: シェルコマンド実行
  - Weaknesses: シェルインジェクション
  - Bypass Vectors: EDITOR/VISUALに任意コマンドを設定

### Resources (操作対象)

- **OSシェル**: Critical
  - Operation: コマンド実行
  - Protection: 

### Policy Violations

#### CLI-SHELL-CMD-INJ: ユーザ制御可能な環境変数をバリデーションせずshell=True付きで実行している

- **Path**: click/_termui_impl.py:_pipepager, _tempfilepager, edit_file, open_url
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

Clickの_termui_impl.pyでは、環境変数(PAGER/EDITOR/VISUALなど)をそのままshell=True付きのsubprocess.Popenやos.systemに渡して外部コマンドを実行しています。これにより、ユーザ制御可能な値でシェルコマンドインジェクションが発生し、リモートコード実行(RCE)につながる可能性があります。

## PoC（概念実証コード）

```text
# POC: PAGER経由で任意コマンド実行
import os, click._termui_impl as tui
os.environ['PAGER'] = 'echo hacked; touch /tmp/pwned'
def gen(): yield 'test'
tui.pager(gen())
```

## 修復ガイダンス

### _pipepager/_tempfilepager

- **Required**: shell=Trueをやめ、リスト形式でコマンドを指定する
- **Guidance**: subprocess.run(['less'], stdin=..., env=...)のようにshell=Falseで実行し、外部入力は完全にホワイトリスト化してください
- **Priority**: high

### edit_file/open_url

- **Required**: 環境変数値をホワイトリストで制限し、検証を追加する
- **Guidance**: EDITOR/VISUALは予め許可する実行可能ファイル名のみ受け付けるようvalidationを実装してください
- **Priority**: medium

## 解析ノート

環境変数PAGER/EDITOR等のuntrusted inputがshell=True付きのsubprocess.Popenやos.systemに渡されている箇所を調査。_pipepager, _tempfilepager, edit_file, open_urlで任意シェル実行可能な危険を確認。RCE脆弱性と判断。 remediationではshell=Falseへの変更と入力検証を推奨。

