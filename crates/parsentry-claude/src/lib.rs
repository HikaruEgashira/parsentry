//! Claude Code session reader
//!
//! Reads session JSONL files from `~/.claude/` and extracts events
//! such as tool calls, text responses, and completion markers.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};

/// Active Claude Code session
#[derive(Debug, Clone)]
pub struct Session {
    pub pid: u32,
    pub session_id: String,
    pub cwd: PathBuf,
    pub started_at: u64,
}

/// Event extracted from session JSONL
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// Tool invocation (Read, Write, Bash, etc.)
    ToolUse {
        name: String,
        summary: String,
        timestamp: String,
    },
    /// Text output from assistant
    Text { content: String, timestamp: String },
    /// Session completed (last-prompt marker)
    Complete,
}

/// Subagent metadata
#[derive(Debug, Clone)]
pub struct SubagentMeta {
    pub agent_type: String,
    pub description: String,
    pub jsonl_path: PathBuf,
}

// --- Internal deserialization types ---

#[derive(Deserialize)]
struct SessionFile {
    pid: u32,
    #[serde(rename = "sessionId")]
    session_id: String,
    cwd: String,
    #[serde(rename = "startedAt")]
    started_at: u64,
}

#[derive(Deserialize)]
struct JournalEntry {
    #[serde(rename = "type")]
    entry_type: String,
    timestamp: Option<String>,
    message: Option<MessageBody>,
    content: Option<String>,
}

#[derive(Deserialize)]
struct MessageBody {
    content: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct SubagentMetaFile {
    #[serde(rename = "agentType")]
    agent_type: String,
    description: String,
}

// --- Public API ---

/// List active Claude Code sessions by reading `~/.claude/sessions/`.
/// Only returns sessions whose PID is still alive.
pub fn list_active_sessions() -> Result<Vec<Session>> {
    let sessions_dir = claude_home()?.join("sessions");
    if !sessions_dir.exists() {
        return Ok(vec![]);
    }

    let mut sessions = Vec::new();
    for entry in fs::read_dir(&sessions_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let data =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let sf: SessionFile = match serde_json::from_str(&data) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Check if PID is alive via kill(pid, 0)
        if !is_pid_alive(sf.pid) {
            continue;
        }

        sessions.push(Session {
            pid: sf.pid,
            session_id: sf.session_id,
            cwd: PathBuf::from(sf.cwd),
            started_at: sf.started_at,
        });
    }

    Ok(sessions)
}

/// Resolve the project sessions directory for a given working directory.
///
/// Claude Code stores project data under `~/.claude/projects/{encoded-path}/`.
/// The path encoding replaces `/` with `-`.
pub fn project_sessions_dir(cwd: &Path) -> Result<PathBuf> {
    let encoded = encode_project_path(cwd);
    Ok(claude_home()?.join("projects").join(encoded))
}

/// Find session IDs in a project directory that are currently active.
pub fn find_active_project_sessions(project_dir: &Path) -> Result<Vec<String>> {
    let active = list_active_sessions()?;
    let active_ids: std::collections::HashSet<String> =
        active.iter().map(|s| s.session_id.clone()).collect();

    let mut result = Vec::new();
    if !project_dir.exists() {
        return Ok(result);
    }

    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if active_ids.contains(stem) {
                    result.push(stem.to_string());
                }
            }
        }
    }

    Ok(result)
}

/// List subagents for a given session.
pub fn list_subagents(project_dir: &Path, session_id: &str) -> Result<Vec<SubagentMeta>> {
    let subagents_dir = project_dir.join(session_id).join("subagents");
    if !subagents_dir.exists() {
        return Ok(vec![]);
    }

    let mut agents = Vec::new();
    for entry in fs::read_dir(&subagents_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".meta.json"))
        {
            let data = fs::read_to_string(&path)?;
            let meta: SubagentMetaFile = match serde_json::from_str(&data) {
                Ok(m) => m,
                Err(_) => continue,
            };

            // Derive JSONL path from meta path: agent-xxx.meta.json → agent-xxx.jsonl
            let jsonl_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".meta.json", ".jsonl");
            let jsonl_path = subagents_dir.join(jsonl_name);

            agents.push(SubagentMeta {
                agent_type: meta.agent_type,
                description: meta.description,
                jsonl_path,
            });
        }
    }

    Ok(agents)
}

/// Read new events from a JSONL file starting at `offset` bytes.
/// Returns the events and the new offset for the next read.
pub fn read_events_from(path: &Path, offset: u64) -> Result<(Vec<SessionEvent>, u64)> {
    let file = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let file_len = file.metadata()?.len();

    if offset >= file_len {
        return Ok((vec![], offset));
    }

    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(offset))?;

    let mut events = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let entry: JournalEntry = match serde_json::from_str(trimmed) {
            Ok(e) => e,
            Err(_) => continue,
        };

        let timestamp = entry.timestamp.clone().unwrap_or_default();

        match entry.entry_type.as_str() {
            "assistant" => {
                if let Some(msg) = &entry.message {
                    if let Some(content) = &msg.content {
                        extract_events_from_content(content, &timestamp, &mut events);
                    }
                }
            }
            "last-prompt" => {
                events.push(SessionEvent::Complete);
            }
            _ => {}
        }
    }

    let new_offset = reader.stream_position()?;
    Ok((events, new_offset))
}

