# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `SQLI`
- `RCE`
- `LFI`
- `SSRF`
- `AFO`

## 解析結果

以下のコードには、ユーザー入力の不十分なサニタイズやブラックリスト方式の不完全なフィルタリングによる複数の深刻な脆弱性が存在します。

1. XSS (Cross-Site Scripting)
   - /bypass/xss/filter-test では input をブラックリストと部分的なHTMLエンコードで処理していますが、`<div>${input}</div>` や JavaScript コンテキスト内に直接埋め込む実装（コード行: res.send 内）によりあらゆるケースでのエスケープが行われず、スクリプト挿入が可能です。

2. SQL Injection (SQLI)
   - /bypass/sql/filter-test ではシンプルなクォートエスケープと大文字キーワード排除のみで不完全。クエリ文字列を直接組み立て (`const query = ...`) して返却しており、実際のDB実行時には任意のSQL挿入が可能です。

3. コマンドインジェクション (RCE)
   - /bypass/command/bypass-test では特殊文字を一部除去するのみで、`execSync(fullCommand)` による実行を行っているため、パラメータやエンコーディングを工夫することで任意コマンド実行が可能。

4. パストラバーサル (LFI)
   - /bypass/path/traversal-test では `../../` を単純に置換除去するのみで、`path.join('/safe/uploads/', filtered)` を通す実装により、URLエンコードや別文字列でのバイパス手法（例: `%2e%2e/`）が有効。

5. SSRF
   - /bypass/url/validation-bypass では private IP や localhost を検出する正規表現は実装しているものの、`new URL()` による解析後のホワイトリストチェックのみで、HTTPリクエスト自体は行っていないとはいえ、同様手法でリクエスト機能実装時にバイパス可能。

6. 認証バイパス (AFO)
   - /bypass/auth/bypass-demo では debugMode や adminKey、空パスワード、JWT 検証の不完全さなど多数のバイパスロジックを許容し、実質的に簡単に管理者認証を回避可能。

これらは実際に悪用可能な脆弱性であり、正規表現やブラックリストでは根本的な対策になっていません。適切なエスケープ・パラメタライズドクエリ、サンドボックス実行、ファイルパス正規化、外部リクエスト許可制御、堅牢な認証／認可フレームワーク導入が必要です。

## PoC（概念実証コード）

```text
// POC例: コマンドインジェクション検証
// curlでpayloadを送信し、サーバ上の/etc/passwdを取得
curl -X POST http://localhost:3000/bypass/command/bypass-test \
  -H 'Content-Type: application/json' \
  -d '{"command":"echo test; cat /etc/passwd","args":[]}'
```

## 関連コードコンテキスト

### 関数名: /xss/filter-test
- 理由: ユーザー入力をHTML本文にエスケープなしで埋め込み、XSSが可能
- パス: repo/routes/bypass.js
```rust
res.send(`...<p>Rendered: <div>${input}</div>...`)
```

### 関数名: /sql/filter-test
- 理由: 文字列連結でSQLクエリを組み立てており、SQLインジェクションが可能
- パス: repo/routes/bypass.js
```rust
const query = `SELECT * FROM users WHERE username = '${filteredUsername}' AND password = '${filteredPassword}'`
```

### 関数名: /command/bypass-test
- 理由: ユーザー制御下のコマンドをそのままexecSyncで実行しており、コマンドインジェクションリスク
- パス: repo/routes/bypass.js
```rust
const output = execSync(fullCommand, ...)
```

### 関数名: /path/traversal-test
- 理由: 単純に「..」を削除するだけで、URLエンコードや別表現でのパストラバーサルを防げない
- パス: repo/routes/bypass.js
```rust
filtered = filtered.replace(/\.\./g, '');
const fullPath = path.join('/safe/uploads/', filtered)
```

### 関数名: /url/validation-bypass
- 理由: 許可／不許可判定が不完全で、IPエンコードや認証情報を含むURLでのSSRFを防げない
- パス: repo/routes/bypass.js
```rust
const parsed = new URL(targetUrl);
const isAllowed = Object.values(validations).every(v => v);
```

### 関数名: /auth/bypass-demo
- 理由: デバッグフラグや固定キー、空パスワードなどで容易に認証をバイパス可能
- パス: repo/routes/bypass.js
```rust
if (debugMode === 'true' || debugMode === '1') bypassAttempts.push(...);
if (adminKey === 'admin123' || adminKey === 'bypass_key') ...
```

## 解析ノート

ユーザー入力のフィルタが全てブラックリストや文字列置換に依存しているため、あらゆる分野でバイパスが可能。
エントリポイント: 各ルートの req.query, req.body
データフロー: input→filtered→出力／実行
防御策: パラメタライズドクエリ, HTMLエスケープ, サンドボックス, 正規化, 強固な認証フレームワーク必要。

