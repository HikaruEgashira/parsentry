<div align="center">

  <img width="250" src="./logo.png" alt="Vulnhuntrs Logo">

A tool to identify remotely exploitable vulnerabilities using LLMs and static code analysis.

**Autonomous AI-discovered 0day vulnerabilities**

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/vulnhuntrs)

Vulnhuntrs is a security analysis tool designed to detect vulnerabilities in applications. It provides static analysis capabilities to identify potential security issues in your codebase.


## Features

- Static code analysis for security vulnerabilities
- Multi-language support
  - Supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby.
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

## 🐳 Docker での実行方法

```bash
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest

docker run ghcr.io/hikaruegashira/vulnhuntrs:latest --repo https://github.com/PentesterLab/cr-go

docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  --user $(id -u):$(id -g) \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  --repo https://github.com/PentesterLab/cr-go --output-dir /reports --summary

docker run ghcr.io/hikaruegashira/vulnhuntrs:latest --help
```

### multi architecture image build

```bash
docker buildx create --use
docker buildx build --platform linux/amd64 -t ghcr.io/hikaruegashira/vulnhuntrs:latest --push .
```

### オプション

- `-r, --root <ROOT>`: スキャンするプロジェクトのルートディレクトリを指定
- `--repo <REPO>`: GitHubリポジトリのURLを指定して解析
- `-a, --analyze <ANALYZE>`: 特定のファイルやディレクトリを指定して解析
- `-v`: 詳細なログを表示（複数指定でより詳細に）
- `--min-confidence <MIN_CONFIDENCE>`: 表示する脆弱性の最小信頼度を指定（デフォルト: 0）
- `--vuln-types <TYPES>`: 特定の脆弱性タイプでフィルタリング（カンマ区切り）
- `--summary`: サマリーレポートを表示

## Documentation Structure

- [Concepts](docs/concepts/) - Details about the LLM integration and prompts
  - [Analysis Flow](docs/concepts/analysis_flow.md) - How the analysis process works
  - [Context Building](docs/concepts/context.md) - Code context generation
  - [Prompts](docs/concepts/prompts.md) - LLM prompt templates
  - [Response Schema](docs/concepts/response_schema.md) - Output format specification
  - [Security Patterns](docs/concepts/security_patterns.md) - Pattern matching details

## Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

## License

AGPL 3.0

## Acknowledgements

This project was inspired by [protectai/vulnhuntr](https://github.com/protectai/vulnhuntr).
