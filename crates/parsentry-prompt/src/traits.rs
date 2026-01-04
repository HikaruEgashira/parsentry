//! Prompt trait definitions.

/// A prompt that can be rendered to a string.
pub trait Prompt {
    /// Render the prompt to a string suitable for LLM input.
    fn render(&self) -> String;
}
