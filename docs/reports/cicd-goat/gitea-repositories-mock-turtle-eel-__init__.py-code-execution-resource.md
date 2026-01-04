# Security Analysis: __init__.py - Code execution resource

## ファイル情報

- **ファイルパス**: `repo/gitea/repositories/mock-turtle/eel/__init__.py`
- **検出パターン**: Code execution resource

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`

## PAR Policy Analysis

### Principals (データ源)

- **JavaScript file function names extracted via pyparsing from eel.expose() calls** - Untrusted

### Actions (セキュリティ制御)

- **Incomplete validation - only checks for '(' and '=' characters, missing other dangerous patterns** - Missing

### Resources (操作対象)

- **Python exec() function with globals() scope - arbitrary code execution** - Critical

### Policy Violations

- **RCE**: JS_file_content → os.walk() → EXPOSED_JS_FUNCTIONS.parseString() → regex_filter → _mock_js_function(f) → exec(string, globals()) → code_execution
  - Severity: error | Confidence: 80%

## マッチしたソースコード

```python
exec('%s = lambda *args: _js_call("%s", args)' % (f, f), globals())
```

## 詳細解析

## リモートコード実行脆弱性

### 概要
JavaScriptファイルから抽出された関数名が不十分な検証のみで `exec()` に直接渡されています。

### 脆弱なコード
```python
def _mock_js_function(f):
    exec('%s = lambda *args: _mock_call("%s", args)' % (f, f), globals())
```

### PAR分析

**Principal（主体）**
- 入力源: JavaScriptファイル内の `eel.expose()` 呼び出しから抽出された関数名
- 信頼度: 低（ファイルシステム上のJSファイルは編集可能な場合がある）

**Action（アクション）**
- 検証: 正規表現で `(` と `=` のみ禁止 (123行)
  ```python
  assert rgx.findall(r'[\(=]', expose_call) == [], msg
  ```
- 不足点:
  - ダブルクォート、バックティックなどの特殊文字の処理不足
  - 複雑な式やメタ文字の組み合わせに対する防御なし
  - exec()の第2引数が`globals()`のため、グローバル名前空間が直接汚染対象

**Resource（リソース）**
- 目標: Python インタプリタのコード実行エンジン
- 影響: アプリケーション全体のメモリとシステムリソースへのアクセス

### 攻撃経路
1. 攻撃者がJSファイルに悪意のある関数名を埋め込む
2. `init()` 関数がファイルをスキャンし、関数名を抽出
3. 不完全な正規表現フィルターを迂回する文字列パターンを使用
4. `_mock_js_function()` で `exec()` が評価時に任意コードを実行

### 具体例（攻撃ペイロード）
```javascript
eel.expose("x\\n__import__('os').system('malicious_command')\\n#")
```

実行される結果:
```python
x
__import__('os').system('malicious_command')
# = lambda *args: _mock_call("...", args)
```

### 影響
- リモートコード実行（RCE）
- システムコマンド実行
- ファイルシステムへの不正アクセス
- データの窃取・改ざん

### 推奨される修正
1. `exec()` の使用を避ける
2. ホワイトリスト方式で許可する文字のみを受け入れる (英数字とアンダースコアのみ)
3. より安全な実装方法（例：`setattr()` や辞書操作）を使用
4. 入力値の厳密なバリデーション

### データフロー
JSファイル → `init()` の os.walk() → 正規表現フィルタ → `_mock_js_function(f)` → `exec()` → コード実行

