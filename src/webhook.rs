//! GitHub webhook receiver that enqueues incremental scans.
//!
//! `POST /webhooks/github` accepts `push` and `pull_request` events and pushes a
//! [`ScanRequest`] onto a queue. A worker drains the queue and runs
//! `parsentry scan <owner/repo>`; the cache keys make that run incremental.
//! `POST /api/scans` enqueues onto the same queue, and job lifecycle events are
//! broadcast on an [`EventBus`] that `GET /api/events` streams as SSE.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, watch};

pub const WEBHOOK_PATH: &str = "/webhooks/github";
pub const SCANS_PATH: &str = "/api/scans";
pub const EVENTS_PATH: &str = "/api/events";
const MAX_HEADER_BYTES: usize = 16 * 1024;
const MAX_BODY_BYTES: usize = 25 * 1024 * 1024;

/// A scan requested by a webhook delivery or `POST /api/scans`.
///
/// `repository` is the scan target (owner/repo or path). `id` is assigned by
/// [`enqueue`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanRequest {
    pub id: u64,
    pub event: String,
    pub repository: String,
    pub git_ref: Option<String>,
    pub sha: Option<String>,
    pub options: ScanOptions,
}

/// `parsentry scan` options forwarded by the worker.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanOptions {
    pub output: Option<String>,
    pub runtime: Option<String>,
    pub env: Option<String>,
}

impl ScanOptions {
    fn args(&self) -> Vec<String> {
        let mut args = Vec::new();
        for (flag, value) in [
            ("--output", &self.output),
            ("--runtime", &self.runtime),
            ("--env", &self.env),
        ] {
            if let Some(v) = value {
                args.push(format!("{flag}={v}"));
            }
        }
        args
    }
}

/// A job lifecycle event (`queued`, `started`, `log`, `finished`, `failed`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JobEvent {
    pub id: u64,
    pub event: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl JobEvent {
    fn new(id: u64, event: &'static str) -> Self {
        Self {
            id,
            event,
            target: None,
            line: None,
            error: None,
        }
    }
}

/// Broadcasts [`JobEvent`]s to `/api/events` subscribers.
#[derive(Debug, Clone)]
pub struct EventBus(broadcast::Sender<JobEvent>);

impl Default for EventBus {
    fn default() -> Self {
        Self(broadcast::channel(1024).0)
    }
}

impl EventBus {
    pub fn subscribe(&self) -> broadcast::Receiver<JobEvent> {
        self.0.subscribe()
    }

    pub fn emit(&self, event: JobEvent) {
        // No subscribers is fine.
        let _ = self.0.send(event);
    }
}

static NEXT_JOB_ID: AtomicU64 = AtomicU64::new(1);

/// Assign an id, push `scan` onto `queue`, and emit `queued`.
pub fn enqueue(
    mut scan: ScanRequest,
    queue: &mpsc::UnboundedSender<ScanRequest>,
    events: &EventBus,
) -> Result<u64> {
    let id = NEXT_JOB_ID.fetch_add(1, Ordering::Relaxed);
    scan.id = id;
    let target = scan.repository.clone();
    queue
        .send(scan)
        .map_err(|_| anyhow::anyhow!("scan queue unavailable"))?;
    events.emit(JobEvent {
        target: Some(target),
        ..JobEvent::new(id, "queued")
    });
    Ok(id)
}

#[derive(Deserialize)]
struct Repository {
    full_name: String,
}

#[derive(Deserialize)]
struct PushPayload {
    repository: Repository,
    #[serde(rename = "ref")]
    git_ref: Option<String>,
    after: Option<String>,
    #[serde(default)]
    deleted: bool,
}

#[derive(Deserialize)]
struct PullRequestHead {
    #[serde(rename = "ref")]
    git_ref: Option<String>,
    sha: Option<String>,
}

#[derive(Deserialize)]
struct PullRequest {
    head: PullRequestHead,
}

#[derive(Deserialize)]
struct PullRequestPayload {
    action: String,
    repository: Repository,
    pull_request: PullRequest,
}

