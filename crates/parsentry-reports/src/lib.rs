//! Report generation for Parsentry.
//!
//! This crate provides various report formats:
//! - Markdown reports
//! - SARIF (Static Analysis Results Interchange Format)
//! - Summary reports
//! - Filename generation utilities

pub mod filename;
pub mod jira;
pub mod linear;
pub mod markdown;
pub mod merge;
pub mod notion;
pub mod report_common;
pub mod sarif;
pub mod summary;
pub mod validation;

pub use filename::{generate_output_filename, generate_pattern_specific_filename};
pub use jira::run_jira_command;
pub use linear::run_linear_command;
pub use markdown::to_markdown;
pub use merge::merge_sarif_dir;
pub use notion::run_notion_command;
pub use report_common::{load_surface_reports, SurfaceReport};
pub use sarif::{SarifReport, SarifResult, SarifResultProperties};
pub use summary::AnalysisSummary;
pub use validation::validate_output_directory;
