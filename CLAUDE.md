## Project Overview

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis.

## After Code Changes

コードの変更後、現時点でのコードが正常か動作確認を行なってください。

```bash
cargo test
cargo check

# Run with verbose output
cargo test -- --nocapture
```

make sure all tests pass.

## After Task Completion

タスクの完了後、commitやPull Requestを作成する前に、以下のコマンドを実行して動作確認を行ってください。

```bash
# Run tests with snapshot testing
cargo test --features snapshot-test

# Run benchmarks
cargo test --features benchmark
```

現時点でbencmarksの精度によるエラーは許容されますが、任されたタスクの範囲で改善が可能であれば修正してください。コンパイルエラーは許容されません。

## Benchmark guide

性能評価はベンチマーク用のサンプルアプリケーションに対して行なってください。

```bash
# Clone validation benchmarks (104 total benchmarks)
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks

# Run individual benchmark validation
cargo run -- --root benchmarks/XBEN-001-24 --output-dir docs/benchmark/results/XBEN-001-24 --generate-patterns

# Validate specific vulnerability types
cargo run -- --root benchmarks/XBEN-010-24 --output-dir docs/benchmark/results/XBEN-010-24 --generate-patterns
cargo run -- --root benchmarks/XBEN-020-24 --output-dir docs/benchmark/results/XBEN-020-24 --generate-patterns
```

出力結果 `docs/benchmark/results/XBEN-XXX-24/*.md` と 答え`benchmarks/XBEN-XXX-24/benchmark.json` の内容を見比べて精度を確認してください。

## Behavior guide

top levelのディレクトリに新たなファイルやディレクトリを作成しないでください、gitのバージョン管理を利用してください
ユーザーとは必ずsayコマンドを使って結果を要約して応答してください
