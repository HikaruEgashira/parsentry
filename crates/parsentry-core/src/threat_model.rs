use serde::{Deserialize, Serialize};

/// A single component surface — the unit of analysis.
///
/// Represents an isolated component (endpoint, table, public API, etc.)
/// that can be independently analyzed. The threat model enumerates these
/// as self-contained units whose impact is scoped to their own boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurface {
    /// Unique identifier (e.g. "SURFACE-001")
    pub id: String,
    /// Free-form kind label (e.g. "endpoint", "db_table", "public_api", "iac_resource")
    pub kind: String,
    /// Human-readable identifier (e.g. "POST /api/users", "users table", "pub fn parse()")
    pub identifier: String,
    /// File paths where this component is defined/used
    pub locations: Vec<String>,
    /// Brief description of why this component warrants analysis
    pub description: String,
    /// Tree-sitter query to locate this component in code
    pub query: String,
}

/// Threat model for a repository — defines isolated components and their relationships.
///
/// A threat model enumerates the independently-analyzable surfaces of a
/// codebase. Each surface has bounded impact: a vulnerability or change
/// in one surface does not automatically affect others. The model maps
/// the dependency graph between these components so that analysis can be
/// parallelized per-surface and cached per-input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatModel {
    /// Repository identifier (path or owner/repo)
    pub repository: String,
    /// ISO-8601 timestamp of generation
    pub generated_at: String,
    /// Application type (e.g. "web_application", "library", "cli", "infrastructure")
    pub app_type: String,
    /// High-level summary of the repository
    pub summary: String,
    /// Identified component surfaces — each becomes an analysis unit
    pub surfaces: Vec<AttackSurface>,
}

impl ThreatModel {
    /// Total number of identified component surfaces.
    pub fn total_surfaces(&self) -> usize {
        self.surfaces.len()
    }

    /// All file paths referenced across all surfaces (deduplicated).
    pub fn all_locations(&self) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for surface in &self.surfaces {
            for loc in &surface.locations {
                if seen.insert(loc.clone()) {
                    result.push(loc.clone());
                }
            }
        }
        result
    }
}
