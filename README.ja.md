<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**AIエージェント向けセキュリティスキャンオーケストレータ。**

Parsentryはリポジトリ構造を解析し、攻撃面を列挙して、攻撃面ごとの分析プロンプトを生成します。任意のCLIエージェントにパイプして並列セキュリティ分析を実行できます。

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### [Agent Skill](https://agentskills.io/)

```bash
npx skills add HikaruEgashira/parsentry
```

インストール後、エージェントにリポジトリのスキャンを依頼するだけで、スキルが自動的にスキャンプロセスをディスパッチします。

### 使い方

```bash
parsentry scan owner/repo | claude -p
```

必要なコマンドは`scan`のみです。キャッシュの状態を調べ、次のフェーズ（脅威モデル → 攻撃面分析 → レポート）のプロンプトを出力します。各プロンプトは`PARSENTRY_SCAN_COMPLETE`が出力されるまで`parsentry scan`を再実行するようエージェントに指示するため、1回のパイプでパイプライン全体が完了します。`-o report.pdf`を付けるとPDFも生成します。

スキャンモードはフラグではなく状態から決定されます。

| 状態 | モード |
|---|---|
| 脅威モデルがない | **全体スキャン** |
| 脅威モデルは最新で、一部のソースが変更された | **インクリメンタルスキャン** — 影響する攻撃面のみ再分析 |
| モデル生成後にリポジトリ構造が変化した | **モデル差分スキャン** — 旧モデルとその検出結果を保持したまま新モデルを生成し、両モデルの検出結果の差分をレポートに出力 |

他のCLIエージェントでの使い方は[HACKING.md](HACKING.md)を参照してください。

### 仕組み

```
parsentry scan  ─┬─ model フェーズ    →  脅威モデルプロンプト                     →  model.json
                 ├─ surface フェーズ  →  オーケストレータ + 攻撃面別プロンプト  →  SARIFファイル
                 └─ report フェーズ   →  merged.sarif.json + report.md (+ PDF)
parsentry log   →  スキャン進捗をリアルタイムでモニタリング
```

1. **Model** — リポジトリのメタデータ（言語、マニフェスト、エントリポイント）を収集し、脅威モデルプロンプトを生成します。エージェントが攻撃面を`model.json`に列挙します。
2. **Surfaces** — 攻撃面ごとに`prompt.md`とオーケストレータプロンプトを生成します。各サブエージェントがSARIF形式で結果を書き出します。定義とソースが変わっていない攻撃面はスキップされます。
3. **Report** — 現在のモデルに含まれる攻撃面のSARIFを統合し、`report.md`を出力します（旧モデルがある場合はモデル差分セクションを含む）。
4. **Log** — `docker compose logs -f`のようにスキャン進捗をストリーミング表示します。

### インストール

```bash
# mise経由
mise use -g github:HikaruEgashira/parsentry

# cargo経由
cargo install parsentry
```

### レポート例

| リポジトリ | 攻撃面数 |
|-----------|----------|
| [langgenius/dify](docs/reports/dify/) | 25 |
| [OWASP/NodeGoat](docs/reports/NodeGoat/) | 19 |
| [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/) | 25 |

### ライセンス

AGPL 3.0
