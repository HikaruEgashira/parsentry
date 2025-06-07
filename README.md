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

### Docker Usage

```bash
docker pull ghcr.io/hikaruegashira/parsentry:latest

# Basic vulnerability analysis
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo PentesterLab/cr-go --output-dir /reports --summary

# Generate custom security patterns from GitHub repository
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  ghcr.io/hikaruegashira/parsentry \
  --repo owner/repository --generate-patterns

# Generate custom security patterns from local codebase
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/your-project:/project \
  ghcr.io/hikaruegashira/parsentry \
  --root /project --generate-patterns
```

### Command Line Options

- `--repo <REPO>`: Analyze GitHub repository (owner/repo format)
- `--root <PATH>`: Analyze local directory
- `--output-dir <DIR>`: Directory for markdown reports
- `--summary`: Generate summary report
- `--generate-patterns`: Generate custom security patterns from codebase
- `--model <MODEL>`: LLM model to use (default: o4-mini)
- `--min-confidence <SCORE>`: Minimum confidence score filter
- `--vuln-types <TYPES>`: Filter by vulnerability types (comma-separated)

## Examples

See actual vulnerability reports generated for each supported language:

- [Python Vulnerable App](docs/reports/python-vulnerable-app.md)
- [Go Vulnerable App](docs/reports/go-vulnerable-app.md)
- [Ruby Vulnerable App](docs/reports/ruby-vulnerable-app.md)
- [Rust Vulnerable App](docs/reports/rust-vulnerable-app.md)
- [Terraform Vulnerable App](docs/reports/terraform-vulnerable-app.md)

## Documentation Structure

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
