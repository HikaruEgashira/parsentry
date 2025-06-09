# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 80**

## 脆弱性タイプ

- `RCE`
- `AFO`
- `IDOR`
- `SSRF`
- `LFI`
- `SQLI`
- `XSS`

## PAR Policy Analysis

## 詳細解析

Gruntfile.js内のdb-resetタスクでは、ユーザ指定の引数（arg）または環境変数NODE_ENVを検証なしでシェルコマンドに埋め込み、child_process.execで実行しているため、任意のシェルコマンドを注入できるRCE脆弱性があります。

## PoC（概念実証コード）

```text
# 任意のシェルコマンドを実行する例
grant db-reset:"development && echo pwned > pwn.txt"

# もしくは
NODE_ENV="production; rm -rf /tmp/*" grunt db-reset

```

## 解析ノート

- db-resetタスクでexecを用いてshell実行
- cmdにfinalEnvを検証せずに連結
- grunt db-reset:<arg>でargにメタ文字注入可能
- RCEリスクあり


