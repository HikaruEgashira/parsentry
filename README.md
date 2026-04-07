<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**Security prompt orchestrator for CLI agents.**

Parsentry analyzes repository structure, enumerates attack surfaces, and generates per-surface analysis prompts. Pipe them to any CLI agent for parallel security analysis.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### How it works

```
parsentry model <repo>              →  threat model prompt  →  pipe to agent
parsentry scan model.json <repo>    →  per-surface prompts + orchestrator prompt
parsentry merge results/            →  merged SARIF report (with baseline tracking)
```

1. **Threat Model** — Collect repo metadata, generate a prompt for attack surface enumeration
2. **Scan** — Read source code for each surface, generate focused analysis prompts + an orchestrator prompt for single-process parallel execution
3. **Merge** — Combine per-surface SARIF results into a single report with baseline comparison and triage preservation

```bash
# Step 1: Generate threat model
parsentry model owner/repo | claude -p > model.json

# Step 2: Generate prompts
parsentry scan model.json owner/repo --output-dir results/

# Step 3: Run analysis (single-process parallel via orchestrator)
claude -p "$(cat results/*/orchestrator.prompt.md)"

# Step 4: Merge results (with optional baseline for triage tracking)
parsentry merge results/*/ -o results/merged.sarif.json
parsentry merge results/*/ --baseline prev.sarif.json -o results/merged.sarif.json
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
claude -p "$(cat results/*/orchestrator.prompt.md)"
parsentry merge results/*/ -o merged.sarif.json

# Scan a local project
parsentry model . | claude -p > model.json
parsentry scan model.json . --output-dir results/
claude -p "$(cat results/*/orchestrator.prompt.md)"
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
Phase 2: parsentry scan    →  threat model + source code  →  prompt files + orchestrator
Phase 3: claude -p         →  orchestrator dispatches subagents in parallel  →  SARIF files
Phase 4: parsentry merge   →  SARIF files + baseline  →  merged report (with triage state)
```

| Crate | Role |
|-------|------|
| `parsentry-core` | Language, RepoMetadata, ThreatModel, AttackSurface types |
| `parsentry-cache` | Content-addressable file cache |
| `parsentry-reports` | SARIF merge with baseline comparison and triage preservation |

### Example Reports

Each report contains a `threat-model.json`, per-surface `SURFACE-*.prompt.md` files, and an `orchestrator.prompt.md`.

| Repository | Type | Surfaces |
|-----------|------|----------|
| [langgenius/dify](docs/reports/dify/dify/) | LLM application platform | 24 |
| [OWASP/NodeGoat](docs/reports/NodeGoat/NodeGoat/) | Vulnerable Node.js app | 21 |
| [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/) | Vulnerable Flask app | 25 |

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
