# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 75**

## 脆弱性タイプ

- `LFI`
- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **file_path**: Untrusted
  - Context: 外部入力 (parameter)
  - Risk Factors: パストラバーサル, 任意ファイルアクセス
- **whitelist_path**: Untrusted
  - Context: 外部入力 (ファイルパス)
  - Risk Factors: パストラバーサル, 不正フォーマット
- **dir_name**: Untrusted
  - Context: 外部入力 (parameter)
  - Risk Factors: パストラバーサル

### Actions (セキュリティ制御)

- **create_or_update_yaml_file**: Missing
  - Function: ファイル書き込み
  - Weaknesses: 入力検証不在, パス正規化欠如
  - Bypass Vectors: ../を用いたパスエスケープ
- **load_data_from_yaml_file**: Missing
  - Function: ファイル読み込み
  - Weaknesses: 入力検証不在, パス正規化欠如
  - Bypass Vectors: ../を用いたパス参照
- **create_dir_if_nonexistent**: Missing
  - Function: ディレクトリ作成
  - Weaknesses: 入力検証不在
  - Bypass Vectors: ../を用いたパスエスケープ

### Resources (操作対象)

- **ファイルシステム**: Critical
  - Operation: write
  - Protection: 
- **ファイルシステム**: Critical
  - Operation: read
  - Protection: 
- **ファイルシステム**: High
  - Operation: mkdir
  - Protection: 

### Policy Violations

#### AFO001: 未検証のパスを用いた任意ファイル書き込み (Arbitrary File Overwrite)

- **Path**: create_or_update_yaml_file -> open(file_path, 'w')
- **Severity**: high
- **Confidence**: 0.80

#### LFI001: 未検証のパスを用いた任意ファイル読み込み (Local File Inclusion)

- **Path**: load_data_from_yaml_file -> open(file_path, 'r')
- **Severity**: medium
- **Confidence**: 0.70

#### DIRTRAV001: dir_nameに対するサニタイズ欠如によるディレクトリトラバーサル

- **Path**: create_dir_if_nonexistent -> os.mkdir
- **Severity**: medium
- **Confidence**: 0.60

## 詳細解析

create_or_update_yaml_fileおよびload_data_from_yaml_file関数では、file_pathパラメータに対する入力検証やパス正規化が行われておらず、../エスケープを用いた任意ファイル書き込み（AFO）／任意ファイル読み込み（LFI）が可能です。また、create_dir_if_nonexistentでもdir_nameに対してサニタイズがなく、ディレクトリトラバーサルが発生するリスクがあります。

## PoC（概念実証コード）

```text
# LFI: 任意ファイル読み込み
from core.python.utils import load_data_from_yaml_file
print(load_data_from_yaml_file('/etc/passwd','any'))

# AFO: 任意ファイル書き込み
def poc():
    from core.python.utils import create_or_update_yaml_file
    create_or_update_yaml_file('/tmp/../etc/evil.txt', {'pwn':'data'})
    with open('/etc/evil.txt','r') as f:
        print(f.read())
```

## 修復ガイダンス

### create_or_update_yaml_file

- **Required**: file_pathを正規化し、ホワイトリストディレクトリの範囲内に限定
- **Guidance**: os.path.realpathで絶対パスを取得し、想定ディレクトリプレフィックスで始まることを検証する
- **Priority**: high

### load_data_from_yaml_file

- **Required**: file_pathを正規化して制限
- **Guidance**: basenameやホワイトリスト方式で読み込み可能なファイルを限定する
- **Priority**: medium

### create_dir_if_nonexistent

- **Required**: dir_nameに対するサニタイズ実装
- **Guidance**: os.path.basenameのみ許可し、相対パスセグメントを除去する
- **Priority**: medium

## 解析ノート

ユーザー制御下のfile_path/dir_nameに対しサニタイズや正規化がなく、../エスケープで任意のファイルIOが可能と判断。load_data_from_yaml_fileで読み込み、create_or_update_yaml_fileで書き込みを行っているためLFI/AFOリスク、create_dir_if_nonexistentもトラバーサルの懸念あり。

