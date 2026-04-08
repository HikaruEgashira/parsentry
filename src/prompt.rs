//! Per-surface prompt generation.
//!
//! Generates a focused prompt for each [`AttackSurface`] by reading actual
//! source code from the surface's locations, so that surfaces can be
//! independently dispatched to CLI agents and cached by content hash.

use std::path::Path;

use parsentry_core::{AttackSurface, FileDiscovery, ThreatModel};
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
        // Resolve the location against the repository root with traversal protection
        let joined = root_dir.join(location);
        // Canonicalize to resolve symlinks and normalize the path
        let full_path = match joined.canonicalize() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Ensure the resolved path stays within the repository root
        let root_canon = root_dir
            .canonicalize()
            .unwrap_or_else(|_| root_dir.to_path_buf());
        if !full_path.starts_with(&root_canon) {
            continue;
        }

        // If the path is a symlink, skip to prevent symlink traversal attacks
        if let Ok(meta) = std::fs::symlink_metadata(&full_path) {
            if meta.file_type().is_symlink() {
                continue;
            }
        }

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
                            sources.push(SourceFile {
                                rel_path: rel,
                                contents,
                            });
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
                            sources.push(SourceFile {
                                rel_path: rel,
                                contents,
                            });
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
pub fn build_surface_prompt(surface: &AttackSurface, root_dir: &Path) -> Option<SurfacePrompt> {
    let sources = resolve_source_files(surface, root_dir);

    if sources.is_empty() {
        return None;
    }

    // Cache key: SHA-256 of concatenated (relative_path + "\0" + file_contents)
    let mut cache_input = String::new();
    for src in &sources {
        cache_input.push_str(&src.rel_path);
        cache_input.push('\0');
        cache_input.push_str(&src.contents);
        cache_input.push('\0');
    }
    let cache_key = hex_sha256(&cache_input);
    let repository_root = root_dir
        .canonicalize()
        .unwrap_or_else(|_| root_dir.to_path_buf());

    let mut prompt = String::new();

    prompt.push_str(
        "You are a security auditor. Read the source files listed in Locations \
         and analyze them for vulnerabilities.\n\n",
    );

    prompt.push_str("Surface Under Analysis\n\n");
    prompt.push_str(&format!("- ID: {}\n", surface.id));
    prompt.push_str(&format!("- Kind: {}\n", surface.kind));
    prompt.push_str(&format!("- Identifier: {}\n", surface.identifier));
    prompt.push_str(&format!("- Description: {}\n", surface.description));
    prompt.push_str(&format!(
        "- Repository Root: {}\n",
        repository_root.display()
    ));
    prompt.push_str(&format!(
        "- Locations: {}\n\n",
        surface.locations.join(", ")
    ));

    prompt.push_str("Output Instructions\n\n");
    prompt.push_str(
        "Resolve every relative path in Locations against Repository Root. \
         Read the relevant files using your file-reading tool, then output valid \
         SARIF v2.1.0 JSON that is compatible with `parsentry merge`.\n",
    );
    prompt.push_str("The SARIF MUST include:\n");
    prompt.push_str("- top-level `$schema`\n");
    prompt.push_str("- top-level `version` set to `2.1.0`\n");
    prompt.push_str("- `runs[0].tool.driver.name`\n");
    prompt.push_str("- `runs[0].tool.driver.version`\n");
    prompt.push_str("For each finding, provide:\n");
    prompt.push_str("- `ruleId`: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)\n");
    prompt.push_str("- `level`: error/warning/note\n");
    prompt.push_str("- `message.text`\n");
    prompt.push_str("- `locations[].physicalLocation.artifactLocation.uri`\n");
    prompt.push_str("- `locations[].physicalLocation.region.startLine` when known\n");
    prompt.push_str("- `properties.confidence`: 0.0-1.0\n");

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
    root_dir: &Path,
) -> Vec<SurfacePrompt> {
    threat_model
        .surfaces
        .iter()
        .filter_map(|s| build_surface_prompt(s, root_dir))
        .collect()
}

