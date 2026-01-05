## Project Overview

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis.

- Principals: 実行主体/Source
- Actions: セキュリティに関連する制御/Validate
- Resources: 対象となるリソース/Sink

## After Code Changes

コードの変更後、現時点でのコードが正常か動作確認を行ないます

```bash
cargo test && cargo check

# Run with verbose output
cargo test -- --nocapture
```

## Dynamic Check Guide

独自のやられアプリがあります。キャッシュが効くため基本はこちらを使い、動作確認を行います

```bash
cargo run -- HikaruEgashira/hikae-vulnerable-python  ---output-dir docs/reports/hikae-vulnerable
```

## Benchmark guide

性能評価はベンチマーク用のサンプルアプリケーションに対して行なってください。
負荷が高いため乱発して利用はできません

```bash
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks

cargo run -- benchmarks/XBEN-001-24 --output-dir docs/benchmark/results/XBEN-001-24 --generate-patterns
```

## Behavior guide

ユーザーとはsayコマンドを使って結果を要約して応答してください
