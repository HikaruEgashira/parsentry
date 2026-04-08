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

### Phase 3: Merge & Report

```bash
parsentry merge <TARGET>
parsentry generate <TARGET>
```

Open the PDF and summarize findings.

### Phase 4: Triage (optional — run when user requests triage/patch/fix)

1. Read the per-surface `result.sarif.json` files under the reports directory.
2. Create a feature branch: `git checkout -b fix/<descriptive-name>`
3. Dispatch **one Agent per surface in parallel**. Each agent receives:

```
You are triaging security findings for one attack surface. Follow these steps for EACH finding in the list.

## Surface: {surface_id}
## Findings
{for each finding in this surface's result.sarif.json:}
- Rule: {rule_id}, File: {file_path}:{line}, Severity: {level}
  Description: {description}

## Per-finding workflow
For each finding:
1. Classify — Read the source file at the reported location. Based on the actual code:
   - TP: Exploitable or clearly unsafe code → proceed to Patch
   - FP: Built-in protection, operator-controlled input, by-design, or duplicate → skip
   - Low Risk: Theoretically possible but practically unexploitable → skip
2. Patch (TP only) — Apply a minimal patch. Do NOT modify tests or unrelated code.
3. Verify — After all patches for this surface, run the project quality gate. If it fails, fix and re-run.

## Output
Return a table:
| Rule | File | Classification | Reason / Patch |
```

4. Collect results from all surface agents. Present a unified summary table.

## Error Handling

- Agent failure → retry that surface once
- 429/529 → wait and retry with backoff
- Invalid SARIF → log and skip, note in summary
