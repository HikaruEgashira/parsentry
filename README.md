<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**Security prompt orchestrator for CLI agents.**

Parsentry analyzes repository structure, enumerates attack surfaces, and generates per-surface analysis prompts. Pipe them to any CLI agent for parallel security analysis.

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### Usage

```bash
# 1. Generate threat model → 2. Generate & run analysis prompts
parsentry model owner/repo | claude -p
parsentry scan owner/repo | claude -p
```

That's it. `model` produces a threat model, `scan` generates per-surface prompts and outputs an orchestrator prompt to stdout. Pipe it to any CLI agent and it dispatches parallel subagents automatically.

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
