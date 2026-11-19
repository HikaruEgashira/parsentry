//! Runtime progress event stream.
//!
//! `parsentry scan` appends one JSON object per line to `<cache>/events.jsonl`;
//! `parsentry log` tails the file to show progress. The stream is agent-agnostic:
//! it only records what the runtime itself does.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub const EVENTS_FILE: &str = "events.jsonl";

pub fn events_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join(EVENTS_FILE)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressEvent {
    /// RFC 3339 timestamp.
    pub ts: String,
    /// Event kind: `phase`, `surface_queued`, `report`, ...
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    pub message: String,
}

impl ProgressEvent {
    pub fn new(kind: &str, surface: Option<&str>, message: impl Into<String>) -> Self {
        Self {
            ts: chrono::Local::now().to_rfc3339(),
            kind: kind.to_string(),
            surface: surface.map(str::to_string),
            message: message.into(),
        }
    }
}

/// Append an event to the stream. Failures are non-fatal for the scan.
pub fn emit(cache_dir: &Path, event: &ProgressEvent) {
    let write = || -> Result<()> {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(events_path(cache_dir))?;
        writeln!(f, "{}", serde_json::to_string(event)?)?;
        Ok(())
    };
    if let Err(e) = write() {
        log::debug!("failed to emit progress event: {e}");
    }
}

/// Read complete events written after `offset`. Returns the events and the new
/// offset; a trailing partial line is left for the next read.
pub fn read_events_from(path: &Path, offset: u64) -> Result<(Vec<ProgressEvent>, u64)> {
    let mut f = std::fs::File::open(path)?;
    let len = f.metadata()?.len();
    // Truncated or replaced file: start over.
    let offset = if len < offset { 0 } else { offset };
    f.seek(SeekFrom::Start(offset))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let complete = buf.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let events = buf[..complete]
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    Ok((events, offset + complete as u64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn emitted_events_are_read_incrementally() {
        let dir = TempDir::new().unwrap();
        emit(dir.path(), &ProgressEvent::new("phase", None, "model"));
        let (events, off) = read_events_from(&events_path(dir.path()), 0).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, "phase");

        emit(
            dir.path(),
            &ProgressEvent::new("surface_queued", Some("SURFACE-001"), "q"),
        );
        let (events, off2) = read_events_from(&events_path(dir.path()), off).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].surface.as_deref(), Some("SURFACE-001"));
        assert!(off2 > off);
    }

    #[test]
    fn partial_line_is_deferred() {
        let dir = TempDir::new().unwrap();
        let path = events_path(dir.path());
        std::fs::write(
            &path,
            "{\"ts\":\"t\",\"kind\":\"phase\",\"message\":\"m\"}\n{\"ts\"",
        )
        .unwrap();
        let (events, off) = read_events_from(&path, 0).unwrap();
        assert_eq!(events.len(), 1);
        let (events, off2) = read_events_from(&path, off).unwrap();
        assert!(events.is_empty());
        assert_eq!(off, off2);
    }
}
