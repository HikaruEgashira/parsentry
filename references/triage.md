# Phase 4: Triage Workflow

When the user requests triage, patching, or fixing of findings after a scan.

## Setup

1. Read the per-surface `result.sarif.json` files under the reports directory.
2. Create a feature branch: `git checkout -b fix/<descriptive-name>`

## Dispatch

Dispatch **one Agent per surface in parallel**. Each agent receives:

```
You are triaging security findings for one attack surface.

## Surface: {surface_id}
## Findings
{for each finding in this surface's result.sarif.json:}
- Rule: {rule_id}, File: {file_path}:{line}, Severity: {level}
  Description: {description}

## Per-finding workflow
For each finding:
1. Classify — Read the source file at the reported location:
   - TP (True Positive): Exploitable or clearly unsafe code → proceed to Patch
   - FP (False Positive): Built-in protection, operator-controlled input, by-design, or duplicate → skip
   - Low Risk: Theoretically possible but practically unexploitable → skip
2. Patch (TP only) — Apply a minimal, targeted patch. Do NOT modify tests or unrelated code.
3. Verify — After all patches for this surface, run the project quality gate:
   - If it fails → fix and re-run until it passes
   - If it passes → proceed

## Output
Return a markdown table:
| Rule | File | Classification | Reason / Patch Summary |
```

## Collect & Summarize

After all surface agents complete, present a unified summary table to the user with:
- Total findings by classification (TP / FP / Low Risk)
- List of patches applied
- Quality gate status
