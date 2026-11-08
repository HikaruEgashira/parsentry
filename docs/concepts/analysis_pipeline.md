# 解析パイプライン

Parsentry は CLI エージェント向けのセキュリティプロンプトオーケストレータです。LLM を内部で呼び出さず、プロンプトを stdout に出力する方式を採用しています。外部の CLI エージェント（Claude Code、Codex CLI、OpenCode、Crush 等）にパイプして実行します。

## パイプラインの流れ

`parsentry scan` が唯一のコマンドです。実行のたびにキャッシュの状態を調べ、次のフェーズを 1 つだけ実行します。出力されるプロンプトの末尾には「`parsentry scan` を再実行し、`PARSENTRY_SCAN_COMPLETE` が出力されるまで繰り返す」という指示が含まれるため、1 回のパイプでパイプライン全体が完了します。

```
parsentry scan owner/repo | claude -p

  model フェーズ    model.json が無い・不正・入力 (RepoMetadata) が変化
      ↓ stdout: 脅威モデルプロンプト → エージェントが model.json を書き込み
  surface フェーズ  定義またはソースが変わった surface / SARIF が無い・不正な surface がある
      ↓ stdout: オーケストレータプロンプト → 並列サブエージェントが result.sarif.json を書き込み
  report フェーズ   全 surface 完了
      ↓ merged.sarif.json + report.md (+ -o 指定時は PDF)
      ↓ stdout: サマリ + PARSENTRY_SCAN_COMPLETE
```

## スキャンモード

モードはオプションではなくキャッシュの状態から決まります。

| 状態 | モード | 動作 |
|------|--------|------|
| キャッシュ無し | 全体スキャン | model → 全 surface → report |
| `model.key` が現在の RepoMetadata と一致し、一部の surface の `.cache_key` が不一致 | インクリメンタルスキャン | 変化した surface のみ再分析 |
| `model.key` が現在の RepoMetadata と不一致 | モデル差分スキャン | 旧 `model.json` と `merged.sarif.json` を `previous/` に退避し新モデルを生成。surface ID を引き継ぐよう指示するため、変化の無い surface の結果は再利用される。report で旧結果を baseline に差分 (new / absent / unchanged) を出力 |

## フェーズ 1: Model — 脅威モデル生成

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

## フェーズ 2: Surface — 攻撃面分析

### 処理内容

1. キャッシュから `model.json`（ThreatModel）を読み込む
2. 各 AttackSurface の `locations` からソースコードを収集
3. 未完了の surface に `SurfacePrompt`（prompt.md）を生成
4. オーケストレータプロンプトを stdout に出力

### キャッシュの仕組み

各 surface は `cache_key`（surface 定義とソースコードの SHA-256 ハッシュ）で管理されます。定義・内容ともに同じで、有効な SARIF がある surface は再分析をスキップします。

### 外部エージェントの役割

オーケストレータプロンプトを受け取ったエージェントは、未完了の surface ごとに並列サブエージェントをディスパッチします。各サブエージェントは `prompt.md` を読み込み、SARIF v2.1.0 形式で `result.sarif.json` に結果を書き込みます。

## フェーズ 3: Report — レポート生成

### 処理内容

1. 現行モデルに含まれる surface の `result.sarif.json` のみをマージ（`previous/merged.sarif.json` があれば baseline として比較）
2. `merged.sarif.json` と `report.md` を書き出し
3. `-o` 指定時は PDF をレンダリング

### オプション

| フラグ | 説明 |
|--------|------|
| `-o, --output <PATH>` | PDF の出力パス |

## キャッシュディレクトリ構造

```
~/Library/Caches/parsentry/<target>/
├── model.json                              — ThreatModel
├── model.key                               — model.json の入力ハッシュ
├── merged.sarif.json / report.md           — report フェーズの出力
├── previous/                               — モデル差分の比較元
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
