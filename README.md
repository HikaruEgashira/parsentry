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

## Examples

See actual vulnerability reports generated for each supported language:

- [Python Vulnerable App](docs/reports/python-vulnerable-app.md) - SQL injection, XSS, and command injection
- [Go Vulnerable App](docs/reports/go-vulnerable-app.md) - SQL injection, XSS, command injection, and file traversal  
- [Ruby Vulnerable App](docs/reports/ruby-vulnerable-app.md) - SQL injection and XSS vulnerabilities
- [Rust Vulnerable App](docs/reports/rust-vulnerable-app.md) - SQL injection, command injection, and path traversal

## 🐳 Docker Usage

```bash
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest
docker run ghcr.io/hikaruegashira/vulnhuntrs:latest --help

docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  --user $(id -u):$(id -g) \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  --repo PentesterLab/cr-go --output-dir /reports --summary
```

## Command Line Options

- `-r, --root <ROOT>`: Specify the root directory of the project to scan
- `--repo <REPO>`: Specify GitHub repository URL for analysis
- `-a, --analyze <ANALYZE>`: Specify a specific file or directory to analyze
- `-v`: Show verbose logs (specify multiple times for more detail)
- `--min-confidence <MIN_CONFIDENCE>`: Specify minimum confidence level for displayed vulnerabilities (default: 0)
- `--vuln-types <TYPES>`: Filter by specific vulnerability types (comma-separated)
- `--summary`: Display summary report

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
