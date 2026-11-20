use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::cli::ui::{colors, colors_enabled};

const SURFACE_COLORS: &[&str] = &[
    colors::CYAN,
    colors::GREEN,
    colors::YELLOW,
    colors::MAGENTA,
    colors::BLUE,
    colors::BRIGHT_CYAN,
    colors::BRIGHT_GREEN,
    colors::BRIGHT_YELLOW,
];

/// Interval for the fallback poll when filesystem events are missed
const POLL_SECS: u64 = 10;

/// List all report directories under the parsentry cache base.
fn list_all_report_dirs() -> Vec<(String, PathBuf)> {
    let cache_base = super::common::cache_base();
    let mut dirs = Vec::new();

    // Repo targets: cache_base/{owner}__{repo}/reports
    if let Ok(entries) = std::fs::read_dir(&cache_base) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "url" {
                continue;
            }
            let reports = entry.path().join("reports");
            if reports.is_dir() {
                let display_name = name.replace("__", "/");
                dirs.push((display_name, reports));
            }
        }
    }

    // URL targets: cache_base/url/{sha256}/reports
    let url_dir = cache_base.join("url");
    if let Ok(entries) = std::fs::read_dir(&url_dir) {
        for entry in entries.flatten() {
            let reports = entry.path().join("reports");
            if reports.is_dir() {
                // Try to read saved source URL for display
                let display_name = std::fs::read_to_string(entry.path().join("source_url.txt"))
                    .unwrap_or_else(|_| entry.file_name().to_string_lossy().to_string());
                dirs.push((display_name, reports));
            }
        }
    }

    dirs.sort_by(|a, b| a.0.cmp(&b.0));
    dirs
}

