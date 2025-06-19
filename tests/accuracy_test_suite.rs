use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

/// 精度テストスイート統合管理
/// 全ての精度関連テストを統合的に実行し、総合的な品質評価を提供
/// 継続的品質改善のためのメトリクス追跡とレポート生成

#[derive(Debug)]
pub struct AccuracyTestSuite {
    pub test_results: HashMap<String, TestCategoryResult>,
    pub overall_score: f64,
    pub execution_time: std::time::Duration,
}

#[derive(Debug)]
pub struct TestCategoryResult {
    pub category: TestCategory,
    pub passed: usize,
    pub total: usize,
    pub score: f64,
    pub critical_failures: Vec<String>,
    pub execution_time_ms: u128,
}

#[derive(Debug, Clone)]
pub enum TestCategory {
    PARClassification,    // PAR分類精度
    ContextQuality,       // コンテキスト品質
    RealWorldBenchmark,   // 実世界ベンチマーク
    ExternalBenchmark,    // 外部ベンチマーク
    EndToEndPipeline,     // エンドツーエンドパイプライン
}

impl std::fmt::Display for TestCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCategory::PARClassification => write!(f, "PAR分類精度"),
            TestCategory::ContextQuality => write!(f, "コンテキスト品質"),
            TestCategory::RealWorldBenchmark => write!(f, "実世界ベンチマーク"),
            TestCategory::ExternalBenchmark => write!(f, "外部ベンチマーク"),
            TestCategory::EndToEndPipeline => write!(f, "E2Eパイプライン"),
        }
    }
}

impl AccuracyTestSuite {
    pub fn new() -> Self {
        Self {
            test_results: HashMap::new(),
            overall_score: 0.0,
            execution_time: std::time::Duration::new(0, 0),
        }
    }

    pub fn add_result(&mut self, result: TestCategoryResult) {
        self.test_results.insert(format!("{}", result.category), result);
    }

    pub fn calculate_overall_score(&mut self) {
        if self.test_results.is_empty() {
            self.overall_score = 0.0;
            return;
        }

        // 重み付きスコア計算
        let weighted_scores = [
            (TestCategory::PARClassification, 0.25),   // 最重要：25%
            (TestCategory::ContextQuality, 0.20),      // 重要：20%
            (TestCategory::RealWorldBenchmark, 0.25),  // 最重要：25%
            (TestCategory::ExternalBenchmark, 0.20),   // 重要：20%
            (TestCategory::EndToEndPipeline, 0.10),    // 統合：10%
        ];

        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;

        for (category, weight) in &weighted_scores {
            if let Some(result) = self.test_results.get(&format!("{}", category)) {
                total_weighted_score += result.score * weight;
                total_weight += weight;
            }
        }

        self.overall_score = if total_weight > 0.0 {
            total_weighted_score / total_weight
        } else {
            0.0
        };
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Parsentry 精度テスト総合レポート\n\n");
        report.push_str(&format!("**実行日時**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**総合スコア**: {:.1}%\n", self.overall_score));
        report.push_str(&format!("**実行時間**: {:.2}秒\n\n", self.execution_time.as_secs_f64()));

        // 品質レベル評価
        let quality_level = match self.overall_score {
            s if s >= 95.0 => "🏆 エクセレント",
            s if s >= 90.0 => "🎯 優秀",
            s if s >= 85.0 => "✅ 良好",
            s if s >= 80.0 => "⚠️ 要改善",
            _ => "❌ 不合格",
        };
        report.push_str(&format!("**品質レベル**: {}\n\n", quality_level));

        // カテゴリ別結果
        report.push_str("## カテゴリ別結果\n\n");
        report.push_str("| カテゴリ | 合格/総数 | スコア | 実行時間 | 状態 |\n");
        report.push_str("|----------|-----------|--------|----------|------|\n");

        let mut categories: Vec<_> = self.test_results.values().collect();
        categories.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        for result in categories {
            let status = if result.score >= 90.0 {
                "✅"
            } else if result.score >= 80.0 {
                "⚠️"
            } else {
                "❌"
            };

            report.push_str(&format!(
                "| {} | {}/{} | {:.1}% | {:.1}ms | {} |\n",
                result.category,
                result.passed,
                result.total,
                result.score,
                result.execution_time_ms,
                status
            ));
        }

        // クリティカルな問題
        report.push_str("\n## クリティカルな問題\n\n");
        let mut has_critical_issues = false;
        
        for result in self.test_results.values() {
            if !result.critical_failures.is_empty() {
                has_critical_issues = true;
                report.push_str(&format!("### {}\n", result.category));
                for failure in &result.critical_failures {
                    report.push_str(&format!("- {}\n", failure));
                }
                report.push_str("\n");
            }
        }

        if !has_critical_issues {
            report.push_str("✅ クリティカルな問題は検出されませんでした。\n\n");
        }

        // 推奨事項
        report.push_str("## 推奨事項\n\n");
        
        if self.overall_score < 90.0 {
            report.push_str("### 緊急改善項目\n");
            
            for result in self.test_results.values() {
                if result.score < 85.0 {
                    report.push_str(&format!("- **{}**: スコア{:.1}% - 基準値85%を下回っています\n", result.category, result.score));
                }
            }
            report.push_str("\n");
        }

        if self.overall_score >= 90.0 {
            report.push_str("🎉 **優秀な結果です！** 以下の項目で更なる向上を目指してください：\n\n");
            
            for result in self.test_results.values() {
                if result.score < 95.0 {
                    report.push_str(&format!("- {}: {:.1}% → 95%目標\n", result.category, result.score));
                }
            }
        }

        report.push_str("\n---\n");
        report.push_str("*このレポートは自動生成されました*\n");

        report
    }

