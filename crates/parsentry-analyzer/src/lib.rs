//! LLM-based security analysis for Parsentry.
//!
//! This crate provides file and pattern-based vulnerability analysis.

mod analyzer;

pub use analyzer::{analyze_file, analyze_pattern};
