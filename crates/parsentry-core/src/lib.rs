//! Core types and traits for Parsentry security scanner.
//!
//! This crate provides fundamental types used across all Parsentry components:
//! - Vulnerability types (VulnType)
//! - Programming language definitions (Language)
//! - PAR (Principal-Action-Resource) analysis types
//! - Response structures

mod language;
mod par;
mod response;
mod vuln_type;

pub use language::Language;
pub use par::{
    ActionInfo, ParAnalysis, PolicyViolation, PrincipalInfo, RemediationAction,
    RemediationGuidance, ResourceInfo, SecurityFunctionQuality, SensitivityLevel, TrustLevel,
};
pub use response::{response_json_schema, Response};
pub use vuln_type::VulnType;