/// Extract the initial prompt content from a session JSONL (queue-operation).
/// Useful for identifying which SURFACE-XXX a session is analyzing.
pub fn extract_surface_id(path: &Path) -> Option<String> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let entry: JournalEntry = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.entry_type == "queue-operation" || entry.entry_type == "user" {
            if let Some(content) = &entry.content {
                return extract_surface_from_text(content);
            }
            if let Some(msg) = &entry.message {
                if let Some(serde_json::Value::Array(blocks)) = &msg.content {
                    for block in blocks {
                        if let Some(text) = block.get("text").and_then(|t| t.as_str()) {
                            if let Some(id) = extract_surface_from_text(text) {
                                return Some(id);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// --- Internal helpers ---

fn claude_home() -> Result<PathBuf> {
    let home = dirs_next::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".claude"))
}

fn encode_project_path(path: &Path) -> String {
    let abs = path.to_string_lossy();
    // Claude Code encodes paths by replacing both '/' and '.' with '-'
    abs.replace('/', "-").replace('.', "-")
}

fn is_pid_alive(pid: u32) -> bool {
    // SAFETY: kill with signal 0 only checks process existence
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

fn extract_events_from_content(
    content: &serde_json::Value,
    timestamp: &str,
    events: &mut Vec<SessionEvent>,
) {
    if let serde_json::Value::Array(blocks) = content {
        for block in blocks {
            match block.get("type").and_then(|t| t.as_str()) {
                Some("tool_use") => {
                    let name = block
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let summary = summarize_tool_input(
                        &name,
                        block.get("input").unwrap_or(&serde_json::Value::Null),
                    );
                    events.push(SessionEvent::ToolUse {
                        name,
                        summary,
                        timestamp: timestamp.to_string(),
                    });
                }
                Some("text") => {
                    let text = block
                        .get("text")
                        .and_then(|t| t.as_str())
                        .unwrap_or("")
                        .to_string();
                    if !text.is_empty() {
                        let first_line = text.lines().next().unwrap_or("");
                        let truncated = truncate_chars(first_line, 120);
                        events.push(SessionEvent::Text {
                            content: truncated,
                            timestamp: timestamp.to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

fn summarize_tool_input(tool_name: &str, input: &serde_json::Value) -> String {
    match tool_name {
        "Read" => {
            let path = input
                .get("file_path")
                .and_then(|p| p.as_str())
                .unwrap_or("?");
            let short = short_path(path);
            if let Some(limit) = input.get("limit").and_then(|l| l.as_u64()) {
                format!("{} (limit={})", short, limit)
            } else {
                short.to_string()
            }
        }
        "Write" => {
            let path = input
                .get("file_path")
                .and_then(|p| p.as_str())
                .unwrap_or("?");
            short_path(path).to_string()
        }
        "Edit" => {
            let path = input
                .get("file_path")
                .and_then(|p| p.as_str())
                .unwrap_or("?");
            short_path(path).to_string()
        }
        "Bash" => {
            let cmd = input.get("command").and_then(|c| c.as_str()).unwrap_or("?");
            truncate_chars(cmd, 60)
        }
        "Grep" => {
            let pattern = input.get("pattern").and_then(|p| p.as_str()).unwrap_or("?");
            format!("/{}/", pattern)
        }
        "Glob" => {
            let pattern = input.get("pattern").and_then(|p| p.as_str()).unwrap_or("?");
            pattern.to_string()
        }
        "Agent" => {
            let desc = input
                .get("description")
                .and_then(|d| d.as_str())
                .unwrap_or("?");
            desc.to_string()
        }
        _ => {
            if let serde_json::Value::Object(map) = input {
                for (k, v) in map.iter().take(1) {
                    if let Some(s) = v.as_str() {
                        return format!("{}={}", k, truncate_chars(s, 40));
                    }
                }
            }
            String::new()
        }
    }
}

/// Truncate a string to at most `max` characters (not bytes), appending "..." if truncated.
fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let end = s
        .char_indices()
        .nth(max.saturating_sub(3))
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    format!("{}...", &s[..end])
}

fn short_path(path: &str) -> &str {
    // Return the last 2 path components
    let parts: Vec<&str> = path.rsplit('/').take(2).collect();
    if parts.len() == 2 {
        let start = path.len() - parts[0].len() - parts[1].len() - 1;
        &path[start..]
    } else {
        path
    }
}

fn extract_surface_from_text(text: &str) -> Option<String> {
    // Look for "SURFACE-NNN" pattern
    let idx = text.find("SURFACE-")?;
    let rest = &text[idx..];
    let end = rest
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '-')
        .unwrap_or(rest.len());
    let surface_id = &rest[..end];
    if surface_id.len() > 8 {
        Some(surface_id.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_project_path() {
        let path = Path::new("/Users/hikae/ghq/github.com/HikaruEgashira/parsentry");
        assert_eq!(
            encode_project_path(path),
            "-Users-hikae-ghq-github-com-HikaruEgashira-parsentry"
        );
    }

    #[test]
    fn test_short_path() {
        assert_eq!(short_path("/a/b/c/d.rs"), "c/d.rs");
        assert_eq!(short_path("file.rs"), "file.rs");
    }

    #[test]
    fn test_extract_surface_from_text() {
        assert_eq!(
            extract_surface_from_text("analyzing SURFACE-001 for vulnerabilities"),
            Some("SURFACE-001".to_string())
        );
        assert_eq!(extract_surface_from_text("no surface here"), None);
    }

    #[test]
    fn test_summarize_tool_input() {
        let input = serde_json::json!({"file_path": "/Users/test/src/main.rs", "limit": 100});
        assert_eq!(
            summarize_tool_input("Read", &input),
            "src/main.rs (limit=100)"
        );

        let input = serde_json::json!({"command": "cargo test"});
        assert_eq!(summarize_tool_input("Bash", &input), "cargo test");
    }
}
