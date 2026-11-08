---
name: parsentry
description: >
  Use this skill when the user wants a repository-wide or directory-wide
  security analysis that produces a structured report. It orchestrates
  parsentry CLI to enumerate attack surfaces, dispatch parallel analysis
  agents, and generate a PDF report with SARIF findings. Triggers: security
  scan, security audit, vulnerability scan, threat model, attack surface
  enumeration, pentest preparation, "parsentry", セキュリティスキャン,
  脆弱性分析.
compatibility: Requires parsentry CLI (cargo install parsentry) and git.
metadata:
  author: HikaruEgashira
  version: "0.23"
allowed-tools: Bash(parsentry:*) Bash(test:*) Bash(git:*) Read Write Agent Glob
---

# Parsentry Scan Orchestrator

`parsentry scan <TARGET>` is the only command. Each run inspects the cache and prints the prompt for the next phase (threat model → per-surface analysis → report). Loop until the output contains `PARSENTRY_SCAN_COMPLETE`. Full, incremental, and model-diff scans are chosen automatically from cache state — there are no mode flags.

## Scan Loop

```bash
OUT=$(parsentry scan <TARGET> 2>/dev/null)
```

Repeat until `$OUT` contains `PARSENTRY_SCAN_COMPLETE`:

- **Threat model prompt** (asks to write `model.json`) → dispatch **one Agent** with `$OUT` verbatim.
- **Orchestrator prompt** (lists `Worker <id>: read <prompt.md>`) → dispatch **one Agent per worker in parallel** (single message, multiple Agent calls), each with the content of its `prompt.md` verbatim.

Then run `parsentry scan <TARGET>` again. Ignore the `Next Phase` section of the prompt when you drive the loop yourself — it is for agents that run the whole pipeline from a single pipe.

## Report

The final output lists `merged.sarif.json` and `report.md` paths. Read `report.md` and summarize findings. When it reports a model diff, compare `previous/model.json` with `model.json` and explain why each model detected what the other did not. Add `-o <path>.pdf` to the final `parsentry scan` run for a PDF.

To file findings as issues, run `parsentry merge <TARGET> --gh-issue <owner/repo>` (also supports `--jira`, `--linear`, `--notion`, `--min-level`, `--dry-run`).

## Phase 4: Triage (on request)

When the user asks to triage, patch, or fix findings, read [references/triage.md](references/triage.md) for the full workflow.

## Gotchas

- Prompts go to **stdout**; progress/logs (including the current phase) go to stderr. Always use `2>/dev/null` or `2>&1` intentionally.
- The cache directory is platform-dependent (`~/Library/Caches/parsentry/` on macOS, `~/.cache/parsentry/` on Linux). Let `parsentry scan` resolve paths — don't hardcode.
- Agent prompts from parsentry are self-contained. **Do not** add system prompts or wrap them — pass verbatim.
- Invalid or missing SARIF keeps that surface pending, so the next `parsentry scan` re-emits it instead of producing a partial report.

## Error Handling

- Agent failure → retry that surface **once**
- Rate limit (429/529) → wait with exponential backoff, then retry
- Invalid SARIF → log, skip surface, note in final summary
