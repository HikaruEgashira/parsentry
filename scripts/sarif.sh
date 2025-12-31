#!/bin/bash
# scripts/sarif.sh
# Usage: sarif.sh <output_dir>
#
# output_dir内のmarkdownファイルを読み取り、
# SARIF形式とSummaryを生成する

set -e
OUTPUT_DIR="${1:-.}"

# markdownファイルから情報を抽出
declare -a RESULTS=()
declare -a SUMMARY_ROWS=()

for md_file in "$OUTPUT_DIR"/*.md; do
  [[ "$(basename "$md_file")" == "summary.md" ]] && continue
  [[ ! -f "$md_file" ]] && continue

  # ファイル情報を抽出（grepでパース）
  file_path=$(grep -m1 '^\*\*パス\*\*:' "$md_file" 2>/dev/null | sed 's/.*`\(.*\)`.*/\1/' || echo "")
  confidence=$(grep -m1 '信頼度スコア' "$md_file" 2>/dev/null | grep -oE '[0-9]+' | head -1 || echo "0")
  vuln_types=$(grep -A1 '## 脆弱性タイプ' "$md_file" 2>/dev/null | tail -1 | sed 's/- `\(.*\)`/\1/' || echo "unknown")

  # 空の場合はスキップ
  [[ -z "$file_path" ]] && continue

  # SARIF result追加
  RESULTS+=("{\"ruleId\":\"$vuln_types\",\"level\":\"warning\",\"message\":{\"text\":\"Detected $vuln_types\"},\"locations\":[{\"physicalLocation\":{\"artifactLocation\":{\"uri\":\"$file_path\"}}}]}")

  # Summary row追加
  SUMMARY_ROWS+=("| $file_path | $vuln_types | $confidence |")
done

# SARIF出力
{
  echo '{'
  echo '  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",'
  echo '  "version": "2.1.0",'
  echo '  "runs": [{'
  echo '    "tool": {"driver": {"name": "parsentry", "version": "0.1.0"}},'
  echo -n '    "results": ['

  first=true
  for result in "${RESULTS[@]}"; do
    if [ "$first" = true ]; then
      first=false
    else
      echo -n ','
    fi
    echo -n "$result"
  done

  echo ']'
  echo '  }]'
  echo '}'
} > "$OUTPUT_DIR/parsentry-results.sarif"

# Summary生成
{
  echo "# Security Analysis Summary"
  echo ""
  echo "| File | Vulnerability | Confidence |"
  echo "|------|--------------|------------|"
  for row in "${SUMMARY_ROWS[@]}"; do
    echo "$row"
  done
} > "$OUTPUT_DIR/summary.md"

echo "Generated: $OUTPUT_DIR/parsentry-results.sarif"
echo "Generated: $OUTPUT_DIR/summary.md"
