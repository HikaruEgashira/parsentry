//! Per-surface prompt generation.
//!
//! Generates a focused prompt for each [`AttackSurface`] by reading actual
//! source code from the surface's locations, so that surfaces can be
//! independently dispatched to CLI agents and cached by content hash.

use std::path::Path;

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
pub fn build_surface_prompt(
    surface: &AttackSurface,
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
) -> Option<SurfacePrompt> {
    let sources = resolve_source_files(surface, root_dir);

    if sources.is_empty() {
        return None;
    }

    let repo_context = repo_metadata.to_prompt_context();

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
    prompt.push_str("Output valid SARIF v2.1.0 JSON.\n");
    prompt.push_str("For each finding, provide:\n");
    prompt.push_str("- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)\n");
    prompt.push_str("- level: error/warning/note\n");
    prompt.push_str("- confidence: 0.0-1.0\n");

    Some(SurfacePrompt {
        surface_id: surface.id.clone(),
        prompt,
        cache_key,
    })
}

/// Build prompts for every surface in a [`ThreatModel`].
///
/// Surfaces that have no readable source files are silently skipped.
pub fn build_all_surface_prompts(
    threat_model: &ThreatModel,
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
) -> Vec<SurfacePrompt> {
    threat_model
        .surfaces
        .iter()
        .filter_map(|s| build_surface_prompt(s, repo_metadata, root_dir))
        .collect()
}

/// Build an orchestrator prompt that dispatches all surface analyses
/// as parallel subagents within a single Claude process.
///
/// The orchestrator reads individual `.prompt.md` files and launches
/// Agent tool calls with `run_in_background: true` for each surface.
pub fn build_orchestrator_prompt(
    surface_prompts: &[SurfacePrompt],
    output_dir: &Path,
) -> String {
    let abs_output_dir = std::fs::canonicalize(output_dir)
        .unwrap_or_else(|_| output_dir.to_path_buf());

    let mut prompt = String::new();

    prompt.push_str(
        "You are a security analysis orchestrator. Your task is to analyze multiple \
         attack surfaces in parallel by dispatching subagents.\n\n",
    );

    prompt.push_str("## Instructions\n\n");
    prompt.push_str(
        "1. Read each prompt file listed below using the Read tool\n\
         2. Launch ALL subagents in a SINGLE message using the Agent tool for maximum parallelism\n\
         3. Each agent must run with `run_in_background: true`\n\
         4. Each agent's prompt must include the content from the prompt file AND \
            the instruction to write SARIF output to the specified path\n\
         5. After all agents complete, provide a summary of total findings\n\n",
    );

    prompt.push_str("## Surfaces to Analyze\n\n");
    prompt.push_str("| Surface ID | Prompt File | SARIF Output |\n");
    prompt.push_str("|------------|-------------|-------------|\n");

    for sp in surface_prompts {
        let prompt_path = abs_output_dir.join(format!("{}.prompt.md", sp.surface_id));
        let sarif_path = abs_output_dir.join(format!("{}.sarif.json", sp.surface_id));
        prompt.push_str(&format!(
            "| {} | {} | {} |\n",
            sp.surface_id,
            prompt_path.display(),
            sarif_path.display(),
        ));
    }

    prompt.push_str("\n## Agent Launch Template\n\n");
    prompt.push_str(
        "For each surface, use the Agent tool like this:\n\n\
         ```\n\
         Agent(\n\
           description: \"Analyze {SURFACE_ID}\",\n\
           prompt: \"<content from prompt file>\\n\\n\
             Write the SARIF JSON output to: {SARIF_OUTPUT_PATH}\\n\
             Write ONLY valid JSON. No markdown, no code fences, no explanation.\",\n\
           run_in_background: true,\n\
           mode: \"bypassPermissions\"\n\
         )\n\
         ```\n\n",
    );

    prompt.push_str(
        "IMPORTANT: Launch ALL agents in a single message. Do NOT wait for one to finish \
         before launching the next.\n\n",
    );

    // Merge step
    let merged_sarif = abs_output_dir.join("merged.sarif.json");
    prompt.push_str("## Post-Analysis: Merge Results\n\n");
    prompt.push_str(
        "After ALL subagents have completed, run the following command to merge \
         the per-surface SARIF files into a single report:\n\n",
    );
    prompt.push_str(&format!(
        "```bash\nparsentry merge {} -o {}\n```\n\n",
        abs_output_dir.display(),
        merged_sarif.display(),
    ));
    prompt.push_str(
        "Then report the total number of findings by severity (error/warning/note) \
         from the merged SARIF output.\n",
    );

    prompt
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
        let surface = make_surface("S-1", vec!["src/nonexistent.py"]);
        let meta = make_metadata(root);
        assert!(build_surface_prompt(&surface, &meta, root).is_none());
    }

    #[test]
    fn builds_prompt_with_file_contents() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("auth.py"), "password = input()\n").unwrap();

        let surface = make_surface("S-1", vec!["src/auth.py"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root).unwrap();
        assert_eq!(sp.surface_id, "S-1");
        assert!(sp.prompt.contains("password = input()"));
        assert!(sp.prompt.contains("src/auth.py"));
        assert!(sp.prompt.contains("SARIF"));
        assert_eq!(sp.cache_key.len(), 64);
    }

    #[test]
    fn resolves_directory_locations() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "import os\nos.system(cmd)\n").unwrap();
        fs::write(src_dir.join("utils.py"), "def helper(): pass\n").unwrap();

        let surface = make_surface("S-1", vec!["src"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root).unwrap();
        assert!(sp.prompt.contains("os.system(cmd)"));
        assert!(sp.prompt.contains("def helper(): pass"));
    }

    #[test]
    fn skips_large_files() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("big.py"), &"x".repeat(60 * 1024)).unwrap();

        let surface = make_surface("S-1", vec!["src/big.py"]);
        let meta = make_metadata(root);
        assert!(build_surface_prompt(&surface, &meta, root).is_none());
    }

    #[test]
    fn cache_key_deterministic() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "os.system(cmd)\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let meta = make_metadata(root);

        let sp1 = build_surface_prompt(&surface, &meta, root).unwrap();
        let sp2 = build_surface_prompt(&surface, &meta, root).unwrap();
        assert_eq!(sp1.cache_key, sp2.cache_key);
    }

    #[test]
    fn cache_key_changes_with_content() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "version_1\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let meta = make_metadata(root);
        let sp1 = build_surface_prompt(&surface, &meta, root).unwrap();

        fs::write(src_dir.join("app.py"), "version_2\n").unwrap();
        let sp2 = build_surface_prompt(&surface, &meta, root).unwrap();
        assert_ne!(sp1.cache_key, sp2.cache_key);
    }

    #[test]
    fn deduplicates_overlapping_locations() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "eval(x)\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py", "src/app.py"]);
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &meta, root).unwrap();
        assert_eq!(sp.prompt.matches("eval(x)").count(), 1);
    }
}
