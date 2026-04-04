## Project Overview

Parsentry is a PAR (Principal-Action-Resource) based security scanner that enumerates attack surfaces via LLM, then runs tree-sitter pattern matching and Claude Code analysis in parallel.

### Architecture

```
Phase 1: RepoMetadata::collect() → LLM → AttackSurface列挙 (endpoint, db_table, public_api, etc.)
Phase 2: surface.locations内のファイルのみtree-sitter pattern match → Principal/Resource filter
Phase 3: 各matchをClaude Codeで並列分析 → SARIF
Phase 4: SARIF + summary.md レポート出力
```

### Key types

- `AttackSurface` — スキャン単位。kind + identifier + locations + query
- `SurfaceKind` — Endpoint, DbTable, PublicApi, FileHandler, ExternalCall, IacResource
- `ThreatModel` — リポジトリ全体のattack surface一覧

## After Code Changes

コードの変更後、現時点でのコードが正常か動作確認を行ないます

```bash
cargo test && cargo check

# Run with verbose output
cargo test -- --nocapture
```

## Dynamic Check Guide

独自のやられアプリがあります。キャッシュが効くため基本はこちらを使い、動作確認を行います

```bash
cargo run -- HikaruEgashira/hikae-vulnerable-python --output-dir docs/reports/hikae-vulnerable
```

## Benchmark guide

性能評価はベンチマーク用のサンプルアプリケーションに対して行なってください。
負荷が高いため乱発して利用はできません

```bash
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks

cargo run -- benchmarks/XBEN-001-24 --output-dir docs/benchmark/results/XBEN-001-24
```

## Behavior guide

ユーザーとはsayコマンドを使って結果を要約して応答してください
