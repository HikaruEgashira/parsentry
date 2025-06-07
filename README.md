<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

A PAR (Principal-Action-Resource) based security scanner using LLMs and static code analysis.

**Next-generation security analysis for all languages**

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis with AI-powered vulnerability detection.

## Features

- **AI-Powered Analysis**: Uses Large Language Models for advanced vulnerability detection
- **PAR Classification**: Principal-Action-Resource framework for security pattern categorization
- **MITRE ATT&CK Integration**: Maps vulnerabilities to MITRE ATT&CK tactics and techniques
- **Multi-Language Support**: C, C++, Go, Java, JavaScript, Python, Ruby, Rust, TypeScript, Terraform
- **Tree-sitter Parsing**: Semantic code analysis for accurate context understanding
- **Comprehensive Reports**: Detailed markdown reports with confidence scoring and PoC examples
- **Vulnerability Detection**: SQLI, XSS, RCE, LFI, SSRF, AFO, IDOR and more

## Usage

### Docker Usage

```bash
docker pull ghcr.io/hikaruegashira/parsentry:latest

# Analyze a GitHub repository
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --output-dir /reports --summary

# Analyze local codebase
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/your-project:/project \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --root /project --output-dir /reports --summary

# Use different LLM models
docker run -e ANTHROPIC_API_KEY=$ANTHROPIC_API_KEY \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --model claude-3-sonnet
```

### Command Line Options

- `--repo <REPO>`: Analyze GitHub repository (owner/repo format)
- `--root <PATH>`: Analyze local directory  
- `--output-dir <DIR>`: Directory for markdown reports
- `--summary`: Generate summary report
- `--model <MODEL>`: LLM model to use (supports OpenAI, Anthropic, Google, Groq)
- `--min-confidence <SCORE>`: Minimum confidence score filter (0-100)
- `--vuln-types <TYPES>`: Filter by vulnerability types (LFI,RCE,SSRF,AFO,SQLI,XSS,IDOR)

## Examples

See actual vulnerability reports generated for each supported language:

- [Python Vulnerable App](docs/reports/python-vulnerable-app/) - SQLI, XSS, RCE, LFI, IDOR
- [JavaScript Vulnerable App](docs/reports/javascript-vulnerable-app/) - Most comprehensive (15 files)
- [Go Vulnerable App](docs/reports/go-vulnerable-app/) - SQLI, XSS, RCE, LFI  
- [Rust Vulnerable App](docs/reports/rust-vulnerable-app/) - SQLI, RCE, SSRF, AFO, IDOR
- [Ruby Vulnerable App](docs/reports/ruby-vulnerable-app/) - SQLI, XSS
- [C Vulnerable App](docs/reports/c-vulnerable-app/) - Buffer overflows, RCE, LFI
- [C++ Vulnerable App](docs/reports/cpp-vulnerable-app/) - Memory safety, SQLI, RCE
- [Terraform Vulnerable App](docs/reports/terraform-vulnerable-app/) - Configuration security

## Documentation

- [Analysis Flow](docs/concepts/analysis_flow.md) - How the analysis process works
- [Context Building](docs/concepts/context.md) - Code context generation
- [Prompts](docs/concepts/prompts.md) - LLM prompt templates
- [Response Schema](docs/concepts/response_schema.md) - Output format specification
- [Security Patterns](docs/concepts/security_patterns.md) - PAR pattern matching details

## Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

## License

AGPL 3.0
