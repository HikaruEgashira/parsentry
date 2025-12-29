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

                ▲
               ╱ ╲
              ╱   ╲
             ╱ ░░░ ╲
            ╱ ░▓▓▓░ ╲
           ╱ ░▓███▓░ ╲
          ╱ ░▓█████▓░ ╲
         ╱_░▓███████▓░_╲
           ─────┬─────
                │
        P A R S E N T R Y
                │
             v0.12.0

Usage: parsentry [OPTIONS] [TARGET] [COMMAND]

Commands:
  graph     Generate call graphs from source code
  generate  Generate security patterns from source code
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [TARGET]  Target to analyze: local path or GitHub repository (owner/repo)

Options:
  -m, --model <MODEL>                    [default: gpt-5.1-codex]
      --output-dir <OUTPUT_DIR>          [default: ./reports]
      --min-confidence <MIN_CONFIDENCE>  [default: 70]
      --generate-patterns
      --api-base-url <API_BASE_URL>
      --language <LANGUAGE>              [default: ja]
  -c, --config <CONFIG>
      --generate-config

Agent Options:
      --agent <AGENT>                    [default: genai]
                                         Possible values: genai, claude-code
      --agent-path <PATH>                Path to claude CLI binary
      --agent-concurrency <N>            Max concurrent processes [default: 10]
      --agent-poc                        Enable PoC execution
```

### Output Example

```
  Discovered 9 source files
     Matched 2 security patterns
  • HTTP request handlers (1)
  • File operations resource (1)
        Mode Claude Code enabled
⠋ [00:00:42] ━━━━━━━━━━╸━━━━━━━━━━━ 1/2 analyzing with Claude Code
        Done routes.py (42.5s, $0.24)
       Wrote docs/results/summary.md
       Wrote docs/results/parsentry-results.sarif

  FILE                                      CONF  VULNERABILITIES
────────────────────────────────────────────────────────────────────
  app/routes.py                              85%  LFI, SSRF

Summary
  patterns analyzed: 2
  vulnerabilities found: 2
  high confidence: 2

    Finished analysis complete
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

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
