# Parsentry with Crush

Notes for running `parsentry` with Crush.

- Verified on: 2026-04-09

## TL;DR

```bash
TARGET="$PWD"
CACHE="${PARSENTRY_CACHE_DIR:-$HOME/Library/Caches/parsentry}"

PARSENTRY_CACHE_DIR="$CACHE" parsentry model "$TARGET" \
  | crush run -c "$TARGET"

PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | crush run -c "$TARGET"
```

This automatically produces:

- `model.json`
- parallel per-surface analysis
- `merged.sarif.json`
- `report.md`

## Prerequisites

Ensure the agent has write access to the target and cache directories.

```bash
TARGET="$PWD"
CACHE="${PARSENTRY_CACHE_DIR:-$HOME/Library/Caches/parsentry}"
CACHE_KEY=${TARGET//\//__}
PROJECT_CACHE="$CACHE/$CACHE_KEY"
REPORTS_DIR="$PROJECT_CACHE/reports"
```

## 1. Generate the Threat Model

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry model "$TARGET" \
  | crush run -c "$TARGET"
```

This writes `"$PROJECT_CACHE/model.json"`.

## 2. Run Per-Surface Analysis

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | crush run -c "$TARGET"
```

The orchestrator prompt output by `parsentry scan` instructs the agent to:

- launch one subagent per surface
- have each subagent read its own `prompt.md` and write `result.sarif.json`
- create `merged.sarif.json` via a safe temporary-file flow
- create `report.md`

## 3. Generate PDF Report

```bash
parsentry generate "$TARGET"
```

## Main Outputs

- `"$PROJECT_CACHE/model.json"`
- `"$REPORTS_DIR"/SURFACE-*/prompt.md`
- `"$REPORTS_DIR"/SURFACE-*/result.sarif.json`
- `"$REPORTS_DIR"/merged.sarif.json`
- `"$REPORTS_DIR"/report.md`

## Notes

- `parsentry log -f` is still optimized for Claude session logs. With Crush, prefer checking the generated output files directly.
- The `scan` orchestrator requires SARIF fields needed for merge compatibility. If you rerun an individual surface manually, make sure `result.sarif.json` includes `runs[0].tool.driver.version`.
- Crush accepts piped input from stdin via `crush run`. Use `-c` to set the working directory.
