//! Per-surface prompt generation.
//!
//! Generates a focused prompt for each [`AttackSurface`] so that surfaces can be
//! independently dispatched to CLI agents and cached by content hash.

use std::path::{Path, PathBuf};

use parsentry_core::{AttackSurface, RepoMetadata, ThreatModel};
use parsentry_parser::PatternMatch;
use sha2::{Digest, Sha256};

/// A prompt scoped to a single attack surface, ready for agent dispatch.
#[derive(Debug, Clone)]
pub struct SurfacePrompt {
    /// Surface identifier (mirrors [`AttackSurface::id`]).
    pub surface_id: String,
    /// The full prompt text for the agent.
    pub prompt: String,
    /// SHA-256 hex digest of `prompt`, used as a cache key.
    pub cache_key: String,
    /// File path where the agent should write its output.
    pub output_path: PathBuf,
}

/// Generate a prompt for a single [`AttackSurface`].
///
/// Returns `None` when no `pattern_matches` overlap with the surface's
/// `locations` (fuzzy path containment check).
///
/// `output_dir` is the directory where the agent should write its SARIF output.
/// The output file will be named `{surface_id}.sarif.json`.
pub fn build_surface_prompt(
    surface: &AttackSurface,
    pattern_matches: &[(PathBuf, PatternMatch)],
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
    output_dir: &Path,
) -> Option<SurfacePrompt> {
    // Filter & deduplicate matches whose relative path overlaps any surface location.
    let mut seen = std::collections::HashSet::new();
    let relevant: Vec<_> = pattern_matches
        .iter()
        .filter(|(fp, _)| {
            let rel = fp
                .strip_prefix(root_dir)
                .unwrap_or(fp)
                .to_string_lossy();
            surface.locations.iter().any(|loc: &String| {
                rel.contains(loc.as_str()) || loc.contains(rel.as_ref())
            })
        })
        .filter(|(fp, pm)| {
            let key = (
                fp.strip_prefix(root_dir)
                    .unwrap_or(fp)
                    .to_string_lossy()
                    .to_string(),
                pm.matched_text.clone(),
            );
            seen.insert(key)
        })
        .collect();

    if relevant.is_empty() {
        return None;
    }

    let repo_context = repo_metadata.to_prompt_context();
    let output_path = output_dir.join(format!("{}.sarif.json", surface.id));

    let mut prompt = String::new();

    prompt.push_str(
        "You are a security auditor. Analyze the following code patterns \
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
    prompt.push_str("\n\n## Detected Patterns\n\n");

    for (file_path, pm) in &relevant {
        let rel_path = file_path
            .strip_prefix(root_dir)
            .unwrap_or(file_path)
            .display();
        prompt.push_str(&format!(
            "### {} — {}\n```\n{}\n```\n\n",
            rel_path, pm.pattern_config.description, pm.matched_text,
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

    let cache_key = hex_sha256(&prompt);

    Some(SurfacePrompt {
        surface_id: surface.id.clone(),
        prompt,
        cache_key,
        output_path,
    })
}

/// Build prompts for every surface in a [`ThreatModel`].
///
/// Surfaces that have no matching patterns are silently skipped.
pub fn build_all_surface_prompts(
    threat_model: &ThreatModel,
    pattern_matches: &[(PathBuf, PatternMatch)],
    repo_metadata: &RepoMetadata,
    root_dir: &Path,
    output_dir: &Path,
) -> Vec<SurfacePrompt> {
    threat_model
        .surfaces
        .iter()
        .filter_map(|s| build_surface_prompt(s, pattern_matches, repo_metadata, root_dir, output_dir))
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
    use parsentry_parser::{PatternConfig, PatternQuery};
    use std::collections::HashMap;

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

    fn make_pattern(text: &str, desc: &str) -> PatternMatch {
        PatternMatch {
            pattern_config: PatternConfig {
                pattern_type: PatternQuery::Reference {
                    reference: String::new(),
                },
                description: desc.to_string(),
                attack_vector: vec![],
            },
            start_byte: 0,
            end_byte: text.len(),
            matched_text: text.to_string(),
        }
    }

    fn make_surface(id: &str, locations: Vec<&str>) -> AttackSurface {
        AttackSurface {
            id: id.to_string(),
            kind: "endpoint".to_string(),
            identifier: format!("GET /api/{}", id),
            locations: locations.into_iter().map(String::from).collect(),
            description: "test surface".to_string(),
            query: String::new(),
        }
    }

    #[test]
    fn returns_none_when_no_matches() {
        let root = Path::new("/repo");
        let out = Path::new("/tmp/out");
        let surface = make_surface("S-1", vec!["src/auth.py"]);
        let matches = vec![(
            PathBuf::from("/repo/src/db.py"),
            make_pattern("SELECT *", "sql query"),
        )];
        let meta = make_metadata(root);

        assert!(build_surface_prompt(&surface, &matches, &meta, root, out).is_none());
    }

    #[test]
    fn builds_prompt_for_matching_surface() {
        let root = Path::new("/repo");
        let out = Path::new("/tmp/out");
        let surface = make_surface("S-1", vec!["src/auth.py"]);
        let matches = vec![(
            PathBuf::from("/repo/src/auth.py"),
            make_pattern("password = input()", "user input"),
        )];
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &matches, &meta, root, out).unwrap();
        assert_eq!(sp.surface_id, "S-1");
        assert!(sp.prompt.contains("S-1"));
        assert!(sp.prompt.contains("password = input()"));
        assert!(sp.prompt.contains("Write the complete SARIF"));
        assert!(sp.prompt.contains("S-1.sarif.json"));
        assert_eq!(sp.output_path, out.join("S-1.sarif.json"));
        assert_eq!(sp.cache_key.len(), 64);
    }

    #[test]
    fn deduplicates_same_file_and_text() {
        let root = Path::new("/repo");
        let out = Path::new("/tmp/out");
        let surface = make_surface("S-1", vec!["src/app.py"]);
        let pm = make_pattern("eval(x)", "eval call");
        let matches = vec![
            (PathBuf::from("/repo/src/app.py"), pm.clone()),
            (PathBuf::from("/repo/src/app.py"), pm),
        ];
        let meta = make_metadata(root);

        let sp = build_surface_prompt(&surface, &matches, &meta, root, out).unwrap();
        let count = sp.prompt.matches("eval(x)").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn cache_key_deterministic() {
        let root = Path::new("/repo");
        let out = Path::new("/tmp/out");
        let surface = make_surface("S-1", vec!["src/app.py"]);
        let matches = vec![(
            PathBuf::from("/repo/src/app.py"),
            make_pattern("os.system(cmd)", "command injection"),
        )];
        let meta = make_metadata(root);

        let sp1 = build_surface_prompt(&surface, &matches, &meta, root, out).unwrap();
        let sp2 = build_surface_prompt(&surface, &matches, &meta, root, out).unwrap();
        assert_eq!(sp1.cache_key, sp2.cache_key);
    }
}
