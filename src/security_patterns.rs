//! Re-exports from parsentry-parser and parsentry-core crates.
//!
//! This module provides security pattern matching for vulnerability detection.

pub use parsentry_core::Language;
pub use parsentry_parser::{
    LanguagePatterns, PatternConfig, PatternMatch, PatternQuery, PatternType, SecurityRiskPatterns,
};
