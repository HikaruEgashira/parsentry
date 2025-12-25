# PR Validation Skill - Reference Guide

Parsentry PR の検証スキルの詳細ガイドです。

## スキル概要

このスキルは Parsentry プロジェクトのプルリクエストに対して、以下の検証を自動実行します：

1. **ユニットテスト実行** - すべてのテストが成功することを確認
2. **コンパイルチェック** - 構文エラーがないことを確認
3. **Release ビルド** - 最適化されたバイナリが構築できることを確認
4. **動的セキュリティパターン検出** - 実際のコードスキャンで検出機能を検証

## 使用方法

### 基本的な検証（テスト + コンパイル + ビルド）

```bash
# PR #160 の検証
gh pr checkout 160
cargo test
cargo check
cargo build --release
```

または、提供されたスクリプトを使用：

```bash
.claude/skills/pr-validation/validate.sh 160
```

### 動的チェック付き検証

動的セキュリティスキャンを含める場合：

```bash
.claude/skills/pr-validation/validate.sh 160 --dynamic-check
```

この場合、以下も実行されます：
- Python/JavaScript/Java のサンプル脆弱性コード作成
- `cargo run --release` でのセキュリティスキャン
- SARIF レポート生成確認

## 検証項目の詳細

### 1. ユニットテスト

テスト実行コマンド：
```bash
cargo test
```

期待される結果：
```
test result: ok. 166 passed; 0 failed; 0 ignored
```

すべてのテストスイートが成功することを確認：
- `lib.rs` テスト (29 テスト)
- `security_patterns_unit_test.rs` (14 テスト)
- `vuln_patterns_test.rs` (3 テスト)
- `prompt_accuracy_unit_test.rs` (4 テスト)
- など計 166 テスト

### 2. コンパイルチェック

```bash
cargo check
```

期待される結果：
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs
```

warning は許容されますが、error があってはいけません。

### 3. Release ビルド

```bash
cargo build --release
```

期待される結果：
```
Finished `release` profile [optimized] target(s) in X.XXs
```

**注意**: 初回実行時はダウンロード・コンパイルで時間がかかります（40-50秒）。
2 回目以降は高速化（10-15秒）。

### 4. 動的セキュリティパターン検出

テストコードでスキャン実行：

```bash
cargo run --release -- --root /tmp/parsentry_test --output-dir /tmp/parsentry_results
```

期待される検出パターン：
- Path validation matches
- SQL execution resource matches
- DOM access operations matches
- File system operations matches
- Process building matches
- Code evaluation resource matches
- HTML injection resource matches
- Database result fetching matches

### 5. 出力レポート

SARIF フォーマットでレポート生成：

```
/tmp/parsentry_results/parsentry-results.sarif
```

検証内容：
```json
{
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "Parsentry",
          "version": "0.8.0"
        }
      },
      "invocation": {
        "executionSuccessful": true
      }
    }
  ]
}
```

## PR #160 の検証例

PR #160 はサブモジュールと build.rs を削除し、published tree-sitter crates に移行した PR です。

### 検証項目

1. **依存関係の正確性** ✅
   - 11 個の tree-sitter published crates を正常に解決
   - Cargo.lock で 105 行追加、4 行削除

2. **API 互換性** ✅
   - `parser.rs` の tree-sitter API が正常に機能
   - `security_patterns.rs` の API が正常に機能
   - 既存テストすべて成功

3. **ビルド簡素化** ✅
   - build.rs 削除（C コンパイル不要）
   - .gitmodules 削除（サブモジュール不要）
   - ビルドプロセスが簡潔に

4. **セキュリティスキャナー動作** ✅
   - すべての言語パーサー動作確認
   - パターン検出機能正常
   - SARIF レポート生成確認

## トラブルシューティング

### テストが失敗する場合

```bash
# verbose 出力で詳細を確認
cargo test -- --nocapture

# 特定のテストを実行
cargo test test_name
```

### ビルドが失敗する場合

```bash
# 依存関係を更新
cargo update

# clean してリビルド
cargo clean
cargo build --release
```

### スキャンが API エラーで失敗する場合

LLM API キーがない場合、パターン検出のみ実行されます。これは正常な動作です。

```bash
# API キー設定で自動分析も実行可能
export OPENAI_API_KEY="..."
cargo run --release -- --root /tmp/parsentry_test
```

## 関連ファイル

- `.claude/skills/pr-validation/SKILL.md` - スキル定義
- `.claude/skills/pr-validation/validate.sh` - 検証スクリプト
- `CLAUDE.md` - プロジェクト指示書
- `Cargo.toml` - プロジェクト設定

## 次のステップ

検証成功後：

```bash
# main ブランチに戻す
git checkout main

# PR をマージ（管理者権限が必要）
gh pr merge 160 --squash
```

または GitHub Web UI から直接マージできます。
