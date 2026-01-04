# Security Analysis: configs.js - DOM access operations

## ファイル情報

- **ファイルパス**: `repo/ctfd/data/CTFd/CTFd/themes/admin/assets/js/pages/configs.js`
- **検出パターン**: DOM access operations

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 75**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **User-controlled JSON keys from theme settings editor** - Untrusted

### Actions (セキュリティ制御)

- **No validation of key names before DOM selector construction** - Missing

### Resources (操作対象)

- **DOM element selection and manipulation** - Critical

### Policy Violations

- **XSS**: theme_settings_editor.getValue() → JSON.parse() → form.find([name='${key}'])
  - Severity: warning | Confidence: 75%

## マッチしたソースコード

```javascript
document.querySelector
```

## 詳細解析

### DOM-based XSS 脆弱性

**箇所**: `configs.js:385`

**問題点**:
```javascript
var ctrl = form.find(`[name='${key}']`);
```

テーマ設定のJSONパース結果(`data`)から取得した`key`がテンプレートリテラルに直接埋め込まれています。ユーザーが制御可能なJSONコンテンツから取得したキー値が、そのまま属性セレクタの構築に使用されます。

**攻撃ベクトル**:
- JSONに`"]`を含むキー名を挿入することで、セレクタを閉じて任意のセレクタを追加可能
- 例: キー名が`test'] [type='hidden`の場合、セレクタは`[name='test'] [type='hidden']`となり、意図しない要素が選択される

**Principal**: テーマ設定JSON内のユーザー制御可能なキー名 (line 379)

**Action**: セレクタ構築時のバリデーション不足

**Resource**: DOM操作と属性値の設定

**Data Flow**: `theme_settings_editor.getValue()` → `JSON.parse()` → `$.each(data, ...)` → `form.find(`[name='${key}']`)`


