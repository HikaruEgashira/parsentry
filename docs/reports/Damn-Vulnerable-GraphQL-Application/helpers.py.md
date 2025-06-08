# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `AFO`
- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **token**: Untrusted
  - Context: 外部入力
  - Risk Factors: 未署名／改竄可能 JWT
- **cmd**: Untrusted
  - Context: 外部入力
  - Risk Factors: 任意コマンド文字列
- **filename**: Untrusted
  - Context: 外部入力
  - Risk Factors: パス・トラバーサル, 絶対パス挿入
- **text**: Untrusted
  - Context: 外部入力
  - Risk Factors: 任意ファイル内容挿入

### Actions (セキュリティ制御)

- **run_cmd**: Insufficient
  - Function: OSコマンド実行
  - Weaknesses: コマンドインジェクション
  - Bypass Vectors: ; && | ` $( )
- **get_identity**: Bypassed
  - Function: JWTデコード／認証
  - Weaknesses: 署名検証バイパス
  - Bypass Vectors: verify_signature=Falseによる署名検証無効化
- **save_file**: Insufficient
  - Function: ファイル書き込み
  - Weaknesses: パス・トラバーサル, 任意ファイル書き込み
  - Bypass Vectors: ../ や絶対パス挿入

### Resources (操作対象)

- **shell**: Critical
  - Operation: OSコマンド実行
  - Protection: 
- **filesystem**: High
  - Operation: ファイル書き込み
  - Protection: 

### Policy Violations

#### RCE-001: 外部入力を検証せずOSコマンドを直接実行している

- **Path**: run_cmd(cmd) → os.popen(cmd)
- **Severity**: critical
- **Confidence**: 0.90

#### AFO-002: JWT 署名検証を無効化してデコードしている

- **Path**: get_identity(token) → decode(token, verify_signature=False)
- **Severity**: high
- **Confidence**: 0.90

#### LFI-003: filename を検証せずファイルパス連結して書き込みしている

- **Path**: save_file(filename, text) → open(WEB_UPLOADDIR + filename, 'w')
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

本コードには外部入力を検証せずに直接危険な操作を行う関数が含まれており、主に以下の脆弱性が存在します。
1. run_cmd：ユーザ制御下の文字列を os.popen に渡すことでコマンドインジェクション（RCE）を許します。
2. get_identity：JWT の署名検証を無効化（verify_signature=False）しており、認証バイパス（AFO）を引き起こします。
3. save_file：filename を検証せずにファイルパス連結して書き込むため、パス・トラバーサル／任意ファイル書き込み（LFI）を許します。

## PoC（概念実証コード）

```text
# RCE PoC
from core.helpers import run_cmd
# 悪意ある入力例
evil = 'echo hacked > /tmp/pwned.txt'
run_cmd(evil)

# AFO PoC
from core.helpers import get_identity
# 署名無効化により任意のペイロードを受理
token = 'eyJhbGciOiJIUzI1NiJ9.eyJpZGVudGl0eSI6ImFkbWluIn0.invalidsig'
print(get_identity(token))

# LFI PoC
from core.helpers import save_file
# /etc/passwd を上書き
save_file('../etc/passwd', 'hacked')
```

## 修復ガイダンス

### run_cmd

- **Required**: 外部入力の検証・サニタイズ
- **Guidance**: subprocess.run を使用し、引数をリスト形式で渡す。shell=True を避ける。
- **Priority**: high

### get_identity

- **Required**: JWT 署名検証有効化
- **Guidance**: decode 時にオプションで verify_signature=True, verify_exp=True とし、公開鍵／共有鍵で検証を行う。
- **Priority**: high

### save_file

- **Required**: ファイルパスの正規化と検証
- **Guidance**: os.path.abspath で安全なディレクトリ内に収まることを確認し、許可された拡張子・ファイル名のホワイトリストを利用する。
- **Priority**: medium

## 解析ノート

・run_cmd で os.popen の直接呼び出しを確認。外部入力無検証のため RCE。
・get_identity は verify_signature=False で署名検証をバイパス。認証機能が無効化された AFO。
・save_file は filename をそのまま結合して open。パス・トラバーサル／任意書き込みが可能。
・各関数に対して実装品質を評価し、policy_violations に該当ルールを紐づけた。
・Remediation では具体的な API 変更と入力検証指針を提示。

