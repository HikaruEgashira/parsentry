use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
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

/// Interval for session discovery (heavier operation)
const SESSION_POLL_SECS: u64 = 10;

pub async fn run_watch_command(
    output_dir: &Path,
    follow: bool,
    _tail: Option<usize>,
    timestamps: bool,
    _interval_secs: u64,
    timeout_secs: Option<u64>,
    no_color: bool,
) -> Result<()> {
    let use_colors = !no_color && colors_enabled();
    let start = Instant::now();

    if !follow && !output_dir.exists() {
        anyhow::bail!("Output directory does not exist: {}", output_dir.display());
    }

    let cwd = std::env::current_dir()?;
    let project_dir = parsentry_claude::project_sessions_dir(&cwd)?;

    print_log(
        "parsentry",
        &format!("watching {}", output_dir.display()),
        use_colors,
        timestamps,
        colors::BOLD,
    );

    // State
    let mut known_surfaces: Vec<String> = Vec::new();
    let mut surface_colors_map: HashMap<String, &str> = HashMap::new();
    let mut completed: HashSet<String> = HashSet::new();
    let mut offsets: HashMap<PathBuf, u64> = HashMap::new();
    let mut session_jsonls: Vec<(String, PathBuf)> = Vec::new();
    let mut last_session_count: Option<usize> = None;
    let mut dir_existed = output_dir.exists();

    // Initial discovery
    if dir_existed {
        discover_new_surfaces(output_dir, &mut known_surfaces, &mut surface_colors_map, use_colors, timestamps);
        check_completed_surfaces(output_dir, &known_surfaces, &mut completed, &surface_colors_map, use_colors, timestamps);
    }

    if !follow {
        if known_surfaces.is_empty() {
            print_log("parsentry", "no surfaces found yet", use_colors, timestamps, colors::DIM);
        }
        print_summary(&known_surfaces, &completed, start.elapsed(), use_colors, timestamps);
        return Ok(());
    }

    if !known_surfaces.is_empty() && completed.len() == known_surfaces.len() {
        print_summary(&known_surfaces, &completed, start.elapsed(), use_colors, timestamps);
        return Ok(());
    }

    // Set up filesystem watcher
    let (fs_tx, fs_rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(fs_tx)?;

    // Watch output directory (for prompt.md and sarif.json)
    if dir_existed {
        watcher.watch(output_dir, RecursiveMode::NonRecursive)?;
    }
    // Watch project sessions directory (for JSONL changes)
    if project_dir.exists() {
        watcher.watch(&project_dir, RecursiveMode::Recursive)?;
    }

    let mut last_session_poll = Instant::now();
    // Do initial session discovery
    poll_sessions(
        &project_dir, &mut session_jsonls, &mut last_session_count,
        use_colors, timestamps,
    );
    // Read initial events from already-tracked JSONL files
    flush_jsonl_events(
        &session_jsonls, &mut offsets, &surface_colors_map,
        use_colors, timestamps,
    );

    loop {
        // Check timeout
        if let Some(timeout) = timeout_secs {
            if start.elapsed().as_secs() >= timeout {
                print_log("parsentry", &format!("timeout after {}s", timeout), use_colors, timestamps, colors::BRIGHT_RED);
                print_summary(&known_surfaces, &completed, start.elapsed(), use_colors, timestamps);
                std::process::exit(1);
            }
        }

        // Wait for fs events with a short timeout so we can do periodic session polls
        let recv_timeout = Duration::from_millis(500);
        let mut got_event = false;

        // Drain all pending events
        loop {
            match fs_rx.recv_timeout(if got_event { Duration::from_millis(50) } else { recv_timeout }) {
                Ok(Ok(event)) => {
                    got_event = true;
                    // If output dir just appeared, start watching it
                    if !dir_existed && output_dir.exists() {
                        dir_existed = true;
                        let _ = watcher.watch(output_dir, RecursiveMode::NonRecursive);
                        print_log("parsentry", &format!("directory appeared: {}", output_dir.display()), use_colors, timestamps, colors::BRIGHT_GREEN);
                    }
                    // If project dir appeared, watch it
                    if project_dir.exists() {
                        let _ = watcher.watch(&project_dir, RecursiveMode::Recursive);
                    }

                    // Process based on which paths changed
                    for path in &event.paths {
                        let is_jsonl = path.extension().and_then(|e| e.to_str()) == Some("jsonl");
                        let is_in_output = path.starts_with(output_dir);

                        if is_jsonl && matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            // JSONL changed — read new events immediately
                            if let Some((surface_id, _)) = session_jsonls.iter().find(|(_, p)| p == path) {
                                let surface_id = surface_id.clone();
                                let offset = offsets.get(path).copied().unwrap_or(0);
                                if let Ok((events, new_offset)) = parsentry_claude::read_events_from(path, offset) {
                                    offsets.insert(path.clone(), new_offset);
                                    let color = surface_colors_map.get(&surface_id).unwrap_or(&colors::RESET);
                                    for ev in &events {
                                        print_event(&surface_id, ev, use_colors, timestamps, color);
                                    }
                                }
                            }
                        }

                        if is_in_output && matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                            // New file in output dir — check for new surfaces or completions
                            discover_new_surfaces(output_dir, &mut known_surfaces, &mut surface_colors_map, use_colors, timestamps);
                            check_completed_surfaces(output_dir, &known_surfaces, &mut completed, &surface_colors_map, use_colors, timestamps);
                        }
                    }
                }
                Ok(Err(_)) => { got_event = true; }
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    anyhow::bail!("filesystem watcher disconnected");
                }
            }
        }

        // Periodic session discovery (new sessions, new subagents)
        if last_session_poll.elapsed() >= Duration::from_secs(SESSION_POLL_SECS) {
            last_session_poll = Instant::now();

            if !dir_existed && output_dir.exists() {
                dir_existed = true;
                let _ = watcher.watch(output_dir, RecursiveMode::NonRecursive);
                print_log("parsentry", &format!("directory appeared: {}", output_dir.display()), use_colors, timestamps, colors::BRIGHT_GREEN);
            }

            if dir_existed {
                discover_new_surfaces(output_dir, &mut known_surfaces, &mut surface_colors_map, use_colors, timestamps);
            }

            poll_sessions(
                &project_dir, &mut session_jsonls, &mut last_session_count,
                use_colors, timestamps,
            );

            // Read any new events from newly discovered JSONL files
            flush_jsonl_events(
                &session_jsonls, &mut offsets, &surface_colors_map,
                use_colors, timestamps,
            );

            check_completed_surfaces(output_dir, &known_surfaces, &mut completed, &surface_colors_map, use_colors, timestamps);
        }

        // All done?
        if !known_surfaces.is_empty() && completed.len() == known_surfaces.len() {
            print_summary(&known_surfaces, &completed, start.elapsed(), use_colors, timestamps);
            return Ok(());
        }
    }
}

