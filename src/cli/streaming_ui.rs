//! Streaming UI component for real-time output display
//!
//! Provides callback implementations for displaying streaming output
//! from Codex CLI in real-time.

use std::collections::VecDeque;
use std::io::{stderr, Write};
use std::sync::Mutex;

use parsentry_codex::{StreamCallback as CodexStreamCallback, StreamEvent as CodexStreamEvent};

use crate::cli::ui::{colors, colors_enabled, StatusPrinter};

const MAX_DISPLAY_LINES: usize = 3;

/// ANSI escape sequences for terminal control
mod ansi {
    pub const CLEAR_LINE: &str = "\x1b[2K";
    pub const CURSOR_UP: &str = "\x1b[A";
    pub const CURSOR_START: &str = "\r";
}

/// Streaming output display for terminal with scroll buffer
///
/// Implements `StreamCallback` to receive and display streaming events
/// from execution in real-time. Maintains a buffer of the last
/// 3 lines and scrolls them as new content arrives, clearing old lines.
pub struct StreamingDisplay {
    printer: StatusPrinter,
    current_file: Mutex<Option<String>>,
    line_buffer: Mutex<VecDeque<String>>,
    displayed_lines: Mutex<usize>,
    use_colors: bool,
}

impl StreamingDisplay {
    /// Create a new streaming display
    ///
    /// # Arguments
    /// * `_show_tokens` - Reserved for future use (token-by-token output)
    pub fn new(_show_tokens: bool) -> Self {
        Self {
            printer: StatusPrinter::new(),
            current_file: Mutex::new(None),
            line_buffer: Mutex::new(VecDeque::with_capacity(MAX_DISPLAY_LINES)),
            displayed_lines: Mutex::new(0),
            use_colors: colors_enabled(),
        }
    }

    /// Set the current file being analyzed (for context in output)
    pub fn set_current_file(&self, file: &str) {
        if let Ok(mut current_file) = self.current_file.lock() {
            *current_file = Some(file.to_string());
        }
    }

    /// Clear the current file context
    pub fn clear_current_file(&self) {
        self.clear_display();
        if let Ok(mut current_file) = self.current_file.lock() {
            *current_file = None;
        }
    }

    /// Clear the streaming display area
    pub fn clear_display(&self) {
        if let (Ok(mut displayed), Ok(mut buffer)) =
            (self.displayed_lines.lock(), self.line_buffer.lock())
        {
            if *displayed > 0 {
                let mut output = String::new();
                for _ in 0..*displayed {
                    output.push_str(ansi::CURSOR_UP);
                    output.push_str(ansi::CURSOR_START);
                    output.push_str(ansi::CLEAR_LINE);
                }
                let _ = stderr().write_all(output.as_bytes());
                let _ = stderr().flush();
            }
            *displayed = 0;
            buffer.clear();
        }
    }

    /// Add a line to the scroll buffer and display it (thread-safe, atomic output)
    fn add_line_with_scroll(&self, line: String) {
        if let (Ok(mut buffer), Ok(mut displayed)) =
            (self.line_buffer.lock(), self.displayed_lines.lock())
        {
            if buffer.len() >= MAX_DISPLAY_LINES {
                buffer.pop_front();
            }
            buffer.push_back(line);

            // Build entire output as single string for atomic write
            let mut output = String::new();

            // Clear previous lines
            for _ in 0..*displayed {
                output.push_str(ansi::CURSOR_UP);
                output.push_str(ansi::CURSOR_START);
                output.push_str(ansi::CLEAR_LINE);
            }

            // Redraw buffer
            for line in buffer.iter() {
                let truncated = if line.len() > 80 {
                    format!("{}...", &line[..77])
                } else {
                    line.clone()
                };
                if self.use_colors {
                    output.push_str(&format!("{}{}{}\n", colors::DIM, truncated, colors::RESET));
                } else {
                    output.push_str(&truncated);
                    output.push('\n');
                }
            }

            // Single atomic write
            let _ = stderr().write_all(output.as_bytes());
            let _ = stderr().flush();

            *displayed = buffer.len();
        }
    }

    /// Handle success completion
    pub fn complete_success(&self, file: &str) {
        self.clear_display();
        self.printer.success("Analyzed", file);
    }

    /// Handle error
    pub fn complete_error(&self, err: &str) {
        self.clear_display();
        self.printer.error("Error", err);
    }
}

