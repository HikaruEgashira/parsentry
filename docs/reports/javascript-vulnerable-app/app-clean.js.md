# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `SQLI`
- `AFO`

## 解析結果

以下のコードにおいて、特にユーザー入力をそのまま内部処理に渡すエンドポイントおよび情報漏洩リスクの高いエンドポイントを発見しました。

1. /api/v1/db/query エンドポイントでは、リクエストボディの query と params をそのまま userRepository.executeQuery に渡しており、SQL インジェクションが可能です。
2. /health エンドポイントでは process.env を含む環境変数やメモリ使用量などの内部情報をそのままクライアントに返却しており、重大な情報漏洩(AFO)を引き起こします。
3. CORS 設定が origin: true, allowedHeaders: ['*'] と広範すぎるため、任意のオリジンからクッキー付きリクエストを送信・読み取りされる恐れがあります。

## PoC（概念実証コード）

```text
# SQL インジェクション POC
curl -X POST http://localhost:3000/api/v1/db/query \
  -H 'Content-Type: application/json' \
  -d '{
    "query": "SELECT username, password FROM users WHERE id = 1 OR 1=1--",
    "params": []
  }'
```

## 関連コードコンテキスト

### 関数名: /api/v1/db/query エンドポイント
- 理由: ユーザー入力の SQL 文をバリデーションやエスケープなしに直接実行しているため、SQL インジェクションが可能
- パス: repo/app-clean.js
```rust
this.app.post('/api/v1/db/query', async (req, res) => { const { query, params } = req.body; const result = await this.userRepository.executeQuery(query, params);
```

### 関数名: /health エンドポイント
- 理由: 環境変数を含む内部情報をそのまま返却しており、機密情報が漏洩する (AFO)
- パス: repo/app-clean.js
```rust
this.app.get('/health', (req, res) => { res.json({ system: { memory: process.memoryUsage(), version: process.version, environment: process.env } });
```

### 関数名: CORS 設定
- 理由: 任意のオリジンからクッキー付きリクエストを許可しており、不正クロスサイトリクエストが可能
- パス: repo/app-clean.js
```rust
this.app.use(cors({ origin: true, credentials: true, methods: ['GET','POST','PUT','DELETE','OPTIONS'], allowedHeaders: ['*'] }));
```

### 関数名: リクエストログ
- 理由: リクエストボディやヘッダ、セッション情報をそのままログ出力しており、ログから機密情報が漏洩する可能性
- パス: repo/app-clean.js
```rust
this.app.use((req, res, next) => { console.log(`${req.method} ${req.path}`, { body: req.body, query: req.query, headers: req.headers, session: req.session }); next(); });
```

## 解析ノート

エンドポイントとユーザー制御可能な入力を洗い出し
↓
/db/query: 生の SQL を直接実行 → SQLI
/health: process.env など返却 → 情報漏洩
CORS: origin: true で広範 → CSRF_API 脆弱
ログ: セッション含む → ログ漏洩
これらを JSON でレポート

