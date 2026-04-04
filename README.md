<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**AI-only scanners are slow and miss vulnerabilities.**

Parsentry uses static analysis to enumerate attack surfaces, then orchestrates AI agents for deep inspection.
Scan large repositories 10x faster (or more) while catching what others miss.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### How it works

1. Attack Surface Enumeration — LLM analyzes repo metadata and enumerates endpoints, DB tables, public APIs
2. Pattern Matching — Tree-sitter finds security-relevant code in attack surface locations
3. AI Analysis — Claude Code agents analyze each pattern in parallel, outputting SARIF
4. Universal — Support `C, C++, Go, Java, JavaScript, Python, Ruby, Rust, TypeScript, Terraform`

<div align="center">
  <img src="./docs/images/run1.png" width="32%" alt="Run 1">
  <img src="./docs/images/run2.png" width="32%" alt="Run 2">
  <img src="./docs/images/run3.png" width="32%" alt="Run 3">
</div>

### Installation

```bash
mise use -g github:HikaruEgashira/parsentry
```

Download the latest release for your platform from [GitHub Releases](https://github.com/HikaruEgashira/parsentry/releases):

### Usage

```bash
# Analyze a GitHub repository
parsentry owner/repository

# Analyze a local directory
parsentry /path/to/code

# Only scan changed files against a base branch
parsentry owner/repository --diff-base origin/main
```

### Command Line Options

```
❯ parsentry --help

Usage: parsentry [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target to analyze: local path or GitHub repository (owner/repo)

Core Options:
  -m, --model <MODEL>                    [default: gpt-5.4]
      --output-dir <OUTPUT_DIR>          [default: ./reports]
      --min-confidence <MIN_CONFIDENCE>  [default: 70]
      --diff-base <DIFF_BASE>            Git ref to diff against (e.g., "origin/main")
      --filter-lang <FILTER_LANG>        Filter by language (comma-separated)

Agent Options:
      --agent <AGENT>                    [default: claude-code]
                                         Possible values: genai, claude-code
      --agent-path <AGENT_PATH>          Path to claude CLI binary
      --agent-concurrency <N>            Max concurrent processes [default: 10]
      --agent-poc                        Enable PoC execution

Cache Options:
      --cache                            Enable LLM response cache [default: true]
      --cache-only                       Use cache only (fail if cache miss)
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
- [cider-security-research/cicd-goat](docs/reports/cicd-goat/parsentry-results.sarif) - CI/CD Pipeline

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
