use crate::file_classifier::FileClassifier;
use crate::file_discovery::FileDiscovery;
use crate::language::Language;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Metadata collected from a repository for threat modeling.
#[derive(Debug, Clone)]
pub struct RepoMetadata {
    /// Root directory of the repository
    pub root_dir: PathBuf,
    /// Top-level directory tree (depth-limited)
    pub directory_tree: String,
    /// Detected languages and file counts
    pub languages: HashMap<Language, usize>,
    /// Contents of dependency manifests found
    pub dependency_manifests: Vec<ManifestInfo>,
    /// Entry point files detected
    pub entry_points: Vec<String>,
    /// Total number of source files
    pub total_files: usize,
    /// Source URL when collected from a web page (None for local repos)
    pub source_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ManifestInfo {
    pub path: String,
    pub content: String,
}

const MANIFEST_FILES: &[&str] = &[
    "requirements.txt",
    "Pipfile",
    "pyproject.toml",
    "setup.py",
    "package.json",
    "Cargo.toml",
    "go.mod",
    "pom.xml",
    "build.gradle",
    "Gemfile",
    "composer.json",
];

const ENTRY_POINT_PATTERNS: &[&str] = &[
    "main.py",
    "app.py",
    "wsgi.py",
    "asgi.py",
    "manage.py",
    "index.js",
    "index.ts",
    "server.js",
    "server.ts",
    "app.js",
    "app.ts",
    "main.rs",
    "main.go",
    "Main.java",
    "Application.java",
    "main.tf",
];

impl RepoMetadata {
    /// Collect metadata from the given repository root.
    pub fn collect(root_dir: &Path) -> Result<Self> {
        let discovery = FileDiscovery::new(root_dir.to_path_buf());
        let files = discovery.get_files()?;

        let mut languages: HashMap<Language, usize> = HashMap::new();
        for file_path in &files {
            let filename = file_path.to_string_lossy();
            let content = std::fs::read_to_string(file_path).unwrap_or_default();
            let lang = FileClassifier::classify(&filename, &content);
            if lang != Language::Other {
                *languages.entry(lang).or_insert(0) += 1;
            }
        }

        let directory_tree = build_directory_tree(root_dir, 3)?;

        let dependency_manifests = collect_manifests(root_dir)?;

        let entry_points = detect_entry_points(root_dir, &files);

        Ok(Self {
            root_dir: root_dir.to_path_buf(),
            directory_tree,
            languages,
            dependency_manifests,
            entry_points,
            total_files: files.len(),
            source_url: None,
        })
    }

    /// Filter metadata to only include the given set of files.
    /// Recalculates language counts and total_files.
    pub fn filter_to_files(&mut self, files: &HashSet<PathBuf>) {
        let mut languages: HashMap<Language, usize> = HashMap::new();
        for file_path in files {
            let filename = file_path.to_string_lossy();
            let content = std::fs::read_to_string(file_path).unwrap_or_default();
            let lang = FileClassifier::classify(&filename, &content);
            if lang != Language::Other {
                *languages.entry(lang).or_insert(0) += 1;
            }
        }
        self.languages = languages;
        self.total_files = files.len();
    }