pub async fn run_log_command(
    target: Option<&str>,
    follow: bool,
    _tail: Option<usize>,
    timestamps: bool,
    _interval_secs: u64,
    timeout_secs: Option<u64>,
    no_color: bool,
) -> Result<()> {
    let use_colors = !no_color && colors_enabled();
    let start = Instant::now();

    // If no target, list all sessions summary
    if target.is_none() {
        let all = list_all_report_dirs();
        if all.is_empty() {
            print_log(
                "parsentry",
                "no sessions found",
                use_colors,
                timestamps,
                colors::DIM,
            );
            return Ok(());
        }
        for (name, reports_dir) in &all {
            // Surface directories contain prompt.md; orchestrator is a flat file
            let prompt_count = std::fs::read_dir(reports_dir)
                .map(|e| {
                    e.flatten()
                        .filter(|f| f.path().is_dir() && f.path().join("prompt.md").exists())
                        .count()
                })
                .unwrap_or(0);
            let sarif_count = std::fs::read_dir(reports_dir)
                .map(|e| {
                    e.flatten()
                        .filter(|f| {
                            f.path().is_dir() && f.path().join("result.sarif.json").exists()
                        })
                        .count()
                })
                .unwrap_or(0);
            let status = if sarif_count >= prompt_count && prompt_count > 0 {
                "complete"
            } else if sarif_count > 0 {
                "in progress"
            } else if prompt_count > 0 {
                "waiting"
            } else {
                "empty"
            };
            print_log(
                name,
                &format!(
                    "{} surfaces, {} results ({})",
                    prompt_count, sarif_count, status
                ),
                use_colors,
                timestamps,
                colors::CYAN,
            );
        }
        return Ok(());
    }

    let target = target.unwrap();
    let output_dir = super::common::cache_dir_for(target).join("reports");

    if !follow && !output_dir.exists() {
        anyhow::bail!(
            "No reports found for {}. Run `parsentry scan {}` first.",
            target,
            target
        );
    }

    let events_file = super::events::events_path(&super::common::cache_dir_for(target));

    print_log(
        "parsentry",
        &format!("watching {}", target),
        use_colors,
        timestamps,
        colors::BOLD,
    );

    // State
    let mut known_surfaces: Vec<String> = Vec::new();
    let mut surface_colors_map: HashMap<String, &str> = HashMap::new();
    let mut completed: HashSet<String> = HashSet::new();
    let mut events_offset: u64 = 0;
    let mut dir_existed = output_dir.exists();

    flush_scan_events(&events_file, &mut events_offset, use_colors, timestamps);

    // Initial discovery
    if dir_existed {
        discover_new_surfaces(
            &output_dir,
            &mut known_surfaces,
            &mut surface_colors_map,
            use_colors,
            timestamps,
        );
        check_completed_surfaces(
            &output_dir,
            &known_surfaces,
            &mut completed,
            &surface_colors_map,
            use_colors,
            timestamps,
        );
    }

    if !follow {
        if known_surfaces.is_empty() {
            print_log(
                "parsentry",
                "no surfaces found yet",
                use_colors,
                timestamps,
                colors::DIM,
            );
        }
        print_summary(
            &known_surfaces,
            &completed,
            start.elapsed(),
            use_colors,
            timestamps,
        );
        return Ok(());
    }

    if !known_surfaces.is_empty() && completed.len() == known_surfaces.len() {
        print_summary(
            &known_surfaces,
            &completed,
            start.elapsed(),
            use_colors,
            timestamps,
        );
        return Ok(());
    }

    // Set up filesystem watcher
    let (fs_tx, fs_rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(fs_tx)?;

    // Watch output directory (for prompt.md and sarif.json)
    if dir_existed {
        watcher.watch(&output_dir, RecursiveMode::NonRecursive)?;
    }
    // Watch the cache root (for events.jsonl appends)
    let mut events_dir_watched = false;
    if let Some(parent) = events_file.parent()
        && parent.exists()
    {
        events_dir_watched = watcher.watch(parent, RecursiveMode::NonRecursive).is_ok();
    }

    let mut last_poll = Instant::now();

    loop {
        // Check timeout
        if let Some(timeout) = timeout_secs
            && start.elapsed().as_secs() >= timeout
        {
            print_log(
                "parsentry",
                &format!("timeout after {}s", timeout),
                use_colors,
                timestamps,
                colors::BRIGHT_RED,
            );
            print_summary(
                &known_surfaces,
                &completed,
                start.elapsed(),
                use_colors,
                timestamps,
            );
            std::process::exit(1);
        }

        // Wait for fs events with a short timeout so we can do periodic polls
        let recv_timeout = Duration::from_millis(500);
        let mut got_event = false;

        // Drain all pending events
        loop {
            match fs_rx.recv_timeout(if got_event {
                Duration::from_millis(50)
            } else {
                recv_timeout
            }) {
                Ok(Ok(event)) => {
                    got_event = true;
                    // If output dir just appeared, start watching it
                    if !dir_existed && output_dir.exists() {
                        dir_existed = true;
                        let _ = watcher.watch(&output_dir, RecursiveMode::NonRecursive);
                        print_log(
                            "parsentry",
                            &format!("directory appeared: {}", output_dir.display()),
                            use_colors,
                            timestamps,
                            colors::BRIGHT_GREEN,
                        );
                    }

                    if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                        continue;
                    }
                    if event.paths.iter().any(|p| p == &events_file) {
                        flush_scan_events(&events_file, &mut events_offset, use_colors, timestamps);
                    }
                    if event.paths.iter().any(|p| p.starts_with(&output_dir)) {
                        // New file in output dir — check for new surfaces or completions
                        discover_new_surfaces(
                            &output_dir,
                            &mut known_surfaces,
                            &mut surface_colors_map,
                            use_colors,
                            timestamps,
                        );
                        check_completed_surfaces(
                            &output_dir,
                            &known_surfaces,
                            &mut completed,
                            &surface_colors_map,
                            use_colors,
                            timestamps,
                        );
                    }
                }
                Ok(Err(_)) => {
                    got_event = true;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    anyhow::bail!("filesystem watcher disconnected");
                }
            }
        }

        // Periodic fallback poll (directories that appeared late, missed events)
        if last_poll.elapsed() >= Duration::from_secs(POLL_SECS) {
            last_poll = Instant::now();

            if !dir_existed && output_dir.exists() {
                dir_existed = true;
                let _ = watcher.watch(&output_dir, RecursiveMode::NonRecursive);
                print_log(
                    "parsentry",
                    &format!("directory appeared: {}", output_dir.display()),
                    use_colors,
                    timestamps,
                    colors::BRIGHT_GREEN,
                );
            }
            if !events_dir_watched
                && let Some(parent) = events_file.parent()
                && parent.exists()
            {
                events_dir_watched = watcher.watch(parent, RecursiveMode::NonRecursive).is_ok();
            }

            flush_scan_events(&events_file, &mut events_offset, use_colors, timestamps);

            if dir_existed {
                discover_new_surfaces(
                    &output_dir,
                    &mut known_surfaces,
                    &mut surface_colors_map,
                    use_colors,
                    timestamps,
                );
            }

            check_completed_surfaces(
                &output_dir,
                &known_surfaces,
                &mut completed,
                &surface_colors_map,
                use_colors,
                timestamps,
            );
        }

        // All done?
        if !known_surfaces.is_empty() && completed.len() == known_surfaces.len() {
            print_summary(
                &known_surfaces,
                &completed,
                start.elapsed(),
                use_colors,
                timestamps,
            );
            return Ok(());
        }
    }
}