fn poll_sessions(
    project_dir: &Path,
    session_jsonls: &mut Vec<(String, PathBuf)>,
    last_session_count: &mut Option<usize>,
    use_colors: bool,
    timestamps: bool,
) {
    if let Ok(active_sessions) = parsentry_claude::find_active_project_sessions(project_dir) {
        let count = active_sessions.len();
        if *last_session_count != Some(count) {
            print_log("parsentry", &format!("active sessions: {}", count), use_colors, timestamps, colors::DIM);
            *last_session_count = Some(count);
        }

        for session_id in &active_sessions {
            let jsonl_path = project_dir.join(format!("{}.jsonl", session_id));
            if !session_jsonls.iter().any(|(_, p)| p == &jsonl_path) {
                if let Some(surface_id) = parsentry_claude::extract_surface_id(&jsonl_path) {
                    session_jsonls.push((surface_id, jsonl_path.clone()));
                }
            }

            if let Ok(subagents) = parsentry_claude::list_subagents(project_dir, session_id) {
                for agent in subagents {
                    if !session_jsonls.iter().any(|(_, p)| p == &agent.jsonl_path) {
                        let surface_id = extract_surface_from_description(&agent.description)
                            .or_else(|| parsentry_claude::extract_surface_id(&agent.jsonl_path));
                        if let Some(sid) = surface_id {
                            session_jsonls.push((sid, agent.jsonl_path.clone()));
                        }
                    }
                }
            }
        }
    }
}