/// Turn a GitHub event into a scan request. `Ok(None)` means the event is
/// valid but does not warrant a scan.
pub fn parse_event(event: &str, body: &[u8]) -> Result<Option<ScanRequest>> {
    match event {
        "push" => {
            let p: PushPayload = serde_json::from_slice(body).context("invalid push payload")?;
            if p.deleted {
                return Ok(None);
            }
            Ok(Some(ScanRequest {
                event: event.to_string(),
                repository: p.repository.full_name,
                git_ref: p.git_ref,
                sha: p.after,
                ..Default::default()
            }))
        }
        "pull_request" => {
            let p: PullRequestPayload =
                serde_json::from_slice(body).context("invalid pull_request payload")?;
            if !matches!(p.action.as_str(), "opened" | "synchronize" | "reopened") {
                return Ok(None);
            }
            Ok(Some(ScanRequest {
                event: event.to_string(),
                repository: p.repository.full_name,
                git_ref: p.pull_request.head.git_ref,
                sha: p.pull_request.head.sha,
                ..Default::default()
            }))
        }
        _ => Ok(None),
    }
}

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut block = [0u8; 64];
    if key.len() > 64 {
        block[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let ipad: Vec<u8> = block.iter().map(|b| b ^ 0x36).collect();
    let opad: Vec<u8> = block.iter().map(|b| b ^ 0x5c).collect();
    let inner = Sha256::new()
        .chain_update(&ipad)
        .chain_update(msg)
        .finalize();
    Sha256::new()
        .chain_update(&opad)
        .chain_update(inner)
        .finalize()
        .into()
}

/// Verify an `X-Hub-Signature-256` header (`sha256=<hex>`) in constant time.
pub fn verify_signature(secret: &str, body: &[u8], header: Option<&str>) -> bool {
    let Some(hex) = header.and_then(|h| h.strip_prefix("sha256=")) else {
        return false;
    };
    let expected: String = hmac_sha256(secret.as_bytes(), body)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    if hex.len() != expected.len() {
        return false;
    }
    hex.bytes()
        .zip(expected.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a.to_ascii_lowercase() ^ b))
        == 0
}

/// A parsed HTTP request.
#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// Route a request and return `(status, body)`.
pub fn handle(
    req: &Request,
    secret: Option<&str>,
    queue: &mpsc::UnboundedSender<ScanRequest>,
    events: &EventBus,
) -> (u16, String) {
    let path = req.path.split('?').next().unwrap_or("");
    if path == "/healthz" && req.method == "GET" {
        return (200, "ok".into());
    }
    if path != WEBHOOK_PATH {
        return (404, "not found".into());
    }
    if req.method != "POST" {
        return (405, "method not allowed".into());
    }
    if let Some(secret) = secret
        && !verify_signature(secret, &req.body, req.header("x-hub-signature-256"))
    {
        return (401, "invalid signature".into());
    }
    let Some(event) = req.header("x-github-event") else {
        return (400, "missing X-GitHub-Event".into());
    };
    match parse_event(event, &req.body) {
        Ok(Some(scan)) => {
            let repo = scan.repository.clone();
            match enqueue(scan, queue, events) {
                Ok(_) => (202, format!("scan enqueued for {repo}")),
                Err(_) => (503, "scan queue unavailable".into()),
            }
        }
        Ok(None) => (202, "ignored".into()),
        Err(e) => (400, e.to_string()),
    }
}

async fn read_request(stream: &mut TcpStream) -> Result<Request> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    let header_end = loop {
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break pos;
        }
        anyhow::ensure!(buf.len() <= MAX_HEADER_BYTES, "headers too large");
        let n = stream.read(&mut chunk).await?;
        anyhow::ensure!(n > 0, "connection closed");
        buf.extend_from_slice(&chunk[..n]);
    };
    let head = std::str::from_utf8(&buf[..header_end]).context("non-utf8 headers")?;
    let mut lines = head.split("\r\n");
    let mut start = lines.next().unwrap_or("").split_whitespace();
    let mut req = Request {
        method: start.next().unwrap_or("").to_string(),
        path: start.next().unwrap_or("").to_string(),
        ..Default::default()
    };
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            req.headers
                .push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let len: usize = req
        .header("content-length")
        .map(|v| v.parse())
        .transpose()
        .context("invalid content-length")?
        .unwrap_or(0);
    anyhow::ensure!(len <= MAX_BODY_BYTES, "body too large");
    let mut body = buf[header_end + 4..].to_vec();
    while body.len() < len {
        let n = stream.read(&mut chunk).await?;
        anyhow::ensure!(n > 0, "connection closed");
        body.extend_from_slice(&chunk[..n]);
    }
    body.truncate(len);
    req.body = body;
    Ok(req)
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        500 => "Internal Server Error",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Service Unavailable",
    }
}

/// Shared configuration for connections accepted by [`serve`].
#[derive(Debug, Clone, Default)]
pub struct ServeConfig {
    pub webhook_secret: Option<String>,
    /// Bearer token required on `/api/` when set.
    pub api_token: Option<String>,
    /// Cache root whose runs `/api/` exposes.
    pub cache_base: PathBuf,
    /// Job lifecycle events streamed on `/api/events`.
    pub events: EventBus,
}

