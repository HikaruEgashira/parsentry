//! Parsentry - PAR-based security scanner.

pub mod cli;
pub mod config;
pub mod github;
pub mod repo;
pub mod response;

// Re-exports from parsentry-parser for convenience
pub mod parser;
pub mod security_patterns;

// Re-export core types for convenience
pub use parsentry_core::{Language, VulnType};