/// Read complete lines appended to `path` since `offset`.
///
/// Returns the lines and the offset just past the last newline, so a partially
/// written trailing line is re-read on the next call.
fn read_lines_from(path: &Path, offset: u64) -> std::io::Result<(Vec<String>, u64)> {
    let mut file = std::fs::File::open(path)?;
    let len = file.metadata()?.len();
    // File was truncated or recreated: start over.
    let offset = if len < offset { 0 } else { offset };
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let consumed = buf.iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let lines = String::from_utf8_lossy(&buf[..consumed])
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect();
    Ok((lines, offset + consumed as u64))
}

/// Render one events.jsonl record as `(service, message)`.
fn format_scan_event(line: &str) -> Option<(String, String)> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    let num = |k: &str| v.get(k).and_then(|x| x.as_u64()).unwrap_or(0);
    let parsentry = || "parsentry".to_string();
    let event = v.get("event")?.as_str()?;
    Some(match event {
        "scan_started" => (
            parsentry(),
            format!(
                "scan started ({} files, {} languages)",
                num("total_files"),
                num("languages")
            ),
        ),
        "model_prompted" => (parsentry(), "threat model prompt written".to_string()),
        "model_written" => (
            parsentry(),
            format!("threat model ready ({} surfaces)", num("surfaces")),
        ),
        "surfaces_pending" => (
            parsentry(),
            format!(
                "{} surfaces pending, {} cached",
                num("pending"),
                num("cached")
            ),
        ),
        "surface_completed" => (
            v.get("surface_id")?.as_str()?.to_string(),
            "surface completed".to_string(),
        ),
        "report_written" => (
            parsentry(),
            format!("report written ({} findings)", num("findings")),
        ),
        other => (parsentry(), other.to_string()),
    })
}

fn flush_scan_events(path: &Path, offset: &mut u64, use_colors: bool, timestamps: bool) {
    let Ok((lines, new_offset)) = read_lines_from(path, *offset) else {
        return;
    };
    *offset = new_offset;
    for line in &lines {
        if let Some((service, message)) = format_scan_event(line) {
            print_log(&service, &message, use_colors, timestamps, colors::DIM);
        }
    }
}

fn discover_new_surfaces(
    output_dir: &Path,
    known: &mut Vec<String>,
    color_map: &mut HashMap<String, &'static str>,
    use_colors: bool,
    timestamps: bool,
) {
    let entries = match std::fs::read_dir(output_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let known_set: HashSet<&str> = known.iter().map(|s| s.as_str()).collect();
    let mut new_surfaces = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Surface directories contain prompt.md
        if entry.path().is_dir() && entry.path().join("prompt.md").exists() {
            let surface_id = name_str.to_string();
            if !known_set.contains(surface_id.as_str()) {
                new_surfaces.push(surface_id);
            }
        }
    }

    new_surfaces.sort();
    for s in new_surfaces {
        let color_idx = known.len() % SURFACE_COLORS.len();
        color_map.insert(s.clone(), SURFACE_COLORS[color_idx]);
        let color = SURFACE_COLORS[color_idx];
        print_log(&s, "waiting", use_colors, timestamps, color);
        known.push(s);
    }
}

