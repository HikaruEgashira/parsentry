# Security Analysis: configs.js - DOM access operations

## ファイル情報

- **ファイルパス**: `repo/ctfd/data/CTFd/CTFd/themes/admin/assets/js/pages/configs.js`
- **検出パターン**: DOM access operations

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **User-supplied HTML/CSS in theme header via CodeMirror editor** - Untrusted

### Actions (セキュリティ制御)

- **HTMLMixed mode accepts arbitrary HTML without sanitization or validation** - Bypassed

### Resources (操作対象)

- **Theme configuration stored server-side and delivered to all site visitors** - Critical

### Policy Violations

- **XSS**: CodeMirror input → getDoc().setValue() → patch_config_list() API → Server storage → Client-side rendering
  - Severity: warning | Confidence: 80%

## マッチしたソースコード

```javascript
document.querySelector
```

## 詳細解析

## テーマヘッダーの XSS 脆弱性

### 問題の説明
Line 317-325 で `document.getElementById("theme-header")` により、HTMLMixed モードの CodeMirror エディタが初期化されています。このエディタはユーザー入力を受け付け、HTML/CSS コンテンツを管理します。

### PAR フレームワーク分析

**Principal (信頼できない情報源)**
- ユーザーが CodeMirror エディタを通じて入力するテーマヘッダーHTML
- サーバーから返されたテーマヘッダー設定値

**Action (セキュリティ制御)**
- HTMLMixed モードでの直接入力受け入れ
- バリデーションやサニタイズが見当たらない
- ユーザー権限チェックなし（管理者のみとしても、XSS は内部脅威になり得る）

**Resource (攻撃対象)**
- `theme_header_editor.getDoc().setValue(new_css)` (Line 431)
- サーバー設定として保存される可能性
- 他のユーザーに配信されるテーマコンテンツ

### データフロー
```
ユーザー入力 (CodeMirror HTMLMixed) → 
getValue() 取得 → 
serializeJSON() で JSON化 → 
CTFd.api.patch_config_list() でサーバー送信 → 
他ユーザーが受信・レンダリング → 
XSS 実行
```

### 攻撃シナリオ
1. 攻撃者が管理画面にアクセス
2. テーマヘッダーエディタに悪意ある JavaScript を含むHTML を入力
3. 例: `<style id="theme-color">\n<script>alert('xss')</script>\n</style>`
4. 設定を保存
5. 他のユーザーがサイトを訪問時に、テーマヘッダーが注入スクリプトを実行

### 推奨される改善
1. HTML入力の厳密なバリデーション（ホワイトリストベース）
2. DOMPurify ライブラリ等を使用したサニタイズ
3. Content Security Policy (CSP) ヘッダーの導入
4. 入力の HTML エスケープ処理

