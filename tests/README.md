# Parsentry 精度テストスイート

このディレクトリには、Parsentryの検出精度を包括的に評価するためのテストスイートが含まれています。

## テストカテゴリ

### 1. PAR分類精度テスト (`par_classification_accuracy_test.rs`)

**目的**: Principal-Action-Resource分類の正確性を測定  
**重要度**: 🔥 最重要（全体精度の根幹）

- **高信頼度テスト**: 明確に分類できるケースで90%以上の精度を要求
- **境界ケーステスト**: 分類が困難なケースで70%以上の精度を要求  
- **非セキュリティ関数拒否テスト**: 偽陽性制御で85%以上の精度を要求
- **包括的精度テスト**: 全体で80%以上の精度を要求

```bash
# PAR分類テストの実行
cargo test par_classification_accuracy
```

### 2. コンテキスト品質精度テスト (`context_quality_accuracy_test.rs`)

**目的**: Tree-sitter解析とデータフロー追跡の品質を測定  
**重要度**: 🔥 重要（解析品質の基盤）

- **関数定義抽出精度**: 95%以上
- **参照追跡精度**: 85%以上  
- **データフロー追跡精度**: 75%以上
- **総合コンテキスト品質**: 85%以上

```bash
# コンテキスト品質テストの実行
cargo test context_quality_accuracy
```

### 3. 実世界ベンチマークテスト (`real_world_benchmark_test.rs`)

**目的**: 実際のCVE事例と脆弱性パターンでの検出精度を測定  
**重要度**: 🔥 最重要（実用性の証明）

- **クリティカル脆弱性**: 95%以上の検出精度
- **複数脆弱性検出**: F1スコア80%以上
- **包括的ベンチマーク**: 総合85%以上

```bash
# 実世界ベンチマークテストの実行
cargo test real_world_benchmark
```

### 4. 外部ベンチマーク統合テスト (`external_benchmark_integration_test.rs`)

**目的**: 業界標準ベンチマークでの性能評価  
**重要度**: 🔥 重要（業界比較）

- **Validation Benchmarks**: F1スコア0.8以上
- **高深刻度ベンチマーク**: F1スコア0.85以上
- **性能特性**: 平均実行時間10秒以下

```bash
# 外部ベンチマークテストの実行
cargo test external_benchmark_integration
```

### 5. エンドツーエンド精度テスト (`end_to_end_accuracy_test.rs`)

**目的**: パターンマッチング→コンテキスト構築→LLM解析の全パイプライン精度  
**重要度**: 🔥 統合（全体品質の確認）

- **単一ファイルE2E**: 90%以上の精度
- **マルチファイルE2E**: 85%以上の精度
- **パターンエッジケース**: LLM補完率80%以上

```bash
# E2Eテストの実行
cargo test end_to_end_accuracy
```

### 6. 統合テストスイート (`accuracy_test_suite.rs`)

**目的**: 全テストの統合管理とレポート生成  
**重要度**: 📊 管理（品質追跡）

- 重み付き総合スコア計算
- 詳細レポート生成
- 品質基準チェック

```bash
# 統合テストスイートの実行
cargo test accuracy_test_suite
```

## 実行方法

### 前提条件

```bash
# 必要な環境変数を設定
export OPENAI_API_KEY="your-api-key-here"

# または他のLLMプロバイダー
export ANTHROPIC_API_KEY="your-claude-key"
export GOOGLE_API_KEY="your-gemini-key"
```

### 全精度テストの実行

```bash
# 全ての精度テストを実行
cargo test --test accuracy_test_suite
cargo test --test par_classification_accuracy_test
cargo test --test context_quality_accuracy_test
cargo test --test real_world_benchmark_test
cargo test --test external_benchmark_integration_test  
cargo test --test end_to_end_accuracy_test

# または一括実行
cargo test accuracy
```

### 高速テスト（CI用）

```bash
# APIを使用しないテストのみ
cargo test --test par_classification_accuracy_test test_par_classification_structure
cargo test --test context_quality_accuracy_test test_definition_extraction_accuracy
cargo test --test accuracy_test_suite test_suite_health_check
```

### レポート生成

```bash
# テスト実行と詳細レポート生成
cargo test --test accuracy_test_suite test_report_generation
```

## 品質基準

### 最低品質基準

- **総合スコア**: 85%以上
- **各カテゴリ**: 80%以上  
- **クリティカル失敗**: なし

### 目標品質基準

- **総合スコア**: 90%以上
- **PAR分類**: 90%以上
- **実世界ベンチマーク**: 90%以上
- **コンテキスト品質**: 85%以上

## ベンチマークデータセット

### Validation Benchmarks (Xbow Engineering)

```bash
# 自動クローンされるが、手動でも可能
git clone https://github.com/xbow-engineering/validation-benchmarks.git benchmarks
```

- 104個の検証ベンチマーク
- XBEN-XXX-24 形式
- benchmark.json で設定

### OSSF CVE Benchmark (予定)

```bash
# 将来実装予定
git clone https://github.com/ossf-cve-benchmark/ossf-cve-benchmark.git
```

## 継続的品質改善

### 1. 毎日の実行

```bash
# CI/CDパイプラインに組み込み
cargo test accuracy -- --nocapture | tee accuracy_report.log
```

### 2. 品質追跡

- 精度スコアの時系列変化
- 失敗パターンの分析
- 改善箇所の特定

### 3. ベンチマーク更新

- 新しいCVE事例の追加
- エッジケースの拡充
- 新言語・フレームワーク対応

## トラブルシューティング

### API Key関連

```bash
# キーが設定されていない場合
export OPENAI_API_KEY="sk-..."

# 複数プロバイダーの設定
export ANTHROPIC_API_KEY="..." 
export GOOGLE_API_KEY="..."
```

### ベンチマーククローン失敗

```bash
# 手動クローン
git clone https://github.com/xbow-engineering/validation-benchmarks.git benchmarks

# アクセス権限の確認
ls -la benchmarks/
```

### 実行時間の短縮

```bash
# サンプルサイズを制限
ACCURACY_TEST_SAMPLE_SIZE=5 cargo test accuracy

# 高速モデルの使用
ACCURACY_TEST_MODEL="gpt-4.1-mini" cargo test accuracy
```

## テスト結果の解釈

### スコア指標

- **Precision (適合率)**: 検出された脆弱性のうち、実際に脆弱だった割合
- **Recall (再現率)**: 実際の脆弱性のうち、検出できた割合
- **F1 Score**: PrecisionとRecallの調和平均
- **Accuracy (正確度)**: 全体の正解率

### 品質レベル

- 🏆 **エクセレント** (95%+): 業界最高水準
- 🎯 **優秀** (90-95%): 本番利用可能
- ✅ **良好** (85-90%): 追加改善推奨
- ⚠️ **要改善** (80-85%): 大幅改善必要
- ❌ **不合格** (<80%): 本番利用不可

## 寄与とフィードバック

新しいテストケースや改善提案は Issues または Pull Requests で歓迎します。

### 新テストケースの追加

1. 該当カテゴリのテストファイルに追加
2. 期待値と根拠を明確に記載
3. エッジケースや実世界事例を優先

### バグ報告

- 再現手順の詳細記載
- 環境情報（OS、Rustバージョン等）
- ログやエラーメッセージの添付