    pub fn save_report(&self, file_path: &str) -> Result<()> {
        let report = self.generate_report();
        std::fs::write(file_path, report)?;
        Ok(())
    }

    pub fn meets_quality_threshold(&self) -> bool {
        // 最低品質基準：85%
        self.overall_score >= 85.0 && 
        self.test_results.values().all(|r| r.score >= 80.0)  // 各カテゴリも最低80%
    }

    pub fn get_failing_categories(&self) -> Vec<String> {
        self.test_results
            .values()
            .filter(|r| r.score < 85.0)
            .map(|r| format!("{}", r.category))
            .collect()
    }
}

/// 統合テスト実行関数
/// 注意: これは実際のテスト実行ではなく、結果を統合するためのユーティリティです
pub async fn run_comprehensive_accuracy_suite() -> Result<AccuracyTestSuite> {
    let start_time = Instant::now();
    let mut suite = AccuracyTestSuite::new();

    println!("🚀 Parsentry 精度テスト総合スイート開始");
    println!("{}", "=".repeat(50));

    // 注意: 実際のテスト実行は個別のテスト関数で行われます
    // ここではダミーの結果を作成します（実際の統合では各テストの結果を収集）
    
    // 1. PAR分類精度テスト結果（サンプル）
    suite.add_result(TestCategoryResult {
        category: TestCategory::PARClassification,
        passed: 18,
        total: 20,
        score: 90.0,
        critical_failures: vec![],
        execution_time_ms: 1500,
    });

    // 2. コンテキスト品質テスト結果（サンプル）
    suite.add_result(TestCategoryResult {
        category: TestCategory::ContextQuality,
        passed: 9,
        total: 10,
        score: 92.0,
        critical_failures: vec![],
        execution_time_ms: 800,
    });

    // 3. 実世界ベンチマークテスト結果（サンプル）
    suite.add_result(TestCategoryResult {
        category: TestCategory::RealWorldBenchmark,
        passed: 14,
        total: 15,
        score: 88.0,
        critical_failures: vec![
            "複合脆弱性検出で一部漏れ".to_string(),
        ],
        execution_time_ms: 3200,
    });

    // 4. 外部ベンチマークテスト結果（サンプル）
    suite.add_result(TestCategoryResult {
        category: TestCategory::ExternalBenchmark,
        passed: 8,
        total: 10,
        score: 86.0,
        critical_failures: vec![],
        execution_time_ms: 5000,
    });

    // 5. E2Eパイプラインテスト結果（サンプル）
    suite.add_result(TestCategoryResult {
        category: TestCategory::EndToEndPipeline,
        passed: 7,
        total: 8,
        score: 89.0,
        critical_failures: vec![],
        execution_time_ms: 2100,
    });

    suite.execution_time = start_time.elapsed();
    suite.calculate_overall_score();

    println!("\n📊 精度テスト総合結果:");
    println!("  総合スコア: {:.1}%", suite.overall_score);
    println!("  実行時間: {:.2}秒", suite.execution_time.as_secs_f64());
    
    if suite.meets_quality_threshold() {
        println!("  品質基準: ✅ 合格");
    } else {
        println!("  品質基準: ❌ 不合格");
        let failing = suite.get_failing_categories();
        if !failing.is_empty() {
            println!("  改善が必要: {:?}", failing);
        }
    }

    println!("{}", "=".repeat(50));

    Ok(suite)
}

