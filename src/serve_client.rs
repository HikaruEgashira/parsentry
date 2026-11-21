//! Client for a running `parsentry serve`.
//!
//! `parsentry scan` and `parsentry log -f` probe the serve address
//! (`PARSENTRY_SERVE_URL`, default `http://127.0.0.1:8765`). When serve
//! answers, scans are enqueued through `POST /api/scans` and `log -f` streams
//! `GET /api/events`; otherwise both run in-process. Set
//! `PARSENTRY_SERVE_URL=off` to always run in-process (the serve worker does
//! this for the scans it spawns). `PARSENTRY_API_TOKEN` is sent as a bearer token.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::time::Duration;

use crate::webhook::{EVENTS_PATH, SCANS_PATH};

pub const SERVE_URL_ENV: &str = "PARSENTRY_SERVE_URL";
pub const DEFAULT_SERVE_URL: &str = "http://127.0.0.1:8765";
const PROBE_TIMEOUT: Duration = Duration::from_millis(500);

/// A reachable serve instance.
#[derive(Debug, Clone)]
pub struct ServeClient {
    base: String,
    token: Option<String>,
    http: reqwest::Client,
}

/// The configured serve base URL, or `None` when disabled.
pub fn configured_url() -> Option<String> {
    let url = std::env::var(SERVE_URL_ENV).unwrap_or_else(|_| DEFAULT_SERVE_URL.into());
    let url = url.trim().trim_end_matches('/');
    if url.is_empty() || url.eq_ignore_ascii_case("off") {
        None
    } else {
        Some(url.to_string())
    }
}

/// Probe the configured serve address.
pub async fn discover() -> Option<ServeClient> {
    let token = std::env::var("PARSENTRY_API_TOKEN")
        .ok()
        .filter(|t| !t.is_empty());
    ServeClient::probe(&configured_url()?, token).await
}

/// Make a local path absolute so serve resolves it like the caller would.
/// Non-existent values (URLs, `owner/repo`) pass through unless `is_file`.
fn absolutize(value: &str, is_file: bool) -> String {
    let path = Path::new(value);
    if let Ok(p) = path.canonicalize() {
        return p.to_string_lossy().into_owned();
    }
    if is_file
        && path.is_relative()
        && let Ok(cwd) = std::env::current_dir()
    {
        return cwd.join(path).to_string_lossy().into_owned();
    }
    value.to_string()
}

/// Render one `/api/events` JSON payload as a log line.
pub fn format_event(event: &serde_json::Value) -> String {
    let id = event.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let kind = event.get("event").and_then(|v| v.as_str()).unwrap_or("?");
    let detail = ["target", "line", "error"]
        .iter()
        .find_map(|k| event.get(*k).and_then(|v| v.as_str()))
        .unwrap_or("");
    format!("job {id} {kind} {detail}").trim_end().to_string()
}

impl ServeClient {
    /// Return a client when `GET /api/runs` on `base` succeeds.
    pub async fn probe(base: &str, token: Option<String>) -> Option<Self> {
        let http = reqwest::Client::builder()
            .connect_timeout(PROBE_TIMEOUT)
            .no_proxy()
            .build()
            .ok()?;
        let client = Self {
            base: base.trim_end_matches('/').to_string(),
            token,
            http,
        };
        let resp = client
            .request(reqwest::Method::GET, "/api/runs")
            .timeout(PROBE_TIMEOUT)
            .send()
            .await
            .ok()?;
        resp.status().is_success().then_some(client)
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let req = self.http.request(method, format!("{}{path}", self.base));
        match &self.token {
            Some(t) => req.bearer_auth(t),
            None => req,
        }
    }

    /// `POST /api/scans`; returns the job id.
    pub async fn enqueue_scan(
        &self,
        target: &str,
        output: Option<&str>,
        runtime: Option<&str>,
        env: Option<&str>,
    ) -> Result<u64> {
        let mut body = serde_json::json!({ "target": absolutize(target, false) });
        for (key, value) in [("output", output), ("runtime", runtime), ("env", env)] {
            if let Some(v) = value {
                let v = if key == "runtime" {
                    v.to_string()
                } else {
                    absolutize(v, true)
                };
                body[key] = serde_json::Value::String(v);
            }
        }
        let resp = self
            .request(reqwest::Method::POST, SCANS_PATH)
            .json(&body)
            .send()
            .await
            .context("POST /api/scans failed")?;
        let status = resp.status();
        let json: serde_json::Value = resp.json().await.unwrap_or_default();
        if !status.is_success() {
            let msg = json.get("error").and_then(|e| e.as_str()).unwrap_or("");
            bail!("serve rejected scan ({status}): {msg}");
        }
        json.get("id")
            .and_then(|v| v.as_u64())
            .context("serve response has no job id")
    }

    /// Stream `GET /api/events`, calling `on_event` with each JSON payload
    /// until the stream ends or `on_event` returns `false`.
    pub async fn follow_events(
        &self,
        mut on_event: impl FnMut(serde_json::Value) -> bool,
    ) -> Result<()> {
        let mut resp = self
            .request(reqwest::Method::GET, EVENTS_PATH)
            .send()
            .await
            .context("GET /api/events failed")?;
        if !resp.status().is_success() {
            bail!("serve rejected event stream ({})", resp.status());
        }
        let mut buf = String::new();
        while let Some(chunk) = resp.chunk().await? {
            buf.push_str(&String::from_utf8_lossy(&chunk));
            while let Some(end) = buf.find("\n\n") {
                let frame: String = buf.drain(..end + 2).collect();
                let data: Vec<&str> = frame
                    .lines()
                    .filter_map(|l| l.strip_prefix("data:"))
                    .map(str::trim_start)
                    .collect();
                if data.is_empty() {
                    continue;
                }
                if let Ok(v) = serde_json::from_str(&data.join("\n"))
                    && !on_event(v)
                {
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
