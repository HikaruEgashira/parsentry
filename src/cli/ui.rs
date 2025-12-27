//! Unified CLI UI components for consistent, polished output
//!
//! Inspired by: cargo, ripgrep, bat, fd


/// ANSI color codes for terminal styling
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";

    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const RED: &str = "\x1b[31m";
    pub const WHITE: &str = "\x1b[37m";

    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
}

/// Check if colors should be enabled
pub fn colors_enabled() -> bool {
    // Respect NO_COLOR and TERM conventions
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    if std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false) {
        return false;
    }
    // Check if stderr is a terminal
    atty::is(atty::Stream::Stderr)
}

/// Get terminal width, defaulting to 80
pub fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

/// Truncate string with ellipsis if too long
pub fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        return path.to_string();
    }
    if max_len <= 3 {
        return "...".to_string();
    }
    format!("...{}", &path[path.len() - (max_len - 3)..])
}

/// Status line printer with consistent formatting
/// Inspired by cargo's output style: `   Compiling foo v0.1.0`
pub struct StatusPrinter {
    use_colors: bool,
}

impl StatusPrinter {
    pub fn new() -> Self {
        Self {
            use_colors: colors_enabled(),
        }
    }

    fn styled(&self, color: &str, bold: bool, text: &str) -> String {
        if self.use_colors {
            let bold_code = if bold { colors::BOLD } else { "" };
            format!("{}{}{}{}", bold_code, color, text, colors::RESET)
        } else {
            text.to_string()
        }
    }

    /// Print a status line: `  Scanning  target/path`
    pub fn status(&self, keyword: &str, message: &str) {
        let keyword_styled = self.styled(colors::BRIGHT_GREEN, true, &format!("{:>12}", keyword));
        eprintln!("{} {}", keyword_styled, message);
    }

    /// Print an info line: `      Info  some information`
    pub fn info(&self, keyword: &str, message: &str) {
        let keyword_styled = self.styled(colors::BRIGHT_CYAN, true, &format!("{:>12}", keyword));
        eprintln!("{} {}", keyword_styled, message);
    }

    /// Print a warning line
    pub fn warning(&self, keyword: &str, message: &str) {
        let keyword_styled = self.styled(colors::BRIGHT_YELLOW, true, &format!("{:>12}", keyword));
        eprintln!("{} {}", keyword_styled, message);
    }

    /// Print an error line
    pub fn error(&self, keyword: &str, message: &str) {
        let keyword_styled = self.styled(colors::BRIGHT_RED, true, &format!("{:>12}", keyword));
        eprintln!("{} {}", keyword_styled, message);
    }

    /// Print a success line
    pub fn success(&self, keyword: &str, message: &str) {
        let keyword_styled = self.styled(colors::GREEN, true, &format!("{:>12}", keyword));
        eprintln!("{} {}", keyword_styled, message);
    }

    /// Print a dim/secondary info line
    pub fn dim(&self, message: &str) {
        let msg = if self.use_colors {
            format!("{}{}{}", colors::DIM, message, colors::RESET)
        } else {
            message.to_string()
        };
        eprintln!("             {}", msg);
    }

    /// Print section header
    pub fn section(&self, title: &str) {
        let title_styled = self.styled(colors::BOLD, false, title);
        eprintln!();
        eprintln!("{}", title_styled);
    }

    /// Print a key-value pair
    pub fn kv(&self, key: &str, value: &str) {
        let key_styled = self.styled(colors::DIM, false, key);
        eprintln!("  {}: {}", key_styled, value);
    }

    /// Print a bullet point
    pub fn bullet(&self, text: &str) {
        let bullet = self.styled(colors::DIM, false, "•");
        eprintln!("  {} {}", bullet, text);
    }

    /// Print confidence with color coding
    pub fn confidence(&self, score: u8) {
        let (color, label) = match score {
            90..=100 => (colors::RED, "critical"),
            70..=89 => (colors::BRIGHT_YELLOW, "high"),
            50..=69 => (colors::YELLOW, "medium"),
            30..=49 => (colors::CYAN, "low"),
            _ => (colors::DIM, "info"),
        };
        let styled = self.styled(color, true, &format!("{} ({}%)", label, score));
        eprint!("{}", styled);
    }
}

impl Default for StatusPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary table for final results
pub struct SummaryTable {
    rows: Vec<SummaryRow>,
    use_colors: bool,
}

pub struct SummaryRow {
    pub file: String,
    pub pattern: String,
    pub confidence: u8,
    pub vuln_types: Vec<String>,
}

impl SummaryTable {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            use_colors: colors_enabled(),
        }
    }

    pub fn add(&mut self, row: SummaryRow) {
        self.rows.push(row);
    }

    pub fn print(&self) {
        if self.rows.is_empty() {
            return;
        }

        let term_width = terminal_width();
        let file_width = (term_width / 3).min(40);

        eprintln!();
        let header = if self.use_colors {
            format!(
                "{}{}  {:file_width$}  {:>6}  {}{}",
                colors::BOLD,
                colors::WHITE,
                "FILE",
                "CONF",
                "VULNERABILITIES",
                colors::RESET,
                file_width = file_width
            )
        } else {
            format!(
                "  {:file_width$}  {:>6}  {}",
                "FILE",
                "CONF",
                "VULNERABILITIES",
                file_width = file_width
            )
        };
        eprintln!("{}", header);

        let sep_len = term_width.min(100);
        if self.use_colors {
            eprintln!("{}{}{}", colors::DIM, "─".repeat(sep_len), colors::RESET);
        } else {
            eprintln!("{}", "-".repeat(sep_len));
        }

        for row in &self.rows {
            let file_display = truncate_path(&row.file, file_width);

            let conf_color = match row.confidence {
                90..=100 => colors::RED,
                70..=89 => colors::BRIGHT_YELLOW,
                50..=69 => colors::YELLOW,
                30..=49 => colors::CYAN,
                _ => colors::DIM,
            };

            let vulns = row.vuln_types.join(", ");

            if self.use_colors {
                eprintln!(
                    "  {:file_width$}  {}{:>5}%{}  {}",
                    file_display,
                    conf_color,
                    row.confidence,
                    colors::RESET,
                    vulns,
                    file_width = file_width
                );
            } else {
                eprintln!(
                    "  {:file_width$}  {:>5}%  {}",
                    file_display, row.confidence, vulns,
                    file_width = file_width
                );
            }
        }

        eprintln!();
    }
}

impl Default for SummaryTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress indicator styles
pub mod progress {
    use indicatif::{ProgressBar, ProgressStyle};

    /// Create a styled progress bar
    pub fn create_bar(total: u64) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.cyan} [{elapsed_precise}] {bar:30.cyan/blue} {pos}/{len} {msg:.dim}")
                .unwrap()
                .progress_chars("━━╸")
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb
    }

    /// Create a spinner for indeterminate progress
    pub fn create_spinner(message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.set_message(message.to_string());
        pb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_path() {
        assert_eq!(truncate_path("short", 10), "short");
        assert_eq!(truncate_path("verylongpath", 8), "...gpath");
        assert_eq!(truncate_path("abc", 3), "abc"); // fits exactly
        assert_eq!(truncate_path("abcd", 3), "..."); // too short for ellipsis
    }

    #[test]
    fn test_terminal_width() {
        let width = terminal_width();
        assert!(width >= 20); // Should have some reasonable minimum
    }
}
