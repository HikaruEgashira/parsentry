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
- [Example Reports](docs/reports/) - See actual vulnerability reports from the example applications

## 🐳 Docker での実行方法

```bash
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest
docker run ghcr.io/hikaruegashira/vulnhuntrs:latest --help

docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  --user $(id -u):$(id -g) \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  --repo PentesterLab/cr-go --output-dir /reports --summary
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
