//! Streaming UI component for real-time Claude Code output display
//!
//! Provides callback implementations for displaying streaming output
//! from Claude Code CLI in real-time.

use std::io::{stderr, Write};
use std::sync::Mutex;

use parsentry_claude_code::{StreamCallback, StreamEvent};

use crate::cli::ui::{colors, colors_enabled, StatusPrinter};

/// Streaming output display for terminal
///
/// Implements `StreamCallback` to receive and display streaming events
/// from Claude Code execution in real-time.
pub struct StreamingDisplay {
    printer: StatusPrinter,
    current_file: Mutex<Option<String>>,
    show_tokens: bool,
    use_colors: bool,
}

impl StreamingDisplay {
    /// Create a new streaming display
    ///
    /// # Arguments
    /// * `show_tokens` - If true, display token-by-token output (very verbose)
    pub fn new(show_tokens: bool) -> Self {
        Self {
            printer: StatusPrinter::new(),
            current_file: Mutex::new(None),
            show_tokens,
            use_colors: colors_enabled(),
        }
    }

    /// Set the current file being analyzed (for context in output)
    pub fn set_current_file(&self, file: &str) {
        *self.current_file.lock().unwrap() = Some(file.to_string());
    }

    /// Clear the current file context
    pub fn clear_current_file(&self) {
        *self.current_file.lock().unwrap() = None;
    }

    fn format_tool_name(&self, name: &str) -> String {
        if self.use_colors {
            format!("{}{}{}", colors::CYAN, name, colors::RESET)
        } else {
            name.to_string()
        }
    }
}

impl StreamCallback for StreamingDisplay {
    fn on_event(&self, event: StreamEvent) {
        match event {
            StreamEvent::Text(text) => {
                if self.show_tokens {
                    eprint!("{}", text);
                    let _ = stderr().flush();
                }
            }
            StreamEvent::ToolUse { name, .. } => {
                let tool = self.format_tool_name(&name);
                self.printer.info("Tool", &format!("Using {}", tool));
            }
            StreamEvent::ToolComplete { name, success } => {
                if success {
                    self.printer.success("Done", &name);
                } else {
                    self.printer.warning("Failed", &name);
                }
            }
            StreamEvent::Progress(msg) => {
                let trimmed = msg.trim();
                if !trimmed.is_empty() {
                    self.printer.dim(trimmed);
                }
            }
            StreamEvent::Complete(_) => {
                let file = self.current_file.lock().unwrap();
                if let Some(ref f) = *file {
                    self.printer.success("Analyzed", f);
                }
            }
            StreamEvent::Error(err) => {
                self.printer.error("Error", &err);
            }
        }
    }
}

/// Minimal streaming display that only shows progress
///
/// Use this for less verbose output that still shows analysis progress.
pub struct MinimalStreamingDisplay {
    printer: StatusPrinter,
}

impl MinimalStreamingDisplay {
    pub fn new() -> Self {
        Self {
            printer: StatusPrinter::new(),
        }
    }
}

impl Default for MinimalStreamingDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamCallback for MinimalStreamingDisplay {
    fn on_event(&self, event: StreamEvent) {
        match event {
            StreamEvent::Progress(msg) => {
                // Only show significant progress messages
                let trimmed = msg.trim();
                if trimmed.contains("Analyzing")
                    || trimmed.contains("Processing")
                    || trimmed.contains("Scanning")
                {
                    self.printer.dim(trimmed);
                }
            }
            StreamEvent::Error(err) => {
                self.printer.error("Error", &err);
            }
            StreamEvent::Complete(_) => {}
            _ => {}
        }
    }
}

/// Silent callback that collects events without displaying
///
/// Useful for testing or when you want to process events programmatically.
pub struct SilentStreamingDisplay {
    events: Mutex<Vec<StreamEvent>>,
}

impl SilentStreamingDisplay {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }

    /// Get all collected events
    pub fn events(&self) -> Vec<StreamEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Clear collected events
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl Default for SilentStreamingDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamCallback for SilentStreamingDisplay {
    fn on_event(&self, event: StreamEvent) {
        self.events.lock().unwrap().push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_display_creation() {
        let display = StreamingDisplay::new(false);
        assert!(!display.show_tokens);
    }

    #[test]
    fn test_silent_display_collects_events() {
        let display = SilentStreamingDisplay::new();
        display.on_event(StreamEvent::Text("hello".to_string()));
        display.on_event(StreamEvent::Progress("working".to_string()));

        let events = display.events();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_set_current_file() {
        let display = StreamingDisplay::new(false);
        display.set_current_file("test.rs");
        assert_eq!(
            *display.current_file.lock().unwrap(),
            Some("test.rs".to_string())
        );
        display.clear_current_file();
        assert_eq!(*display.current_file.lock().unwrap(), None);
    }
}
