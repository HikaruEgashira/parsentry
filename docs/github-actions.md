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
    #   version: 'v0.15.0'  # default: latest
    #   model: 'gpt-5.1-codex'  # default: gpt-5.1-codex
```

## 入力パラメータ

| パラメータ | 必須 | デフォルト | 説明 |
|-----------|------|-----------|------|
| `version` | No | `latest` | 使用するParsentryのバージョン (e.g., `v0.15.0`) |
| `model` | No | `gpt-5.1-codex` | 分析に使用するモデル |

## 仕組み

1. GitHub Releaseからビルド済みバイナリをダウンロード
2. `git diff --diff-filter=ACMR` でPRの差分ファイルを取得
3. ソースコード拡張子のみをフィルタ（`.md` や `.txt` は除外）
4. `--diff-base origin/<base-branch>` で差分ファイルのみスキャン
5. パターンベースキャッシュ（`.parsentry/cache`）をCI間で共有

## キャッシュ

| キャッシュ | 対象 | キー |
|-----------|------|------|
| Parsentry | スキャン結果 | ベースブランチ + コミットSHA |

同じパターン（`pattern_type + matched_text`）が検出されれば、ファイルやリポジトリが異なってもキャッシュヒットします。

## ローカルでの差分スキャン

```bash
# 直近コミットの差分のみスキャン
parsentry . --diff-base HEAD~1

# mainブランチとの差分
parsentry . --diff-base origin/main

# 特定コミットとの差分
parsentry . --diff-base abc1234
```
