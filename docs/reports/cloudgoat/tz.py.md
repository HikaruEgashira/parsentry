# PAR Security Analysis Report

![中信頼度](https://img.shields.io/badge/信頼度-中-yellow) **信頼度スコア: 50**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **gettz nameパラメータ**: Untrusted
  - Context: 関数引数
  - Risk Factors: 未検証の入力, パス操作の可能性

### Actions (セキュリティ制御)

- **open(fileobj, 'rb')**: Missing
  - Function: ファイルパス検証
  - Weaknesses: パスサニタイズ欠如
  - Bypass Vectors: ../パストラバーサル, 絶対パス指定

### Resources (操作対象)

- **ファイルシステム**: Critical
  - Operation: 読み取り
  - Protection: 

### Policy Violations

#### FILE_01: 未検証のパスからのファイル読み込み禁止

- **Path**: gettz -> tzfile.__init__ -> open
- **Severity**: high
- **Confidence**: 0.80

## 詳細解析

dateutil.tz.gettz関数に渡されたnameパラメータがサニタイズなしにそのままopen(fileobj, 'rb')で読み込まれており、攻撃者が相対パス（"../"）や絶対パスを指定して任意のファイルアクセス（LFI）が可能です。

## PoC（概念実証コード）

```text
# 攻撃者による相対パストラバーサル例
from dateutil.tz import gettz
# 意図せぬファイルを読み込もうとする
tz = gettz('../../etc/passwd')
print(tz)
```

## 修復ガイダンス

### gettz/tzfile

- **Required**: ファイルパスの検証と制限
- **Guidance**: 許可されたゾーン情報ディレクトリのみアクセス可能にし、os.path.realpath で正規化後にホワイトリストを適用して許可外アクセスを防止
- **Priority**: high

## 解析ノート

コードを精査し、gettz関数のname引数が未検証でopenに渡されていることを確認。パストラバーサルや絶対パス指定によるLFIが可能と判断。修正案として許可ディレクトリのホワイトリスト検査を提案。

