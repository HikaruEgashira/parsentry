# Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `XSS`
- `IDOR`
- `SSRF`
- `SQLI`

## PAR Policy Analysis

### Principals (データ源)

- **request.form['name']**: Untrusted
  - Context: HTTPリクエスト
  - Risk Factors: 攻撃者制御可能

### Actions (セキュリティ制御)

- **xss_input_handling**: Insufficient
  - Function: 入力バリデーション
  - Weaknesses: 不完全なブラックリストフィルタ, 文字列エスケープ未実装
  - Bypass Vectors: " );alert(1);//, JSFuck

### Resources (操作対象)

- **browser DOM**: High
  - Operation: スクリプト実行
  - Protection: 

## 詳細解析

このコードはユーザ入力をJavaScriptの文字列リテラル内に直接埋め込んでおり、十分なエスケープやサニタイズが行われていません。ブラックリストでは英数字と'<','>'のみを禁止しているため、ダブルクオート(")やセミコロン、スラッシュなどを利用した文字列終了＋任意スクリプト実行が可能です。さらにX-XSS-Protectionを無効化しているため、ブラウザ側保護も働きません。結果としてリフレクティッドXSSが成立します。

## PoC（概念実証コード）

```text
curl -i -X POST http://localhost:5000/page -d 'name=");alert(1);//'
```

## 修復ガイダンス

### xss32ハンドラ

- **Required**: 入力値を適切にエスケープまたはホワイトリスト化
- **Guidance**: JavaScript埋め込み時はmsgをJSON.stringifyでエスケープするか、サーバー側で厳格なホワイトリストに基づき値を制限してください。
- **Priority**: 高

## 解析ノート

1. msgはブラックリストで英数字と<,>のみ排除 → " は通過可能
2. script_content内で直接埋め込み、エスケープなし
3. ");alert(1);// によりスクリプト文を分断・実行可能
4. X-XSS-Protection無効化によりブラウザ防御不在

