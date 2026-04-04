use anyhow::Result;
use parsentry_core::Language;
use parsentry_utils::{FileClassifier, FileDiscovery};
use std::collections::HashMap;
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
        })
    }

    /// Render metadata as a compact string for LLM consumption.
    pub fn to_prompt_context(&self) -> String {
        let mut ctx = String::new();

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

    let mut entries: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();
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
        fs::write(root.join("src/main.py"), "import flask\napp = flask.Flask(__name__)").unwrap();
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
}
