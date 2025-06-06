# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`
- `RCE`
- `AFO`

## 解析結果

このルート定義には、ファイル操作に関する複数の重大な脆弱性が含まれています。

1. ファイルアップロード(/upload)で拡張子・MIMEタイプ検証がないため、任意のファイルをアップロード・サーバ上に予測可能なパスで保存可能(AFO)。
2. ダウンロード(/download)、読み取り(/read)、一覧(/list)、削除(/delete)エンドポイントでユーザ入力をそのままpath.joinに渡しており、ディレクトリトラバーサルやLFIが可能(LFI/AFO)。
3. アーカイブ解凍(/extract)にZip Slip対策が無く、圧縮ファイル内のパス指定を悪用してサーバの任意パスへファイルを展開可能(AFO)。
4. 圧縮(/compress)でexecSyncにユーザ制御のファイル名リストをそのまま渡しており、シェルコマンドインジェクションが可能(RCE)。
5. メタデータ取得(/metadata)でファイル内容をプレビューとして返却しており、LFIによる機密情報漏洩が可能(LFI)。

## PoC（概念実証コード）

```text
curl -X POST http://<host>/compress -H "Content-Type: application/json" -d '{"files":["dummy.txt;touch /tmp/pwned"],"archive_name":"pwned.tar.gz"}'
```

## 関連コードコンテキスト

### 関数名: /upload
- 理由: ファイルタイプ検証なしで任意ファイルを予測可能パスへ配置(AFO)
- パス: example/javascript-vulnerable-app/routes/files.js
```rust
const uploadPath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, req.file.originalname);
```

### 関数名: /download
- 理由: ユーザ入力でディレクトリトラバーサル(LFI)
- パス: example/javascript-vulnerable-app/routes/files.js
```rust
const filePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, filename);
```

### 関数名: /read
- 理由: 検証なしで任意ファイル読み取り(LFI)
- パス: example/javascript-vulnerable-app/routes/files.js
```rust
const content = fs.readFileSync(filePath, 'utf8');
```

### 関数名: /extract
- 理由: Zip Slip対策なしで任意パスへファイル展開(AFO)
- パス: example/javascript-vulnerable-app/routes/files.js
```rust
await extract(req.file.path, { dir: extractDir });
```

### 関数名: /compress
- 理由: ユーザ制御文字列をシェルに渡しコマンドインジェクション(RCE)
- パス: example/javascript-vulnerable-app/routes/files.js
```rust
const output = execSync(command, { encoding: 'utf8' });
```

## 解析ノート

各エンドポイントのユーザ入力→ファイルシステム/コマンド呼び出しをスキャンし、LFI/AFO/RCE脆弱性を特定

