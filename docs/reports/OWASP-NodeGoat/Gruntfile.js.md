# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 70**

## 脆弱性タイプ

- `RCE`
- `AFO`
- `IDOR`
- `XSS`
- `LFI`
- `SQLI`
- `SSRF`
- `AFO`
- `IDOR`
- `XSS`
- `LFI`
- `SQLI`
- `XSS`
- `SSRF`
- `LFI`
- `SQLI`
- `IDOR`
- `XSS`
- `SSRF`
- `LFI`
- `SQLI`
- `IDOR`
- `SSRF`
- `XSS`
- `SQLI`
- `LFI`
- `IDOR`
- `SSRF`
- `XSS`

## PAR Policy Analysis

### Principals (データ源)


### Actions (セキュリティ制御)


### Resources (操作対象)


## 詳細解析

Gruntfile.js defines a "db-reset" custom task that calls child_process.exec() using a shell string built from the task argument or NODE_ENV. The `finalEnv` variable (derived from either process.env.NODE_ENV or the CLI argument) is concatenated directly into the shell command without any validation or sanitization, enabling an attacker with control over the argument to inject arbitrary shell commands. This constitutes a Remote Code Execution (RCE) vulnerability via shell metacharacter injection.

## PoC（概念実証コード）

```text
// 悪意ある Grunt 実行例:
// ターミナルで以下を実行すると、任意のコマンドが実行される
// Grunt タスクの引数としてシェルインジェクション
// 例: `grunt db-reset=production; touch hacked.txt; #`
// これにより、Gruntfile.js の exec が `production; touch hacked.txt; #` を含むシェルコマンドを実行し
// カレントディレクトリに hacked.txt が作成される

```

## 解析ノート

1. 特に危険なのは `grunt.registerTask("db-reset", ...)` 部分。
2. 引数 arg および環境変数 NODE_ENV が `finalEnv` として `cmd` の一部に無検証で埋め込まれる。
3. child_process.exec がシェル経由でコマンドを実行するため、シェルメタ文字で任意コマンド注入可能。
4. 攻撃シナリオ: `grunt db-reset=production; rm -rf /; #` など。
5. 対策: コマンド実行には execFile を使用、または入力をホワイトリスト検証して不正文字を除去する。

