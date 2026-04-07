//! Per-surface prompt generation.
//!
//! Generates a focused prompt for each [`AttackSurface`] by reading actual
//! source code from the surface's locations, so that surfaces can be
//! independently dispatched to CLI agents and cached by content hash.

use std::path::{Path, PathBuf};

use parsentry_core::{AttackSurface, FileDiscovery, RepoMetadata, ThreatModel};
use sha2::{Digest, Sha256};

/// Maximum file size (in bytes) to include in a prompt.
const MAX_FILE_SIZE: u64 = 50 * 1024;

/// A prompt scoped to a single attack surface, ready for agent dispatch.
#[derive(Debug, Clone)]
pub struct SurfacePrompt {
    /// Surface identifier (mirrors [`AttackSurface::id`]).
    pub surface_id: String,
    /// The full prompt text for the agent.
    pub prompt: String,
    /// SHA-256 hex digest of resolved source contents, used as a cache key.
    pub cache_key: String,
    /// File path where the agent should write its output.
    pub output_path: PathBuf,
}

/// Resolved source file: relative path + contents.
struct SourceFile {
    rel_path: String,
    contents: String,
}

/// Resolve all readable source files for a surface's locations.
fn resolve_source_files(surface: &AttackSurface, root_dir: &Path) -> Vec<SourceFile> {
    let discovery = FileDiscovery::new(root_dir.to_path_buf());
    let mut sources: Vec<SourceFile> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for location in &surface.locations {
        let full_path = root_dir.join(location);

        if full_path.is_file() {
            // Single file
            if let Ok(meta) = std::fs::metadata(&full_path) {
                if meta.len() <= MAX_FILE_SIZE {
                    let rel = full_path
                        .strip_prefix(root_dir)
                        .unwrap_or(&full_path)
                        .to_string_lossy()
                        .to_string();
                    if seen.insert(rel.clone()) {
                        if let Ok(contents) = std::fs::read_to_string(&full_path) {
                            sources.push(SourceFile { rel_path: rel, contents });
                        }
                    }
                }
            }
        } else if full_path.is_dir() {
            // Directory — find all source files under it
            if let Ok(files) = discovery.get_files_in_path(&full_path) {
                for file_path in files {
                    if let Ok(meta) = std::fs::metadata(&file_path) {
                        if meta.len() > MAX_FILE_SIZE {
                            continue;
                        }
                    }
                    let rel = file_path
                        .strip_prefix(root_dir)
                        .unwrap_or(&file_path)
                        .to_string_lossy()
                        .to_string();
                    if seen.insert(rel.clone()) {
                        if let Ok(contents) = std::fs::read_to_string(&file_path) {
                            sources.push(SourceFile { rel_path: rel, contents });
                        }
                    }
                }
            }
        }
        // If the path doesn't exist, silently skip it.
    }

    sources
}