fn check_completed_surfaces(
    output_dir: &Path,
    surfaces: &[String],
    completed: &mut HashSet<String>,
    color_map: &HashMap<String, &str>,
    use_colors: bool,
    timestamps: bool,
) {
    for surface in surfaces {
        if !completed.contains(surface) && sarif_exists(output_dir, surface) {
            let findings = count_sarif_findings(output_dir, surface);
            completed.insert(surface.clone());
            let color = color_map.get(surface).unwrap_or(&colors::RESET);
            print_log(
                surface,
                &format!("✓ complete ({} findings)", findings),
                use_colors,
                timestamps,
                color,
            );
        }
    }
}

fn sarif_exists(output_dir: &Path, surface_id: &str) -> bool {
    output_dir
        .join(surface_id)
        .join("result.sarif.json")
        .exists()
}

fn count_sarif_findings(output_dir: &Path, surface_id: &str) -> usize {
    let path = output_dir.join(surface_id).join("result.sarif.json");
    let data = match std::fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return 0,
    };
    let value: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    value
        .get("runs")
        .and_then(|r| r.as_array())
        .and_then(|runs| runs.first())
        .and_then(|run| run.get("results"))
        .and_then(|r| r.as_array())
        .map(|results| results.len())
        .unwrap_or(0)
}

fn print_log(service: &str, message: &str, use_colors: bool, show_timestamps: bool, color: &str) {
    let ts = if show_timestamps {
        let now = chrono::Local::now();
        format!("[{}] ", now.format("%H:%M:%S"))
    } else {
        String::new()
    };

    for line in message.lines() {
        if use_colors {
            eprintln!(
                "{}{}{:<14}{} {}|{} {}{}",
                color,
                colors::BOLD,
                service,
                colors::RESET,
                colors::DIM,
                colors::RESET,
                ts,
                line,
            );
        } else {
            eprintln!("{:<14} | {}{}", service, ts, line);
        }
    }

    // Empty message still prints one line
    if message.is_empty() {
        if use_colors {
            eprintln!(
                "{}{}{:<14}{} {}|{} {}",
                color,
                colors::BOLD,
                service,
                colors::RESET,
                colors::DIM,
                colors::RESET,
                ts.trim_end(),
            );
        } else {
            eprintln!("{:<14} | {}", service, ts.trim_end());
        }
    }
}

fn print_summary(
    surfaces: &[String],
    completed: &HashSet<String>,
    elapsed: Duration,
    use_colors: bool,
    timestamps: bool,
) {
    let elapsed_str = if elapsed.as_secs() >= 60 {
        format!("{}m {}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
    } else {
        format!("{}s", elapsed.as_secs())
    };

    let total = if surfaces.is_empty() {
        "?".to_string()
    } else {
        surfaces.len().to_string()
    };
    let msg = format!(
        "{}/{} surfaces complete (elapsed {})",
        completed.len(),
        total,
        elapsed_str
    );
    print_log("parsentry", &msg, use_colors, timestamps, colors::BOLD);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::events::{ScanEvent, append_event, events_path};

    #[test]
    fn reads_only_new_complete_lines() {
        let dir = tempfile::tempdir().unwrap();
        append_event(dir.path(), &ScanEvent::ModelPrompted).unwrap();
        let path = events_path(dir.path());
        let (lines, off) = read_lines_from(&path, 0).unwrap();
        assert_eq!(lines.len(), 1);

        append_event(
            dir.path(),
            &ScanEvent::SurfaceCompleted {
                surface_id: "SURFACE-1".into(),
            },
        )
        .unwrap();
        // Partial trailing line is not consumed yet.
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        std::io::Write::write_all(&mut f, b"{\"event\":").unwrap();
        let (lines, off2) = read_lines_from(&path, off).unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(
            format_scan_event(&lines[0]),
            Some(("SURFACE-1".to_string(), "surface completed".to_string()))
        );
        let (lines, _) = read_lines_from(&path, off2).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn formats_scan_events() {
        let (svc, msg) =
            format_scan_event(r#"{"event":"surfaces_pending","pending":2,"cached":1}"#).unwrap();
        assert_eq!(svc, "parsentry");
        assert_eq!(msg, "2 surfaces pending, 1 cached");
        assert!(format_scan_event("not json").is_none());
    }
}
