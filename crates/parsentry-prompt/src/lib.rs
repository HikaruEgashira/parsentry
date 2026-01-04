//! Declarative prompt builder for Parsentry security analysis.
//!
//! This crate provides a declarative API for constructing LLM prompts
//! for security vulnerability analysis.
//!
//! # Example
//!
//! ```rust
//! use parsentry_prompt::{Prompt, SecurityAnalysisPrompt, TargetFile, PocMode, OutputFormat};
//! use parsentry_i18n::Language;
//!
//! let prompt = SecurityAnalysisPrompt {
//!     target: TargetFile::new("src/main.rs", "fn main() {}"),
//!     pattern: None,
//!     language: Language::Japanese,
//!     poc_mode: PocMode::GenerateOnly,
//!     deep_context: true,
//!     output_format: OutputFormat::Json,
//! };
//!
//! let rendered = prompt.render();
//! ```

mod types;
mod traits;
mod security_analysis;
mod verification;
mod file_reference;
mod iac;
mod templates;
mod builder;

pub use types::*;
pub use traits::*;
pub use security_analysis::*;
pub use verification::*;
pub use file_reference::*;
pub use iac::*;
pub use builder::PromptBuilder;