/// Stream `events` as `text/event-stream` until the client disconnects.
async fn stream_events(
    mut stream: TcpStream,
    mut events: broadcast::Receiver<JobEvent>,
) -> Result<()> {
    stream
        .write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n: connected\n\n",
        )
        .await?;
    loop {
        let frame = match events.recv().await {
            Ok(ev) => format!(
                "event: {}\ndata: {}\n\n",
                ev.event,
                serde_json::to_string(&ev)?
            ),
            Err(broadcast::error::RecvError::Lagged(n)) => format!(": dropped {n} events\n\n"),
            Err(broadcast::error::RecvError::Closed) => break,
        };
        stream.write_all(frame.as_bytes()).await?;
    }
    stream.shutdown().await?;
    Ok(())
}

async fn serve_conn(
    mut stream: TcpStream,
    config: Arc<ServeConfig>,
    queue: mpsc::UnboundedSender<ScanRequest>,
) -> Result<()> {
    let mut ctype = "text/plain";
    let mut extra = "";
    let (status, body) = match read_request(&mut stream).await {
        Ok(req) if req.path.starts_with(crate::api::API_PREFIX) => {
            ctype = "application/json";
            let token = config.api_token.as_deref();
            let path = req.path.split('?').next().unwrap_or("");
            let (status, body) = if !crate::api::authorized(&req, token) {
                crate::api::handle(&req, token, &config.cache_base)
            } else if path == EVENTS_PATH && req.method == "GET" {
                return stream_events(stream, config.events.subscribe()).await;
            } else if path == SCANS_PATH {
                crate::api::enqueue_scan(&req, &queue, &config.events)
            } else {
                crate::api::handle(&req, token, &config.cache_base)
            };
            (status, body.into_bytes())
        }
        Ok(req) => match (req.method == "GET")
            .then(|| crate::ui::asset(&req.path))
            .flatten()
        {
            Some((asset_type, data)) => {
                ctype = asset_type;
                extra = "X-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\n";
                (200, data)
            }
            None => {
                let (status, body) = handle(
                    &req,
                    config.webhook_secret.as_deref(),
                    &queue,
                    &config.events,
                );
                (status, body.into_bytes())
            }
        },
        Err(e) => (400, e.to_string().into_bytes()),
    };
    let auth = if status == 401 && ctype == "application/json" {
        "WWW-Authenticate: Bearer\r\n"
    } else {
        ""
    };
    let csp = if extra.is_empty() {
        String::new()
    } else {
        format!("Content-Security-Policy: {}\r\n", crate::ui::CSP)
    };
    let head = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: {ctype}\r\n{auth}{extra}{csp}Content-Length: {}\r\nConnection: close\r\n\r\n",
        reason(status),
        body.len()
    );
    stream.write_all(head.as_bytes()).await?;
    stream.write_all(&body).await?;
    stream.shutdown().await?;
    Ok(())
}

/// Process-wide shutdown signal observed by `serve`, set by [`shutdown`].
static SHUTDOWN: std::sync::LazyLock<watch::Sender<bool>> =
    std::sync::LazyLock::new(|| watch::Sender::new(false));

/// Signal running servers and workers to stop, then wait until they have closed
/// their listeners and terminated their scan processes.
pub async fn shutdown() {
    SHUTDOWN.send_replace(true);
    SHUTDOWN.closed().await;
}

async fn stopped(shutdown: &mut watch::Receiver<bool>) {
    if shutdown.wait_for(|s| *s).await.is_err() {
        std::future::pending::<()>().await;
    }
}

/// Accept connections forever, enqueueing scans onto `queue`.
pub async fn serve(
    listener: TcpListener,
    config: ServeConfig,
    queue: mpsc::UnboundedSender<ScanRequest>,
) -> Result<()> {
    let (_tx, rx) = watch::channel(false);
    serve_until(listener, config, queue, rx).await
}

/// [`serve`] until `shutdown` becomes true, then close the listener and return.
pub async fn serve_until(
    listener: TcpListener,
    config: ServeConfig,
    queue: mpsc::UnboundedSender<ScanRequest>,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    let config = Arc::new(config);
    loop {
        let (stream, _) = tokio::select! {
            r = listener.accept() => r?,
            _ = stopped(&mut shutdown) => return Ok(()),
        };
        let config = config.clone();
        let queue = queue.clone();
        tokio::spawn(async move {
            if let Err(e) = serve_conn(stream, config, queue).await {
                log::warn!("webhook connection error: {e}");
            }
        });
    }
}

