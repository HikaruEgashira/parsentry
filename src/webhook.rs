//! GitHub webhook receiver that enqueues incremental scans.
//!
//! `POST /webhooks/github` accepts `push` and `pull_request` events and pushes a
//! [`ScanRequest`] onto a queue. A worker drains the queue and runs
//! `parsentry scan <owner/repo>`; the cache keys make that run incremental.

use anyhow::{Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub const WEBHOOK_PATH: &str = "/webhooks/github";
const MAX_HEADER_BYTES: usize = 16 * 1024;
const MAX_BODY_BYTES: usize = 25 * 1024 * 1024;

/// An incremental scan requested by a webhook delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanRequest {
    pub event: String,
    pub repository: String,
    pub git_ref: Option<String>,
    pub sha: Option<String>,
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
            match queue.send(scan) {
                Ok(()) => (202, format!("scan enqueued for {repo}")),
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
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Service Unavailable",
    }
}

async fn serve_conn(
    mut stream: TcpStream,
    secret: Option<String>,
    queue: mpsc::UnboundedSender<ScanRequest>,
) -> Result<()> {
    let (status, body) = match read_request(&mut stream).await {
        Ok(req) => handle(&req, secret.as_deref(), &queue),
        Err(e) => (400, e.to_string()),
    };
    let resp = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        reason(status),
        body.len()
    );
    stream.write_all(resp.as_bytes()).await?;
    stream.shutdown().await?;
    Ok(())
}

/// Accept connections forever, enqueueing scans onto `queue`.
pub async fn serve(
    listener: TcpListener,
    secret: Option<String>,
    queue: mpsc::UnboundedSender<ScanRequest>,
) -> Result<()> {
    loop {
        let (stream, _) = listener.accept().await?;
        let secret = secret.clone();
        let queue = queue.clone();
        tokio::spawn(async move {
            if let Err(e) = serve_conn(stream, secret, queue).await {
                log::warn!("webhook connection error: {e}");
            }
        });
    }
}

/// Drain the queue, running `parsentry scan <repo>` for each request in order.
pub async fn run_worker(mut queue: mpsc::UnboundedReceiver<ScanRequest>) -> Result<()> {
    let bin = std::env::current_exe()?;
    while let Some(scan) = queue.recv().await {
        eprintln!(
            "Scanning {} ({} {})",
            scan.repository,
            scan.event,
            scan.sha.as_deref().unwrap_or("-")
        );
        let status = tokio::process::Command::new(&bin)
            .arg("scan")
            .arg(&scan.repository)
            .stdout(std::process::Stdio::null())
            .status()
            .await;
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => eprintln!("scan of {} exited with {s}", scan.repository),
            Err(e) => eprintln!("failed to run scan for {}: {e}", scan.repository),
        }
    }
    Ok(())
}

/// Run the webhook server and scan worker until either stops.
pub async fn run_serve_command(addr: &str) -> Result<()> {
    let secret = std::env::var("GITHUB_WEBHOOK_SECRET")
        .ok()
        .filter(|s| !s.is_empty());
    if secret.is_none() {
        eprintln!("warning: GITHUB_WEBHOOK_SECRET is unset; webhook signatures are not verified");
    }
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;
    eprintln!(
        "Listening on http://{}{WEBHOOK_PATH}",
        listener.local_addr()?
    );
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::select! {
        r = serve(listener, secret, tx) => r,
        r = run_worker(rx) => r,
    }
}
