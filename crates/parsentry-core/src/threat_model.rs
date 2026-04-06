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
    #[serde(default)]
    pub repository: String,
    /// ISO-8601 timestamp of generation
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_surface(id: &str, locations: Vec<&str>) -> AttackSurface {
        AttackSurface {
            id: id.to_string(),
            kind: "endpoint".to_string(),
            identifier: format!("GET /api/{}", id),
            locations: locations.into_iter().map(|s| s.to_string()).collect(),
            description: "test".to_string(),
            query: "".to_string(),
        }
    }

    fn make_model(surfaces: Vec<AttackSurface>) -> ThreatModel {
        ThreatModel {
            repository: "test/repo".to_string(),
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            app_type: "web_application".to_string(),
            summary: "Test".to_string(),
            surfaces,
        }
    }

    #[test]
    fn test_total_surfaces_empty() {
        let model = make_model(vec![]);
        assert_eq!(model.total_surfaces(), 0);
    }

    #[test]
    fn test_total_surfaces_returns_correct_count() {
        let model = make_model(vec![
            make_surface("1", vec!["a.py"]),
            make_surface("2", vec!["b.py"]),
            make_surface("3", vec!["c.py"]),
        ]);
        assert_eq!(model.total_surfaces(), 3);
    }

    #[test]
    fn test_all_locations_deduplicates() {
        let model = make_model(vec![
            make_surface("1", vec!["shared.py", "a.py"]),
            make_surface("2", vec!["shared.py", "b.py"]),
        ]);
        let locs = model.all_locations();
        assert_eq!(locs.len(), 3);
        assert_eq!(locs, vec!["shared.py", "a.py", "b.py"]);
    }

    #[test]
    fn test_all_locations_empty_surfaces() {
        let model = make_model(vec![]);
        assert!(model.all_locations().is_empty());
    }

    #[test]
    fn test_all_locations_preserves_order() {
        let model = make_model(vec![
            make_surface("1", vec!["z.py", "a.py"]),
            make_surface("2", vec!["m.py"]),
        ]);
        let locs = model.all_locations();
        assert_eq!(locs, vec!["z.py", "a.py", "m.py"]);
    }
}
