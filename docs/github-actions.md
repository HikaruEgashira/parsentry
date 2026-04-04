# GitHub Actions Integration

ParsentryをGitHub ActionsでPRの差分スキャンに利用するガイド。

## セットアップ

`.github/workflows/scan.yml` が同梱されています。PRが作成・更新されるたびに、変更されたソースファイルのみを対象にセキュリティスキャンを実行します。

## 仕組み

1. `git diff --diff-filter=ACMR` でPRの差分ファイルを取得
2. ソースコード拡張子のみをフィルタ（`.md` や `.txt` は除外）
3. `--diff-base origin/<base-branch>` で差分ファイルのみスキャン
4. パターンベースキャッシュ（`.parsentry/cache`）をCI間で共有

## キャッシュ

2層のキャッシュを利用:

| キャッシュ | 対象 | キー |
|-----------|------|------|
| Cargo | ビルド成果物 | `Cargo.lock` のハッシュ |
| Parsentry | スキャン結果 | ベースブランチ + コミットSHA |

同じパターン（`pattern_type + matched_text`）が検出されれば、ファイルやリポジトリが異なってもキャッシュヒットします。

## カスタマイズ

### モデル変更

リポジトリの Variables に `PARSENTRY_MODEL` を設定:

```
Settings → Secrets and variables → Actions → Variables → New repository variable
Name: PARSENTRY_MODEL
Value: gpt-5.1-codex
```

### ローカルでの差分スキャン

```bash
# 直近コミットの差分のみスキャン
parsentry . --diff-base HEAD~1

# mainブランチとの差分
parsentry . --diff-base origin/main

# 特定コミットとの差分
parsentry . --diff-base abc1234
```
