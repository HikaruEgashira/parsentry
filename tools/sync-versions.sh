#!/usr/bin/env bash
# Sync workspace crate versions to the root package version
set -euo pipefail

version="$1"
root_dir="$(cd "$(dirname "$0")/.." && pwd)"

for crate in "$root_dir"/crates/*/Cargo.toml; do
  sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$crate"
done

cd "$root_dir" && cargo update -p parsentry-core -p parsentry-claude -p parsentry-parser -p parsentry-reports -p parsentry-cache
