---
name: parsentry
description: >
  Parsentry security scan orchestrator. Run a full security audit on a GitHub repository
  or local codebase using parsentry CLI, dispatching parallel subagents for per-surface
  analysis without spawning external claude processes.
  Trigger when: user asks to "scan", "security audit", "parsentry",
  "セキュリティスキャン", "脆弱性分析" a repository or codebase.
---

# Parsentry Scan Orchestrator

Act as the orchestrator for parsentry security scans. Dispatch subagents via Agent tool.

## Workflow

### Phase 1: Threat Model

```bash
MODEL_PROMPT=$(parsentry model <TARGET> 2>/dev/null)
```

Capture stdout into a variable. The prompt contains:
- System prompt (attack surface enumerator role)
- Repository metadata (directory tree, languages, manifests, entry points)
- Output path: `~/Library/Caches/parsentry/<owner>__<repo>/model.json`
- JSON schema for ThreatModel

Dispatch one Agent

```
Prompt: $MODEL_PROMPT
```

The prompt already instructs the agent to write JSON to the output path via Write tool. No additional instruction needed — pass the prompt verbatim.

**Verify** model.json was created before proceeding:
```bash
test -s "~/Library/Caches/parsentry/<owner>__<repo>/model.json"
```

### Phase 2: Per-Surface Analysis

```bash
parsentry scan <TARGET> 2>&1
```

If output says "all N surfaces cached, no analysis needed" → skip to Phase 3.

Otherwise, scan generates `SURFACE-*/prompt.md` files under the reports directory. For each pending surface:

1. List all `prompt.md` files that have no corresponding `result.sarif.json`
2. Dispatch **all agents in parallel** (single message with multiple Agent tool calls)

Each agent prompt — pass the prompt.md content verbatim. It already contains:
- Surface metadata (ID, kind, identifier, locations)
- Instructions to read source files and output SARIF v2.1.0
- Output path for result.sarif.json

### Phase 3: Generate Report

```bash
parsentry generate <TARGET>
```

Open the PDF and summarize findings.

## Error Handling

- Agent failure → retry that surface once
- 429/529 → wait and retry with backoff
- Invalid SARIF → log and skip, note in summary
