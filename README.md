<div align="center">

  <img width="250" src="./logo.png" alt="Vulnhuntrs Logo">

A tool to identify remotely exploitable vulnerabilities using LLMs and static code analysis.

**Autonomous AI-discovered 0day vulnerabilities**

</div>

Vulnhuntrs is a security analysis tool designed to detect vulnerabilities in applications. It provides static analysis capabilities to identify potential security issues in your codebase.

## Features

- Static code analysis for security vulnerabilities
- Multi-language support
- Detailed vulnerability reports
- Example vulnerable applications for testing

**出力例:**

```
🔍 Vulnhuntrs - セキュリティ解析ツール
📁 関連するソースファイルを検出しました (1件)
  [1] example/python-vulnerable-app/app.py
🔎 セキュリティパターン該当ファイルを検出しました (1件)
  [P1] example/python-vulnerable-app/app.py
📄 解析対象: example/python-vulnerable-app/app.py (1 / 1)
================================================================================

📝 解析レポート
================================================================================

🔍 解析結果:
--------------------------------------------------------------------------------
このアプリケーションには3つの主要な脆弱性があります。まず、/sqli エンドポイントでは、ユーザー提供の 'username' パラメータが直接SQLクエリに埋め込まれており、サニタイズが行われていないためSQLインジェクションが可能です。次に、/xss エンドポイントでは、'name' パラメータがHTMLコンテンツ内に無加工で出力されており、クロスサイトスクリプティング(XSS)の脆弱性があります。最後に、/cmdi エンドポイントでは、'hostname' パラメータがOSコマンド( ping )の引数として使用されており、コマンドインジェクション（リモートコード実行、RCE）に繋がる恐れがあります。各エントリポイントでユーザー入力の検証やエスケープ処理が欠如しているため、悪用されるとデータベースからの情報漏洩、セッションハイジャック、サーバ制御など重大なセキュリティインパクトが発生する可能性があります。

🔨 PoC（概念実証コード）:
--------------------------------------------------------------------------------
【SQLインジェクション】
URL: /sqli?username=' OR '1'='1

【XSS】
URL: /xss?name=<script>alert(1)</script>

【コマンドインジェクション(RCE)】
URL: /cmdi?hostname=localhost;whoami

📄 関連コードコンテキスト:
--------------------------------------------------------------------------------
関数名: sql_injection
理由: ユーザーの入力が直接SQLクエリに挿入されており、サニタイズがされていないためSQLインジェクションのリスクがある。
コード: query = f"SELECT * FROM users WHERE username = '{username}'"

関数名: xss
理由: ユーザーの入力がHTMLテンプレート内にそのまま表示され、エスケープ処理が施されていないためXSS攻撃が可能。
コード: template = f"\n    <h2>XSS Example</h2>\n    ...\n    <div>Hello, {name}!</div>\n    "

関数名: command_injection
理由: ユーザーの入力が直接シェルコマンドに挿入され、コマンドインジェクションを引き起こす可能性がある。
コード: output = os.popen(f"ping -c 1 {hostname}").read()


📓 解析ノート:
--------------------------------------------------------------------------------
エントリポイントは /sqli, /xss, /cmdi で、各々にユーザー入力が直接利用されている。SQLインジェクション、XSS、コマンドインジェクションの順で対策が必要な点を特定した。

✅ 解析が完了しました
```

### Usage

```bash
docker run --rm -v $(pwd):/work -w /work ghcr.io/HikaruEgashira/vulnhuntrs:latest -r <path-to-project>
```

## Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

## License

AGPL 3.0

## Acknowledgements

This project was inspired by [protectai/vulnhunter](https://github.com/protectai/vulnhuntr).
