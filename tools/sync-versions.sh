#!/usr/bin/env bash
# Sync workspace crate versions to the root package version
set -euo pipefail

version="$1"
root_dir="$(cd "$(dirname "$0")/.." && pwd)"

for crate in "$root_dir"/crates/*/Cargo.toml; do
  sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$crate"
done
