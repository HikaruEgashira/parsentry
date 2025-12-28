#!/bin/bash
# Validate parsentry pattern JSON files
# Usage: validate-patterns.sh <json-file>
# Exit codes: 0 = valid, 1 = invalid

set -e

FILE="$1"

if [ -z "$FILE" ]; then
    echo "ERROR: No file specified"
    echo "Usage: validate-patterns.sh <json-file>"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "ERROR: File not found: $FILE"
    exit 1
fi

ERRORS=()

# 1. Check JSON syntax
if ! jq empty "$FILE" 2>/dev/null; then
    echo "ERROR: Invalid JSON syntax"
    exit 1
fi

# 2. Check required structure
if ! jq -e '.patterns' "$FILE" >/dev/null 2>&1; then
    echo "ERROR: Missing 'patterns' array"
    exit 1
fi

# 3. Validate each pattern
PATTERN_COUNT=$(jq '.patterns | length' "$FILE")

for i in $(seq 0 $((PATTERN_COUNT - 1))); do
    PATTERN=$(jq -r ".patterns[$i]" "$FILE")

    # Check required fields
    CLASSIFICATION=$(echo "$PATTERN" | jq -r '.classification // empty')
    FUNCTION_NAME=$(echo "$PATTERN" | jq -r '.function_name // empty')
    QUERY_TYPE=$(echo "$PATTERN" | jq -r '.query_type // empty')
    QUERY=$(echo "$PATTERN" | jq -r '.query // empty')

    if [ -z "$CLASSIFICATION" ]; then
        ERRORS+=("Pattern $i: missing 'classification'")
    elif [[ ! "$CLASSIFICATION" =~ ^(principals|actions|resources|none)$ ]]; then
        ERRORS+=("Pattern $i: invalid classification '$CLASSIFICATION' (must be principals|actions|resources|none)")
    fi

    if [ -z "$FUNCTION_NAME" ]; then
        ERRORS+=("Pattern $i: missing 'function_name'")
    fi

    if [ -z "$QUERY_TYPE" ]; then
        ERRORS+=("Pattern $i: missing 'query_type'")
    elif [[ ! "$QUERY_TYPE" =~ ^(definition|reference)$ ]]; then
        ERRORS+=("Pattern $i: invalid query_type '$QUERY_TYPE' (must be definition|reference)")
    fi

    if [ -z "$QUERY" ]; then
        ERRORS+=("Pattern $i: missing 'query'")
    else
        # Check tree-sitter query syntax basics
        # Count parentheses
        OPEN_PARENS=$(echo "$QUERY" | tr -cd '(' | wc -c)
        CLOSE_PARENS=$(echo "$QUERY" | tr -cd ')' | wc -c)

        if [ "$OPEN_PARENS" -ne "$CLOSE_PARENS" ]; then
            ERRORS+=("Pattern $i ($FUNCTION_NAME): unbalanced parentheses in query (open=$OPEN_PARENS, close=$CLOSE_PARENS)")
        fi

        # Check for capture names (@)
        if ! echo "$QUERY" | grep -q '@'; then
            ERRORS+=("Pattern $i ($FUNCTION_NAME): query missing capture names (@name, @function, etc)")
        fi

        # Check query starts with (
        if [[ ! "$QUERY" =~ ^\( ]]; then
            ERRORS+=("Pattern $i ($FUNCTION_NAME): query should start with '('")
        fi
    fi
done

# Output results
if [ ${#ERRORS[@]} -eq 0 ]; then
    echo "OK: $PATTERN_COUNT patterns validated successfully"
    exit 0
else
    echo "ERRORS: ${#ERRORS[@]} issues found"
    for err in "${ERRORS[@]}"; do
        echo "  - $err"
    done
    exit 1
fi