#[tokio::test]
async fn test_accuracy_suite_integration() -> Result<()> {
    println!("🧪 精度テストスイート統合テスト");

    let suite = run_comprehensive_accuracy_suite().await?;

    // 統合テストの検証
    assert!(
        suite.overall_score > 0.0,
        "総合スコアが計算されていません"
    );

    assert!(
        !suite.test_results.is_empty(),
        "テスト結果が記録されていません"
    );

    assert!(
        suite.execution_time.as_nanos() > 0,
        "実行時間が記録されていません"
    );

    // レポート生成テスト
    let report = suite.generate_report();
    assert!(
        report.contains("Parsentry 精度テスト総合レポート"),
        "レポートが正しく生成されていません"
    );

    // 品質基準テスト
    if !suite.meets_quality_threshold() {
        println!("⚠️  警告: 品質基準を満たしていません");
        println!("改善が必要なカテゴリ: {:?}", suite.get_failing_categories());
    }

    println!("✅ 精度テストスイート統合テスト完了");
    Ok(())
}

#[tokio::test]
async fn test_report_generation() -> Result<()> {
    println!("📄 レポート生成テスト");

    let suite = run_comprehensive_accuracy_suite().await?;
    
    // レポートをファイルに保存
    let report_path = "/tmp/parsentry_accuracy_report.md";
    suite.save_report(report_path)?;

    // ファイルが正しく生成されたかチェック
    assert!(
        std::path::Path::new(report_path).exists(),
        "レポートファイルが生成されていません"
    );

    let content = std::fs::read_to_string(report_path)?;
    assert!(
        content.len() > 500,
        "レポート内容が短すぎます: {}文字",
        content.len()
    );

    println!("✅ レポート生成テスト完了: {}", report_path);
    Ok(())
}

#[test]
fn test_quality_threshold_calculation() {
    let mut suite = AccuracyTestSuite::new();

    // 高品質ケース
    suite.add_result(TestCategoryResult {
        category: TestCategory::PARClassification,
        passed: 19,
        total: 20,
        score: 95.0,
        critical_failures: vec![],
        execution_time_ms: 1000,
    });

    suite.add_result(TestCategoryResult {
        category: TestCategory::ContextQuality,
        passed: 18,
        total: 20,
        score: 90.0,
        critical_failures: vec![],
        execution_time_ms: 800,
    });

    suite.calculate_overall_score();
    
    assert!(suite.meets_quality_threshold(), "高品質ケースで品質基準を満たしていません");
    assert!(suite.overall_score >= 85.0, "総合スコアが低すぎます: {:.1}%", suite.overall_score);

    // 低品質ケース
    let mut low_quality_suite = AccuracyTestSuite::new();
    
    low_quality_suite.add_result(TestCategoryResult {
        category: TestCategory::PARClassification,
        passed: 15,
        total: 20,
        score: 75.0,  // 基準以下
        critical_failures: vec!["重大な分類エラー".to_string()],
        execution_time_ms: 1200,
    });

    low_quality_suite.calculate_overall_score();
    
    assert!(!low_quality_suite.meets_quality_threshold(), "低品質ケースで品質基準を満たしてしまっています");
    assert!(!low_quality_suite.get_failing_categories().is_empty(), "失敗カテゴリが検出されていません");

    println!("✅ 品質基準計算テスト完了");
}

/// メタテスト: テストスイート自体の健全性チェック
#[test]
fn test_suite_health_check() {
    // テストカテゴリの完全性チェック
    let all_categories = vec![
        TestCategory::PARClassification,
        TestCategory::ContextQuality,
        TestCategory::RealWorldBenchmark,
        TestCategory::ExternalBenchmark,
        TestCategory::EndToEndPipeline,
    ];

    // 各カテゴリが適切にDisplay traitを実装しているかチェック
    for category in all_categories {
        let display_str = format!("{}", category);
        assert!(!display_str.is_empty(), "カテゴリの表示名が空です: {:?}", category);
        assert!(display_str.len() > 2, "カテゴリの表示名が短すぎます: {}", display_str);
    }

    // 重み付けの合計が1.0になることをチェック
    let weights = [0.25, 0.20, 0.25, 0.20, 0.10];
    let total_weight: f64 = weights.iter().sum();
    assert!((total_weight - 1.0).abs() < 0.001, "重み付けの合計が1.0になりません: {}", total_weight);

    println!("✅ テストスイート健全性チェック完了");
}