---
name: pr-validation
description: Validate PR changes by running tests, compilation checks, and dynamic security pattern detection on sample code. Use when reviewing PR #160, verifying build integrity, or confirming security pattern detection functionality.
allowed-tools: Bash, Read, Write, Glob
---

# PR Validation Skill

このスキルは Parsentry PR の検証を自動化します。テスト、ビルド、動的スキャンを実施します。

## 検証対象

### 1. ユニットテスト実行
```bash
cargo test
```
- すべてのテストが成功することを確認
- エラーログがないことを確認

### 2. コンパイルチェック
```bash
cargo check
```
- コンパイルエラーがないことを確認
- 警告を確認

### 3. ビルド検証（Release）
```bash
cargo build --release
```
- リリースビルドが成功することを確認
- ビルド時間を記録

### 4. 動的セキュリティパターン検出（Dynamic Check）
テストコードを作成してスキャン実行：

#### Python SQL Injection サンプル
```python
# SQL インジェクション脆弱性を含むコード
def get_user(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
```

#### JavaScript XSS サンプル
```javascript
// XSS 脆弱性
comment.innerHTML = userInput;
```

#### Java Command Injection サンプル
```java
// コマンドインジェクション脆弱性
Runtime.getRuntime().exec("ls " + userInput);
```

スキャン実行：
```bash
cargo run --release -- --root /tmp/parsentry_test --output-dir /tmp/parsentry_results
```

### 5. 出力結果の検証
- SARIF レポート生成確認
- 検出パターン数の確認
- エラーログがないことを確認

## 検証チェックリスト

- [ ] すべてのユニットテスト成功（0 failures）
- [ ] コンパイルエラーなし
- [ ] `cargo build --release` 成功
- [ ] サンプルコードスキャン実行成功
- [ ] セキュリティパターン検出確認（複数パターン）
- [ ] SARIF レポート生成確認

## PR 検証コマンド

PR #160 の検証：
```bash
gh pr checkout 160
cargo test
cargo check
cargo build --release
```

動的チェック実行：
```bash
mkdir -p /tmp/parsentry_test/{python,js,java}
# サンプルコードを各ディレクトリに配置
cargo run --release -- --root /tmp/parsentry_test --output-dir /tmp/parsentry_results
cat /tmp/parsentry_results/parsentry-results.sarif
```

## 結果サマリー

検証完了後、以下を報告：

```
✅ 動作確認完了

**テスト結果:**
- ユニットテスト: X passed; 0 failed
- コンパイル: OK
- ビルド時間: Xs

**動的チェック:**
- 検出ファイル数: X個
- 検出パターン数: X個
- SARIF レポート: 生成確認

**結論:** PR マージ準備完了
```

## 使用例

- PR 作成後の自動検証
- リリース前の品質確認
- セキュリティスキャナー動作確認
- 依存関係更新後の検証
- 新言語パーサー追加時の検証
