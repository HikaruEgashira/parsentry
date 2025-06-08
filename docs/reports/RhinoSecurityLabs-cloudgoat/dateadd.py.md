# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **CLI引数 --holiday-file**: Untrusted
  - Context: argparse でユーザーから取得
  - Risk Factors: 未検証入力, 潜在的なディレクトリトラバーサル

### Actions (セキュリティ制御)

- **open(args.holiday_file)**: Missing
  - Function: ファイル読み込み
  - Weaknesses: パストラバーサル脆弱性, 入力検証欠如
  - Bypass Vectors: ../パスを含む相対パス, 絶対パス指定

### Resources (操作対象)

- **ファイルシステム任意ファイル**: Medium
  - Operation: read
  - Protection: 

### Policy Violations

#### CWE-23: CWE-23: パス・トラバーサルにより任意ファイル読取が可能

- **Path**: main 関数内の open(args.holidays_file)
- **Severity**: medium
- **Confidence**: 0.80

## 詳細解析

ユーザーが指定する--holiday-file引数をそのままopen()に渡してファイルを読み込んでいるため、任意のローカルファイルを読み込むことが可能です。正当な検証・サニタイズがなく、ディレクトリトラバーサルを含むパス操作が防止されていません。

## PoC（概念実証コード）

```text
$ python dateadd.py --holiday-file /etc/passwd
```

## 修復ガイダンス

### dateadd.py

- **Required**: ファイルパスの検証・サニタイズ
- **Guidance**: 入力されたパスを os.path.abspath で正規化し、ホワイトリスト化したディレクトリ配下のみアクセスを許可する
- **Priority**: high

## 解析ノート

1. argparse で --holiday-file を取得 2. バリデーションなしで open() に渡している 3. 相対パスや絶対パスで任意ファイル読取可能 4. LFI (CWE-23) として識別

