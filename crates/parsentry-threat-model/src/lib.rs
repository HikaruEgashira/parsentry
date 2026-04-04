//! Threat modeling phase for Parsentry.
//!
//! Analyzes repository metadata (structure, dependencies, frameworks)
//! and generates a threat model with tree-sitter queries for targeted scanning.

mod collector;
mod generator;
mod model;
mod report;

pub use collector::RepoMetadata;
pub use generator::{
    ThreatModelGenerator, SYSTEM_PROMPT as THREAT_MODEL_SYSTEM_PROMPT,
    build_prompt as build_threat_model_prompt, parse_response as parse_threat_model_response,
    threat_model_schema,
};
pub use model::{ThreatEntry, ThreatModel, ThreatQuery};
pub use report::render_threat_model_md;
