# Analysis Report

![高信頼度](https://img.shields.io/badge/信頼度-高-red) **信頼度スコア: 90**

## 脆弱性タイプ

- `LFI`
- `AFO`
- `IDOR`
- `SSRF`
- `RCE`
- `SQLI`
- `XSS`
- `LFI`

## PAR Policy Analysis

## 詳細解析

このコードでは、ユーザ入力（request.args["input"]）をパス結合し、os.path.normpathで正規化した後、ベースディレクトリのみを許可するチェックを行っています。しかし、ベースディレクトリ以下の任意のファイルが読み取り可能なため、本来公開すべきでない内部ソースコードや設定ファイルなどを取得できるLFI（Local File Inclusion）の脆弱性が存在します。

## PoC（概念実証コード）

```text
以下のようにリクエストを送ると、サーバのソースコード(solution.py)を取得できます。

curl 'http://localhost:5000/?input=solution.py'
```

## 解析ノート

コード内でstartwithによるベースディレクトリチェックはあるものの、ベースディレクトリ配下の任意ファイル読み取りを禁止していないため、LFIにつながる。

