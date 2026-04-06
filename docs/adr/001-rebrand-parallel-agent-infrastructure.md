# ADR-001: Parsentry を並列 CLI Agent 実行基盤へリブランディングする

## Status

Accepted (実施中)

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

### ThreatModel の意味論

ThreatModel は「脅威」のモデルではなく、**影響が分離したコンポーネントの列挙とその依存関係の定義**である。

- **AttackSurface**: 独立して分析可能なコンポーネント単位。API endpoint, DB table, public API, IaC resource など任意の粒度で定義できる。`kind` フィールドは自由な文字列であり、コード側で固定の enum を持たない
- **ThreatModel**: リポジトリ内の AttackSurface 一覧。各 surface は影響範囲が限定されており、1つの surface の変更・脆弱性が他の surface に自動的に波及しない。これにより surface 単位で並列分析・キャッシュが可能になる

この設計により:
- セキュリティ分析: `kind: "endpoint"`, `kind: "db_table"` 等で attack surface を列挙
- コードレビュー: `kind: "module"`, `kind: "api_boundary"` 等で変更影響範囲を列挙
- テスト生成: `kind: "public_function"`, `kind: "integration_point"` 等でテスト対象を列挙

いずれのユースケースでも同じ ThreatModel 構造を使い回せる。

## Changes (実施済み)

### Phase 1: Dead code removal ✅

| Crate | Action |
|-------|--------|
| `parsentry-analyzer` | 削除 (genai 依存) |
| `parsentry-codex` | 削除 (OpenAI Codex 固有) |
| `parsentry-i18n` | 削除 (LLM レスポンス多言語化) |
| `parsentry-prompt` | 削除 (scan.rs に統合済み) |
| `parsentry-threat-model` | 削除 (core に移動) |

### Phase 2: Core generalization ✅

| Before | After |
|--------|-------|
| `parsentry-claude-code` | `parsentry-executor` (agent-agnostic) |
| `SurfaceKind` enum (固定6種) | `kind: String` (自由定義) |
| Config: model, min_confidence, vuln_types 等 | Config: paths, agent, cache, verbosity のみ |

### Phase 3: Branding ✅

- CLI about: "Parallel CLI agent execution platform with caching"
- CLAUDE.md: アーキテクチャ説明更新
- 全 crate description から "security scanner" 削除

## Consequences

### Positive
- セキュリティ以外のユースケース (コードレビュー、リファクタリング、テスト生成) にも適用可能
- `kind` が自由文字列になったことで、ユーザーが独自のコンポーネント分類を定義できる
- キャッシュにより CI/CD での実行コスト大幅削減
- Agent 非依存により、ベンダーロックイン回避

### Negative
- 既存のセキュリティ特化ユーザーにとって breaking change (SurfaceKind enum の削除)
- `parsentry-parser` (tree-sitter) がセキュリティ固有のまま残る → 将来プラグイン化

### Risks
- 汎化しすぎてセキュリティスキャンの品質が下がるリスク → セキュリティは first-class preset として維持
- kind が自由文字列になったことでバリデーションが弱くなるリスク → プロンプト側で推奨値を提示し、LLM 出力をそのまま受け入れる設計
