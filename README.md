<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

A PAR (Principal-Action-Resource) based security scanner using LLMs and static code analysis.

**Next-generation security analysis for all languages**

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

Parsentry is a PAR (Principal-Action-Resource) based security scanner designed to detect vulnerabilities across multiple languages including IaC. It provides comprehensive static analysis capabilities to identify potential security issues in your codebase.

## Features

- Static code analysis for security vulnerabilities
- Multi-language support (9 programming languages + 3 IaC languages)
- Detailed vulnerability reports

## Usage

```bash
docker pull ghcr.io/hikaruegashira/parsentry:latest
docker run ghcr.io/hikaruegashira/parsentry:latest --help

docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  --user $(id -u):$(id -g) \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo PentesterLab/cr-go --output-dir /reports --summary
```

## Examples

See actual vulnerability reports generated for each supported language:

- [Python Vulnerable App](docs/reports/python-vulnerable-app.md)
- [Go Vulnerable App](docs/reports/go-vulnerable-app.md)
- [Ruby Vulnerable App](docs/reports/ruby-vulnerable-app.md)
- [Rust Vulnerable App](docs/reports/rust-vulnerable-app.md)
- [Terraform Vulnerable App](docs/reports/terraform-vulnerable-app.md)

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
