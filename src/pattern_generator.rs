use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::file_classifier::FileClassifier;
use crate::parser::CodeParser;
use crate::repo::RepoOps;
use crate::security_patterns::Language;

#[derive(Default)]
struct LanguageStats {
    definition_count: usize,
    reference_count: usize,
    files: usize,
}

fn max_lines_allowed() -> usize {
    1_000
}

fn count_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|content| content.lines().count())
        .unwrap_or_default()
}

fn language_label(language: &Language) -> &'static str {
    match language {
        Language::Python => "Python",
        Language::JavaScript => "JavaScript",
        Language::Rust => "Rust",
        Language::TypeScript => "TypeScript",
        Language::Java => "Java",
        Language::Go => "Go",
        Language::Ruby => "Ruby",
        Language::C => "C",
        Language::Cpp => "C++",
        Language::Terraform => "Terraform",
        Language::CloudFormation => "CloudFormation",
        Language::Kubernetes => "Kubernetes",
        Language::Yaml => "YAML",
        Language::Bash => "Bash",
        Language::Shell => "Shell",
        Language::Php => "PHP",
        Language::Other => "Other",
    }
}

pub async fn generate_custom_patterns(
    root_dir: &Path,
    _model: &str,
    _api_base_url: Option<&str>,
) -> Result<()> {
    println!("⚙️  genaiクレート依存の削除に伴い、LLMベースのパターン生成は無効化されています。");
    println!(
        "🧭  代替として、コードベースのシグナルを集計し、手動でのパターン定義に役立つ統計を出力します。\n"
    );

    let repo = RepoOps::new(root_dir.to_path_buf());
    let files = repo.get_files_to_analyze(None)?;
    if files.is_empty() {
        println!("📭  対象ファイルが見つかりませんでした。");
        return Ok(());
    }

    let mut language_stats: HashMap<Language, LanguageStats> = HashMap::new();
    let mut detailed_examples: Vec<(Language, PathBuf, Vec<String>, Vec<String>)> = Vec::new();

    for file_path in files.iter() {
        let line_count = count_lines(file_path);
        if line_count > max_lines_allowed() {
            println!(
                "⚠️  {} は{}行を超えるためサマリー対象から除外しました",
                file_path.display(),
                max_lines_allowed()
            );
            continue;
        }

        let mut parser = CodeParser::new()?;
        if parser.add_file(file_path).is_err() {
            continue;
        }

        let context = match parser.build_context_from_file(file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if context.definitions.is_empty() && context.references.is_empty() {
            continue;
        }

        let filename = file_path.to_string_lossy();
        let content = fs::read_to_string(file_path).unwrap_or_default();
        let language = FileClassifier::classify(&filename, &content);

        let stats = language_stats.entry(language).or_default();
        stats.files += 1;
        stats.definition_count += context.definitions.len();
        stats.reference_count += context.references.len();

        let top_definitions = context
            .definitions
            .iter()
            .take(3)
            .map(|d| d.name.clone())
            .collect::<Vec<_>>();
        let top_references = context
            .references
            .iter()
            .take(3)
            .map(|r| r.name.clone())
            .collect::<Vec<_>>();

        detailed_examples.push((language, file_path.clone(), top_definitions, top_references));
    }

    if language_stats.is_empty() {
        println!("📄  解析可能な抽象構造を含むファイルは見つかりませんでした。");
        return Ok(());
    }

    println!("🗂️  言語別の抽出結果サマリー:");
    for (language, stats) in language_stats
        .iter()
        .sorted_by_key(|(lang, _)| language_label(lang))
    {
        println!(
            "  • {:<12} — 対象ファイル {:>3} 件 / 定義 {:>4} 件 / 参照 {:>4} 件",
            language_label(language),
            stats.files,
            stats.definition_count,
            stats.reference_count
        );
    }

    println!("\n🔍  サンプル抽出 (最大10件):");
    for (idx, (language, path, defs, refs)) in detailed_examples.iter().take(10).enumerate() {
        println!("{}.", idx + 1);
        println!("   言語: {}", language_label(language));
        println!("   ファイル: {}", path.display());
        if !defs.is_empty() {
            println!("   定義候補: {}", defs.join(", "));
        }
        if !refs.is_empty() {
            println!("   参照候補: {}", refs.join(", "));
        }
        println!("   → tree-sitterクエリを手動で作成する際の種にしてください\n");
    }

    println!(
        "📌  次のステップ: patterns ディレクトリにある既存YAMLを参考に、上記サマリーから必要なクエリを作成してください。"
    );

    Ok(())
}

trait IteratorSortExt: Iterator {
    fn sorted_by_key<K, F>(self, f: F) -> Vec<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K;
}

impl<I> IteratorSortExt for I
where
    I: Iterator,
{
    fn sorted_by_key<K, F>(self, mut f: F) -> Vec<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_by_key(|item| f(item));
        items
    }
}
