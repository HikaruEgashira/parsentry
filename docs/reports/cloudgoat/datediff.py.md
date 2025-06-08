# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **args.holiday_file**: Untrusted
  - Context: コマンドライン引数
  - Risk Factors: path_traversal, unauthorized_file_access

### Actions (セキュリティ制御)

- **read_holiday_file**: Missing
  - Function: 入力バリデーションなしでのファイル読み込み
  - Weaknesses: unsanitized_input
  - Bypass Vectors: ../path_traversal

### Resources (操作対象)

- **filesystem**: Medium
  - Operation: file_read
  - Protection: 

### Policy Violations

#### POL001: 信頼できない入力をファイルパスとして直接使用している

- **Path**: main() -> open(args.holiday_file)
- **Severity**: medium
- **Confidence**: 0.70

## 詳細解析

本スクリプトでは、ユーザーから渡される「--holiday-file」引数のファイルパスを検証せずに open() で読み込み、dateutil.parse に渡しています。これにより、任意のローカルファイルパスを指定してサーバ上のファイルを読み込ませ、parse() が例外を出力するときにファイル内容の一部がエラーメッセージやスタックトレースとして露出する可能性があり、LFI（Local File Inclusion）のリスクがあります。

## PoC（概念実証コード）

```text
# 任意のファイルを読み込ませる例
# /etc/passwd を読み込ませ parse() が失敗した際に一部行が例外メッセージに含まれる
python datediff.py 2020-01-01 --holiday-file /etc/passwd
```

## 修復ガイダンス

### HolidayFileInputValidation

- **Required**: ホリデーファイルのパスを検証・制限する
- **Guidance**: 許可されたディレクトリ内のみ読み込みを許可する、またはホワイトリストで管理されたファイル名のみ受け付ける
- **Priority**: high

## 解析ノート

ユーザー入力（--holiday-file）から直接 open() でファイルを読み込み、dateutil.parse() に渡している。入力検証がなく、path traversal 等で任意ファイルにアクセス可能。parse() の例外でファイル内容が漏洩する可能性があるためLFIと判断。動的モジュールロードは argparse の const 制限内であり重大なRCEには至らないので除外。

