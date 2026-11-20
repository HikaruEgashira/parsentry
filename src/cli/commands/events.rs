//! Runtime progress events appended to `events.jsonl` in the scan cache.
//!
//! Each line is one JSON object tagged by `event`, so `parsentry log` and
//! external tools can follow a scan without parsing human-readable output.

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

pub const EVENTS_FILE: &str = "events.jsonl";

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ScanEvent {
    ScanStarted {
        total_files: usize,
        languages: usize,
    },
    ModelPrompted,
    ModelWritten {
        surfaces: usize,
    },
    SurfacesPending {
        pending: usize,
        cached: usize,
    },
    SurfaceCompleted {
        surface_id: String,
    },
    ReportWritten {
        findings: usize,
    },
}

#[derive(Serialize)]
struct Record<'a> {
    timestamp: String,
    #[serde(flatten)]
    event: &'a ScanEvent,
}

pub fn events_path(cache_root: &Path) -> PathBuf {
    cache_root.join(EVENTS_FILE)
}

/// Serialize `event` as a single JSON line with an RFC 3339 timestamp.
pub fn to_line(event: &ScanEvent) -> Result<String> {
    let record = Record {
        timestamp: chrono::Utc::now().to_rfc3339(),
        event,
    };
    Ok(serde_json::to_string(&record)?)
}

/// Append `event` to `<cache_root>/events.jsonl`.
pub fn append_event(cache_root: &Path, event: &ScanEvent) -> Result<()> {
    let path = events_path(cache_root);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    writeln!(file, "{}", to_line(event)?)
        .with_context(|| format!("failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_tagged_event_with_timestamp() {
        let line = to_line(&ScanEvent::SurfacesPending {
            pending: 2,
            cached: 1,
        })
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&line).unwrap();
        assert_eq!(v["event"], "surfaces_pending");
        assert_eq!(v["pending"], 2);
        assert_eq!(v["cached"], 1);
        assert!(v["timestamp"].as_str().unwrap().contains('T'));
        assert!(!line.contains('\n'));
    }

    #[test]
    fn appends_one_line_per_event() {
        let dir = tempfile::tempdir().unwrap();
        append_event(dir.path(), &ScanEvent::ModelPrompted).unwrap();
        append_event(
            dir.path(),
            &ScanEvent::SurfaceCompleted {
                surface_id: "S-1".into(),
            },
        )
        .unwrap();
        let text = std::fs::read_to_string(events_path(dir.path())).unwrap();
        let lines: Vec<serde_json::Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0]["event"], "model_prompted");
        assert_eq!(lines[1]["event"], "surface_completed");
        assert_eq!(lines[1]["surface_id"], "S-1");
    }
}
