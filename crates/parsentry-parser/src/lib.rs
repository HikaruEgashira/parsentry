//! Tree-sitter based code parser and security pattern matcher.
//!
//! This crate provides:
//! - Code parsing using tree-sitter grammars
//! - Security pattern matching for vulnerability detection
//! - PAR (Principal-Action-Resource) pattern classification

mod parser;
mod patterns;

pub use parser::{CodeParser, Context, Definition};
pub use patterns::{
    LanguagePatterns, PatternConfig, PatternMatch, PatternQuery, PatternType, SecurityRiskPatterns,
};

// Re-export tree-sitter types for downstream crates
pub use streaming_iterator::StreamingIterator;
pub use tree_sitter::{Query, QueryCursor};
