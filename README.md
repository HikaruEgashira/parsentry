<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**Security prompt orchestrator for CLI agents.**

Parsentry analyzes repository structure, enumerates attack surfaces, and generates per-surface analysis prompts. Pipe them to any CLI agent for parallel security analysis.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### How it works

```
parsentry model <repo>    →  threat model prompt   →  pipe to agent
parsentry <repo>          →  per-surface prompts   →  pipe to agent (parallel)
```

1. **Threat Model** — Collect repo metadata (structure, languages, dependencies), generate a prompt for attack surface enumeration
2. **Source Resolution** — For each surface, resolve file locations and read actual source code
3. **Prompt Generation** — Output focused analysis prompts (one per surface) for piping to any CLI agent

```bash
# Step 1: Generate threat model
parsentry model owner/repo | claude -p > model.json

# Step 2: Generate per-surface prompts
parsentry --threat-model model.json --output-dir prompts/ owner/repo

# Step 3: Run in parallel with any CLI agent
ls prompts/*.prompt.md | parallel -j10 'claude -p "$(cat {})" > {.}.sarif.json'
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
# Scan a GitHub repo (generates threat model prompt to stdout)
parsentry model owner/repo | claude -p > model.json

# Generate per-surface analysis prompts
parsentry --threat-model model.json --output-dir results/ owner/repo

# Only scan changed files (great for PR reviews)
parsentry model . --diff-base origin/main | claude -p > model.json
parsentry --threat-model model.json --diff-base origin/main --output-dir results/ .
```

### Command Line Options

```
Usage: parsentry [OPTIONS] [TARGET] [COMMAND]

Commands:
  model  Generate threat model prompt from repo metadata
  query  Show surface locations and resolved source files
  cache  Manage result cache

Arguments:
  [TARGET]  Local path or GitHub repository (owner/repo)

Options:
  --threat-model <PATH>          Path to threat model JSON
  --output-dir <DIR>             Output directory for prompt files
  --diff-base <REF>              Git ref to diff against (only changed files)
  --filter-lang <LANG>           Filter by language (comma-separated)
  -v, --verbosity                Increase verbosity
  --generate-config              Print default config
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

- [langgenius/dify](docs/reports/dify/) - LLM application platform (Python/TypeScript, 8531 files)
- [assafelovic/gpt-researcher](docs/reports/gpt-researcher/) - AI research agent (Python, 298 files)
- [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/) - Vulnerable Flask app

### Security

This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.

### License

AGPL 3.0
