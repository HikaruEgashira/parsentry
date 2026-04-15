#!/usr/bin/env bash
# Trigger evaluation script for parsentry skill
# Usage: ./trigger_eval.sh [queries.json] [runs_per_query]
#
# Tests whether the parsentry skill triggers correctly for each query.
# Requires: claudex (or claude), jq

QUERIES_FILE="${1:-evals/trigger_queries.json}"
RUNS="${2:-1}"
SKILL_NAME="parsentry"
CLAUDE_CMD="${CLAUDE_CMD:-claudex}"
MAX_BUDGET="${MAX_BUDGET:-0.15}"

command -v "$CLAUDE_CMD" >/dev/null 2>&1 || { echo "Error: $CLAUDE_CMD not found" >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "Error: jq not found" >&2; exit 1; }

check_triggered() {
  local query="$1"
  local output
  output=$("$CLAUDE_CMD" -p "$query" \
    --output-format stream-json \
    --verbose \
    --permission-mode bypassPermissions \
    --max-budget-usd "$MAX_BUDGET" \
    2>/dev/null) || true

  echo "$output" | jq -e \
    --arg skill "$SKILL_NAME" \
    'select(.type == "assistant") | .message.content[] | select(.type == "tool_use" and .name == "Skill" and .input.skill == $skill)' \
    >/dev/null 2>&1
}

count=$(jq length "$QUERIES_FILE")
pass=0
fail=0

echo "Evaluating $count queries with $RUNS run(s) each (cmd: $CLAUDE_CMD, budget: \$$MAX_BUDGET/query)"
echo "---"

i=0
while [ "$i" -lt "$count" ]; do
  query=$(jq -r ".[$i].query" "$QUERIES_FILE")
  should_trigger=$(jq -r ".[$i].should_trigger" "$QUERIES_FILE")
  note=$(jq -r ".[$i].note" "$QUERIES_FILE")
  triggers=0

  run=1
  while [ "$run" -le "$RUNS" ]; do
    if check_triggered "$query"; then
      triggers=$((triggers + 1))
    fi
    run=$((run + 1))
  done

  if [ "$should_trigger" = "true" ]; then
    if [ "$triggers" -gt 0 ]; then
      status="PASS"
      pass=$((pass + 1))
    else
      status="FAIL"
      fail=$((fail + 1))
    fi
  else
    if [ "$triggers" -eq 0 ]; then
      status="PASS"
      pass=$((pass + 1))
    else
      status="FAIL"
      fail=$((fail + 1))
    fi
  fi

  printf "%-4s | should_trigger=%-5s triggers=%d/%d | %s\n" "$status" "$should_trigger" "$triggers" "$RUNS" "$note"
  i=$((i + 1))
done

echo ""
echo "Results: $pass passed, $fail failed out of $count queries"
[ "$fail" -eq 0 ]
