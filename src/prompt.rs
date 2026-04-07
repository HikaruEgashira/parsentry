//! Per-surface Agent Skills generation.
//!
//! Generates a SKILL.md file for each [`AttackSurface`] by reading actual
//! source code from the surface's locations, so that surfaces can be
//! independently dispatched to CLI agents and cached by content hash.
//!
//! Output follows the [Agent Skills](https://agentskills.io/specification.md) format:
//! each skill is a directory containing a `SKILL.md` with YAML frontmatter.

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
    /// Agent Skills-compatible directory name (lowercase, hyphens only).
    pub skill_name: String,
    /// SKILL.md content (YAML frontmatter + Markdown body).
    pub prompt: String,
    /// SHA-256 hex digest of resolved source contents, used as a cache key.
    pub cache_key: String,
}

/// Convert an arbitrary surface ID to a valid Agent Skills name.
///
/// Rules: lowercase `[a-z0-9-]`, no leading/trailing hyphens,
/// no consecutive hyphens, max 64 chars.
fn sanitize_skill_name(id: &str) -> String {
    let mut name: String = id
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();

    while name.contains("--") {
        name = name.replace("--", "-");
    }

    let name = name.trim_matches('-').to_string();
    let name = if name.is_empty() { "surface".to_string() } else { name };

    if name.len() > 64 {
        name[..64].trim_end_matches('-').to_string()
    } else {
        name
    }
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

/// Generate a SKILL.md for a single [`AttackSurface`].
///
/// Returns `None` when no readable source files overlap with the surface's
/// `locations`.
pub fn build_surface_prompt(
    surface: &AttackSurface,
    root_dir: &Path,
) -> Option<SurfacePrompt> {
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

    let skill_name = sanitize_skill_name(&surface.id);

    // Build SKILL.md description (max 1024 chars per spec)
    let raw_desc = format!(
        "Security vulnerability analysis for {} {}. \
         Analyze source files for OWASP vulnerabilities and output SARIF v2.1.0.",
        surface.kind, surface.identifier
    );
    let description = if raw_desc.len() > 1024 {
        format!("{}…", &raw_desc[..1023])
    } else {
        raw_desc
    };

    let mut skill_md = String::new();

    // YAML frontmatter
    skill_md.push_str("---\n");
    skill_md.push_str(&format!("name: {}\n", skill_name));
    skill_md.push_str(&format!("description: {}\n", description));
    skill_md.push_str("compatibility: Designed for Claude Code with Read tool access\n");
    skill_md.push_str("allowed-tools: Read\n");
    skill_md.push_str("metadata:\n");
    skill_md.push_str(&format!("  surface_id: \"{}\"\n", surface.id));
    skill_md.push_str(&format!("  kind: \"{}\"\n", surface.kind));
    skill_md.push_str("---\n\n");

    // Body
    skill_md.push_str(
        "You are a security auditor. Read the source files listed in Locations \
         and analyze them for vulnerabilities.\n\n",
    );

    skill_md.push_str("## Surface Under Analysis\n\n");
    skill_md.push_str(&format!("- **ID**: {}\n", surface.id));
    skill_md.push_str(&format!("- **Kind**: {}\n", surface.kind));
    skill_md.push_str(&format!("- **Identifier**: {}\n", surface.identifier));
    skill_md.push_str(&format!("- **Description**: {}\n", surface.description));
    skill_md.push_str(&format!(
        "- **Locations**: {}\n\n",
        surface.locations.join(", ")
    ));

    skill_md.push_str("## Output Instructions\n\n");
    skill_md.push_str("Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.\n");
    skill_md.push_str("For each finding, provide:\n");
    skill_md.push_str("- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)\n");
    skill_md.push_str("- level: error/warning/note\n");
    skill_md.push_str("- confidence: 0.0-1.0\n");

    Some(SurfacePrompt {
        surface_id: surface.id.clone(),
        skill_name,
        prompt: skill_md,
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

/// Build an orchestrator SKILL.md that dispatches all surface analyses
/// as parallel subagents within a single agent process.
///
/// Each subagent reads the surface's `SKILL.md` from its skill directory.
pub fn build_orchestrator_prompt(
    surface_prompts: &[SurfacePrompt],
    output_dir: &Path,
) -> String {
    let mut skill_md = String::new();

    // YAML frontmatter
    skill_md.push_str("---\n");
    skill_md.push_str("name: security-analysis-orchestrator\n");
    skill_md.push_str(
        "description: Orchestrate parallel security analysis across all attack surfaces. \
         Run all surface analyses in parallel and merge SARIF results.\n",
    );
    skill_md.push_str("allowed-tools: Agent Bash\n");
    skill_md.push_str("---\n\n");

    skill_md.push_str(
        "Launch ALL of the following Agent tool calls in a SINGLE message for maximum parallelism. \
         Do NOT use `run_in_background: true` — the orchestrator must wait for all agents to complete \
         before running the post-analysis step. Use `mode: \"dontAsk\"`.\n\n",
    );

    for sp in surface_prompts {
        let skill_path = output_dir.join(&sp.skill_name).join("SKILL.md");
        skill_md.push_str(&format!(
            "- Agent(description: \"Analyze {id}\", prompt: \"Read {path} and execute the instructions in it.\", \
             mode: \"dontAsk\")\n",
            id = sp.surface_id,
            path = skill_path.display(),
        ));
    }

    let merged_sarif = output_dir.join("merged.sarif.json");
    let report_md = output_dir.join("report.md");
    skill_md.push_str(&format!(
        "\nAfter ALL agents complete, run:\n\
         ```bash\n\
         parsentry merge {dir} -o {merged}\n\
         ```\n\
         Then read {merged} and write a security report to {report} with:\n\
         - Executive summary (finding counts by severity)\n\
         - Per-finding details (rule ID, severity, confidence, location, description)\n\
         - Remediation recommendations\n",
        dir = output_dir.display(),
        merged = merged_sarif.display(),
        report = report_md.display(),
    ));

    skill_md
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
        assert_eq!(sp.skill_name, "s-1");
        assert!(sp.prompt.contains("src/auth.py"));
        assert!(sp.prompt.contains("SARIF"));
        assert!(!sp.prompt.contains("password = input()"));
        // SKILL.md has YAML frontmatter
        assert!(sp.prompt.starts_with("---\n"));
        assert!(sp.prompt.contains("name: s-1"));
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
