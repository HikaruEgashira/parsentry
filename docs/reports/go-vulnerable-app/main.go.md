# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `XSS`
- `RCE`
- `LFI`

## 解析結果

アプリケーションには以下の深刻な脆弱性が確認されました。

1. SQLインジェクション（/sqli）
   • ユーザー入力をエスケープせずに`fmt.Sprintf`で直接クエリ文字列に埋め込んでいるため、任意のSQL実行が可能。

2. クロスサイトスクリプティング（/xss）
   • ユーザー入力を`fmt.Sprintf`でテンプレートに埋め込み、そのまま出力しているため、スクリプトが実行される。

3. コマンドインジェクション（/cmdi）
   • `hostname`パラメータを`sh -c`で実行する`exec.Command`に直接渡しており、シェル挙動を含む任意コマンド実行が可能。

4. ローカルファイル参照（LFI）（/file）
   • ディレクトリトラバーサルのチェックが不十分（「..」の検出のみ）で、任意のファイルを読み取れる。

さらに認証・認可の仕組みが全く実装されておらず、全エンドポイントに匿名でアクセス可能です。

## PoC（概念実証コード）

```text
1. SQLインジェクション
   URL: http://127.0.0.1:8080/sqli?username=admin'%20OR%201=1--

2. XSS
   URL: http://127.0.0.1:8080/xss?name=<script>alert(1)</script>

3. コマンドインジェクション
   URL: http://127.0.0.1:8080/cmdi?hostname=localhost;id

4. LFI
   URL: http://127.0.0.1:8080/file?name=../main.go
```

## 関連コードコンテキスト

### 関数名: sqliHandler
- 理由: ユーザー入力を直接SQLに埋め込んでおり、SQLインジェクションが可能
- パス: repo/main.go
```rust
query := fmt.Sprintf("SELECT id, username, password FROM users WHERE username = '%s'", username)
```

### 関数名: xssHandler
- 理由: ユーザー入力をHTMLコンテキストに無加工で埋め込んでいるため、XSSが発生する
- パス: repo/main.go
```rust
tmplStr := fmt.Sprintf(`
<!DOCTYPE html>
<html>
<head>
    <title>XSS Test</title>
</head>
<body>
    <h2>Hello, %s!</h2>
    <p>Your input was: %s</p>
</body>
</html>`, name, name)
```

### 関数名: cmdiHandler
- 理由: 入力をシェルコマンドに直接埋め込んでおり、コマンドインジェクションが可能
- パス: repo/main.go
```rust
command := fmt.Sprintf("ping -c 1 %s", hostname)
```

### 関数名: fileHandler
- 理由: ファイル名を検証せず任意のパスを読めるため、LFI攻撃が可能
- パス: repo/main.go
```rust
content, err := os.ReadFile(filename)
```

## 解析ノート

1. 各ハンドラのユーザー入力取得箇所を特定
2. fmt.Sprintfやexec.Command、os.ReadFileの引数に未サニタイズの入力が渡されている点を抽出
3. 影響範囲としてSQL実行、HTML出力、シェル実行、ファイル参照を確認
4. 認証なしで全エンドポイントへアクセス可能な点も確認

