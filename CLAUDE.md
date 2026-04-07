## Project Overview

Parsentry is a security prompt orchestrator. It analyzes repository structure, enumerates attack surfaces via threat model, and generates per-surface analysis prompts. Prompts are output as files for piping to any CLI agent externally.

### Architecture

```
Phase 1: RepoMetadata::collect() → リポジトリメタデータ収集 (languages, manifests, entry points)
Phase 2: ThreatModel → surface単位でソースコード読み込み → プロンプト生成
Phase 3: プロンプトファイル出力 → ユーザーが外部agentにpipe
```

### Crate structure

| Crate | Role |
|-------|------|
| `parsentry-core` | Language, RepoMetadata, ThreatModel, AttackSurface types |
| `parsentry-reports` | SARIF/Markdown report generation |

### Key types

- `RepoMetadata` — リポジトリのメタデータ (directory tree, languages, manifests, entry points)
- `AttackSurface` — 分析単位。kind (自由文字列) + identifier + locations + description。影響が分離した独立コンポーネント
- `ThreatModel` — リポジトリ内の AttackSurface 一覧。コンポーネント間の影響分離を定義し、surface 単位で並列分析・キャッシュを可能にする
- `SurfacePrompt` — surface_id + prompt + cache_key。外部agentに渡すプロンプト

## After Code Changes

コードの変更後、現時点でのコードが正常か動作確認を行ないます

```bash
cargo test && cargo check

# Run with verbose output
cargo test -- --nocapture
```

## Dynamic Check Guide

独自のやられアプリがあります。動作確認に使います

```bash
# Step 1: threat model生成
cargo run -- model HikaruEgashira/hikae-vulnerable-python | claude -p > /tmp/model.json

# Step 2: per-surface prompt生成
cargo run -- scan /tmp/model.json HikaruEgashira/hikae-vulnerable-python --output-dir docs/reports/hikae-vulnerable

# Step 3: agent実行（1プロセスでparallel実行）
claude -p "$(cat docs/reports/hikae-vulnerable/hikae-vulnerable-python/orchestrator.prompt.md)"

# Step 4: 結果集約
cargo run -- merge docs/reports/hikae-vulnerable/hikae-vulnerable-python -o merged.sarif.json
```

## Benchmark guide

性能評価はベンチマーク用のサンプルアプリケーションに対して行なってください。
負荷が高いため乱発して利用はできません

```bash
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks

cargo run -- model benchmarks/XBEN-001-24 | claude -p > /tmp/model.json
cargo run -- scan /tmp/model.json benchmarks/XBEN-001-24 --output-dir docs/benchmark/results/XBEN-001-24
```

## Behavior guide

ユーザーとはsayコマンドを使って結果を要約して応答してください
