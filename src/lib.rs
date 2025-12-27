//! Parsentry - PAR-based security scanner.
//!
//! This crate provides the main CLI and orchestration layer for Parsentry.
//! The core functionality is split into the following crates:
//!
//! - `parsentry-core`: Core types (VulnType, Language, Response, etc.)
//! - `parsentry-parser`: Tree-sitter based parsing
//! - `parsentry-i18n`: Internationalization (ja/en)
//! - `parsentry-utils`: Utility functions (FileClassifier)
//! - `parsentry-reports`: Report generation (Markdown, SARIF, etc.)
//! - `parsentry-analyzer`: LLM-based security analysis

// Orchestration layer (main crate only)
pub mod call_graph;
pub mod call_graph_output;
pub mod cli;
pub mod config;
pub mod github;
pub mod mvra;
pub mod pattern_generator_claude_code;
pub mod repo;
pub mod response;

// Re-exports from parsentry-parser for convenience
pub mod parser;
pub mod security_patterns;

// Re-export core types for convenience
pub use parsentry_core::{Language, VulnType};
