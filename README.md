<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**Security prompt orchestrator for CLI agents.**

Parsentry analyzes repository structure, enumerates attack surfaces, and generates per-surface analysis prompts. Pipe them to any CLI agent for parallel security analysis.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### Usage

```bash
# Claude Code
parsentry model owner/repo | claude -p
parsentry scan owner/repo | claude -p

# Codex CLI (local target example)
CACHE="${PARSENTRY_CACHE_DIR:-$HOME/Library/Caches/parsentry}"
parsentry model . | codex -a never exec -C . -s workspace-write --add-dir "$CACHE" -
parsentry scan . | codex -a never exec -C . -s workspace-write --add-dir "$CACHE" -
```

That's it. `model` produces a threat model, `scan` generates per-surface prompts and outputs an orchestrator prompt to stdout. Pipe it to a CLI agent with subagent support and it dispatches parallel workers automatically. Tested with Claude Code and Codex CLI.

### How it works

```
parsentry model   →  repo metadata  →  threat model prompt (stdout)
parsentry scan    →  threat model + source code  →  orchestrator prompt (stdout)
  └─ orchestrator dispatches subagents per surface  →  SARIF files
parsentry log     →  monitor scan progress in real-time
```

1. **Model** — Collect repo metadata (languages, manifests, entry points), generate a threat model prompt. Pipe to an agent to enumerate attack surfaces.
2. **Scan** — Load the threat model, read source code per surface, generate focused `.prompt.md` files. Outputs an orchestrator prompt that launches parallel subagents — each reads its own prompt file and writes SARIF results.
3. **Log** — Stream scan progress like `docker compose logs -f`.

### Install

```bash
# Via mise (recommended)
mise use -g github:HikaruEgashira/parsentry

# Via cargo
cargo install parsentry
```

### Claude Code Skill

Install the orchestrator skill to run scans directly inside Claude Code without spawning external processes:

```bash
npx @anthropic-ai/claude-code skill add ./parsentry.skill
# or from a remote URL
npx @anthropic-ai/claude-code skill add https://github.com/HikaruEgashira/parsentry/releases/latest/download/parsentry.skill
```

Once installed, just ask Claude Code to scan a repository — the skill dispatches parallel subagents for each attack surface automatically.

### Commands

```
parsentry model [TARGET]    Generate threat model prompt (default: .)
parsentry scan  [TARGET]    Generate analysis prompts + orchestrator (default: .)
parsentry log   [TARGET]    Monitor scan progress
```

### Architecture

| Crate | Role |
|-------|------|
| `parsentry-core` | Language, RepoMetadata, ThreatModel, AttackSurface types |
| `parsentry-reports` | SARIF/Markdown report generation |

### Example Reports

| Repository | Surfaces |
|-----------|----------|
| [langgenius/dify](docs/reports/dify/) | 25 |
| [OWASP/NodeGoat](docs/reports/NodeGoat/) | 19 |
| [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/) | 25 |

### License

AGPL 3.0
