# 解析パイプライン

Parsentry は CLI エージェント向けのセキュリティプロンプトオーケストレータです。LLM を内部で呼び出さず、プロンプトを stdout に出力する方式を採用しています。外部の CLI エージェント（Claude Code、Codex CLI、OpenCode、Crush 等）にパイプして実行します。

## パイプラインの流れ

```
parsentry model [TARGET]
    ↓ stdout: 脅威モデルプロンプト
    ↓ パイプ → 外部エージェントが model.json をキャッシュに書き込み

parsentry scan [TARGET]
    ↓ model.json を読み込み
    ↓ 各 AttackSurface のソースコードを収集
    ↓ reports/<surface_id>/prompt.md を生成
    ↓ stdout: オーケストレータプロンプト
    ↓ パイプ → 外部エージェントが並列サブエージェントをディスパッチ
    ↓            各サブエージェントが result.sarif.json を書き込み

parsentry generate [TARGET]
    ↓ 全 result.sarif.json をマージ
    ↓ PDF レポートを生成
```

## フェーズ 1: Model — 脅威モデル生成

```
parsentry model owner/repo | claude -p
```

### 処理内容

`RepoMetadata` を収集し、脅威モデルプロンプトを stdout に出力します。

### RepoMetadata の構成

| フィールド | 内容 |
|-----------|------|
| `directory_tree` | リポジトリのディレクトリ構造 |
| `languages` | 検出された言語とファイル数 |
| `dependency_manifests` | 依存関係マニフェスト（package.json 等） |
| `entry_points` | エントリポイントファイル |
| `total_files` | 総ファイル数 |

### 外部エージェントの役割

プロンプトを受け取ったエージェントは `ThreatModel`（`AttackSurface` のリスト）を JSON でキャッシュに書き込みます。

### ThreatModel の構成

| フィールド | 内容 |
|-----------|------|
| `repository` | リポジトリ識別子 |
| `app_type` | アプリケーション種別（web_application, library 等） |
| `surfaces` | AttackSurface の配列 |

### AttackSurface の構成

| フィールド | 内容 |
|-----------|------|
| `id` | 一意識別子（SURFACE-001 等） |
| `kind` | 自由形式の種別（endpoint, db_table, public_api 等） |
| `identifier` | 人間可読な識別子 |
| `locations` | 関連ファイルパス |
| `description` | 分析が必要な理由 |

## フェーズ 2: Scan — 攻撃面分析

```
parsentry scan owner/repo | claude -p
```

### 処理内容

1. キャッシュから `model.json`（ThreatModel）を読み込む
2. 各 AttackSurface の `locations` からソースコードを収集
3. 各 surface に `SurfacePrompt`（prompt.md）を生成
4. オーケストレータプロンプトを stdout に出力

### オプション

| フラグ | 説明 |
|--------|------|
| `--diff-base <REF>` | 差分ベースの Git ref（変更ファイルのみを対象） |
| `--filter-lang <LANGS>` | 言語フィルタ（カンマ区切り） |

### キャッシュの仕組み

各 surface は `cache_key`（ソースコードの SHA-256 ハッシュ）で管理されます。同じ内容の surface は再分析をスキップします。

### 外部エージェントの役割

オーケストレータプロンプトを受け取ったエージェントは、未完了の surface ごとに並列サブエージェントをディスパッチします。各サブエージェントは `prompt.md` を読み込み、SARIF v2.1.0 形式で `result.sarif.json` に結果を書き込みます。

## フェーズ 3: Generate — レポート生成

```
parsentry generate owner/repo
```

### 処理内容

1. 全 surface の `result.sarif.json` をマージ
2. PDF レポートを生成（デフォルト出力先: キャッシュディレクトリ内）

### オプション

| フラグ | 説明 |
|--------|------|
| `-o, --output <PATH>` | PDF の出力パス |

## キャッシュディレクトリ構造

```
~/Library/Caches/parsentry/<target>/
├── model.json                              — ThreatModel
├── reports/
│   ├── SURFACE-001/
│   │   ├── prompt.md                       — 分析プロンプト
│   │   └── result.sarif.json               — SARIF v2.1.0 結果
│   └── SURFACE-002/
│       ├── prompt.md
│       └── result.sarif.json
```

キャッシュパスは `PARSENTRY_CACHE_DIR` 環境変数で上書き可能です。

## モニタリング

```
parsentry log owner/repo -f
```

スキャンの進捗をリアルタイムで表示します（`docker compose logs -f` と同様のインターフェース）。
