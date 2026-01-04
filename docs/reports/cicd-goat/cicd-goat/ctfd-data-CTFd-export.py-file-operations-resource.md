# Security Analysis: export.py - File operations resource

## ファイル情報

- **ファイルパス**: `repo/ctfd/data/CTFd/export.py`
- **検出パターン**: File operations resource

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `AFO`

## PAR Policy Analysis

### Principals (データ源)

- **sys.argv[1] - コマンドライン引数（ユーザー入力）** - Untrusted

### Actions (セキュリティ制御)

- **検証なし - セキュリティ制御が欠落** - Missing

### Resources (操作対象)

- **ファイルシステムへの書き込み操作** - Critical

### Policy Violations

- **AFO**: sys.argv[1] → open() → ファイル書き込み
  - Severity: error | Confidence: 90%

## マッチしたソースコード

```python
open(full_name, "wb")
```

## 詳細解析

## 脆弱性分析

### Principal (実行主体)
コマンドライン引数 `sys.argv[1]` - 完全に信頼できないユーザー入力

### Action (セキュリティ制御)
入力ファイルパスに対する検証やサニタイゼーションが存在しません。以下の制御が欠落しています：
- パストラバーサル攻撃への対策
- ファイルパスの正規化
- ホワイトリストベースの検証

### Resource (対象リソース)
`open(sys.argv[1], "wb")` - ファイルシステムへの直接的な書き込み操作

### 攻撃フロー
1. 攻撃者が `python export.py "../../../etc/passwd"` を実行
2. `sys.argv[1]` に制御されたパストラバーサルシーケンスが格納される
3. バリデーションなくファイルが開かれ、CTFエクスポートデータが任意の場所に書き込まれる
4. システムの重要なファイルが上書きされたり、権限昇格が発生する可能性

### リスク
- 任意の場所へのファイル上書き
- 重要なシステムファイルの破損
- 権限がある場合、システムの整合性侵害
- 情報漏洩（既存ファイルを指定した場合）