fn flush_jsonl_events(
    session_jsonls: &[(String, PathBuf)],
    offsets: &mut HashMap<PathBuf, u64>,
    surface_colors_map: &HashMap<String, &str>,
    use_colors: bool,
    timestamps: bool,
) {
    for (surface_id, jsonl_path) in session_jsonls {
        let offset = offsets.get(jsonl_path).copied().unwrap_or(0);
        if let Ok((events, new_offset)) = parsentry_claude::read_events_from(jsonl_path, offset) {
            offsets.insert(jsonl_path.clone(), new_offset);
            let color = surface_colors_map.get(surface_id).unwrap_or(&colors::RESET);
            for ev in &events {
                print_event(surface_id, ev, use_colors, timestamps, color);
            }
        }
    }
}

fn print_event(
    surface_id: &str,
    event: &parsentry_claude::SessionEvent,
    use_colors: bool,
    timestamps: bool,
    color: &str,
) {
    match event {
        parsentry_claude::SessionEvent::ToolUse { name, summary, .. } => {
            print_log(surface_id, &format!("{} {}", name, summary), use_colors, timestamps, color);
        }
        parsentry_claude::SessionEvent::Text { content, .. } => {
            print_log(surface_id, content, use_colors, timestamps, color);
        }
        parsentry_claude::SessionEvent::Complete => {}
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

        if name_str.ends_with(".prompt.md") && name_str != "orchestrator.prompt.md" {
            let surface_id = name_str.trim_end_matches(".prompt.md").to_string();
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
            print_log(surface, &format!("✓ complete ({} findings)", findings), use_colors, timestamps, color);
        }
    }
}

fn sarif_exists(output_dir: &Path, surface_id: &str) -> bool {
    output_dir.join(format!("{}.sarif.json", surface_id)).exists()
}

fn count_sarif_findings(output_dir: &Path, surface_id: &str) -> usize {
    let path = output_dir.join(format!("{}.sarif.json", surface_id));
    let data = match std::fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return 0,
    };
    let value: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    value.get("runs")
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
                "{}{}{:<14}{} {} {}",
                color, colors::BOLD, service, colors::RESET,
                format!("{}|{}", colors::DIM, colors::RESET),
                format!("{}{}", ts, line),
            );
        } else {
            eprintln!("{:<14} | {}{}", service, ts, line);
        }
    }

    // Empty message still prints one line
    if message.is_empty() {
        if use_colors {
            eprintln!(
                "{}{}{:<14}{} {} {}",
                color, colors::BOLD, service, colors::RESET,
                format!("{}|{}", colors::DIM, colors::RESET),
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

    let total = if surfaces.is_empty() { "?".to_string() } else { surfaces.len().to_string() };
    let msg = format!("{}/{} surfaces complete (elapsed {})", completed.len(), total, elapsed_str);
    print_log("parsentry", &msg, use_colors, timestamps, colors::BOLD);
}

fn extract_surface_from_description(desc: &str) -> Option<String> {
    let idx = desc.find("SURFACE-")?;
    let rest = &desc[idx..];
    let end = rest.find(|c: char| !c.is_ascii_alphanumeric() && c != '-').unwrap_or(rest.len());
    let surface_id = &rest[..end];
    if surface_id.len() > 8 { Some(surface_id.to_string()) } else { None }
}
