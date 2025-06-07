# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `IDOR`
- `SQLI`
- `LFI`
- `SSRF`
- `RCE`
- `AFO`

## 解析結果

本APIには入力検証や認可チェックがほとんど存在せず、以下の深刻な脆弱性が多数確認されました。

1. IDOR：ログイン済みユーザーが他ユーザーの情報やログを取得可能
2. SQLインジェクション：`search_documents`でユーザー入力が直接DBクエリに渡される
3. ファイル読み込みパス包含（LFI）：`get_document_content`でドキュメントIDを経路に展開
4. サーバーサイドリクエスト偽造（SSRF）：任意のURLへのHTTPリクエストを許可
5. コマンドインジェクション（RCE）：`subprocess.run(shell=True)`による任意コマンド実行
6. パストラバーサル/任意ファイル上書き（AFO）：アップロード時のファイル名検証なし
7. Zip Slip（AFO）：ZIP展開時のパス検証なし

## PoC（概念実証コード）

```text
# コマンドインジェクションのPoC
curl -X POST http://localhost:5000/api/exec/command \
  -H 'Content-Type: application/json' \
  -d '{"command":"id; cat /etc/passwd"}'
```

## 関連コードコンテキスト

### 関数名: get_user
- 理由: IDOR: 認可チェックなしで任意のユーザー情報を取得可能
- パス: repo/api.py
```rust
user = user_model.get_user_by_id(user_id)
```

### 関数名: search_documents
- 理由: SQLI: ユーザー入力(query)がエスケープなしでDBクエリに渡される
- パス: repo/api.py
```rust
documents = doc_model.search_documents(query, int(user_id))
```

### 関数名: get_document_content
- 理由: LFI: ファイルパス操作に対する入力サニタイズなし
- パス: repo/api.py
```rust
content = doc_model.get_document_content(doc_id)
```

### 関数名: ssrf_fetch
- 理由: SSRF: 任意の外部/内部URLへリクエスト送信可能
- パス: repo/api.py
```rust
response = requests.get(url, timeout=10)
```

### 関数名: execute_command
- 理由: RCE: shell=Trueで任意コマンド実行可能
- パス: repo/api.py
```rust
result = subprocess.run(command, shell=True, capture_output=True, text=True, timeout=10)
```

### 関数名: upload_file
- 理由: AFO: パストラバーサルを用いた任意ファイル上書き可能
- パス: repo/api.py
```rust
file_path = os.path.join(upload_dir, file.filename)
```

### 関数名: extract_archive
- 理由: AFO: Zip Slipにより任意のパスへファイル展開可能
- パス: repo/api.py
```rust
zip_ref.extractall(extract_dir)
```

## 解析ノート

1. 全エンドポイントを走査し、ユーザー制御可能なパラメータを抽出 2. 各種処理（DBクエリ、ファイルI/O、外部リクエスト、コマンド実行）への入力流入経路を追跡 3. 認可チェック・サニタイズ・バリデーションの有無を確認 4. 脆弱性タイプをIDOR, SQLI, LFI, SSRF, RCE, AFOに分類 5. 影響範囲と悪用方法を評価