    /// Render metadata as a compact string for LLM consumption.
    pub fn to_prompt_context(&self) -> String {
        let mut ctx = String::new();

        if let Some(ref url) = self.source_url {
            ctx.push_str(&format!("## Source URL\n{}\n\n", url));
            ctx.push_str("Assets collected from a live web page via HTTP.\n\n");
        }

        ctx.push_str("## Directory Structure\n```\n");
        ctx.push_str(&self.directory_tree);
        ctx.push_str("\n```\n\n");

        ctx.push_str("## Languages\n");
        let mut langs: Vec<_> = self.languages.iter().collect();
        langs.sort_by(|a, b| b.1.cmp(a.1));
        for (lang, count) in &langs {
            ctx.push_str(&format!("- {:?}: {} files\n", lang, count));
        }
        ctx.push('\n');

        if !self.dependency_manifests.is_empty() {
            ctx.push_str("## Dependencies\n");
            for manifest in &self.dependency_manifests {
                ctx.push_str(&format!("### {}\n```\n", manifest.path));
                // Truncate large manifests
                let content = if manifest.content.len() > 2000 {
                    &manifest.content[..2000]
                } else {
                    &manifest.content
                };
                ctx.push_str(content);
                ctx.push_str("\n```\n\n");
            }
        }

        if !self.entry_points.is_empty() {
            ctx.push_str("## Entry Points\n");
            for ep in &self.entry_points {
                ctx.push_str(&format!("- {}\n", ep));
            }
            ctx.push('\n');
        }

        ctx.push_str(&format!("Total source files: {}\n", self.total_files));

        ctx
    }
}

fn build_directory_tree(root: &Path, max_depth: usize) -> Result<String> {
    let mut tree = String::new();
    build_tree_recursive(root, root, &mut tree, 0, max_depth)?;
    Ok(tree)
}

fn build_tree_recursive(
    root: &Path,
    dir: &Path,
    out: &mut String,
    depth: usize,
    max_depth: usize,
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden dirs and common non-essential dirs
        if name.starts_with('.')
            || name == "node_modules"
            || name == "target"
            || name == "__pycache__"
            || name == "venv"
            || name == ".git"
            || name == "vendor"
            || name == "dist"
            || name == "build"
        {
            continue;
        }

        // Skip symlinks to prevent traversal outside repo
        if let Ok(ft) = entry.file_type() {
            if ft.is_symlink() {
                continue;
            }
        }

        let indent = "  ".repeat(depth);
        let path = entry.path();

        if path.is_dir() {
            out.push_str(&format!("{}{}/ \n", indent, name));
            build_tree_recursive(root, &path, out, depth + 1, max_depth)?;
        } else if depth <= 1 {
            // Only show files at top levels to keep it compact
            out.push_str(&format!("{}{}\n", indent, name));
        }
    }

    Ok(())
}

fn collect_manifests(root: &Path) -> Result<Vec<ManifestInfo>> {
    let mut manifests = Vec::new();

    for &name in MANIFEST_FILES {
        let path = root.join(name);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                manifests.push(ManifestInfo {
                    path: name.to_string(),
                    content,
                });
            }
        }
    }

    Ok(manifests)
}

