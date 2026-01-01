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

Usage: parsentry [OPTIONS] [TARGET] [COMMAND]

Commands:
  graph     Generate call graphs from source code
  generate  Generate security patterns from source code
  cache     Manage LLM response cache
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [TARGET]  Target to analyze: local path or GitHub repository (owner/repo)

Core Options:
  -a, --analyze <ANALYZE>                Analysis target
  -m, --model <MODEL>                    [default: gpt-5.1-codex]
  -v, --verbosity...                     Increase verbosity level
  -e, --evaluate                         Enable evaluation mode
      --output-dir <OUTPUT_DIR>          [default: ./reports]
      --min-confidence <MIN_CONFIDENCE>  [default: 70]
      --vuln-types <VULN_TYPES>          Vulnerability types to analyze
      --generate-patterns                Generate security patterns
      --debug                            Enable debug mode
      --api-base-url <API_BASE_URL>      Custom API base URL
      --language <LANGUAGE>              [default: ja]
  -c, --config <CONFIG>                  Config file path
      --generate-config                  Generate config file

Agent Options:
      --agent <AGENT>                    [default: genai]
                                         Possible values: genai, claude-code
      --agent-path <PATH>                Path to claude CLI binary
      --agent-concurrency <N>            Max concurrent processes [default: 10]
      --agent-poc                        Enable PoC execution

Multi-Repository Variant Analysis (MVRA):
      --mvra                             Enable multi-repository variant analysis
      --search-query <MVRA_SEARCH_QUERY> GitHub search query for MVRA
      --code-query <MVRA_CODE_QUERY>     Code search query for MVRA
      --max-repos <MVRA_MAX_REPOS>       Max repos to analyze [default: 10]
      --cache-dir <MVRA_CACHE_DIR>       Cache directory [default: .parsentry-cache]
      --mvra-cache[=<bool>]          Enable repository cache [default: true]

Cache Options:

      --cache[=<bool>]              Enable LLM response cache [default: true]
      --cache-only                       Use cache only (fail if miss)
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
- [cider-security-research/cicd-goat](docs/reports/cicd-goat/parsentry-results.sarif) - CI/CD Pipeline (analyzed with `--agent claude-code`)

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

#### Cache Directory Policy

The `.parsentry-cache` directory stores cloned repositories for MVRA (Multi-Repository Variant Analysis) and LLM response caches. To improve development efficiency, some intentionally vulnerable sample applications are committed to GitHub for testing and benchmarking.

**Important:**
- The `.parsentry-cache` directory is explicitly listed in `.gitignore` by default
- Only pre-approved intentionally vulnerable applications (e.g., OWASP goat apps, CTF challenges) may be committed via `git add -f`
- Never commit cache data from real-world application scans
- LLM response caches may contain sensitive information and should never be committed

### License

AGPL 3.0
