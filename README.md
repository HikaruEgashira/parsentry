<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**Security prompt orchestrator for CLI agents.**

Parsentry analyzes repository structure, enumerates attack surfaces, and generates per-surface analysis prompts. Pipe them to any CLI agent for parallel security analysis.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### How it works

```
parsentry model <repo>              →  threat model prompt  →  pipe to agent
parsentry scan model.json <repo>    →  per-surface prompts  →  pipe to agent (parallel)
parsentry merge results/            →  merged SARIF report
```

1. **Threat Model** — Collect repo metadata, generate a prompt for attack surface enumeration
2. **Scan** — Read source code for each surface, generate focused analysis prompts
3. **Merge** — Combine per-surface SARIF results into a single report

```bash
# Step 1: Generate threat model
parsentry model owner/repo | claude -p > model.json

# Step 2: Generate per-surface prompts
parsentry scan model.json owner/repo --output-dir results/

# Step 3: Run in parallel with any CLI agent
ls results/*.prompt.md | parallel -j10 'claude -p "$(cat {})"'

# Step 4: Merge results
parsentry merge results/ -o results/merged.sarif.json
```

### Prerequisites

Parsentry generates prompts. You need a CLI agent to process them:

```bash
# Install Claude Code (recommended)
npm install -g @anthropic-ai/claude-code
```

Any CLI agent works: `claude`, `codex`, `aider`, etc.

### Installation

```bash
# Via mise (recommended)
mise use -g github:HikaruEgashira/parsentry

# Via cargo
cargo install parsentry

# Or download binary from GitHub Releases
```

### Quick Start

```bash
# Scan a GitHub repo
parsentry model owner/repo | claude -p > model.json
parsentry scan model.json owner/repo --output-dir results/
ls results/*.prompt.md | parallel 'claude -p "$(cat {})"'
parsentry merge results/ -o results/merged.sarif.json

# Scan a local project
parsentry model . | claude -p > model.json
parsentry scan model.json . --output-dir results/
```

### Command Line Options

```
Usage: parsentry <COMMAND>

Commands:
  model  Generate threat model prompt from repo metadata
  scan   Generate per-surface analysis prompts from a threat model
  merge  Merge per-surface SARIF files into a single report
  query  Show surface locations and resolved source files
  cache  Manage result cache
```

### Architecture

```
Phase 1: parsentry model   →  repo metadata  →  threat model prompt (stdout)
Phase 2: parsentry scan    →  threat model + source code  →  prompt files (output-dir)
Phase 3: user orchestrates →  prompt files  →  CLI agent  →  SARIF results
```

| Crate | Role |
|-------|------|
| `parsentry-core` | Language, RepoMetadata, ThreatModel, AttackSurface types |
| `parsentry-cache` | Content-addressable file cache |
| `parsentry-reports` | SARIF/Markdown report generation |

### Example Reports

Each report contains a `threat-model.json` and per-surface `SURFACE-*.prompt.md` files.

| Repository | Type | Files | Surfaces |
|-----------|------|-------|----------|
| [langgenius/dify](docs/reports/dify/dify/) | LLM application platform | 8531 | 25 |
| [OWASP/NodeGoat](docs/reports/NodeGoat/NodeGoat/) | Vulnerable Node.js app | 76 | 21 |
| [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/hikae-vulnerable-python/) | Vulnerable Flask app | 3 | 7 |

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
