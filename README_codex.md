# Parsentry with Codex CLI

Notes for running `parsentry` with Codex CLI.

- Verified on: 2026-04-08
- Verified Codex CLI: `codex-cli 0.118.0`
- Also confirmed via `npm view --registry=https://registry.npmjs.org @openai/codex version`

## TL;DR

```bash
TARGET="$PWD"
CACHE="${PARSENTRY_CACHE_DIR:-$HOME/Library/Caches/parsentry}"

PARSENTRY_CACHE_DIR="$CACHE" parsentry model "$TARGET" \
  | codex -a never exec -C "$TARGET" -s workspace-write --add-dir "$CACHE" -

PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | codex -a never exec -C "$TARGET" -s workspace-write --add-dir "$CACHE" -
```

This automatically produces:

- `model.json`
- parallel per-surface analysis
- `merged.sarif.json`
- `report.md`

## Prerequisites

With `workspace-write`, `codex exec` cannot write outside the workspace unless you explicitly allow the cache directory with `--add-dir`.

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
  | codex -a never exec \
      -C "$TARGET" \
      -s workspace-write \
      --add-dir "$CACHE" \
      -
```

This writes `"$PROJECT_CACHE/model.json"`.

## 2. Pipe `scan` Directly into Codex

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | codex -a never exec \
      -C "$TARGET" \
      -s workspace-write \
      --add-dir "$CACHE" \
      -
```

The current orchestrator prompt is aligned with the wording used for a CLI agent with subagent support, and Codex CLI will automatically:

- launch one subagent per surface
- have each subagent read its own `prompt.md` and write `result.sarif.json`
- create `merged.sarif.json` via a safe temporary-file flow
- create `report.md`

## Main Outputs

- `"$PROJECT_CACHE/model.json"`
- `"$REPORTS_DIR"/SURFACE-*/prompt.md`
- `"$REPORTS_DIR"/SURFACE-*/result.sarif.json`
- `"$REPORTS_DIR"/merged.sarif.json`
- `"$REPORTS_DIR"/report.md`

## Notes

- Without `--add-dir "$CACHE"`, Codex cannot write `model.json` or `result.sarif.json`.
- `parsentry log -f` is still optimized for Claude session logs. With Codex, prefer checking the generated output files directly.
- The `scan` orchestrator requires SARIF fields needed for merge compatibility. If you rerun an individual surface manually, make sure `result.sarif.json` includes `runs[0].tool.driver.version`.
