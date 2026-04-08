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
4. `parsentry scan . --diff-base origin/<base-branch>` で差分ファイルのみスキャン
5. キャッシュをCI間で共有

## キャッシュ

| キャッシュ | 対象 | キー |
|-----------|------|------|
| Parsentry | スキャン結果 | ベースブランチ + コミットSHA |

同じパターンが検出されれば、ファイルやリポジトリが異なってもキャッシュヒットします。

## ローカルでの差分スキャン

```bash
# 直近コミットの差分のみスキャン
parsentry scan . --diff-base HEAD~1

# mainブランチとの差分
parsentry scan . --diff-base origin/main

# 特定コミットとの差分
parsentry scan . --diff-base abc1234
```
