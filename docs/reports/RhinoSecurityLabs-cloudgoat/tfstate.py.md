# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **file_path引数**: Untrusted
  - Context: 外部から関数呼び出し時の引数
  - Risk Factors: ユーザー制御可能, パス操作攻撃可能

### Actions (セキュリティ制御)

- **load_file -> open(file_path)**: Missing
  - Function: 入力検証／パス制限
  - Weaknesses: 入力パスの検証欠如, ホワイトリスト未実装
  - Bypass Vectors: ../, 絶対パス指定

### Resources (操作対象)

- **任意ファイルシステム**: High
  - Operation: file read
  - Protection: OSファイルパーミッション

### Policy Violations

#### INSECURE_FILE_READ: 外部入力を検証せずにファイルを直接開くとLFIのリスクがある

- **Path**: Tfstate.load_file -> open(file_path)
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

Tfstate.load_file関数は引数file_pathをそのままos.path.existsおよびopenに渡してJSONをロードしています。入力されたパスに対して適切な検証・制限を行っておらず、相対パスや絶対パスを用いた任意のファイル読み込み（LFI）攻撃が可能です。

## PoC（概念実証コード）

```text
# LFIの概念実証
from core.python.python_terraform.tfstate import Tfstate
# 攻撃者が/etc/passwdを読み込む例
tf = Tfstate.load_file('/etc/passwd')
print(tf.native_data)
```

## 修復ガイダンス

### Tfstate.load_file

- **Required**: file_pathに対するホワイトリストまたはベースディレクトリ制限を実装
- **Guidance**: os.path.realpathで正規化後、許可されたディレクトリ配下かどうかをチェックし、範囲外ならエラーとする。パス操作文字列（../）を除去するか拒否。
- **Priority**: high

## 解析ノート

1. load_fileがfile_pathをそのままopenに渡している点を確認
2. 入力検証やパス制限が実装されておらず任意のファイル読み込みが可能
3. LFI脆弱性と判断し、PARモデルでPrincipal(file_path untrusted), Action(open missing validation), Resource(file読み込み sensitive)を整理
4. remediationとしてホワイトリスト／ディレクトリ制限を提案

