---
name: parsentry
description: >
  Parsentry security scan orchestrator. Runs a full security audit on a GitHub
  repository or local codebase — enumerates attack surfaces via threat model,
  dispatches parallel subagents for per-surface SARIF analysis, merges results,
  and generates a PDF report.
  Trigger when: user asks to "scan", "security audit", "parsentry",
  "セキュリティスキャン", "脆弱性分析", "vulnerability scan",
  "threat model", or "attack surface analysis" on a repository or codebase.
compatibility: Requires parsentry CLI (cargo install parsentry) and git.
metadata:
  author: HikaruEgashira
  version: "0.21"
allowed-tools: Bash(parsentry:*) Bash(test:*) Bash(git:*) Read Write Agent Glob
---

# Parsentry Scan Orchestrator

Run `parsentry model` → agent writes model.json → `parsentry scan` → parallel agents write SARIF → `parsentry merge` + `parsentry generate`.

## Phase 1: Threat Model

```bash
MODEL_PROMPT=$(parsentry model <TARGET> 2>/dev/null)
```

Dispatch **one Agent** with `$MODEL_PROMPT` as the prompt verbatim. The prompt already instructs the agent to write `model.json` to the correct cache path — no additional wrapping needed.

**Validate** before proceeding:

```bash
parsentry scan <TARGET> 2>&1 | head -1
# Must NOT say "model.json not found"
```

## Phase 2: Per-Surface Analysis

```bash
SCAN_OUTPUT=$(parsentry scan <TARGET> 2>&1)
```

- If output says **"all N surfaces cached, no analysis needed"** → skip to Phase 3.
- Otherwise, list all `prompt.md` files that have **no** sibling `result.sarif.json`:

```bash
# Find pending surfaces
for d in ~/Library/Caches/parsentry/*/reports/*/; do
  [ -f "$d/prompt.md" ] && [ ! -f "$d/result.sarif.json" ] && echo "$d"
done
```

Dispatch **all pending surface agents in parallel** (single message, multiple Agent tool calls). Pass each `prompt.md` content verbatim — it already contains surface metadata, source file paths, SARIF schema, and output path.

**Validate** — re-run `parsentry scan <TARGET>` and confirm all surfaces are cached.

## Phase 3: Merge & Report

```bash
parsentry merge <TARGET>
parsentry generate <TARGET>
```

Open the generated PDF path (printed to stdout) and summarize findings to the user.

## Phase 4: Triage (on request)

When the user asks to triage, patch, or fix findings, read [references/triage.md](references/triage.md) for the full workflow.

## Gotchas

- `parsentry model` output goes to **stdout**; progress/logs go to stderr. Always use `2>/dev/null` or `2>&1` intentionally.
- The cache directory is platform-dependent (`~/Library/Caches/parsentry/` on macOS, `~/.cache/parsentry/` on Linux). Let `parsentry scan` resolve paths — don't hardcode.
- Agent prompts from parsentry are self-contained. **Do not** add system prompts or wrap them — pass verbatim.
- `parsentry merge` must run **before** `parsentry generate`. Merge consolidates per-surface SARIF into a unified report; generate converts it to PDF.
- If an agent produces invalid SARIF, `parsentry merge` will skip that surface and log a warning. Check merge output for skipped surfaces.

## Error Handling

- Agent failure → retry that surface **once**
- Rate limit (429/529) → wait with exponential backoff, then retry
- Invalid SARIF → log, skip surface, note in final summary
