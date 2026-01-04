# Security Analysis: models.py - File operations resource

## ファイル情報

- **ファイルパス**: `repo/models.py`
- **検出パターン**: File operations resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

## マッチしたソースコード

```code
open(file_path, 'r')
```

## 詳細解析

## Local File Inclusion (LFI) 脆弱性

### 位置
モデルファイルの`get_document_content`メソッド（195行目）

### 脆弱性の詳細
データベースから取得したファイルパスに対する検証が行われずに`open()`関数で直接使用されています。

```python
with open(file_path, 'r') as f:  # 211行目
    return f.read()
```

### PAR分析
**Principal**: ユーザー入力の`doc_id`（信頼できない）
**Action**: セキュリティ制御なし。入力値に対するバリデーションや正規化がない
**Resource**: ファイルシステム（`open()`操作）

### 攻撃ベクトル
1. SQLインジェクション（200行目）でデータベースクエリを改竄
2. 任意のファイルパスをレスポンスとして返させる
3. サーバー上の任意のファイル（/etc/passwd等）を読み込み可能

### 影響
- システムファイルの読取
- 認証情報やAPIキーの露出
- 機密情報の漏洩

### 推奨される修正
1. パラメータ化クエリ（プリペアドステートメント）を使用
2. ファイルパスのホワイトリスト検証
3. ディレクトリトラバーサル対策（os.path.realpath()で正規化し、許可ディレクトリ内であることを確認）

