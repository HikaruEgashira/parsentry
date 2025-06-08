# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **user_input**: Untrusted
  - Context: テストコードのinput変数
  - Risk Factors: 外部制御入力, 不十分な検証

### Actions (セキュリティ制御)

- **input_validation_sanitization**: Missing
  - Function: パストラバーサル防御
  - Weaknesses: パス正規化欠如, ホワイトリスト検証未実装
  - Bypass Vectors: ../ path traversal

### Resources (操作対象)

- **get_prof_picture ファイル読み取り**: High
  - Operation: file_read
  - Protection: 
- **get_tax_form_attachment ファイル読み取り**: High
  - Operation: file_read
  - Protection: 

### Policy Violations

#### PT-01: ユーザー制御のパスにより任意ファイル読み取りが可能

- **Path**: TestTaxPayer.test_1 -> get_prof_picture
- **Severity**: high
- **Confidence**: 0.90

#### PT-01: ユーザー制御のパスにより任意ファイル読み取りが可能

- **Path**: TestTaxPayer.test_2 -> get_tax_form_attachment
- **Severity**: high
- **Confidence**: 0.90

## 詳細解析

テストコードはユーザー入力のパスをファイルシステム上で直接解決しようとしており、“../../../../etc/passwd”などの相対パスを用いたディレクトリ・トラバーサル攻撃（LFI）が可能であることを示しています。get_prof_picture／get_tax_form_attachment関数内で入力パスの正規化やホワイトリスト検証が行われていないため、機密ファイルの不正取得や情報漏えいを引き起こす恐れがあります。

## PoC（概念実証コード）

```text
# POC: path traversalによる/etc/passwd読み取り例
from repo.Season-1.Level-3.hack import TestTaxPayer
# フレームワークを経由せず直接呼び出し
tp = __import__('code').TaxPayer('u','p')
print(tp.get_prof_picture('../../../../etc/passwd'))
print(tp.get_tax_form_attachment('/full/path/to/tests/' + '../../../../etc/passwd'))
```

## 修復ガイダンス

### get_prof_picture / get_tax_form_attachment

- **Required**: 入力パスの正規化と許可済みディレクトリのホワイトリスト検証を実装
- **Guidance**: os.path.abspathで絶対パスに変換後、想定ルートディレクトリの下位かをチェックし、許可外なら拒否する
- **Priority**: high

## 解析ノート

・テストコードは../../で/etc/passwdへ到達
・入力検証（sanitization）なしでファイル読み取り
・LFIの典型パターン
・PRINCIPAL=ユーザー入力(untrusted)
・ACTION=パス検証欠如(missing)
・RESOURCE=ファイル読み取り(high)
・POLICY VIOLATION=PT-01 標準的なパストラバーサルルール違反
・修正: パス正規化＋ホワイトリスト検査を追加

