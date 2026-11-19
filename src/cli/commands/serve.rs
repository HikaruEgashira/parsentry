//! `parsentry serve` — HTTP API that queues scans and drives them to completion.
//!
//! The scan index, queue and event log live in `serve.db` (SQLite) under the
//! cache base directory, next to the per-target caches. A single worker pulls
//! queued scans and loops `parsentry scan <target>`, piping each emitted
//! prompt to the configured agent command until the scan prints
//! `PARSENTRY_SCAN_COMPLETE`.
//!
//! Endpoints:
//! - `POST /scans` `{"target": "..."}` → `201 {"id", "status", ...}`
//! - `GET /scans/:id` → scan record
//! - `GET /scans/:id/events` → Server-Sent Events of the scan's event log
//! - `GET /scans/:id/findings` → merged SARIF once the scan is done

use anyhow::{Context, Result};
use axum::extract::{Path as AxPath, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::stream::Stream;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::Notify;

use crate::prompt::SCAN_COMPLETE_MARKER;

use super::common::{cache_base, cache_dir_for};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    Queued,
    Running,
    Done,
    Failed,
}

impl ScanStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Done => "done",
            Self::Failed => "failed",
        }
    }
    fn is_terminal(self) -> bool {
        matches!(self, Self::Done | Self::Failed)
    }
}

