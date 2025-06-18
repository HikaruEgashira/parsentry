use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use parsentry::analyzer::analyze_pattern;
use parsentry::args::{Args, validate_args};
use parsentry::file_classifier::FileClassifier;
use parsentry::language::Language;
use parsentry::locales;
use parsentry::pattern_generator::generate_custom_patterns;
use parsentry::sarif::SarifReport;
use parsentry::security_patterns::SecurityRiskPatterns;
use std::path::PathBuf;

use parsentry::repo::RepoOps;
use parsentry::repo_clone::clone_github_repo;
use parsentry::response::{AnalysisSummary, VulnType};

use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    println!(
        r#"
                ▲
               ╱ ╲
              ╱   ╲
             ╱ ░░░ ╲
            ╱ ░▓▓▓░ ╲
           ╱ ░▓███▓░ ╲
          ╱ ░▓█████▓░ ╲
         ╱_░▓███████▓░_╲
           ─────┬─────
                │
        P A R S E N T R Y
                │
             v{}
"#,
        env!("CARGO_PKG_VERSION")
    );

    let args = Args::parse();

    validate_args(&args)?;

    // Create language configuration
    let language = Language::from_string(&args.language);
    let messages = locales::get_messages(&language);

    // Get API base URL from CLI arg or environment variable
    let env_base_url = std::env::var("API_BASE_URL").ok();
    let api_base_url = args
        .api_base_url
        .as_deref()
        .or_else(|| env_base_url.as_deref());

    let (root_dir, repo_name) = if let Some(repo) = &args.repo {
        let dest = PathBuf::from("repo");
        if dest.exists() {
            std::fs::remove_dir_all(&dest).map_err(|e| {
                anyhow::anyhow!(
                    "{}: {}",
                    messages
                        .get("error_clone_failed")
                        .map_or("Failed to delete clone directory", |s| s),
                    e
                )
            })?;
        }
        println!(
            "🛠️  {}: {} → {}",
            messages
                .get("cloning_repo")
                .map_or("Cloning GitHub repository", |s| s),
            repo,
            dest.display()
        );
        clone_github_repo(repo, &dest).map_err(|e| {
            anyhow::anyhow!(
                "{}: {}",
                messages
                    .get("github_repo_clone_failed")
                    .map_or("Failed to clone GitHub repository", |s| s),
                e
            )
        })?;

        // Extract repository name for output directory
        let repo_name = if repo.contains('/') {
            repo.split('/')
                .last()
                .unwrap_or("unknown-repo")
                .replace(".git", "")
        } else {
            repo.replace(".git", "")
        };

        (dest, Some(repo_name))
    } else if let Some(root) = &args.root {
        (root.clone(), None)
    } else {
        panic!("root path or repo must be set");
    };

    // Handle pattern generation mode
    if args.generate_patterns {
        println!(
            "🔧 {}",
            messages
                .get("custom_pattern_generation_start")
                .unwrap_or(&"Starting custom pattern generation mode")
        );
        generate_custom_patterns(&root_dir, &args.model, api_base_url).await?;
        println!(
            "✅ {}",
            messages
                .get("pattern_generation_completed")
                .unwrap_or(&"Pattern generation completed")
        );
    }


    let repo = RepoOps::new(root_dir.clone());

    let files = repo.get_relevant_files();
    println!(
        "📁 {} ({}件)",
        messages
            .get("relevant_files_detected")
            .unwrap_or(&"Detected relevant source files"),
        files.len()
    );

    // Collect all pattern matches across all files
    let mut all_pattern_matches = Vec::new();
    
    for file_path in &files {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            // Skip files with more than 50,000 characters
            if content.len() > 50_000 {
                continue;
            }
            
            let filename = file_path.to_string_lossy();
            let lang = FileClassifier::classify(&filename, &content);

            let patterns = SecurityRiskPatterns::new_with_root(lang, Some(&root_dir));
            let matches = patterns.get_pattern_matches(&content);
            
            for pattern_match in matches {
                all_pattern_matches.push((file_path.clone(), pattern_match));
            }
        }
    }

    println!(
        "🔎 {} ({}件のパターンマッチ)",
        messages
            .get("security_pattern_files_detected")
            .unwrap_or(&"Detected security patterns"),
        all_pattern_matches.len()
    );
    
    // Group patterns by type for display
    let mut pattern_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, pattern_match) in &all_pattern_matches {
        *pattern_types.entry(pattern_match.pattern_config.description.clone()).or_insert(0) += 1;
    }
    
    for (pattern_desc, count) in pattern_types {
        println!("  [{}] {} matches", count, pattern_desc);
    }

    let total = all_pattern_matches.len();
    let root_dir = Arc::new(root_dir);

    // Create repository-specific output directory
    let output_dir = if let Some(base_output_dir) = args.output_dir.clone() {
        if let Some(ref name) = repo_name {
            let mut repo_output_dir = base_output_dir;
            repo_output_dir.push(name);
            Some(repo_output_dir)
        } else {
            Some(base_output_dir)
        }
    } else {
        None
    };

    let model = args.model.clone();
    let files = files.clone();
    let verbosity = args.verbosity;
    let debug = args.debug;

    let mut summary = AnalysisSummary::new();

    // プログレスバーを設定
    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    progress_bar.set_message("Analyzing files...");

    // 並列度を制御してタスクを実行 - パターンごとに分析
    let results = stream::iter(all_pattern_matches.iter().enumerate())
        .map(|(idx, (file_path, pattern_match))| {
            let file_path = file_path.clone();
            let pattern_match = pattern_match.clone();
            let _root_dir = Arc::clone(&root_dir);
            let output_dir = output_dir.clone();
            let model = model.clone();
            let files = files.clone();
            let progress_bar = progress_bar.clone();
            let debug = debug;
            let messages = messages.clone();
            let language = language.clone();

            async move {
                let file_name = file_path.display().to_string();
                let pattern_desc = &pattern_match.pattern_config.description;
                progress_bar.set_message(format!("Analyzing pattern '{}' in: {}", pattern_desc, file_name));
                if verbosity > 0 {
                    println!(
                        "📄 {}: {} - Pattern: {} ({} / {})",
                        messages
                            .get("analysis_target")
                            .unwrap_or(&"Analysis target"),
                        file_name,
                        pattern_desc,
                        idx + 1,
                        total
                    );
                    println!("{}", "=".repeat(80));
                }

                let analysis_result = match analyze_pattern(
                    &file_path,
                    &pattern_match,
                    &model,
                    &files,
                    verbosity,
                    0,
                    debug,
                    &output_dir,
                    api_base_url,
                    &language,
                )
                .await
                {
                    Ok(Some(res)) => res,
                    Ok(None) => {
                        progress_bar.inc(1);
                        return None;
                    }
                    Err(e) => {
                        if verbosity > 0 {
                            println!(
                                "❌ {}: {}: {}",
                                messages
                                    .get("analysis_failed")
                                    .unwrap_or(&"Analysis failed"),
                                file_path.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                };

                if analysis_result.vulnerability_types.is_empty()
                    || analysis_result.confidence_score < 1
                {
                    progress_bar.inc(1);
                    return None;
                }

                if let Some(ref output_dir) = output_dir {
                    if let Err(e) = std::fs::create_dir_all(output_dir) {
                        if verbosity > 0 {
                            println!(
                                "❌ {}: {}: {}",
                                messages
                                    .get("error_directory_creation")
                                    .map_or("Failed to create directory", |s| s),
                                output_dir.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                    let fname = generate_pattern_specific_filename(&file_path, &_root_dir, &pattern_match.pattern_config.description);
                    let mut out_path = output_dir.clone();
                    out_path.push(fname);
                    if let Err(e) = std::fs::write(&out_path, analysis_result.to_markdown()) {
                        if verbosity > 0 {
                            println!(
                                "❌ {}: {}: {}",
                                messages
                                    .get("markdown_report_output_failed")
                                    .map_or("Failed to output Markdown report", |s| s),
                                out_path.display(),
                                e
                            );
                        }
                        progress_bar.inc(1);
                        return None;
                    }
                    if verbosity > 0 {
                        println!(
                            "📝 {}: {}",
                            messages
                                .get("markdown_report_output")
                                .map_or("Output Markdown report", |s| s),
                            out_path.display()
                        );
                    }
                }

                if verbosity > 0 {
                    analysis_result.print_readable();
                }

                progress_bar.inc(1);
                Some((file_path, analysis_result))
            }
        })
        .buffer_unordered(50) // パターンベース分析での並列処理強化
        .collect::<Vec<_>>()
        .await;
    for result in results.into_iter() {
        if let Some((file_path, response)) = result {
            // Generate the same filename that was used for the actual file output
            let output_filename = if let Some(pattern_desc) = &response.pattern_description {
                generate_pattern_specific_filename(&file_path, &root_dir, pattern_desc)
            } else {
                generate_output_filename(&file_path, &root_dir)
            };
            
            summary.add_result(file_path, response, output_filename);
        }
    }

    progress_bar.finish_with_message(
        messages
            .get("analysis_completed")
            .map_or("Analysis completed!", |s| s),
    );

    summary.sort_by_confidence();

    let mut filtered_summary = if args.min_confidence > 0 {
        summary.filter_by_min_confidence(args.min_confidence)
    } else {
        summary
    };

    if let Some(types_str) = args.vuln_types {
        let vuln_types: Vec<VulnType> = types_str
            .split(',')
            .map(|s| match s.trim() {
                "LFI" => VulnType::LFI,
                "RCE" => VulnType::RCE,
                "SSRF" => VulnType::SSRF,
                "AFO" => VulnType::AFO,
                "SQLI" => VulnType::SQLI,
                "XSS" => VulnType::XSS,
                "IDOR" => VulnType::IDOR,
                other => VulnType::Other(other.to_string()),
            })
            .collect();

        filtered_summary = filtered_summary.filter_by_vuln_types(&vuln_types);
    }

    // Always generate summary report
    {
        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                println!(
                    "❌ {}: {}: {}",
                    messages
                        .get("error_directory_creation")
                        .map_or("Failed to create directory", |s| s),
                    final_output_dir.display(),
                    e
                );
            } else {
                if !filtered_summary.results.is_empty() {
                    let mut summary_path = final_output_dir.clone();
                    summary_path.push("summary.md");
                    if let Err(e) = std::fs::write(&summary_path, filtered_summary.to_markdown()) {
                        println!(
                            "❌ {}: {}: {}",
                            messages
                                .get("summary_report_output_failed")
                                .map_or("Failed to output summary report", |s| s),
                            summary_path.display(),
                            e
                        );
                    } else {
                        println!(
                            "📊 {}: {}",
                            messages
                                .get("summary_report_output")
                                .map_or("Output summary report", |s| s),
                            summary_path.display()
                        );
                    }
                }
            }
        } else {
            println!(
                "⚠ {}",
                messages
                    .get("summary_report_needs_output_dir")
                    .map_or("Summary report output requires --output-dir option", |s| s)
            );
        }
    }

    // Always generate SARIF report
    {
        let sarif_report = SarifReport::from_analysis_summary(&filtered_summary);

        if let Some(ref final_output_dir) = output_dir {
            if let Err(e) = std::fs::create_dir_all(final_output_dir) {
                println!(
                    "❌ {}: {}: {}",
                    messages
                        .get("error_directory_creation")
                        .map_or("Failed to create directory", |s| s),
                    final_output_dir.display(),
                    e
                );
            } else {
                let mut sarif_path = final_output_dir.clone();
                sarif_path.push("parsentry-results.sarif");
                if let Err(e) = sarif_report.save_to_file(&sarif_path) {
                    println!(
                        "❌ {}: {}: {}",
                        messages
                            .get("sarif_report_output_failed")
                            .map_or("Failed to output SARIF report", |s| s),
                        sarif_path.display(),
                        e
                    );
                } else {
                    println!(
                        "📋 {}: {}",
                        messages
                            .get("sarif_report_output")
                            .map_or("Output SARIF report", |s| s),
                        sarif_path.display()
                    );
                }
            }
        } else {
            // Output SARIF to stdout if no output directory specified
            match sarif_report.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => println!(
                    "❌ {}: {}",
                    messages
                        .get("sarif_output_failed")
                        .map_or("Failed to output SARIF", |s| s),
                    e
                ),
            }
        }
    }

    println!(
        "✅ {}",
        messages
            .get("analysis_completed")
            .map_or("Analysis completed", |s| s)
    );

    Ok(())
}

/// Generate a unique output filename based on the relative path from root directory
/// 
/// This function creates unique filenames by:
/// - Stripping the root directory prefix from the file path
/// - Replacing path separators with hyphens to maintain readability
/// - Removing dangerous path components like ".."
/// - Appending ".md" extension
/// 
/// # Arguments
/// * `file_path` - The full path to the source file
/// * `root_dir` - The root directory to strip from the path
/// 
/// # Returns
/// A unique filename string suitable for filesystem use
fn generate_output_filename(file_path: &std::path::Path, root_dir: &std::path::Path) -> String {
    
    // Strip the root directory prefix to get relative path
    let relative_path = match file_path.strip_prefix(root_dir) {
        Ok(rel_path) => rel_path,
        Err(_) => file_path, // Fallback to full path if strip fails
    };
    
    // Convert path to string and replace separators with hyphens
    let path_str = relative_path.to_string_lossy();
    
    // Replace path separators and clean up dangerous characters
    let cleaned = path_str
        .replace(std::path::MAIN_SEPARATOR, "-")
        .replace('/', "-")  // Handle both Unix and Windows separators
        .replace('\\', "-")
        .replace("..", "dotdot")  // Remove dangerous path traversal
        .replace(':', "_")  // Replace colon (problematic on Windows)
        .replace('*', "_")  // Replace wildcard characters
        .replace('?', "_")
        .replace('<', "_")
        .replace('>', "_")
        .replace('|', "_")
        .replace('"', "_");
    
    // Append .md extension
    format!("{}.md", cleaned)
}

fn generate_pattern_specific_filename(
    file_path: &std::path::Path, 
    root_dir: &std::path::Path, 
    pattern_description: &str
) -> String {
    // First get the base filename without .md extension
    let base_filename = generate_output_filename(file_path, root_dir);
    let base_without_md = base_filename.trim_end_matches(".md");
    
    // Create a safe pattern identifier from the description
    // First replace various characters with dashes, then filter and clean up
    let pattern_id = pattern_description
        .to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("/", "-")
        .replace("\\", "-")
        .replace("(", "-")
        .replace(")", "-")
        .replace("&", "-")
        .replace(".", "-")
        .replace(",", "-")
        .replace(":", "-")
        .replace(";", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        // Remove consecutive dashes and trim
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-");
    
    // Ensure pattern_id is not empty
    let pattern_id = if pattern_id.is_empty() {
        "pattern".to_string()
    } else {
        pattern_id
    };
    
    // Combine base filename with pattern identifier
    format!("{}-{}.md", base_without_md, pattern_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_generate_output_filename_uniqueness() {
        let root = Path::new("/project");
        
        // Different paths should generate different names
        let file1 = Path::new("/project/app/routes.py");
        let file2 = Path::new("/project/api/routes.py");
        let file3 = Path::new("/project/utils/routes.py");
        
        let name1 = generate_output_filename(file1, root);
        let name2 = generate_output_filename(file2, root);
        let name3 = generate_output_filename(file3, root);
        
        assert_ne!(name1, name2);
        assert_ne!(name1, name3);
        assert_ne!(name2, name3);
        
        assert_eq!(name1, "app-routes.py.md");
        assert_eq!(name2, "api-routes.py.md");
        assert_eq!(name3, "utils-routes.py.md");
    }
    
    #[test]
    fn test_generate_output_filename_readability() {
        let root = Path::new("/project");
        
        // Path structure should be preserved in readable form
        let file = Path::new("/project/src/components/auth/LoginForm.tsx");
        let result = generate_output_filename(file, root);
        
        assert_eq!(result, "src-components-auth-LoginForm.tsx.md");
        
        // Should maintain file extension information
        assert!(result.contains("LoginForm.tsx"));
        assert!(result.ends_with(".md"));
    }
    
    #[test]
    fn test_generate_output_filename_safety() {
        let root = Path::new("/project");
        
        // Dangerous path traversal should be handled
        let file = Path::new("/project/../admin/config.php");
        let result = generate_output_filename(file, root);
        
        // Should not contain ".." 
        assert!(!result.contains(".."));
        assert!(result.contains("dotdot"));
        
        // Test other dangerous characters
        let file2 = Path::new("/project/file:with*special?chars<>|.py");
        let result2 = generate_output_filename(file2, root);
        
        // Dangerous characters should be replaced with underscores
        assert!(!result2.contains(':'));
        assert!(!result2.contains('*'));
        assert!(!result2.contains('?'));
        assert!(!result2.contains('<'));
        assert!(!result2.contains('>'));
        assert!(!result2.contains('|'));
        assert!(result2.contains('_'));
    }
    
    #[test]
    fn test_generate_output_filename_consistency() {
        let root = Path::new("/project");
        let file = Path::new("/project/app/routes.py");
        
        // Same input should always produce same output
        let result1 = generate_output_filename(file, root);
        let result2 = generate_output_filename(file, root);
        let result3 = generate_output_filename(file, root);
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
        assert_eq!(result1, "app-routes.py.md");
    }
    
    #[test]
    fn test_generate_output_filename_edge_cases() {
        let root = Path::new("/project");
        
        // File directly in root
        let file1 = Path::new("/project/main.rs");
        let result1 = generate_output_filename(file1, root);
        assert_eq!(result1, "main.rs.md");
        
        // Deep nested path
        let file2 = Path::new("/project/very/deep/nested/path/file.js");
        let result2 = generate_output_filename(file2, root);
        assert_eq!(result2, "very-deep-nested-path-file.js.md");
        
        // No extension
        let file3 = Path::new("/project/src/Dockerfile");
        let result3 = generate_output_filename(file3, root);
        assert_eq!(result3, "src-Dockerfile.md");
    }
    
    #[test]
    fn test_generate_output_filename_fallback() {
        // Test when file path can't be stripped from root
        let root = Path::new("/different/root");
        let file = Path::new("/project/app/routes.py");
        
        let result = generate_output_filename(file, root);
        
        // Should still generate a valid filename using the full path
        assert!(result.ends_with(".md"));
        assert!(!result.is_empty());
    }
    
    #[test]
    fn test_filename_collision_resolution() {
        // This test demonstrates that the original problem is solved
        let root = Path::new("/repo");
        
        // These files would have caused collisions with the old implementation
        let file1 = Path::new("/repo/app/routes.py");
        let file2 = Path::new("/repo/api/routes.py");
        let file3 = Path::new("/repo/admin/routes.py");
        let file4 = Path::new("/repo/components/Button.tsx");
        let file5 = Path::new("/repo/pages/Button.tsx");
        
        let results = vec![
            generate_output_filename(file1, root),
            generate_output_filename(file2, root),
            generate_output_filename(file3, root),
            generate_output_filename(file4, root),
            generate_output_filename(file5, root),
        ];
        
        // Verify all results are unique (no collisions)
        for i in 0..results.len() {
            for j in (i + 1)..results.len() {
                assert_ne!(
                    results[i], results[j],
                    "Collision detected between {} and {}",
                    results[i], results[j]
                );
            }
        }
        
        // Verify expected format
        assert_eq!(results[0], "app-routes.py.md");
        assert_eq!(results[1], "api-routes.py.md");
        assert_eq!(results[2], "admin-routes.py.md");
        assert_eq!(results[3], "components-Button.tsx.md");
        assert_eq!(results[4], "pages-Button.tsx.md");
    }
    
    #[test] 
    fn test_old_vs_new_filename_generation() {
        // Demonstrate the difference between old and new implementations
        use std::path::Path;
        
        let root = Path::new("/repo");
        let files = vec![
            Path::new("/repo/app/routes.py"),
            Path::new("/repo/api/routes.py"),
            Path::new("/repo/utils/routes.py"),
        ];
        
        // Old implementation (what used to happen)
        let old_results: Vec<String> = files
            .iter()
            .map(|file_path| {
                file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string() + ".md")
                    .unwrap_or_else(|| "report.md".to_string())
            })
            .collect();
            
        // New implementation 
        let new_results: Vec<String> = files
            .iter()
            .map(|file_path| generate_output_filename(file_path, root))
            .collect();
        
        // Old implementation would create collisions (all same name)
        assert_eq!(old_results[0], "routes.py.md");
        assert_eq!(old_results[1], "routes.py.md");
        assert_eq!(old_results[2], "routes.py.md");
        // All three would overwrite each other!
        
        // New implementation creates unique names
        assert_eq!(new_results[0], "app-routes.py.md");
        assert_eq!(new_results[1], "api-routes.py.md");
        assert_eq!(new_results[2], "utils-routes.py.md");
        
        // Verify uniqueness
        assert_ne!(new_results[0], new_results[1]);
        assert_ne!(new_results[1], new_results[2]);
        assert_ne!(new_results[0], new_results[2]);
    }

    #[test]
    fn test_pattern_overwrite_issue() {
        // This test demonstrates the current issue where multiple patterns 
        // on the same file generate the same filename, causing overwrites
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        // Simulate multiple patterns analyzing the same file
        let filename1 = generate_output_filename(file_path, root);
        let filename2 = generate_output_filename(file_path, root);
        
        // Currently, both patterns generate the same filename
        // This causes the second analysis to overwrite the first
        assert_eq!(filename1, filename2); // This demonstrates the problem
        assert_eq!(filename1, "routes.py.md");
        
        // This is the bug we need to fix: same file + different patterns = same filename
        // The solution should make filenames unique per pattern
    }

    #[test]
    fn test_generate_pattern_specific_filename_basic() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        let filename1 = generate_pattern_specific_filename(file_path, root, "SQL Injection");
        let filename2 = generate_pattern_specific_filename(file_path, root, "XSS Vulnerability");
        
        assert_eq!(filename1, "routes.py-sql-injection.md");
        assert_eq!(filename2, "routes.py-xss-vulnerability.md");
        assert_ne!(filename1, filename2);
    }

    #[test]
    fn test_generate_pattern_specific_filename_special_chars() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/api/users.py");
        
        // Test pattern descriptions with special characters
        let filename1 = generate_pattern_specific_filename(file_path, root, "IDOR (Insecure Direct Object Reference)");
        let filename2 = generate_pattern_specific_filename(file_path, root, "Command_Injection & RCE");
        let filename3 = generate_pattern_specific_filename(file_path, root, "Path/Directory Traversal");
        
        assert_eq!(filename1, "api-users.py-idor-insecure-direct-object-reference.md");
        assert_eq!(filename2, "api-users.py-command-injection-rce.md");
        assert_eq!(filename3, "api-users.py-path-directory-traversal.md");
    }

    #[test]
    fn test_generate_pattern_specific_filename_empty_pattern() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/app.py");
        
        // Test with empty pattern description
        let filename1 = generate_pattern_specific_filename(file_path, root, "");
        let filename2 = generate_pattern_specific_filename(file_path, root, "   ");
        let filename3 = generate_pattern_specific_filename(file_path, root, "---");
        
        assert_eq!(filename1, "app.py-pattern.md");
        assert_eq!(filename2, "app.py-pattern.md");
        assert_eq!(filename3, "app.py-pattern.md");
    }

    #[test]
    fn test_generate_pattern_specific_filename_consistency() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/controllers/auth.py");
        
        // Same inputs should produce same outputs
        let filename1 = generate_pattern_specific_filename(file_path, root, "Authentication Bypass");
        let filename2 = generate_pattern_specific_filename(file_path, root, "Authentication Bypass");
        
        assert_eq!(filename1, filename2);
        assert_eq!(filename1, "controllers-auth.py-authentication-bypass.md");
    }

    #[test]
    fn test_pattern_specific_fixes_overwrite_issue() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        // Now with pattern-specific filenames, different patterns generate different filenames
        let filename1 = generate_pattern_specific_filename(file_path, root, "SQL Injection");
        let filename2 = generate_pattern_specific_filename(file_path, root, "XSS Vulnerability");
        let filename3 = generate_pattern_specific_filename(file_path, root, "CSRF Token Missing");
        
        // All filenames should be unique
        assert_ne!(filename1, filename2);
        assert_ne!(filename1, filename3);
        assert_ne!(filename2, filename3);
        
        // Verify expected format
        assert_eq!(filename1, "routes.py-sql-injection.md");
        assert_eq!(filename2, "routes.py-xss-vulnerability.md");
        assert_eq!(filename3, "routes.py-csrf-token-missing.md");
    }
}
