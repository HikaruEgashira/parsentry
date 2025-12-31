//! Report generation for Parsentry security scanner.
//!
//! This crate provides various report formats:
//! - Markdown reports
//! - SARIF (Static Analysis Results Interchange Format)
//! - Summary reports
//! - Filename generation utilities

pub mod filename;
pub mod markdown;
pub mod prompts;
pub mod sarif;
pub mod summary;
pub mod validation;

pub use filename::{generate_output_filename, generate_pattern_specific_filename};
pub use markdown::to_markdown;
pub use sarif::{SarifReport, SarifResult, SarifResultProperties};
pub use summary::AnalysisSummary;
pub use validation::validate_output_directory;