/// Generate a prompt for a single [`AttackSurface`].
///
/// Returns `None` when no readable source files overlap with the surface's
/// `locations`.
///
/// `output_dir` is the directory where the agent should write its SARIF output.
/// The output file will be named `{surface_id}.sarif.json`.
pub fn build_surface_prompt(
    surface: &AttackSurface,
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
    output_dir: &Path,
) -> Option<SurfacePrompt> {
    let sources = resolve_source_files(surface, root_dir);

    if sources.is_empty() {
        return None;
    }

    let repo_context = repo_metadata.to_prompt_context();
    let output_path = output_dir.join(format!("{}.sarif.json", surface.id));

    // Cache key: SHA-256 of concatenated (relative_path + "\0" + file_contents)
    let mut cache_input = String::new();
    for src in &sources {
        cache_input.push_str(&src.rel_path);
        cache_input.push('\0');
        cache_input.push_str(&src.contents);
        cache_input.push('\0');
    }
    let cache_key = hex_sha256(&cache_input);

    let mut prompt = String::new();

    prompt.push_str(
        "You are a security auditor. Analyze the following source code \
         for vulnerabilities.\n\n",
    );

    // Surface context
    prompt.push_str("## Surface Under Analysis\n\n");
    prompt.push_str(&format!("- **ID**: {}\n", surface.id));
    prompt.push_str(&format!("- **Kind**: {}\n", surface.kind));
    prompt.push_str(&format!("- **Identifier**: {}\n", surface.identifier));
    prompt.push_str(&format!("- **Description**: {}\n", surface.description));
    prompt.push_str(&format!(
        "- **Locations**: {}\n\n",
        surface.locations.join(", ")
    ));

    // Repository context
    prompt.push_str("## Repository Context\n\n");
    prompt.push_str(&repo_context);
    prompt.push_str("\n\n## Source Code\n\n");

    for src in &sources {
        // Infer language hint from extension
        let lang_hint = Path::new(&src.rel_path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        prompt.push_str(&format!(
            "### {}\n```{}\n{}\n```\n\n",
            src.rel_path, lang_hint, src.contents,
        ));
    }

    prompt.push_str("## Output Instructions\n\n");
    prompt.push_str("For each finding, provide:\n");
    prompt.push_str("- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)\n");
    prompt.push_str("- level: error/warning/note\n");
    prompt.push_str("- confidence: 0.0-1.0\n\n");
    prompt.push_str(&format!(
        "Write the complete SARIF v2.1.0 JSON output to the file: {}\n",
        output_path.display()
    ));
    prompt.push_str("Do NOT output the SARIF to stdout. Write it to the specified file only.\n");

    Some(SurfacePrompt {
        surface_id: surface.id.clone(),
        prompt,
        cache_key,
        output_path,
    })
}

/// Build prompts for every surface in a [`ThreatModel`].
///
/// Surfaces that have no readable source files are silently skipped.
pub fn build_all_surface_prompts(
    threat_model: &ThreatModel,
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
    output_dir: &Path,
) -> Vec<SurfacePrompt> {
    threat_model
        .surfaces
        .iter()
        .filter_map(|s| build_surface_prompt(s, repo_metadata, root_dir, output_dir))
        .collect()
}

fn hex_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    fn make_metadata(root: &Path) -> RepoMetadata {
        RepoMetadata {
            root_dir: root.to_path_buf(),
            directory_tree: "src/\n".to_string(),
            languages: HashMap::new(),
            dependency_manifests: vec![],
            entry_points: vec![],
            total_files: 1,
        }
    }

    fn make_surface(id: &str, locations: Vec<&str>) -> AttackSurface {
        AttackSurface {
            id: id.to_string(),
            kind: "endpoint".to_string(),
            identifier: format!("GET /api/{}", id),
            locations: locations.into_iter().map(String::from).collect(),
            description: "test surface".to_string(),
        }
    }

    #[test]
    fn returns_none_when_no_files_exist() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let surface = make_surface("S-1", vec!["src/nonexistent.py"]);
        let meta = make_metadata(root);

        assert!(build_surface_prompt(&surface, &meta, root, &out).is_none());
    }

    #[test]
    fn builds_prompt_with_file_contents() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("auth.py"), "password = input()\n").unwrap();

        let surface = make_surface("S-1", vec!["src/auth.py"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root, &out).unwrap();
        assert_eq!(sp.surface_id, "S-1");
        assert!(sp.prompt.contains("S-1"));
        assert!(sp.prompt.contains("password = input()"));
        assert!(sp.prompt.contains("src/auth.py"));
        assert!(sp.prompt.contains("Write the complete SARIF"));
        assert!(sp.prompt.contains("S-1.sarif.json"));
        assert_eq!(sp.output_path, out.join("S-1.sarif.json"));
        assert_eq!(sp.cache_key.len(), 64);
    }

    #[test]
    fn resolves_directory_locations() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "import os\nos.system(cmd)\n").unwrap();
        fs::write(src_dir.join("utils.py"), "def helper(): pass\n").unwrap();

        let surface = make_surface("S-1", vec!["src"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root, &out).unwrap();
        assert!(sp.prompt.contains("os.system(cmd)"));
        assert!(sp.prompt.contains("def helper(): pass"));
    }

    #[test]
    fn skips_large_files() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        // Create a file > 50KB
        let large_content = "x".repeat(60 * 1024);
        fs::write(src_dir.join("big.py"), &large_content).unwrap();

        let surface = make_surface("S-1", vec!["src/big.py"]);
        let meta = make_metadata(root);

        assert!(build_surface_prompt(&surface, &meta, root, &out).is_none());
    }

    #[test]
    fn cache_key_deterministic() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "os.system(cmd)\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let meta = make_metadata(root);

        let sp1 = build_surface_prompt(&surface, &meta, root, &out).unwrap();
        let sp2 = build_surface_prompt(&surface, &meta, root, &out).unwrap();
        assert_eq!(sp1.cache_key, sp2.cache_key);
    }

    #[test]
    fn cache_key_changes_with_content() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "version_1\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let meta = make_metadata(root);

        let sp1 = build_surface_prompt(&surface, &meta, root, &out).unwrap();

        fs::write(src_dir.join("app.py"), "version_2\n").unwrap();
        let sp2 = build_surface_prompt(&surface, &meta, root, &out).unwrap();

        assert_ne!(sp1.cache_key, sp2.cache_key);
    }

    #[test]
    fn deduplicates_overlapping_locations() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();

        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "eval(x)\n").unwrap();

        // Both locations resolve to the same file
        let surface = make_surface("S-1", vec!["src/app.py", "src/app.py"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root, &out).unwrap();
        let count = sp.prompt.matches("eval(x)").count();
        assert_eq!(count, 1);
    }
}