/// Drain the queue, running `parsentry scan <target>` for each request in order.
pub async fn run_worker(
    queue: mpsc::UnboundedReceiver<ScanRequest>,
    events: EventBus,
) -> Result<()> {
    run_worker_with(std::env::current_exe()?, queue, events).await
}

/// [`run_worker`] with an explicit binary, emitting lifecycle events on `events`.
pub async fn run_worker_with(
    bin: PathBuf,
    queue: mpsc::UnboundedReceiver<ScanRequest>,
    events: EventBus,
) -> Result<()> {
    let (_tx, rx) = watch::channel(false);
    run_worker_until(bin, queue, events, rx).await
}

/// [`run_worker_with`] until `shutdown` becomes true: stop dequeuing and kill
/// the running scan process.
pub async fn run_worker_until(
    bin: PathBuf,
    mut queue: mpsc::UnboundedReceiver<ScanRequest>,
    events: EventBus,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    use tokio::io::AsyncBufReadExt;
    loop {
        let scan = tokio::select! {
            biased;
            _ = stopped(&mut shutdown) => return Ok(()),
            scan = queue.recv() => match scan {
                Some(scan) => scan,
                None => return Ok(()),
            },
        };
        eprintln!(
            "Scanning {} ({} {})",
            scan.repository,
            scan.event,
            scan.sha.as_deref().unwrap_or("-")
        );
        events.emit(JobEvent {
            target: Some(scan.repository.clone()),
            ..JobEvent::new(scan.id, "started")
        });
        let child = tokio::process::Command::new(&bin)
            .arg("scan")
            .args(scan.options.args())
            .arg("--")
            .arg(&scan.repository)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn();
        let status = match child {
            Ok(mut child) => {
                let stderr = child.stderr.take();
                let run = async {
                    if let Some(stderr) = stderr {
                        let mut lines = tokio::io::BufReader::new(stderr).lines();
                        while let Ok(Some(line)) = lines.next_line().await {
                            eprintln!("{line}");
                            events.emit(JobEvent {
                                line: Some(line),
                                ..JobEvent::new(scan.id, "log")
                            });
                        }
                    }
                    child.wait().await
                };
                let interrupted = tokio::select! {
                    status = run => Some(status),
                    _ = stopped(&mut shutdown) => None,
                };
                match interrupted {
                    Some(status) => status,
                    None => {
                        let _ = child.kill().await;
                        events.emit(JobEvent {
                            error: Some("server shutting down".into()),
                            ..JobEvent::new(scan.id, "failed")
                        });
                        return Ok(());
                    }
                }
            }
            Err(e) => Err(e),
        };
        let error = match status {
            Ok(s) if s.success() => None,
            Ok(s) => Some(format!("scan of {} exited with {s}", scan.repository)),
            Err(e) => Some(format!("failed to run scan for {}: {e}", scan.repository)),
        };
        match error {
            None => events.emit(JobEvent::new(scan.id, "finished")),
            Some(error) => {
                eprintln!("{error}");
                events.emit(JobEvent {
                    error: Some(error),
                    ..JobEvent::new(scan.id, "failed")
                });
            }
        }
    }
}

/// Run the webhook + API server and scan worker until either stops.
///
/// A non-loopback `host` is refused unless `token` is set.
pub async fn run_serve_command(host: &str, port: u16, token: Option<String>) -> Result<()> {
    let token = token.filter(|t| !t.is_empty());
    crate::api::check_bind(host, token.as_deref())?;
    let secret = std::env::var("GITHUB_WEBHOOK_SECRET")
        .ok()
        .filter(|s| !s.is_empty());
    if secret.is_none() {
        eprintln!("warning: GITHUB_WEBHOOK_SECRET is unset; webhook signatures are not verified");
    }
    let listener = TcpListener::bind((host, port))
        .await
        .with_context(|| format!("failed to bind {host}:{port}"))?;
    let local = listener.local_addr()?;
    eprintln!("Listening on http://{local}{WEBHOOK_PATH} and http://{local}/api/");
    let config = ServeConfig {
        webhook_secret: secret,
        api_token: token,
        cache_base: crate::cli::commands::common::cache_base(),
        events: EventBus::default(),
    };
    let events = config.events.clone();
    let (tx, rx) = mpsc::unbounded_channel();
    let bin = std::env::current_exe()?;
    tokio::select! {
        r = serve_until(listener, config, tx, SHUTDOWN.subscribe()) => r,
        r = run_worker_until(bin, rx, events, SHUTDOWN.subscribe()) => r,
    }
}
