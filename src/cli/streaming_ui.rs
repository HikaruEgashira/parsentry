//! Streaming UI component for real-time Claude Code output display
//!
//! Provides callback implementations for displaying streaming output
//! from Claude Code CLI in real-time.

use std::collections::VecDeque;
use std::io::{stderr, Write};
use std::sync::Mutex;

use parsentry_claude_code::{StreamCallback, StreamEvent};

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
/// from Claude Code execution in real-time. Maintains a buffer of the last
/// 3 lines and scrolls them as new content arrives, clearing old lines.
pub struct StreamingDisplay {
    printer: StatusPrinter,
    current_file: Mutex<Option<String>>,
    line_buffer: Mutex<VecDeque<String>>,
    displayed_lines: Mutex<usize>,
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
            line_buffer: Mutex::new(VecDeque::with_capacity(MAX_DISPLAY_LINES)),
            displayed_lines: Mutex::new(0),
            show_tokens,
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
            self.clear_lines(*displayed);
            *displayed = 0;
            buffer.clear();
        }
    }

    /// Clear n lines from the terminal
    fn clear_lines(&self, n: usize) {
        let mut out = stderr();
        for _ in 0..n {
            let _ = write!(out, "{}{}{}", ansi::CURSOR_UP, ansi::CURSOR_START, ansi::CLEAR_LINE);
        }
        let _ = out.flush();
    }

    /// Add a line to the scroll buffer and display it
    fn add_line_with_scroll(&self, line: String) {
        if let (Ok(mut buffer), Ok(mut displayed)) =
            (self.line_buffer.lock(), self.displayed_lines.lock())
        {
            self.clear_lines(*displayed);

            if buffer.len() >= MAX_DISPLAY_LINES {
                buffer.pop_front();
            }
            buffer.push_back(line);

            self.redraw_buffer(&buffer);
            *displayed = buffer.len();
        }
    }

    /// Redraw the entire buffer
    fn redraw_buffer(&self, buffer: &VecDeque<String>) {
        let mut out = stderr();
        for line in buffer.iter() {
            let truncated = if line.len() > 80 {
                format!("{}...", &line[..77])
            } else {
                line.clone()
            };
            if self.use_colors {
                let _ = writeln!(out, "{}{}{}", colors::DIM, truncated, colors::RESET);
            } else {
                let _ = writeln!(out, "{}", truncated);
            }
        }
        let _ = out.flush();
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
                    self.add_line_with_scroll(trimmed.to_string());
                }
            }
            StreamEvent::Complete(_) => {
                self.clear_display();
                if let Ok(file) = self.current_file.lock() {
                    if let Some(ref f) = *file {
                        self.printer.success("Analyzed", f);
                    }
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
