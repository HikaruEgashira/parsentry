use serde::{Deserialize, Serialize};

/// The kind of attack surface identified.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// HTTP/gRPC/GraphQL endpoints (web applications)
    Endpoint,
    /// Database tables or collections
    DbTable,
    /// Public API functions/methods (libraries)
    PublicApi,
    /// File I/O handlers
    FileHandler,
    /// External service calls (HTTP clients, message queues)
    ExternalCall,
    /// Infrastructure-as-Code resources (Terraform, CloudFormation)
    IacResource,
}

impl std::fmt::Display for SurfaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SurfaceKind::Endpoint => write!(f, "endpoint"),
            SurfaceKind::DbTable => write!(f, "db_table"),
            SurfaceKind::PublicApi => write!(f, "public_api"),
            SurfaceKind::FileHandler => write!(f, "file_handler"),
            SurfaceKind::ExternalCall => write!(f, "external_call"),
            SurfaceKind::IacResource => write!(f, "iac_resource"),
        }
    }
}

/// A single attack surface entry — the unit of scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurface {
    /// Unique identifier (e.g. "SURFACE-001")
    pub id: String,
    /// Kind of attack surface
    pub kind: SurfaceKind,
    /// Human-readable identifier (e.g. "POST /api/users", "users table", "pub fn parse()")
    pub identifier: String,
    /// File paths where this surface is defined/used
    pub locations: Vec<String>,
    /// Brief description of why this is an attack surface
    pub description: String,
    /// Tree-sitter query to locate this surface in code
    pub query: String,
}

/// Complete threat model for a repository — enumerates attack surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatModel {
    /// Repository identifier (path or owner/repo)
    pub repository: String,
    /// ISO-8601 timestamp of generation
    pub generated_at: String,
    /// Application type (e.g. "web_application", "library", "cli", "infrastructure")
    pub app_type: String,
    /// High-level summary of the repository's security posture
    pub summary: String,
    /// Identified attack surfaces — each becomes a scan unit
    pub surfaces: Vec<AttackSurface>,
}

impl ThreatModel {
    /// Total number of identified attack surfaces.
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
