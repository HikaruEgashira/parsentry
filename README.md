<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

A PAR (Principal-Action-Resource) based security scanner using LLMs and static code analysis.

**Next-generation security analysis for all languages**

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis with AI-powered vulnerability detection.

## Features

- **AI-Powered Analysis**: Uses Large Language Models for advanced vulnerability detection
- **PAR Classification**: Principal-Action-Resource framework for security issue discovery
- **Multi-Language Support**: C, C++, Go, Java, JavaScript, Python, Ruby, Rust, TypeScript, Terraform
- **Tree-sitter Parsing**: Semantic code analysis for accurate context understanding
- **Comprehensive Reports**: Detailed markdown reports with confidence scoring and PoC examples

## Usage

### Docker Usage

```bash
docker pull ghcr.io/hikaruegashira/parsentry:latest

# replace owner/repository
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --output-dir /reports --summary
```

### Command Line Options

- `--repo <REPO>`: Analyze GitHub repository (owner/repo)
- `--root <PATH>`: Analyze local directory
- `--model <MODEL>`: supports OpenAI, Anthropic, Google, Groq, Ollama, default: o4-mini
- `--output-dir <DIR>`: Directory for markdown reports
- `--summary`: Generate summary report
- `--sarif`: Generate SARIF report
- `--generate-patterns`: Generate custom patterns from codebase

## Examples

See actual vulnerability reports generated for each supported language:

- [Python Vulnerable App](docs/reports/python-vulnerable-app/)
- [JavaScript Vulnerable App](docs/reports/javascript-vulnerable-app/)
- [Go Vulnerable App](docs/reports/go-vulnerable-app/)
- [Rust Vulnerable App](docs/reports/rust-vulnerable-app/)
- [Ruby Vulnerable App](docs/reports/ruby-vulnerable-app/)
- [C Vulnerable App](docs/reports/c-vulnerable-app/)
- [C++ Vulnerable App](docs/reports/cpp-vulnerable-app/)
- [Terraform Vulnerable App](docs/reports/terraform-vulnerable-app/)

## Documentation

- [PAR Framework](docs/concepts/par_framework.md) - Principal-Action-Resource security analysis model
- [Analysis Flow](docs/concepts/analysis_flow.md) - How the analysis process works
- [Context Building](docs/concepts/context.md) - Code context generation
- [Prompts](docs/concepts/prompts.md) - LLM prompt templates
- [Response Schema](docs/concepts/response_schema.md) - Output format specification
- [Security Patterns](docs/concepts/security_patterns.md) - PAR pattern matching details

## Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

## License

AGPL 3.0
