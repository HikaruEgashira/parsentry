# GitHub Actions Integration

ParsentryをGitHub ActionsでPRの差分スキャンに利用するガイド。

## セットアップ

Reusable workflowとして提供されています。呼び出し側のリポジトリに以下のワークフローを配置してください:

```yaml
# .github/workflows/security-scan.yml
name: Security Scan

on:
  pull_request:
    types: [opened, synchronize, reopened]

permissions:
  contents: read

jobs:
  scan:
    uses: HikaruEgashira/parsentry/.github/workflows/scan.yml@main
    # with:
    #   version: 'v0.20.0'  # default: latest
```

## 入力パラメータ

| パラメータ | 必須 | デフォルト | 説明 |
|-----------|------|-----------|------|
| `version` | No | `latest` | 使用するParsentryのバージョン (e.g., `v0.20.0`) |

## 仕組み

1. GitHub Releaseからビルド済みバイナリをダウンロード
2. `git diff --diff-filter=ACMR` でPRの差分ファイルを取得
3. ソースコード拡張子のみをフィルタ（`.md` や `.txt` は除外）
4. `PARSENTRY_CACHE_DIR=.parsentry/cache parsentry scan .` を実行
5. キャッシュをCI間で共有（surface 定義とソースのハッシュが一致する surface は再分析されないため、差分スキャンになる）

## キャッシュ

| キャッシュ | 対象 | キー |
|-----------|------|------|
| Parsentry | スキャン結果 | ベースブランチ + コミットSHA |

同じパターンが検出されれば、ファイルやリポジトリが異なってもキャッシュヒットします。

## ローカルでの差分スキャン

差分の指定は不要です。`parsentry scan` はキャッシュ済みの結果を surface 単位のハッシュで照合し、変更のあった surface のみを再分析します。

```bash
parsentry scan . | claude -p
```
