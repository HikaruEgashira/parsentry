# PAR Security Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `XSS`

## PAR Policy Analysis

### Principals (データ源)

- **req.body / req.query**: Untrusted
  - Context: HTTPリクエストパラメータ
  - Risk Factors: ユーザ入力, インジェクションリスク
- **config.cookieSecret**: Trusted
  - Context: 環境変数/設定ファイル
  - Risk Factors: 

### Actions (セキュリティ制御)

- **CSRF Protection**: Missing
  - Function: CSRFトークン検証
  - Weaknesses: CSRFトークン検証なし
  - Bypass Vectors: cross-siteリクエスト
- **Session Cookie Configuration**: Missing
  - Function: セッションCookie保護
  - Weaknesses: HttpOnly/secure属性未設定
  - Bypass Vectors: スクリプトによるCookie窃取
- **Transport Layer Protection**: Missing
  - Function: 通信の暗号化
  - Weaknesses: HTTPS未強制
  - Bypass Vectors: MITM攻撃
- **Template Output Escaping**: Missing
  - Function: 出力エンコード
  - Weaknesses: 出力エスケープなし
  - Bypass Vectors: スクリプトインジェクション
- **bodyParser**: Adequate
  - Function: リクエストボディ解析
  - Weaknesses: 
  - Bypass Vectors: 

### Resources (操作対象)

- **MongoDB**: High
  - Operation: データベース操作
  - Protection: 
- **session Cookie**: Medium
  - Operation: クッキー送受信
  - Protection: 
- **HTTP通信**: High
  - Operation: ネットワークトランスポート
  - Protection: 
- **テンプレートレンダリング**: Medium
  - Operation: HTML生成
  - Protection: 

### Policy Violations

#### A6: HTTPSを強制せずHTTPのみで通信

- **Path**: http.createServer(app)
- **Severity**: high
- **Confidence**: 0.95

#### A8: CSRF保護が有効化されていない

- **Path**: // app.use(csrf());
- **Severity**: medium
- **Confidence**: 0.90

#### A3: セッションCookieにHttpOnly/secure属性が設定されていない

- **Path**: app.use(session({ ... }))
- **Severity**: medium
- **Confidence**: 0.90

#### A3: テンプレート出力におけるエスケープが保証されていない

- **Path**: app.set("view engine","ejs")
- **Severity**: medium
- **Confidence**: 0.85

## 詳細解析

このコードでは、CSRF保護やセッションCookieのセキュリティ設定、HTTPS強制、テンプレート出力のエスケープが実装されておらず、XSSやセッションハイジャック、中間者攻撃などのリスクが存在します。

## PoC（概念実証コード）

```text
<form action="http://victim.com/transfer" method="POST"><input type="hidden" name="amount" value="1000"><input type="hidden" name="to" value="attacker"><input type="submit"></form><script>document.forms[0].submit();</script>
```

## 修復ガイダンス

### CSRF Protection

- **Required**: csurfミドルウェアを有効化
- **Guidance**: app.use(csrf());でCSRFトークンを生成し、テンプレートに埋め込んで検証する
- **Priority**: high

### Session Cookie Configuration

- **Required**: cookieにhttpOnly/secure属性を設定
- **Guidance**: app.use(session({ cookie:{ httpOnly:true, secure:true, sameSite:'Strict' } }))
- **Priority**: medium

### Transport Layer Protection

- **Required**: HTTPSサーバーを有効化
- **Guidance**: https.createServer(httpsOptions, app)でHTTPSリスニングし、httpからHTTPSへリダイレクトを実装
- **Priority**: high

### Template Output Escaping

- **Required**: 出力にエスケープ/サニタイズを追加
- **Guidance**: <%= %>ではなく<%- escape(userInput) %>などで出力エンコードを行う
- **Priority**: medium

## 解析ノート

1. コードにCSRF, HTTPS, セッションCookie保護, テンプレートエスケープの実装がないことを確認
2. 各Principal(リクエスト入力、設定値)、Action(パーサー、CSRF, セッションクッキー, HTTPS, 出力エスケープ)、Resource(DB, Cookie, 通信, テンプレート)を定義
3. 脆弱性(XSS, CSRF, セッションハイジャック, MITM)に対応するポリシー違反を抽出
4. 改善策としてミドルウェア設定や安全な属性付与、HTTPS化、出力エスケープを推奨

