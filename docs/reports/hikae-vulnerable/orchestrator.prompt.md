You are a security analysis orchestrator. Your task is to analyze multiple attack surfaces in parallel by dispatching subagents.

## Instructions

1. Read each prompt file listed below using the Read tool
2. Launch ALL subagents in a SINGLE message using the Agent tool for maximum parallelism
3. Each agent must run with `run_in_background: true`
4. Each agent's prompt must include the content from the prompt file AND the instruction to write SARIF output to the specified path
5. After all agents complete, provide a summary of total findings

## Surfaces to Analyze

| Surface ID | Prompt File | SARIF Output |
|------------|-------------|-------------|
| SURFACE-001 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-001.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-001.sarif.json |
| SURFACE-002 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-002.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-002.sarif.json |
| SURFACE-003 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-003.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-003.sarif.json |
| SURFACE-004 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-004.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-004.sarif.json |
| SURFACE-005 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-005.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-005.sarif.json |
| SURFACE-006 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-006.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-006.sarif.json |
| SURFACE-007 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-007.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-007.sarif.json |
| SURFACE-008 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-008.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-008.sarif.json |
| SURFACE-009 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-009.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-009.sarif.json |
| SURFACE-010 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-010.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-010.sarif.json |
| SURFACE-011 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-011.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-011.sarif.json |
| SURFACE-012 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-012.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-012.sarif.json |
| SURFACE-013 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-013.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-013.sarif.json |
| SURFACE-014 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-014.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-014.sarif.json |
| SURFACE-015 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-015.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-015.sarif.json |
| SURFACE-016 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-016.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-016.sarif.json |
| SURFACE-017 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-017.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-017.sarif.json |
| SURFACE-018 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-018.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-018.sarif.json |
| SURFACE-019 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-019.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-019.sarif.json |
| SURFACE-020 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-020.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-020.sarif.json |
| SURFACE-021 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-021.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-021.sarif.json |
| SURFACE-022 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-022.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-022.sarif.json |
| SURFACE-023 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-023.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-023.sarif.json |
| SURFACE-024 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-024.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-024.sarif.json |
| SURFACE-025 | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-025.prompt.md | /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-025.sarif.json |

## Agent Launch Template

For each surface, use the Agent tool like this:

```
Agent(
description: "Analyze {SURFACE_ID}",
prompt: "<content from prompt file>\n\nWrite the SARIF JSON output to: {SARIF_OUTPUT_PATH}\nWrite ONLY valid JSON. No markdown, no code fences, no explanation.",
run_in_background: true,
mode: "bypassPermissions"
)
```

IMPORTANT: Launch ALL agents in a single message. Do NOT wait for one to finish before launching the next.

## Post-Analysis: Merge Results

After ALL subagents have completed, run the following command to merge the per-surface SARIF files into a single report:

```bash
parsentry merge /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable -o /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/merged.sarif.json
```

Then report the total number of findings by severity (error/warning/note) from the merged SARIF output.
