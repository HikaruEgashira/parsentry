## Project Overview

Parsentry is a security prompt orchestrator. It analyzes repository structure, enumerates attack surfaces via threat model, and generates per-surface analysis prompts. Prompts are output as files for piping to any CLI agent externally.

### Architecture

`parsentry scan` が唯一のエントリポイント。キャッシュ状態から次のフェーズを決定し、1フェーズだけ実行する
(`src/cli/commands/scan.rs` の `determine_phase`)。各プロンプト末尾で agent に `parsentry scan` の再実行を指示し、
`PARSENTRY_SCAN_COMPLETE` が出るまでループさせる。

```
model   : model.json が無い / 不正 / model.key ≠ RepoMetadata のハッシュ → 脅威モデルプロンプト出力
surface : .cache_key 不一致 or SARIF 不正の surface がある → surface プロンプト + orchestrator 出力
report  : 全 surface 完了 → 現行モデルの surface の SARIF のみ統合 → merged.sarif.json + report.md (+ -o で PDF)
```

モード (フラグではなく状態で決まる):
- 全体スキャン: キャッシュ無し
- インクリメンタル: surface 定義+ソースの SHA-256 (`.cache_key`) が変わった surface のみ再分析
- モデル差分: RepoMetadata が変化 → 旧 model.json / merged.sarif.json を `previous/` に退避し新モデル生成。
  report で `previous/merged.sarif.json` を baseline に merge し、new/absent/unchanged を Model Diff として出力

キャッシュ: `~/Library/Caches/parsentry/<target>/` (macOS)
- `model.json` — 脅威モデル（外部agentが書き込み）
- `model.key` / `model.pending_key` — model.json の入力 (RepoMetadata) ハッシュ
- `previous/model.json`, `previous/merged.sarif.json` — モデル差分の比較元
- `merged.sarif.json`, `report.md` — report フェーズの出力
- `reports/<surface_id>/prompt.md` — surface分析プロンプト
- `reports/<surface_id>/result.sarif.json` — 分析結果（外部agentが書き込み）

### Key types

- `RepoMetadata` — リポジトリのメタデータ (directory tree, languages, manifests, entry points)
- `AttackSurface` — 分析単位。kind (自由文字列) + identifier + locations + description
- `ThreatModel` — リポジトリ内の AttackSurface 一覧。surface 単位で並列分析・キャッシュ可能
- `SurfacePrompt` — surface_id + prompt + cache_key。外部agentに渡すプロンプト

### CLI Commands

```
parsentry scan [TARGET] [-o PDF]          # 状態から次フェーズを実行 (model → surface → report)
parsentry merge [TARGET]                  # SARIF統合 + issue連携 (--gh-issue/--jira/--linear/--notion) (hidden command)
parsentry log [TARGET] [-f] [-n LINES]    # スキャン進捗モニタ
```

## Security Scanning
- SARIF出力のCWEフィールドは必ず配列にすること（文字列ではない）。マージフェーズでのスキーマバリデーションエラーを防ぐ
- マージ前に merged.sarif.json が既に存在しないか、空でないかを確認する。再生成前にstale/emptyなファイルを削除すること

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
cargo run -- scan benchmarks/benchmarks/XBEN-001-24 -o report.pdf | claude -p
```
