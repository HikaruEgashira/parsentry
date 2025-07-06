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
- **Call Graph Visualization**: Generate function call relationships in multiple formats (JSON, DOT, Mermaid, CSV)
- **Cycle Detection**: Identify circular dependencies and potential infinite loops
- **Security-Focused Analysis**: Track attack vectors through function call chains

## Usage

```bash
docker pull ghcr.io/hikaruegashira/parsentry:latest

# replace owner/repository
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --output-dir /reports --generate-patterns
```

### Command Line Options

#### Security Analysis
- `--repo <REPO>`: Analyze GitHub repository (owner/repo)
- `--root <PATH>`: Analyze local directory
- `--model <MODEL>`: supports OpenAI, Anthropic, Google, Groq, Ollama, default: o4-mini
- `--output-dir <DIR>`: Directory for markdown reports
- `--generate-patterns`: Generate custom patterns from codebase

#### Call Graph Analysis
- `graph`: Generate call graph for code visualization
- `--format <FORMAT>`: Output format (json, dot, mermaid, csv), default: json
- `--output <FILE>`: Output file path
- `--start-functions <FUNCS>`: Comma-separated list of starting functions
- `--max-depth <DEPTH>`: Maximum analysis depth, default: 10
- `--include <PATTERNS>`: Include patterns (regex)
- `--exclude <PATTERNS>`: Exclude patterns (regex)
- `--detect-cycles`: Enable cycle detection
- `--security-focus`: Focus on security-relevant functions

## Examples

### Security Analysis
- [skills/secure-code-game](docs/reports/skills-secure-code-game/summary.md) - Security challenges across multiple languages
- [harishsg993010/damn-vulnerable-MCP-server](docs/reports/harishsg993010-damn-vulnerable-MCP-server/summary.md) - MCP protocol vulnerabilities
- [bridgecrewio/terragoat](docs/reports/terragoat/summary.md) - Infrastructure as Code security issues
- [RhinoSecurityLabs/cloudgoat](docs/reports/cloudgoat/summary.md) - AWS security misconfigurations
- [NeuraLegion/brokencrystals](docs/reports/NeuraLegion-brokencrystals/summary.md) - Web application security issues
- [OWASP/NodeGoat](docs/reports/NodeGoat/summary.md) - Node.js vulnerabilities
- [OWASP/railsgoat](docs/reports/railsgoat/summary.md) - Ruby on Rails vulnerabilities
- [dolevf/Damn-Vulnerable-GraphQL-Application](docs/reports/Damn-Vulnerable-GraphQL-Application/summary.md) - GraphQL vulnerabilities

### Call Graph Analysis

```bash
# Generate a JSON call graph for the entire project
parsentry graph --root src --format json --output callgraph.json

# Generate a Mermaid diagram starting from main function
parsentry graph --root src --format mermaid --start-functions main --output callgraph.md

# Generate a DOT file for Graphviz visualization with cycle detection
parsentry graph --root src --format dot --detect-cycles --output callgraph.dot

# Focus on security-relevant functions only
parsentry graph --root src --security-focus --include ".*auth.*,.*security.*" --format mermaid
```

## Understand the Concepts

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