/// Build an orchestrator prompt that dispatches all surface analyses
/// in an agent-neutral way.
pub fn build_orchestrator_prompt(
    surface_prompts: &[SurfacePrompt],
    output_dir: &Path,
    target: &str,
    parsentry_bin: &Path,
) -> String {
    let mut prompt = String::new();

    prompt.push_str(
        "You are a security analysis orchestrator. Dispatch one worker per surface in \
         parallel using your environment's built-in subagent or agent capability.\n\n",
    );
    prompt.push_str("Rules\n\n");
    prompt.push_str("1. Do NOT perform the per-surface analysis yourself unless a worker fails.\n");
    prompt.push_str(
        "2. Launch all workers in parallel. If your environment exposes an explicit \
         Agent tool, use it. Otherwise use the environment's equivalent subagent capability.\n",
    );
    prompt.push_str(
        "3. Give each worker exactly one prompt file and tell it to execute the \
         instructions in that file.\n",
    );
    prompt.push_str(
        "4. Each worker must write SARIF JSON to the output path specified inside \
         its assigned prompt file.\n",
    );
    prompt.push_str("5. Wait for every worker to finish before starting post-processing.\n");
    prompt.push_str("\nWorker Assignments\n\n");

    for sp in surface_prompts {
        let prompt_path = output_dir.join(&sp.surface_id).join("prompt.md");
        prompt.push_str(&format!(
            "- Worker `{id}`: read `{path}` and execute the instructions in it.\n",
            id = sp.surface_id,
            path = prompt_path.display(),
        ));
    }

    let target_q = shell_quote(target);
    let parsentry_bin_q = shell_quote(&parsentry_bin.display().to_string());
    let project_cache = output_dir
        .parent()
        .expect("reports dir must have a project cache parent");
    let cache_base = project_cache
        .parent()
        .expect("project cache dir must have a cache base parent");
    let merged_sarif = output_dir.join("merged.sarif.json");
    let report_md = output_dir.join("report.md");
    let cache_base_q = shell_quote(&cache_base.display().to_string());
    let merged_q = shell_quote(&merged_sarif.display().to_string());
    let report_q = shell_quote(&report_md.display().to_string());
    prompt.push_str(&format!(
        "\nAfter ALL workers complete, run exactly:\n\
         ```bash\n\
         tmp_merged=$(mktemp /tmp/parsentry-merged.XXXXXX.json)\n\
         PARSENTRY_CACHE_DIR={cache_base} {parsentry_bin} merge {target} > \"$tmp_merged\"\n\
         test -s \"$tmp_merged\"\n\
         mv \"$tmp_merged\" {merged}\n\
         ```\n\
         Then read {merged} and write a security report to {report} with:\n\
         - Executive summary (finding counts by severity)\n\
         - Per-finding details (rule ID, severity, confidence, location, description)\n\
         - Remediation recommendations\n\
         The task is not complete until `{report}` exists and is non-empty. \
         Verify that with:\n\
         ```bash\n\
         test -s {report_q}\n\
         ```\n",
        cache_base = cache_base_q,
        parsentry_bin = parsentry_bin_q,
        target = target_q,
        merged = merged_q,
        report = report_md.display(),
        report_q = report_q,
    ));

    prompt
}

fn shell_quote(input: &str) -> String {
    format!("'{}'", input.replace('\'', "'\"'\"'"))
}

fn hex_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

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
        assert!(build_surface_prompt(&surface, root).is_none());
    }

    #[test]
    fn builds_prompt_with_surface_metadata() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("auth.py"), "password = input()\n").unwrap();

        let surface = make_surface("S-1", vec!["src/auth.py"]);
        let sp = build_surface_prompt(&surface, root).unwrap();
        assert_eq!(sp.surface_id, "S-1");
        assert!(sp.prompt.contains("src/auth.py"));
        assert!(sp.prompt.contains("SARIF"));
        assert!(!sp.prompt.contains("password = input()"));
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
        let sp = build_surface_prompt(&surface, root).unwrap();
        // Source code not inlined, but prompt should exist
        assert!(sp.prompt.contains("S-1"));
        assert!(!sp.prompt.contains("os.system(cmd)"));
    }

    #[test]
    fn surface_prompt_mentions_repository_root_and_merge_compatible_sarif() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "print('hi')\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let sp = build_surface_prompt(&surface, root).unwrap();

        assert!(sp.prompt.contains("Repository Root"));
        assert!(sp.prompt.contains("parsentry merge"));
        assert!(sp.prompt.contains("tool.driver.version"));
        assert!(sp.prompt.contains("ruleId"));
    }

    #[test]
    fn orchestrator_prompt_is_agent_neutral_and_uses_safe_merge_flow() {
        let prompts = vec![SurfacePrompt {
            surface_id: "SURFACE-001".to_string(),
            prompt: "irrelevant".to_string(),
            cache_key: "abc".to_string(),
        }];
        let temp = TempDir::new().unwrap();

        let prompt = build_orchestrator_prompt(
            &prompts,
            temp.path(),
            "/tmp/repo with spaces",
            Path::new("/tmp/bin/parsentry"),
        );

        assert!(prompt.contains("subagent or agent capability"));
        assert!(prompt.contains("Worker `SURFACE-001`"));
        assert!(!prompt.contains("Agent("));
        assert!(prompt.contains("PARSENTRY_CACHE_DIR="));
        assert!(prompt.contains("'/tmp/bin/parsentry' merge '/tmp/repo with spaces'"));
        assert!(prompt.contains("tmp_merged=$(mktemp"));
        assert!(prompt.contains("test -s \"$tmp_merged\""));
        assert!(prompt.contains("The task is not complete until"));
        assert!(prompt.contains("test -s '"));
    }

    #[test]
    fn skips_large_files() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("big.py"), &"x".repeat(60 * 1024)).unwrap();

        let surface = make_surface("S-1", vec!["src/big.py"]);
        assert!(build_surface_prompt(&surface, root).is_none());
    }

    #[test]
    fn cache_key_deterministic() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("app.py"), "os.system(cmd)\n").unwrap();

        let surface = make_surface("S-1", vec!["src/app.py"]);
        let sp1 = build_surface_prompt(&surface, root).unwrap();
        let sp2 = build_surface_prompt(&surface, root).unwrap();
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
        let sp1 = build_surface_prompt(&surface, root).unwrap();

        fs::write(src_dir.join("app.py"), "version_2\n").unwrap();
        let sp2 = build_surface_prompt(&surface, root).unwrap();
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
        let sp = build_surface_prompt(&surface, root).unwrap();
        // Cache key should still be deterministic with deduped files
        assert_eq!(sp.cache_key.len(), 64);
    }
}
