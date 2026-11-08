# HACKING — Agent Integration Notes

Verified setup and usage guides for running `parsentry` with various CLI agents.

## Common Setup

```bash
TARGET="$PWD"
CACHE="${PARSENTRY_CACHE_DIR:-$HOME/Library/Caches/parsentry}"
CACHE_KEY=${TARGET//\//__}
PROJECT_CACHE="$CACHE/$CACHE_KEY"
REPORTS_DIR="$PROJECT_CACHE/reports"
```

Ensure the agent has write access to `$TARGET` and `$CACHE`.

## Main Outputs (all agents)

- `"$PROJECT_CACHE/model.json"`
- `"$REPORTS_DIR"/SURFACE-*/prompt.md`
- `"$REPORTS_DIR"/SURFACE-*/result.sarif.json`
- `"$PROJECT_CACHE"/merged.sarif.json`
- `"$PROJECT_CACHE"/report.md`

## Crush

- Verified on: 2026-04-09

### TL;DR

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | crush run -c "$TARGET"
```

### Run the Scan

The agent re-runs `parsentry scan` after each phase (threat model → per-surface analysis → report) until it prints `PARSENTRY_SCAN_COMPLETE`. Append `-o report.pdf` to the `parsentry scan` invocation to also render a PDF.

### Notes

- `parsentry log -f` is still optimized for Claude session logs. With Crush, prefer checking the generated output files directly.
- The report phase requires SARIF fields needed for merge compatibility. If you rerun an individual surface manually, make sure `result.sarif.json` includes `runs[0].tool.driver.version`.
- Crush accepts piped input from stdin via `crush run`. Use `-c` to set the working directory.

## OpenCode

- Verified on: 2026-04-09

### TL;DR

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | opencode run --dir "$TARGET"
```

### Run the Scan

The agent re-runs `parsentry scan` after each phase (threat model → per-surface analysis → report) until it prints `PARSENTRY_SCAN_COMPLETE`. Append `-o report.pdf` to the `parsentry scan` invocation to also render a PDF.

### Notes

- `parsentry log -f` is still optimized for Claude session logs. With OpenCode, prefer checking the generated output files directly.
- The report phase requires SARIF fields needed for merge compatibility. If you rerun an individual surface manually, make sure `result.sarif.json` includes `runs[0].tool.driver.version`.
- OpenCode accepts piped input from stdin via `opencode run`. Use `--dir` to set the working directory.

## Codex CLI

- Verified on: 2026-04-08
- Verified Codex CLI: `codex-cli 0.118.0`
- Also confirmed via `npm view --registry=https://registry.npmjs.org @openai/codex version`

### TL;DR

```bash
PARSENTRY_CACHE_DIR="$CACHE" parsentry scan "$TARGET" \
  | codex -a never exec -C "$TARGET" -s workspace-write --add-dir "$CACHE" -
```

### Run the Scan

The agent re-runs `parsentry scan` after each phase (threat model → per-surface analysis → report) until it prints `PARSENTRY_SCAN_COMPLETE`. Append `-o report.pdf` to the `parsentry scan` invocation to also render a PDF.

### Notes

- Without `--add-dir "$CACHE"`, Codex cannot write `model.json` or `result.sarif.json`.
- `parsentry log -f` is still optimized for Claude session logs. With Codex, prefer checking the generated output files directly.
- The report phase requires SARIF fields needed for merge compatibility. If you rerun an individual surface manually, make sure `result.sarif.json` includes `runs[0].tool.driver.version`.