impl CodexStreamCallback for StreamingDisplay {
    fn on_event(&self, event: CodexStreamEvent) {
        match event {
            CodexStreamEvent::Text(_) => {
                // Token streaming is handled via Progress events
            }
            CodexStreamEvent::ToolUse { name, .. } => {
                self.add_line_with_scroll(format!("Tool Using {}", name));
            }
            CodexStreamEvent::ToolComplete { .. } => {
                // Tool completion is silent - next ToolUse will replace it
            }
            CodexStreamEvent::Progress(msg) => {
                let trimmed = msg.trim();
                if !trimmed.is_empty() {
                    self.add_line_with_scroll(trimmed.to_string());
                }
            }
            CodexStreamEvent::Complete(_) => {
                self.clear_display();
                if let Ok(file) = self.current_file.lock() {
                    if let Some(ref f) = *file {
                        self.printer.success("Analyzed", f);
                    }
                }
            }
            CodexStreamEvent::Error(err) => {
                self.clear_display();
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

impl CodexStreamCallback for MinimalStreamingDisplay {
    fn on_event(&self, event: CodexStreamEvent) {
        match event {
            CodexStreamEvent::Progress(msg) => {
                // Only show significant progress messages
                let trimmed = msg.trim();
                if trimmed.contains("Analyzing")
                    || trimmed.contains("Processing")
                    || trimmed.contains("Scanning")
                {
                    self.printer.dim(trimmed);
                }
            }
            CodexStreamEvent::Error(err) => {
                self.printer.error("Error", &err);
            }
            CodexStreamEvent::Complete(_) => {}
            _ => {}
        }
    }
}

/// Silent callback that collects events without displaying
///
/// Useful for testing or when you want to process events programmatically.
pub struct SilentStreamingDisplay {
    events: Mutex<Vec<CodexStreamEvent>>,
}

impl SilentStreamingDisplay {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }

    /// Get all collected events
    pub fn events(&self) -> Vec<CodexStreamEvent> {
        self.events
            .lock()
            .map(|e| e.clone())
            .unwrap_or_default()
    }

    /// Clear collected events
    pub fn clear(&self) {
        if let Ok(mut events) = self.events.lock() {
            events.clear();
        }
    }
}

impl Default for SilentStreamingDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl CodexStreamCallback for SilentStreamingDisplay {
    fn on_event(&self, event: CodexStreamEvent) {
        self.events.lock().unwrap().push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_display_creation() {
        let display = StreamingDisplay::new(false);
        assert!(display.line_buffer.lock().unwrap().is_empty());
    }

    #[test]
    fn test_silent_display_collects_events() {
        let display = SilentStreamingDisplay::new();
        CodexStreamCallback::on_event(&display, CodexStreamEvent::Text("hello".to_string()));
        CodexStreamCallback::on_event(&display, CodexStreamEvent::Progress("working".to_string()));

        let events = display.events();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_set_current_file() {
        let display = StreamingDisplay::new(false);
        display.set_current_file("test.rs");
        if let Ok(file) = display.current_file.lock() {
            assert_eq!(*file, Some("test.rs".to_string()));
        }
        display.clear_current_file();
        if let Ok(file) = display.current_file.lock() {
            assert_eq!(*file, None);
        }
    }

    #[test]
    fn test_line_buffer_scrolls() {
        let display = StreamingDisplay::new(false);

        if let (Ok(mut buffer), Ok(mut displayed)) =
            (display.line_buffer.lock(), display.displayed_lines.lock())
        {
            buffer.push_back("line1".to_string());
            buffer.push_back("line2".to_string());
            buffer.push_back("line3".to_string());
            *displayed = 3;

            if buffer.len() >= MAX_DISPLAY_LINES {
                buffer.pop_front();
            }
            buffer.push_back("line4".to_string());
            *displayed = buffer.len();

            assert_eq!(buffer.len(), MAX_DISPLAY_LINES);
            assert_eq!(buffer.get(0), Some(&"line2".to_string()));
            assert_eq!(buffer.get(1), Some(&"line3".to_string()));
            assert_eq!(buffer.get(2), Some(&"line4".to_string()));
        }
    }

    #[test]
    fn test_clear_display() {
        let display = StreamingDisplay::new(false);

        if let (Ok(mut buffer), Ok(mut displayed)) =
            (display.line_buffer.lock(), display.displayed_lines.lock())
        {
            buffer.push_back("test".to_string());
            *displayed = 1;
        }

        display.clear_display();

        if let (Ok(buffer), Ok(displayed)) =
            (display.line_buffer.lock(), display.displayed_lines.lock())
        {
            assert!(buffer.is_empty());
            assert_eq!(*displayed, 0);
        }
    }
}
