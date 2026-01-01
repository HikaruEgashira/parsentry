<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

A PAR (Principal-Action-Resource) based security scanner using LLMs and static code analysis.

**Next-generation security analysis for all languages**

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis with AI-powered vulnerability detection.

### Features

```
🎯 PAR Classification
   Principal-Action-Resource framework for security issue discovery

🤖 AI for Security
   Uses Large Language Models for advanced vulnerability detection

🌐 Multi-Language Support
   C, C++, Go, Java, JavaScript, Python, Ruby, Rust, TypeScript, Terraform
   powered by Tree-sitter
```

### Installation

```bash
mise use -g github:HikaruEgashira/parsentry
```

Download the latest release for your platform from [GitHub Releases](https://github.com/HikaruEgashira/parsentry/releases):

### Usage

```bash
# Analyze a GitHub repository
parsentry owner/repository

# Analyze with Claude Code CLI
parsentry owner/repository --agent claude-code

# Analyze a local directory
parsentry /path/to/code

# Generate security patterns
parsentry owner/repository --generate-patterns
```

### Command Line Options

```
❯ parsentry --help

Usage: parsentry [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target to analyze: local path or GitHub repository (owner/repo)

Core Options:
  -a, --analyze <ANALYZE>                Analysis target
  -m, --model <MODEL>                    [default: gpt-5.1-codex]
      --output-dir <OUTPUT_DIR>          [default: ./reports]
      --generate-patterns                Generate security patterns
      --language <LANGUAGE>              [default: ja]

Agent Options:
      --agent <AGENT>                    [default: genai]
                                         Possible values: genai, claude-code
      --agent-poc                        Enable PoC execution

Multi-Repository Variant Analysis (MVRA):
      --mvra                             Enable multi-repository variant analysis
      --search-query <MVRA_SEARCH_QUERY> GitHub search query for MVRA
      --code-query <MVRA_CODE_QUERY>     Code search query for MVRA
      --max-repos <MVRA_MAX_REPOS>       Max repos to analyze [default: 10]
```

### Example Reports

- [skills/secure-code-game](docs/reports/skills-secure-code-game/summary.md) - Security challenges across multiple languages
- [harishsg993010/damn-vulnerable-MCP-server](docs/reports/harishsg993010-damn-vulnerable-MCP-server/summary.md) - MCP Server
- [bridgecrewio/terragoat](docs/reports/terragoat/summary.md) - Terraform
- [RhinoSecurityLabs/cloudgoat](docs/reports/cloudgoat/summary.md) - Infrastructure as Code (IaC)
- [NeuraLegion/brokencrystals](docs/reports/NeuraLegion-brokencrystals/summary.md) - Typescript
- [OWASP/NodeGoat](docs/reports/NodeGoat/summary.md) - Node.js
- [OWASP/railsgoat](docs/reports/railsgoat/summary.md) - Ruby on Rails
- [dolevf/Damn-Vulnerable-GraphQL-Application](docs/reports/Damn-Vulnerable-GraphQL-Application/summary.md) - GraphQL
- [cider-security-research/cicd-goat](docs/reports/cicd-goat/parsentry-results.sarif) - CI/CD Pipeline (analyzed with `--agent claude-code`)

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
