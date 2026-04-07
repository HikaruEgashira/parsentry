You are a security analysis orchestrator. Your task is to analyze multiple attack surfaces in parallel by dispatching subagents.

Instructions

1. Read each prompt file listed below using the Read tool
2. Launch ALL subagents in a SINGLE message using the Agent tool for maximum parallelism
3. Each agent must run with `run_in_background: true`
4. Each agent's prompt must include the content from the prompt file AND the instruction to write SARIF output to the specified path
5. After all agents complete, provide a summary of total findings

Surfaces to Analyze

| Surface ID | Prompt File | SARIF Output |
|------------|-------------|-------------|
| SURFACE-001 | docs/reports/dify/SURFACE-001.prompt.md | docs/reports/dify/SURFACE-001.sarif.json |
| SURFACE-002 | docs/reports/dify/SURFACE-002.prompt.md | docs/reports/dify/SURFACE-002.sarif.json |
| SURFACE-003 | docs/reports/dify/SURFACE-003.prompt.md | docs/reports/dify/SURFACE-003.sarif.json |
| SURFACE-004 | docs/reports/dify/SURFACE-004.prompt.md | docs/reports/dify/SURFACE-004.sarif.json |
| SURFACE-005 | docs/reports/dify/SURFACE-005.prompt.md | docs/reports/dify/SURFACE-005.sarif.json |
| SURFACE-006 | docs/reports/dify/SURFACE-006.prompt.md | docs/reports/dify/SURFACE-006.sarif.json |
| SURFACE-007 | docs/reports/dify/SURFACE-007.prompt.md | docs/reports/dify/SURFACE-007.sarif.json |
| SURFACE-008 | docs/reports/dify/SURFACE-008.prompt.md | docs/reports/dify/SURFACE-008.sarif.json |
| SURFACE-009 | docs/reports/dify/SURFACE-009.prompt.md | docs/reports/dify/SURFACE-009.sarif.json |
| SURFACE-010 | docs/reports/dify/SURFACE-010.prompt.md | docs/reports/dify/SURFACE-010.sarif.json |
| SURFACE-011 | docs/reports/dify/SURFACE-011.prompt.md | docs/reports/dify/SURFACE-011.sarif.json |
| SURFACE-012 | docs/reports/dify/SURFACE-012.prompt.md | docs/reports/dify/SURFACE-012.sarif.json |
| SURFACE-013 | docs/reports/dify/SURFACE-013.prompt.md | docs/reports/dify/SURFACE-013.sarif.json |
| SURFACE-014 | docs/reports/dify/SURFACE-014.prompt.md | docs/reports/dify/SURFACE-014.sarif.json |
| SURFACE-015 | docs/reports/dify/SURFACE-015.prompt.md | docs/reports/dify/SURFACE-015.sarif.json |
| SURFACE-016 | docs/reports/dify/SURFACE-016.prompt.md | docs/reports/dify/SURFACE-016.sarif.json |
| SURFACE-017 | docs/reports/dify/SURFACE-017.prompt.md | docs/reports/dify/SURFACE-017.sarif.json |
| SURFACE-018 | docs/reports/dify/SURFACE-018.prompt.md | docs/reports/dify/SURFACE-018.sarif.json |
| SURFACE-019 | docs/reports/dify/SURFACE-019.prompt.md | docs/reports/dify/SURFACE-019.sarif.json |
| SURFACE-020 | docs/reports/dify/SURFACE-020.prompt.md | docs/reports/dify/SURFACE-020.sarif.json |
| SURFACE-021 | docs/reports/dify/SURFACE-021.prompt.md | docs/reports/dify/SURFACE-021.sarif.json |
| SURFACE-022 | docs/reports/dify/SURFACE-022.prompt.md | docs/reports/dify/SURFACE-022.sarif.json |
| SURFACE-023 | docs/reports/dify/SURFACE-023.prompt.md | docs/reports/dify/SURFACE-023.sarif.json |
| SURFACE-024 | docs/reports/dify/SURFACE-024.prompt.md | docs/reports/dify/SURFACE-024.sarif.json |
| SURFACE-025 | docs/reports/dify/SURFACE-025.prompt.md | docs/reports/dify/SURFACE-025.sarif.json |

Agent Launch Template

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

Post-Analysis

After ALL subagents have completed:

1. Merge SARIF files:
```bash
parsentry merge docs/reports/dify -o docs/reports/dify/merged.sarif.json
```

2. Read docs/reports/dify/merged.sarif.json and generate a security report in markdown at docs/reports/dify/report.md.
The report must include:
- Executive summary with total findings count by severity
- Per-finding details: rule ID, severity, confidence, file location, description
- Remediation recommendations for each finding

