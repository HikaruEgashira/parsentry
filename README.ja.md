<div align="center">

  <img width="250" src="./logo.png" alt="Parsentry Logo">

**AIエージェント向けセキュリティスキャンオーケストレータ。**

Parsentryはリポジトリ構造を解析し、攻撃面を列挙して、攻撃面ごとの分析プロンプトを生成します。任意のCLIエージェントにパイプして並列セキュリティ分析を実行できます。

</div>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/HikaruEgashira/parsentry)

### 使い方

```bash
# Claude Code
parsentry model owner/repo | claude -p
parsentry scan owner/repo | claude -p

# Codex CLI
parsentry model owner/repo | codex -
parsentry scan owner/repo | codex -

# OpenCode
parsentry model owner/repo | opencode run --dir owner/repo
parsentry scan owner/repo | opencode run --dir owner/repo

# Crush
parsentry model owner/repo | crush run -c owner/repo
parsentry scan owner/repo | crush run -c owner/repo

# エージェントセッションのモニタリング
parsentry log owner/repo
```

これだけです。`model`は脅威モデルを生成し、`scan`は攻撃面ごとのプロンプトを生成してオーケストレータプロンプトをstdoutに出力します。サブエージェント対応のCLIエージェントにパイプすると、自動的に並列ワーカーをディスパッチします。Claude Code、Codex CLI、OpenCode、Crushで動作確認済みです。

### 仕組み

```
parsentry model   →  リポジトリメタデータ  →  脅威モデルプロンプト (stdout)
parsentry scan    →  脅威モデル + ソースコード  →  オーケストレータプロンプト (stdout)
  └─ オーケストレータが攻撃面ごとにサブエージェントをディスパッチ  →  SARIFファイル
parsentry log     →  スキャン進捗をリアルタイムでモニタリング
```

1. **Model** — リポジトリのメタデータ（言語、マニフェスト、エントリポイント）を収集し、脅威モデルプロンプトを生成します。エージェントにパイプして攻撃面を列挙させます。
2. **Scan** — 脅威モデルを読み込み、攻撃面ごとにソースコードを解析して、個別の`.prompt.md`ファイルを生成します。並列サブエージェントを起動するオーケストレータプロンプトを出力します。各サブエージェントは自身のプロンプトファイルを読み込み、SARIF形式で結果を書き出します。
3. **Log** — `docker compose logs -f`のようにスキャン進捗をストリーミング表示します。

### インストール

```bash
# mise経由
mise use -g github:HikaruEgashira/parsentry

# cargo経由
cargo install parsentry
```

### Claude Codeスキル

```bash
npx skills add HikaruEgashira/parsentry
```

インストール後、Claude Codeにリポジトリのスキャンを依頼するだけで、スキルが自動的にスキャンプロセスをディスパッチします。

### レポート例

| リポジトリ | 攻撃面数 |
|-----------|----------|
| [langgenius/dify](docs/reports/dify/) | 25 |
| [OWASP/NodeGoat](docs/reports/NodeGoat/) | 19 |
| [HikaruEgashira/hikae-vulnerable-python](docs/reports/hikae-vulnerable/) | 25 |

### ライセンス

AGPL 3.0
