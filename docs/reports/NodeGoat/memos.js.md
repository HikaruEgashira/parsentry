# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `XSS`
- `AFO`
- `IDOR`
- `RCE`
- `LFI`
- `SSRF`
- `SQLI`

## PAR Policy Analysis

## 詳細解析

このコードでは、ユーザから送信されたreq.body.memoをサニタイズせずにデータベースに保存し、出力時にテンプレートへエスケープ処理なしで渡しているため、XSS攻撃が可能です。たとえば「<script>alert(1)</script>」をメモとして登録すると、表示画面でスクリプトが実行されます。

## PoC（概念実証コード）

```text
1. 悪意あるメモを登録
   POST /memos
   Content-Type: application/json
   { "memo": "<script>alert('XSS')</script>" }
2. メモ一覧ページへアクセス
   GET /memos
   → アラートダイアログが表示される
```

## 修復ガイダンス

### 入力値バリデーション

- **Required**: ユーザ入力を受け取る前にホワイトリストまたはエスケープ処理を行う
- **Guidance**: memoフィールドの値に対し、HTMLエンティティエンコードやサニタイズライブラリ（例: DOMPurify）を適用してください。
- **Priority**: high

## 解析ノート

コード解析:
- addMemos: req.body.memoをそのままmemosDAO.insertに渡している
- displayMemos: memosListをres.renderでテンプレートに渡すが、エスケープ処理が見当たらない
- テンプレートエンジン次第だが、多くはデフォルトでHTMLエスケープを行うが、場合によっては生HTML出力やJSコンテキストでの埋め込みでXSSになる
結論: XSS脆弱性あり

