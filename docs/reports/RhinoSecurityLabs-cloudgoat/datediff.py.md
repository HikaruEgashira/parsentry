# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **args.holiday_file**: Untrusted
  - Context: コマンドライン引数
  - Risk Factors: 任意のファイルパス指定, 不適切な入力検証

### Actions (セキュリティ制御)

- **open(args.holiday_file)**: Missing
  - Function: 入力検証
  - Weaknesses: パス検証不足
  - Bypass Vectors: ../ などのディレクトリトラバーサル

### Resources (操作対象)

- **ローカルファイルシステム**: High
  - Operation: file_read
  - Protection: 

### Policy Violations

#### LFI-001: 外部入力からのファイルパスを検証なしに open() に渡している

- **Path**: main -> open(args.holiday_file)
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

コマンドライン引数として渡された --holiday-file の値をそのまま open() に渡し、任意のローカルファイルを読み込めてしまうため、LFI（Local File Inclusion）脆弱性が存在します。ファイルパスの検証や制限がまったく実装されておらず、不正なファイル参照やディレクトリトラバーサル攻撃が可能です。

## PoC（概念実証コード）

```text
# LFI の PoC例
default_command = 'python datediff.py 2020-01-01 --holiday-file /etc/passwd'
print('[*] 実行例:', default_command)
```

## 修復ガイダンス

### main

- **Required**: ファイルパスのバリデーション実装
- **Guidance**: os.path.abspath で正規化した上で、許可されたディレクトリ／ファイルのホワイトリストチェックを行い、不正なパスやディレクトリトラバーサルをブロックする
- **Priority**: high

## 解析ノート

1. args.holiday_file はコマンドラインから与えられる untrusted 入力  2. main 内で open(args.holiday_file) を直接実行、検証なし  3. 任意のローカルファイルを読み込む LFI リスク  4. パス正規化・ホワイトリストによる検証が必要  5. 以上をもとに PAR モデルで整理し、policy_violations を特定


