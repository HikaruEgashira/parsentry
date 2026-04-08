## Project Overview

Parsentry is a security prompt orchestrator. It analyzes repository structure, enumerates attack surfaces via threat model, and generates per-surface analysis prompts. Prompts are output as files for piping to any CLI agent externally.

### Architecture

```
Phase 1: parsentry model  → RepoMetadata収集 → 脅威モデルプロンプト出力(stdout)
Phase 2: 外部agent(claude -p等)がmodel.jsonをキャッシュに書き込み
Phase 3: parsentry scan   → model.json読み込み → surface毎のプロンプト生成・orchestrator出力(stdout)
Phase 4: 外部agentがSARIF結果をキャッシュに書き込み
Phase 5: parsentry generate → SARIF結果からPDFレポート生成
```

キャッシュ: `~/Library/Caches/parsentry/<target>/` (macOS)
- `model.json` — 脅威モデル（外部agentが書き込み）
- `reports/<surface_id>/prompt.md` — surface分析プロンプト
- `reports/<surface_id>/result.sarif.json` — 分析結果（外部agentが書き込み）

### Key types

- `RepoMetadata` — リポジトリのメタデータ (directory tree, languages, manifests, entry points)
- `AttackSurface` — 分析単位。kind (自由文字列) + identifier + locations + description
- `ThreatModel` — リポジトリ内の AttackSurface 一覧。surface 単位で並列分析・キャッシュ可能
- `SurfacePrompt` — surface_id + prompt + cache_key。外部agentに渡すプロンプト

### CLI Commands

```
parsentry model [TARGET]                  # 脅威モデルプロンプトをstdoutに出力
parsentry scan [TARGET]                   # surface分析プロンプト生成 + orchestratorをstdoutに出力
  --diff-base <REF>                       # diffベースのgit ref
  --filter-lang <LANGS>                   # 言語フィルタ(カンマ区切り)
parsentry generate [TARGET] [-o PATH]     # SARIF結果からPDFレポート生成
parsentry log [TARGET] [-f] [-n LINES]    # スキャン進捗モニタ
```

## After Code Changes

```bash
cargo test && cargo check
```

## Dynamic Check Guide

独自のやられアプリがあります。動作確認に使います
- HikaruEgashira/hikae-vulnerable-python
- HikaruEgashira/hikae-vulnerable-[lang]

## Benchmark guide

性能評価はベンチマーク用のサンプルアプリケーションに対して行なってください。
負荷が高いため乱発して利用はできません

```bash
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks
cargo run -- model benchmarks/benchmarks/XBEN-001-24 | claude -p
cargo run -- scan benchmarks/benchmarks/XBEN-001-24 | claude -p
cargo run -- generate benchmarks/benchmarks/XBEN-001-24
```
