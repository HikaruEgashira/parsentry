## Project Overview

Parsentry is a parallel CLI agent execution platform with result caching. It uses static analysis (tree-sitter) to enumerate code patterns, then dispatches them to CLI agents (Claude Code, etc.) for parallel analysis with deduplication via cache.

### Architecture

```
Phase 1: RepoMetadata::collect() → リポジトリメタデータ収集 (languages, manifests, entry points)
Phase 2: tree-sitter pattern match → Principal/Resource filter → セキュリティパターン検出
Phase 3: プロンプト生成 → stdout 出力 (パイプラインで任意の agent に接続)
```

### Crate structure

| Crate | Role |
|-------|------|
| `parsentry-core` | Language, RepoMetadata, ThreatModel, PAR types |
| `parsentry-parser` | tree-sitter pattern matching |
| `parsentry-executor` | Parallel CLI agent execution (Semaphore, timeout, streaming) |
| `parsentry-cache` | Task result cache (SHA2 hash key, file-based storage) |
| `parsentry-reports` | SARIF/Markdown report generation |
| `parsentry-utils` | File classification/discovery utilities |

### Key types

- `RepoMetadata` — リポジトリのメタデータ (directory tree, languages, manifests, entry points)
- `AttackSurface` — スキャン単位。kind + identifier + locations + query
- `ThreatModel` — リポジトリ全体の attack surface 一覧

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
