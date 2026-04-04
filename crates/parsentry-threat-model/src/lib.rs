//! Threat modeling phase for Parsentry.
//!
//! Analyzes repository metadata (structure, dependencies, frameworks)
//! and generates a threat model with tree-sitter queries for targeted scanning.

mod collector;
mod generator;
mod model;
mod report;

pub use collector::RepoMetadata;
pub use generator::ThreatModelGenerator;
pub use model::{ThreatEntry, ThreatModel, ThreatQuery};
pub use report::render_threat_model_md;