impl std::str::FromStr for ScanStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "queued" => Self::Queued,
            "running" => Self::Running,
            "done" => Self::Done,
            "failed" => Self::Failed,
            other => anyhow::bail!("unknown scan status {other}"),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanRecord {
    pub id: i64,
    pub target: String,
    pub status: &'static str,
    pub iterations: i64,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanEvent {
    pub id: i64,
    pub kind: String,
    pub message: String,
    pub created_at: String,
}

/// SQLite-backed scan index, queue and event log.
#[derive(Clone)]
pub struct Store {
    conn: Arc<Mutex<Connection>>,
}

fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let conn =
            Connection::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        Self::init(conn)
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             CREATE TABLE IF NOT EXISTS scans (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 target TEXT NOT NULL,
                 status TEXT NOT NULL,
                 iterations INTEGER NOT NULL DEFAULT 0,
                 error TEXT,
                 created_at TEXT NOT NULL,
                 updated_at TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS events (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 scan_id INTEGER NOT NULL REFERENCES scans(id),
                 kind TEXT NOT NULL,
                 message TEXT NOT NULL,
                 created_at TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS events_scan ON events(scan_id, id);",
        )?;
        // Scans interrupted by a previous shutdown go back to the queue.
        conn.execute(
            "UPDATE scans SET status = 'queued' WHERE status = 'running'",
            [],
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn enqueue(&self, target: &str) -> Result<ScanRecord> {
        let id = {
            let conn = self.conn();
            let ts = now();
            conn.execute(
                "INSERT INTO scans (target, status, created_at, updated_at) VALUES (?1, 'queued', ?2, ?2)",
                params![target, ts],
            )?;
            conn.last_insert_rowid()
        };
        self.add_event(id, "queued", target)?;
        self.get(id)?.context("scan vanished after insert")
    }

    pub fn get(&self, id: i64) -> Result<Option<ScanRecord>> {
        let conn = self.conn();
        let row = conn
            .query_row(
                "SELECT id, target, status, iterations, error, created_at, updated_at FROM scans WHERE id = ?1",
                params![id],
                |r| {
                    Ok((
                        r.get::<_, i64>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, i64>(3)?,
                        r.get::<_, Option<String>>(4)?,
                        r.get::<_, String>(5)?,
                        r.get::<_, String>(6)?,
                    ))
                },
            )
            .optional()?;
        row.map(
            |(id, target, status, iterations, error, created_at, updated_at)| {
                Ok(ScanRecord {
                    id,
                    target,
                    status: status.parse::<ScanStatus>()?.as_str(),
                    iterations,
                    error,
                    created_at,
                    updated_at,
                })
            },
        )
        .transpose()
    }

    /// Atomically take the oldest queued scan and mark it running.
    pub fn claim_next(&self) -> Result<Option<(i64, String)>> {
        let conn = self.conn();
        let next = conn
            .query_row(
                "SELECT id, target FROM scans WHERE status = 'queued' ORDER BY id LIMIT 1",
                [],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()?;
        if let Some((id, _)) = &next {
            conn.execute(
                "UPDATE scans SET status = 'running', updated_at = ?2 WHERE id = ?1",
                params![id, now()],
            )?;
        }
        Ok(next)
    }

    pub fn set_status(&self, id: i64, status: ScanStatus, error: Option<&str>) -> Result<()> {
        self.conn().execute(
            "UPDATE scans SET status = ?2, error = ?3, updated_at = ?4 WHERE id = ?1",
            params![id, status.as_str(), error, now()],
        )?;
        self.add_event(id, status.as_str(), error.unwrap_or(""))
    }

    fn bump_iterations(&self, id: i64) -> Result<()> {
        self.conn().execute(
            "UPDATE scans SET iterations = iterations + 1, updated_at = ?2 WHERE id = ?1",
            params![id, now()],
        )?;
        Ok(())
    }

    pub fn add_event(&self, scan_id: i64, kind: &str, message: &str) -> Result<()> {
        self.conn().execute(
            "INSERT INTO events (scan_id, kind, message, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![scan_id, kind, message, now()],
        )?;
        Ok(())
    }

    pub fn events_after(&self, scan_id: i64, after: i64) -> Result<Vec<ScanEvent>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, kind, message, created_at FROM events WHERE scan_id = ?1 AND id > ?2 ORDER BY id",
        )?;
        let rows = stmt.query_map(params![scan_id, after], |r| {
            Ok(ScanEvent {
                id: r.get(0)?,
                kind: r.get(1)?,
                message: r.get(2)?,
                created_at: r.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

/// How the worker drives a scan.
#[derive(Clone, Debug)]
pub struct WorkerConfig {
    /// `parsentry` binary used for each `scan` phase.
    pub parsentry_bin: PathBuf,
    /// Shell command that receives each phase prompt on stdin.
    pub agent: String,
    /// Upper bound on scan phases before the scan is marked failed.
    pub max_iterations: u32,
}

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub wake: Arc<Notify>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/scans", post(create_scan))
        .route("/scans/{id}", get(get_scan))
        .route("/scans/{id}/events", get(scan_events))
        .route("/scans/{id}/findings", get(scan_findings))
        .with_state(state)
}

#[derive(Deserialize)]
struct CreateScan {
    target: String,
}

fn error(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(serde_json::json!({ "error": msg.into() }))).into_response()
}

fn internal(e: anyhow::Error) -> Response {
    error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

async fn create_scan(State(state): State<AppState>, Json(req): Json<CreateScan>) -> Response {
    let target = req.target.trim();
    if target.is_empty() {
        return error(StatusCode::BAD_REQUEST, "target must not be empty");
    }
    match state.store.enqueue(target) {
        Ok(rec) => {
            state.wake.notify_one();
            (StatusCode::CREATED, Json(rec)).into_response()
        }
        Err(e) => internal(e),
    }
}

async fn get_scan(State(state): State<AppState>, AxPath(id): AxPath<i64>) -> Response {
    match state.store.get(id) {
        Ok(Some(rec)) => Json(rec).into_response(),
        Ok(None) => error(StatusCode::NOT_FOUND, "scan not found"),
        Err(e) => internal(e),
    }
}

async fn scan_findings(State(state): State<AppState>, AxPath(id): AxPath<i64>) -> Response {
    let rec = match state.store.get(id) {
        Ok(Some(rec)) => rec,
        Ok(None) => return error(StatusCode::NOT_FOUND, "scan not found"),
        Err(e) => return internal(e),
    };
    if rec.status != ScanStatus::Done.as_str() {
        return error(
            StatusCode::CONFLICT,
            format!("scan is {}, findings are not available", rec.status),
        );
    }
    let merged = cache_dir_for(&rec.target).join("merged.sarif.json");
    match std::fs::read_to_string(&merged)
        .map_err(anyhow::Error::from)
        .and_then(|s| Ok(serde_json::from_str::<serde_json::Value>(&s)?))
    {
        Ok(v) => Json(v).into_response(),
        Err(e) => internal(e.context(format!("failed to read {}", merged.display()))),
    }
}

async fn scan_events(State(state): State<AppState>, AxPath(id): AxPath<i64>) -> Response {
    match state.store.get(id) {
        Ok(Some(_)) => {}
        Ok(None) => return error(StatusCode::NOT_FOUND, "scan not found"),
        Err(e) => return internal(e),
    }
    Sse::new(event_stream(state.store, id))
        .keep_alive(KeepAlive::default())
        .into_response()
}

/// Replays the scan's event log, then follows it until the scan finishes.
fn event_stream(store: Store, id: i64) -> impl Stream<Item = Result<Event, Infallible>> {
    futures_util::stream::unfold(
        (store, 0i64, false),
        move |(store, mut last, mut finished)| async move {
            loop {
                if finished {
                    return None;
                }
                // Read status before events so the terminal event is never missed.
                let terminal = store
                    .get(id)
                    .ok()
                    .flatten()
                    .and_then(|r| r.status.parse::<ScanStatus>().ok())
                    .is_none_or(ScanStatus::is_terminal);
                let events = store.events_after(id, last).unwrap_or_default();
                if !events.is_empty() {
                    last = events.last().map(|e| e.id).unwrap_or(last);
                    finished = terminal;
                    let batch: Vec<_> = events
                        .into_iter()
                        .map(|e| {
                            Ok(Event::default()
                                .id(e.id.to_string())
                                .event(e.kind.clone())
                                .data(e.message))
                        })
                        .collect();
                    return Some((futures_util::stream::iter(batch), (store, last, finished)));
                }
                if terminal {
                    return None;
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        },
    )
    .flat_map(|s| s)
}

use futures_util::StreamExt;

/// Run queued scans one at a time, forever.
pub async fn run_worker(store: Store, wake: Arc<Notify>, config: WorkerConfig) {
    loop {
        match store.claim_next() {
            Ok(Some((id, target))) => {
                let result = drive_scan(&store, id, &target, &config).await;
                let outcome = match result {
                    Ok(()) => store.set_status(id, ScanStatus::Done, None),
                    Err(e) => store.set_status(id, ScanStatus::Failed, Some(&format!("{e:#}"))),
                };
                if let Err(e) = outcome {
                    eprintln!("serve: failed to record scan {id} outcome: {e:#}");
                }
            }
            Ok(None) => {
                let _ = tokio::time::timeout(Duration::from_secs(5), wake.notified()).await;
            }
            Err(e) => {
                eprintln!("serve: queue error: {e:#}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Loop `parsentry scan` → agent until the scan reports completion.
async fn drive_scan(store: &Store, id: i64, target: &str, config: &WorkerConfig) -> Result<()> {
    for _ in 0..config.max_iterations {
        store.bump_iterations(id)?;
        let out = Command::new(&config.parsentry_bin)
            .arg("scan")
            .arg(target)
            .stdin(Stdio::null())
            .output()
            .await
            .context("failed to run parsentry scan")?;
        let stderr = String::from_utf8_lossy(&out.stderr);
        for line in stderr.lines().filter(|l| !l.trim().is_empty()) {
            store.add_event(id, "progress", line)?;
        }
        if !out.status.success() {
            anyhow::bail!("parsentry scan exited with {}", out.status);
        }
        let prompt = String::from_utf8_lossy(&out.stdout).into_owned();
        if prompt.contains(SCAN_COMPLETE_MARKER) {
            store.add_event(id, "complete", prompt.trim())?;
            return Ok(());
        }
        store.add_event(id, "prompt", &format!("{} bytes → agent", prompt.len()))?;
        run_agent(&config.agent, &prompt).await?;
        store.add_event(id, "agent", "agent finished")?;
    }
    anyhow::bail!(
        "scan did not complete within {} iterations",
        config.max_iterations
    )
}

async fn run_agent(agent: &str, prompt: &str) -> Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(agent)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .with_context(|| format!("failed to spawn agent `{agent}`"))?;
    if let Some(mut stdin) = child.stdin.take() {
        // An agent may exit without reading all of its input.
        let _ = stdin.write_all(prompt.as_bytes()).await;
    }
    let status = child.wait().await?;
    if !status.success() {
        anyhow::bail!("agent `{agent}` exited with {status}");
    }
    Ok(())
}

pub async fn run_serve_command(listen: &str, agent: &str, max_iterations: u32) -> Result<()> {
    let db = cache_base().join("serve.db");
    let store = Store::open(&db)?;
    let wake = Arc::new(Notify::new());
    let config = WorkerConfig {
        parsentry_bin: std::env::current_exe()?,
        agent: agent.to_string(),
        max_iterations,
    };
    tokio::spawn(run_worker(store.clone(), wake.clone(), config));

    let listener = tokio::net::TcpListener::bind(listen)
        .await
        .with_context(|| format!("failed to bind {listen}"))?;
    eprintln!(
        "parsentry serve listening on http://{} (db: {})",
        listener.local_addr()?,
        db.display()
    );
    axum::serve(listener, router(AppState { store, wake })).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    async fn call(app: &Router, req: Request<Body>) -> (StatusCode, String) {
        let res = app.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, String::from_utf8_lossy(&body).into_owned())
    }

    fn post_scan(target: &str) -> Request<Body> {
        Request::post("/scans")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({ "target": target }).to_string(),
            ))
            .unwrap()
    }

    #[test]
    fn queue_claims_in_order_and_requeues_running_on_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("serve.db");
        let store = Store::open(&db).unwrap();
        let a = store.enqueue("a").unwrap();
        store.enqueue("b").unwrap();
        assert_eq!(store.claim_next().unwrap(), Some((a.id, "a".into())));
        drop(store);
        let store = Store::open(&db).unwrap();
        assert_eq!(store.get(a.id).unwrap().unwrap().status, "queued");
        assert_eq!(store.claim_next().unwrap().unwrap().1, "a");
        assert_eq!(store.claim_next().unwrap().unwrap().1, "b");
        assert_eq!(store.claim_next().unwrap(), None);
    }

    #[tokio::test]
    async fn unknown_scan_is_404_and_empty_target_is_400() {
        let store = Store::open_in_memory().unwrap();
        let app = router(AppState {
            store,
            wake: Arc::new(Notify::new()),
        });
        for path in ["/scans/9", "/scans/9/events", "/scans/9/findings"] {
            let (status, _) = call(&app, Request::get(path).body(Body::empty()).unwrap()).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{path}");
        }
        let (status, _) = call(&app, post_scan(" ")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    /// POST /scans runs a scan to completion with a fake `parsentry` that
    /// needs one agent round-trip before it reports completion.
    #[cfg(unix)]
    #[tokio::test]
    async fn post_scan_runs_to_completion() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let state_file = dir.path().join("agent_ran");
        let bin = dir.path().join("fake-parsentry");
        std::fs::write(
            &bin,
            format!(
                "#!/bin/sh\necho 'Phase model' >&2\nif [ -f '{s}' ]; then echo '{m}'; else echo 'write the model'; fi\n",
                s = state_file.display(),
                m = SCAN_COMPLETE_MARKER
            ),
        )
        .unwrap();
        std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();

        let store = Store::open_in_memory().unwrap();
        let wake = Arc::new(Notify::new());
        let app = router(AppState {
            store: store.clone(),
            wake: wake.clone(),
        });
        tokio::spawn(run_worker(
            store.clone(),
            wake,
            WorkerConfig {
                parsentry_bin: bin,
                agent: format!("cat > /dev/null; touch '{}'", state_file.display()),
                max_iterations: 5,
            },
        ));

        let (status, body) = call(&app, post_scan("some/target")).await;
        assert_eq!(status, StatusCode::CREATED, "{body}");
        let id = serde_json::from_str::<serde_json::Value>(&body).unwrap()["id"]
            .as_i64()
            .unwrap();

        let mut rec = serde_json::Value::Null;
        for _ in 0..100 {
            let (_, body) = call(
                &app,
                Request::get(format!("/scans/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
            rec = serde_json::from_str(&body).unwrap();
            if rec["status"] == "done" || rec["status"] == "failed" {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        assert_eq!(rec["status"], "done", "{rec}");
        assert_eq!(rec["iterations"], 2);

        let (status, events) = call(
            &app,
            Request::get(format!("/scans/{id}/events"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        for kind in ["queued", "progress", "prompt", "agent", "complete", "done"] {
            assert!(
                events.contains(&format!("event: {kind}")),
                "{kind}: {events}"
            );
        }
    }

    #[tokio::test]
    async fn findings_conflict_until_done() {
        let store = Store::open_in_memory().unwrap();
        let rec = store.enqueue("x").unwrap();
        let app = router(AppState {
            store,
            wake: Arc::new(Notify::new()),
        });
        let (status, _) = call(
            &app,
            Request::get(format!("/scans/{}/findings", rec.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
    }
}
