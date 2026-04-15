#!/bin/bash
# Trigger evaluation script for parsentry skill
# Usage: ./trigger_eval.sh [queries.json] [runs_per_query]
#
# Tests whether the parsentry skill triggers correctly for each query.
# Requires: claude CLI, jq

set -euo pipefail

QUERIES_FILE="${1:-evals/trigger_queries.json}"
RUNS="${2:-1}"
SKILL_NAME="parsentry"

if ! command -v claude &>/dev/null; then
  echo "Error: claude CLI not found" >&2
  exit 1
fi
if ! command -v jq &>/dev/null; then
  echo "Error: jq not found" >&2
  exit 1
fi

check_triggered() {
  local query="$1"
  claude -p "$query" --output-format json 2>/dev/null \
    | jq -e --arg skill "$SKILL_NAME" \
      'any(.messages[].content[]; .type == "tool_use" and .name == "Skill" and .input.skill == $skill)' \
      >/dev/null 2>&1
}

count=$(jq length "$QUERIES_FILE")
pass=0
fail=0
results=()

for i in $(seq 0 $((count - 1))); do
  query=$(jq -r ".[$i].query" "$QUERIES_FILE")
  should_trigger=$(jq -r ".[$i].should_trigger" "$QUERIES_FILE")
  note=$(jq -r ".[$i].note" "$QUERIES_FILE")
  triggers=0

  for run in $(seq 1 "$RUNS"); do
    check_triggered "$query" && triggers=$((triggers + 1))
  done

  rate=$(echo "scale=2; $triggers / $RUNS" | bc)

  if [ "$should_trigger" = "true" ]; then
    expected_pass=$(echo "$rate > 0.49" | bc -l)
  else
    expected_pass=$(echo "$rate < 0.51" | bc -l)
  fi

  if [ "$expected_pass" -eq 1 ]; then
    status="PASS"
    pass=$((pass + 1))
  else
    status="FAIL"
    fail=$((fail + 1))
  fi

  printf "%-4s | should_trigger=%-5s rate=%.2f | %s\n" "$status" "$should_trigger" "$rate" "$note"
done

echo ""
echo "Results: $pass passed, $fail failed out of $count queries"
