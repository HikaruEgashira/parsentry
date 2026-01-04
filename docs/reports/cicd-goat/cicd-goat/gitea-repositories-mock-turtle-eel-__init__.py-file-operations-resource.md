# Security Analysis: __init__.py - File operations resource

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/mock-turtle/eel/__init__.py`
- **検出パターン**: File operations resource

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `LFI`

## PAR Policy Analysis

### Principals (データ源)

- **init() 関数の 'path' パラメータ（ユーザー入力）** - Untrusted

### Actions (セキュリティ制御)

- **拡張子フィルタリングのみ。パス検証なし。** - Missing

### Resources (操作対象)

- **ローカルファイルシステムの読取（open() 操作）** - Critical

### Policy Violations

- **LFI**: path → _get_real_path() → os.walk() → open(os.path.join(root, name))
  - Severity: warning | Confidence: 70%

## マッチしたソースコード

```python
open(os.path.join(root, name), encoding='utf-8')
```

## 詳細解析

## Local File Inclusion (LFI) 脆弱性

**脅威分類:** T1083 (ファイルおよびディレクトリの発見), T1005 (データソースの収集)

### 脆弱性の詳細

`init()` 関数の `path` パラメータがユーザー入力を受け取っており、これが適切に検証されないまま `os.walk()` で処理されています。

### PAR 分析

**Principal（主体）:**
- `init()` 関数の `path` 引数：呼び出し元から任意の値を受け取る

**Action（アクション）:**
- `_get_real_path()` による絶対パス変換
- `allowed_extensions` による拡張子フィルタリング
- しかし、パス入力自体に対する検証がない

**Resource（リソース）:**
- ローカルファイルシステムの読取
- 拡張子が `.js`, `.html`, `.txt`, `.htm`, `.xhtml`, `.vue` のファイル内容を読取可能

### 攻撃パス

```
ユーザー入力（path）
    ↓
init(path) 関数
    ↓
_get_real_path(path) → os.path.abspath(path)
    ↓
os.walk(root_path)
    ↓
open(os.path.join(root, name))
    ↓
ファイル内容の読取と処理
```

### 具体例

攻撃者は以下のように呼び出すことで、プロジェクト外のファイルを読み込める可能性があります：
```python
init('../../etc')  # または init('..\\..\\sensitive')
```

許可された拡張子のファイル（例：`.txt` ファイル）が存在すれば、その内容を読み込まれます。

### リスク評価

- **信頼度:** 70%
- **影響度:** 中程度（許可された拡張子のファイルのみ読取可能だが、機密情報が含まれる可能性）
- **実現性:** 中程度（アプリケーションが `init()` をユーザー入力で呼び出す場合に発生）


