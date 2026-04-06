# ADR-001: Parsentry を並列 CLI Agent 実行基盤へリブランディングする

## Status

Accepted

## Date

2026-04-07

## Context

Parsentry は PAR (Principal-Action-Resource) ベースのセキュリティスキャナとして開発されてきた。内部で LLM を呼び出し、脅威モデル生成→パターンマッチ→AI分析→SARIFレポートというパイプラインを実行していた。

直近の変更 (ff94a8b) で内部 LLM 呼び出しを削除し、プロンプトを stdout に出力する方式へ移行した。これにより `parsentry | claude` のようなパイプライン構成が可能になった。

この方向性を発展させ、Parsentry を **CLI coding agent を並列実行し、キャッシュで重複実行を排除する、AI 大量並列実行時代の運用基盤** としてリブランディングする。

## Decision

Parsentry を以下のように再定義する:

> **Parsentry: 並列 CLI Agent 実行基盤**
> - タスク定義 → 並列 CLI agent ディスパッチ → 結果キャッシュ → 集約出力
> - セキュリティスキャンは最初のユースケースとして残すが、コア機能はドメイン非依存にする

### Core Principles

1. **Agent-agnostic**: Claude Code, Codex CLI, 任意の CLI ツールを統一的に実行
2. **Cache-first**: 同一入力に対する重複実行をキャッシュで排除
3. **Parallel-native**: Semaphore ベースの並列実行制御をコア機能とする
4. **Composable**: stdin/stdout パイプライン、JSON 入出力で他ツールと合成可能

## Changes Required

### Phase 1: Dead code removal (不要 crate 削除)

| Crate | 理由 | Action |
|-------|------|--------|
| `parsentry-analyzer` | 内部 LLM 呼び出し (`genai` 依存)、削除済み機能の残骸 | **削除** |
| `parsentry-codex` | OpenAI Codex 固有統合、汎用 executor に統合 | **削除** |
| `parsentry-i18n` | LLM レスポンスの多言語化、不要に | **削除** |
| `parsentry-prompt` | プロンプトビルダー、scan.rs の emit_prompt に統合済み | **削除** |
| `parsentry-threat-model` | `genai` 依存、内部 LLM 呼び出し | **削除** (metadata collector のみ `parsentry-core` に移動) |

### Phase 2: Core generalization (コア汎化)

| Component | Current | Target |
|-----------|---------|--------|
| `parsentry-claude-code` | Claude Code 専用 executor | `parsentry-executor`: 任意 CLI agent 実行エンジン (Semaphore, timeout, streaming) |
| `parsentry-cache` | "LLM response cache" | 汎用タスク結果キャッシュ (入力ハッシュ → 出力) |
| `parsentry-core` | VulnType, PAR 分類 | Task, TaskResult, Agent 定義の汎用型 |
| `parsentry-reports` | SARIF 専用 | 汎用レポートフォーマッタ (SARIF はプラグイン化) |

### Phase 3: Config & CLI cleanup

| Item | Change |
|------|--------|
| `AgentConfig.agent_type` | "genai" / "claude-code" → 任意の CLI パス指定に |
| `AnalysisConfig.model` | 削除 (agent 側の責務) |
| `AnalysisConfig.min_confidence` | セキュリティプラグインに移動 |
| `FilteringConfig.vuln_types` | セキュリティプラグインに移動 |
| `ApiConfig`, `RepoConfig`, `GenerationConfig` | 未使用、削除 |
| CLI `--agent`, `--agent-poc` | `--executor`, `--executor-args` に汎化 |
| CLI default subcommand | scan → run (汎用タスク実行) |

### Phase 4: Architecture shift

```
Before:  repo → threat model → pattern match → AI analysis → SARIF
After:   task definition → dispatch → parallel exec → cache → aggregate
                                         ↓
                              任意の CLI agent (claude, codex, etc.)
```

新しいパイプライン:
```bash
# Security scan (ユースケース1)
parsentry scan repo --preset security | claude --output-format sarif

# 汎用タスク実行
parsentry run --tasks tasks.json --executor "claude -p" --concurrency 20
```

### Phase 5: Branding & docs

| Item | Change |
|------|--------|
| README.md | "AI-only scanners are slow" → 並列 agent 実行基盤として書き直し |
| ASCII logo | 維持 (ブランド資産) |
| `scan.yml` workflow | `run.yml` に rename、汎用化 |
| ベンチマークテスト | セキュリティ固有テストは `tests/security/` に分離 |
| CLAUDE.md | アーキテクチャ説明を更新 |
| docs/concepts/ | PAR framework → Task/Executor/Cache モデルに書き直し |

## Consequences

### Positive
- セキュリティ以外のユースケース (コードレビュー、リファクタリング、テスト生成) にも適用可能
- キャッシュにより CI/CD での実行コスト大幅削減
- Agent 非依存により、ベンダーロックイン回避

### Negative
- 既存のセキュリティ特化ユーザーにとって breaking change
- `parsentry-parser` (tree-sitter) がセキュリティ固有のまま残る → プラグイン化の追加工数

### Risks
- 汎化しすぎてセキュリティスキャンの品質が下がるリスク → セキュリティは first-class preset として維持
- キャッシュ無効化ロジックの複雑さ → 入力ハッシュベースのシンプルな戦略を維持
