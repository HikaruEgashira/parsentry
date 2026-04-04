//! Minimal prompt builder for Parsentry security analysis.
//!
//! Prompts contain only origin (where to start) and scope (what to look for).
//! The AI agent autonomously explores the codebase using its own tools.
//!
//! # Example
//!
//! ```rust
//! use parsentry_prompt::{AnalysisPrompt, Origin, Scope, OutputConfig, Prompt};
//! use std::path::PathBuf;
//!
//! let prompt = AnalysisPrompt {
//!     origin: Origin::pattern("src/db.py", "sql_injection", "execute(query)"),
//!     scope: Scope::SecurityAudit,
//!     output: OutputConfig::Sarif(PathBuf::from("/tmp/output.sarif")),
//! };
//!
//! let rendered = prompt.render();
//! // => "Analyze src/db.py for security vulnerabilities.\nsql_injection pattern detected: `execute(query)`\nWrite SARIF v2.1.0 results to /tmp/output.sarif."
//! ```

mod types;
mod traits;
mod analysis;
mod builder;

pub use types::*;
pub use traits::*;
pub use analysis::*;
pub use builder::PromptBuilder;
