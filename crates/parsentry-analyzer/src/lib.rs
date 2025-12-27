//! LLM-based security analysis for Parsentry.
//!
//! This crate provides:
//! - File and pattern-based vulnerability analysis
//! - Custom security pattern generation

mod analyzer;
mod pattern_generator;

pub use analyzer::{analyze_file, analyze_pattern};
pub use pattern_generator::{
    analyze_definitions_for_security_patterns, analyze_references_for_security_patterns,
    filter_files_by_size, generate_custom_patterns, write_patterns_to_file, PatternClassification,
};