fn detect_entry_points(root: &Path, files: &[PathBuf]) -> Vec<String> {
    let mut entry_points = Vec::new();

    for file in files {
        if let Some(filename) = file.file_name() {
            let name = filename.to_string_lossy();
            if ENTRY_POINT_PATTERNS.iter().any(|p| *p == name.as_ref()) {
                let rel = file.strip_prefix(root).unwrap_or(file);
                entry_points.push(rel.to_string_lossy().to_string());
            }
        }
    }

    entry_points
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_collect_basic() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("src/main.py"),
            "import flask\napp = flask.Flask(__name__)",
        )
        .unwrap();
        fs::write(root.join("requirements.txt"), "flask==3.0\nsqlalchemy==2.0").unwrap();

        let meta = RepoMetadata::collect(root).unwrap();

        assert_eq!(meta.total_files, 1);
        assert!(meta.languages.contains_key(&Language::Python));
        assert_eq!(meta.dependency_manifests.len(), 1);
        assert!(meta.dependency_manifests[0].content.contains("flask"));
    }

    #[test]
    fn test_prompt_context_format() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        fs::create_dir_all(root.join("api")).unwrap();
        fs::write(root.join("api/app.py"), "from flask import Flask").unwrap();
        fs::write(root.join("requirements.txt"), "flask").unwrap();

        let meta = RepoMetadata::collect(root).unwrap();
        let ctx = meta.to_prompt_context();

        assert!(ctx.contains("Directory Structure"));
        assert!(ctx.contains("Languages"));
        assert!(ctx.contains("Dependencies"));
    }

    #[test]
    fn test_build_directory_tree_skips_hidden_and_special_dirs() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        for dir in &[
            ".hidden_dir",
            "node_modules",
            "target",
            "__pycache__",
            "venv",
            "vendor",
            "dist",
            "build",
        ] {
            fs::create_dir_all(root.join(dir)).unwrap();
            fs::write(root.join(format!("{}/f.txt", dir)), "x").unwrap();
        }
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "fn main(){}").unwrap();
        let tree = build_directory_tree(root, 3).unwrap();
        assert!(!tree.contains(".hidden_dir"));
        assert!(!tree.contains("node_modules"));
        assert!(!tree.contains("__pycache__"));
        assert!(!tree.contains("venv"));
        assert!(!tree.contains("vendor"));
        assert!(tree.contains("src"));
    }

    #[test]
    fn test_build_directory_tree_depth_limiting() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("a/b/c/d")).unwrap();
        fs::write(root.join("a/b/c/d/deep.txt"), "deep").unwrap();
        let tree = build_directory_tree(root, 2).unwrap();
        assert!(tree.contains("a/"));
        assert!(tree.contains("b/"));
        assert!(tree.contains("c/"));
        assert!(!tree.contains("d/"), "depth > max_depth should be excluded");
    }

    #[test]
    fn test_build_directory_tree_files_only_at_top_levels() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::write(root.join("top.txt"), "top").unwrap();
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("sub/level1.txt"), "l1").unwrap();
        fs::create_dir_all(root.join("sub/deep")).unwrap();
        fs::write(root.join("sub/deep/level2.txt"), "l2").unwrap();
        let tree = build_directory_tree(root, 3).unwrap();
        assert!(tree.contains("top.txt"));
        assert!(tree.contains("level1.txt"));
        assert!(
            !tree.contains("level2.txt"),
            "depth 2 file should be hidden"
        );
    }

    #[test]
    fn test_detect_entry_points_matches_and_rejects() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main.py"), "").unwrap();
        fs::write(root.join("src/helper.py"), "").unwrap();
        fs::write(root.join("main.tf"), "").unwrap();
        let files = vec![
            root.join("src/main.py"),
            root.join("src/helper.py"),
            root.join("main.tf"),
        ];
        let eps = detect_entry_points(root, &files);
        assert!(eps.iter().any(|e| e.contains("main.py")));
        assert!(eps.iter().any(|e| e.contains("main.tf")));
        assert!(!eps.iter().any(|e| e.contains("helper.py")));
    }

    #[test]
    fn test_detect_entry_points_relative_paths() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        let files = vec![root.join("src/main.rs")];
        let eps = detect_entry_points(root, &files);
        assert_eq!(eps, vec!["src/main.rs"]);
    }

    #[test]
    fn test_prompt_context_manifest_truncation() {
        let long_content = "x".repeat(3000);
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp/test"),
            directory_tree: "src/\n".to_string(),
            languages: HashMap::new(),
            dependency_manifests: vec![ManifestInfo {
                path: "package.json".to_string(),
                content: long_content.clone(),
            }],
            entry_points: vec![],
            total_files: 1,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        assert!(ctx.contains("Dependencies"));
        assert!(!ctx.contains(&long_content));
        assert!(ctx.contains(&"x".repeat(2000)));
    }

    #[test]
    fn test_prompt_context_entry_points_section() {
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp/test"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![],
            entry_points: vec!["src/main.py".to_string()],
            total_files: 1,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        assert!(ctx.contains("## Entry Points"));
        assert!(ctx.contains("- src/main.py"));
    }

    #[test]
    fn test_prompt_context_no_entry_points_when_empty() {
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp/test"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![],
            entry_points: vec![],
            total_files: 0,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        assert!(!ctx.contains("## Entry Points"));
    }

    #[test]
    fn test_prompt_context_no_deps_when_empty() {
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp/test"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![],
            entry_points: vec![],
            total_files: 0,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        assert!(!ctx.contains("## Dependencies"));
    }

    #[test]
    fn test_prompt_context_total_files() {
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp/test"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![],
            entry_points: vec![],
            total_files: 42,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        assert!(ctx.contains("Total source files: 42"));
    }

    #[test]
    fn test_language_counting_increments() {
        // Kills += → *= mutant: multiple files of same language should add up
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.py"), "x = 1").unwrap();
        fs::write(root.join("src/b.py"), "y = 2").unwrap();
        fs::write(root.join("src/c.py"), "z = 3").unwrap();

        let meta = RepoMetadata::collect(root).unwrap();
        assert_eq!(*meta.languages.get(&Language::Python).unwrap(), 3);
    }

    #[test]
    fn test_manifest_truncation_boundary() {
        // Kills > → >= at 2000 char boundary
        // Use distinct chars so we can distinguish truncated vs full
        let mut manifest_2000 = "a".repeat(1999);
        manifest_2000.push('Z'); // last char is Z
        let meta = RepoMetadata {
            root_dir: PathBuf::from("/tmp"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![ManifestInfo {
                path: "req.txt".to_string(),
                content: manifest_2000.clone(),
            }],
            entry_points: vec![],
            total_files: 0,
            source_url: None,
        };
        let ctx = meta.to_prompt_context();
        // Exactly 2000 chars: with > 2000, NOT truncated (Z is present)
        // With >= 2000, would be truncated to [..2000] which is same, so this alone can't distinguish.
        // Instead: test len=2001 where truncation removes the last char
        assert!(ctx.contains(&manifest_2000));

        let mut manifest_2001 = "b".repeat(2000);
        manifest_2001.push('Q'); // char 2001 is Q
        let meta2 = RepoMetadata {
            root_dir: PathBuf::from("/tmp"),
            directory_tree: String::new(),
            languages: HashMap::new(),
            dependency_manifests: vec![ManifestInfo {
                path: "req.txt".to_string(),
                content: manifest_2001.clone(),
            }],
            entry_points: vec![],
            total_files: 0,
            source_url: None,
        };
        let ctx2 = meta2.to_prompt_context();
        // 2001 chars: truncated to [..2000], so Q is missing
        assert!(!ctx2.contains("Q"));
        // But the first 2000 chars should be present
        assert!(ctx2.contains(&"b".repeat(2000)));
    }

    #[test]
    fn test_filter_to_files_excludes_other_language() {
        // Kills != → == : Other files must NOT be counted
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.py"), "x = 1").unwrap();
        fs::write(root.join("README.md"), "# hello").unwrap();

        let mut meta = RepoMetadata::collect(root).unwrap();
        assert!(meta.languages.contains_key(&Language::Python));

        let py_only: HashSet<PathBuf> = [root.join("src/a.py")].into_iter().collect();
        meta.filter_to_files(&py_only);
        assert_eq!(*meta.languages.get(&Language::Python).unwrap(), 1);
        assert!(!meta.languages.contains_key(&Language::Other));
    }

    #[test]
    fn test_filter_to_files_updates_counts() {
        // Kills += → -= and += → *= : counts must accumulate correctly
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.py"), "x = 1").unwrap();
        fs::write(root.join("src/b.py"), "y = 2").unwrap();

        let mut meta = RepoMetadata::collect(root).unwrap();
        assert_eq!(*meta.languages.get(&Language::Python).unwrap(), 2);

        let one_file: HashSet<PathBuf> = [root.join("src/a.py")].into_iter().collect();
        meta.filter_to_files(&one_file);
        assert_eq!(*meta.languages.get(&Language::Python).unwrap(), 1);
        assert_eq!(meta.total_files, 1);
    }

    #[test]
    fn test_filter_to_files_actually_filters() {
        // Kills filter_to_files → () : must actually modify state
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.py"), "x = 1").unwrap();
        fs::write(root.join("src/b.py"), "y = 2").unwrap();

        let mut meta = RepoMetadata::collect(root).unwrap();
        assert_eq!(meta.total_files, 2);

        let one: HashSet<PathBuf> = [root.join("src/a.py")].into_iter().collect();
        meta.filter_to_files(&one);
        assert_eq!(meta.total_files, 1);
    }

    #[test]
    fn test_build_tree_skips_dist_and_build() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("dist")).unwrap();
        fs::create_dir_all(root.join("build")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("dist/bundle.js"), "").unwrap();
        fs::write(root.join("build/output.js"), "").unwrap();
        fs::write(root.join("src/main.py"), "").unwrap();

        let tree = build_directory_tree(root, 3).unwrap();
        assert!(!tree.contains("dist"), "dist should be skipped");
        assert!(!tree.contains("build"), "build should be skipped");
        assert!(tree.contains("src"));
    }
}
