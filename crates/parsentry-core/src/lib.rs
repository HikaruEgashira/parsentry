//! Core types and traits for Parsentry.

mod collector;
mod file_classifier;
mod file_discovery;
mod language;
mod par;
mod response;
mod threat_model;
mod threat_model_prompt;
mod threat_model_report;
mod vuln_type;

pub use collector::{ManifestInfo, RepoMetadata};
pub use file_classifier::FileClassifier;
pub use file_discovery::FileDiscovery;
pub use language::Language;
pub use par::{
    ActionInfo, ParAnalysis, PolicyViolation, PrincipalInfo, RemediationAction,
    RemediationGuidance, ResourceInfo, SecurityFunctionQuality, SensitivityLevel, TrustLevel,
};
pub use response::{response_json_schema, Response};
pub use threat_model::{AttackSurface, ThreatModel};
pub use threat_model_prompt::{
    build_threat_model_prompt, parse_threat_model_response, threat_model_schema,
    THREAT_MODEL_SYSTEM_PROMPT,
};
pub use threat_model_report::render_threat_model_md;
pub use vuln_type::VulnType;
