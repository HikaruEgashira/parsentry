//! `parsentry serve`: a local web UI for browsing scan runs and findings.
//!
//! The frontend lives in `tools/web/public` and is embedded with rust-embed.
//! The server binds to localhost by default; binding to any other address
//! requires `--token`, which every `/api/` request must present as a Bearer token.

use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use rust_embed::RustEmbed;
use serde::Serialize;
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::cli::commands::common::cache_base;

#[derive(RustEmbed)]
#[folder = "tools/web/public/"]
struct WebAssets;

const MAX_REQUEST_BYTES: usize = 16 * 1024;

#[derive(Debug, Serialize, PartialEq)]
pub struct RunInfo {
    pub id: String,
    pub completed: bool,
    pub findings: usize,
    pub modified: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Finding {
    pub rule_id: String,
    pub level: String,
    pub verdict: String,
    pub cwe: Vec<String>,
    pub message: String,
    pub location: Option<String>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Default)]
pub struct FindingFilter {
    pub verdict: Option<String>,
    pub level: Option<String>,
    pub cwe: Option<String>,
}

impl FindingFilter {
    fn matches(&self, f: &Finding) -> bool {
        self.verdict
            .as_ref()
            .is_none_or(|v| f.verdict.eq_ignore_ascii_case(v))
            && self
                .level
                .as_ref()
                .is_none_or(|l| f.level.eq_ignore_ascii_case(l))
            && self
                .cwe
                .as_ref()
                .is_none_or(|c| f.cwe.iter().any(|x| x.eq_ignore_ascii_case(c)))
    }
}

struct ServerConfig {
    root: PathBuf,
    token: Option<String>,
}

/// Validate bind address and token: non-loopback binds require a token.
pub fn check_bind(host: &str, token: Option<&str>) -> Result<IpAddr> {
    let ip: IpAddr = if host == "localhost" {
        IpAddr::from([127, 0, 0, 1])
    } else {
        host.parse()
            .with_context(|| format!("invalid --host address: {host}"))?
    };
    if !ip.is_loopback() && token.is_none_or(str::is_empty) {
        bail!("--token is required when binding to a non-loopback address ({ip})");
    }
    Ok(ip)
}

pub async fn run_serve_command(host: &str, port: u16, token: Option<String>) -> Result<()> {
    let ip = check_bind(host, token.as_deref())?;
    let addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;
    let config = Arc::new(ServerConfig {
        root: cache_base(),
        token: token.filter(|t| !t.is_empty()),
    });
    eprintln!("parsentry web UI: http://{}", listener.local_addr()?);
    loop {
        let (stream, _) = listener.accept().await?;
        let config = Arc::clone(&config);
        tokio::spawn(async move {
            let _ = handle_connection(stream, &config).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, config: &ServerConfig) -> Result<()> {
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.len() > MAX_REQUEST_BYTES {
            return write_response(&mut stream, 431, "text/plain", b"request too large").await;
        }
    }
    let request = String::from_utf8_lossy(&buf);
    let (status, content_type, body) = route(&request, config);
    write_response(&mut stream, status, content_type, &body).await
}

fn route(request: &str, config: &ServerConfig) -> (u16, &'static str, Vec<u8>) {
    let mut lines = request.split("\r\n");
    let mut parts = lines.next().unwrap_or("").split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        return (405, "text/plain", b"method not allowed".to_vec());
    }
    let (path, query) = target.split_once('?').unwrap_or((target, ""));

    if let Some(api) = path.strip_prefix("/api/") {
        if let Some(token) = &config.token {
            let expected = format!("Bearer {token}");
            let authorized = lines.any(|l| {
                l.split_once(':').is_some_and(|(k, v)| {
                    k.eq_ignore_ascii_case("authorization") && v.trim() == expected
                })
            });
            if !authorized {
                return (
                    401,
                    "application/json",
                    br#"{"error":"unauthorized"}"#.to_vec(),
                );
            }
        }
        return api_route(api, query, &config.root);
    }

    let asset = match path.trim_start_matches('/') {
        "" => "index.html",
        p => p,
    };
    match WebAssets::get(asset) {
        Some(file) => (200, content_type_for(asset), file.data.into_owned()),
        None => (404, "text/plain", b"not found".to_vec()),
    }
}

fn api_route(api: &str, query: &str, root: &Path) -> (u16, &'static str, Vec<u8>) {
    let params = parse_query(query);
    let get = |k: &str| {
        params
            .iter()
            .find(|(key, v)| key == k && !v.is_empty())
            .map(|(_, v)| v.clone())
    };
    let result = match api {
        "runs" => serde_json::to_vec(&list_runs(root)).map_err(anyhow::Error::from),
        "findings" => match get("run") {
            Some(run) => {
                let filter = FindingFilter {
                    verdict: get("verdict"),
                    level: get("level"),
                    cwe: get("cwe"),
                };
                load_findings(root, &run, &filter)
                    .and_then(|f| serde_json::to_vec(&f).map_err(anyhow::Error::from))
            }
            None => {
                return (
                    400,
                    "application/json",
                    br#"{"error":"missing run"}"#.to_vec(),
                );
            }
        },
        _ => {
            return (
                404,
                "application/json",
                br#"{"error":"not found"}"#.to_vec(),
            );
        }
    };
    match result {
        Ok(body) => (200, "application/json", body),
        Err(e) => (
            404,
            "application/json",
            serde_json::to_vec(&serde_json::json!({ "error": e.to_string() })).unwrap_or_default(),
        ),
    }
}

fn parse_query(query: &str) -> Vec<(String, String)> {
    query
        .split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            let decode = |s: &str| {
                urlencoding::decode(&s.replace('+', " "))
                    .map(|c| c.into_owned())
                    .ok()
            };
            Some((decode(k)?, decode(v)?))
        })
        .collect()
}

fn content_type_for(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

async fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        431 => "Request Header Fields Too Large",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes()).await?;
    stream.write_all(body).await?;
    stream.flush().await?;
    Ok(())
}

/// Run directories: every cache dir directly under the root, plus `url/<hash>`.
fn run_dirs(root: &Path) -> Vec<(String, PathBuf)> {
    let mut dirs = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return dirs;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "url" {
            if let Ok(sub) = std::fs::read_dir(&path) {
                for e in sub.flatten().filter(|e| e.path().is_dir()) {
                    dirs.push((format!("url/{}", e.file_name().to_string_lossy()), e.path()));
                }
            }
        } else {
            dirs.push((name, path));
        }
    }
    dirs.sort();
    dirs
}

pub fn list_runs(root: &Path) -> Vec<RunInfo> {
    run_dirs(root)
        .into_iter()
        .filter(|(_, dir)| {
            dir.join("model.json").exists() || dir.join("merged.sarif.json").exists()
        })
        .map(|(id, dir)| {
            let merged = dir.join("merged.sarif.json");
            let findings = read_findings(&merged).map(|f| f.len()).unwrap_or(0);
            let modified = std::fs::metadata(&merged)
                .and_then(|m| m.modified())
                .ok()
                .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339());
            RunInfo {
                id,
                completed: merged.exists(),
                findings,
                modified,
            }
        })
        .collect()
}

pub fn load_findings(root: &Path, run: &str, filter: &FindingFilter) -> Result<Vec<Finding>> {
    // Only accept run ids enumerated from the cache root (no path traversal).
    let (_, dir) = run_dirs(root)
        .into_iter()
        .find(|(id, _)| id == run)
        .with_context(|| format!("unknown run: {run}"))?;
    let findings = read_findings(&dir.join("merged.sarif.json"))?;
    Ok(findings.into_iter().filter(|f| filter.matches(f)).collect())
}

fn read_findings(path: &Path) -> Result<Vec<Finding>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("no completed report at {}", path.display()))?;
    let sarif: Value = serde_json::from_str(&text)?;
    let mut out = Vec::new();
    for run in sarif["runs"].as_array().into_iter().flatten() {
        for r in run["results"].as_array().into_iter().flatten() {
            let props = &r["properties"];
            let loc = &r["locations"][0]["physicalLocation"];
            let location =
                loc["artifactLocation"]["uri"]
                    .as_str()
                    .map(|uri| match loc["region"]["startLine"].as_u64() {
                        Some(line) => format!("{uri}:{line}"),
                        None => uri.to_string(),
                    });
            out.push(Finding {
                rule_id: r["ruleId"].as_str().unwrap_or_default().to_string(),
                level: r["level"].as_str().unwrap_or("warning").to_string(),
                verdict: props["verdict"]
                    .as_str()
                    .unwrap_or("indeterminate")
                    .to_string(),
                cwe: props["cwe"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|c| c.as_str().map(str::to_string))
                    .collect(),
                message: r["message"]["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                location,
                confidence: props["confidence"].as_f64(),
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SARIF: &str = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"parsentry"}},"results":[
        {"ruleId":"SQLI","level":"error","message":{"text":"sql injection"},
         "locations":[{"physicalLocation":{"artifactLocation":{"uri":"app.py"},"region":{"startLine":12}}}],
         "properties":{"verdict":"confirmed","cwe":["CWE-89"],"confidence":0.9}},
        {"ruleId":"XSS","level":"warning","message":{"text":"xss"},
         "properties":{"verdict":"ruled_out","cwe":["CWE-79"]}}]}]}"#;

    fn fixture() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let run = dir.path().join("owner__repo");
        std::fs::create_dir_all(&run).unwrap();
        std::fs::write(run.join("merged.sarif.json"), SARIF).unwrap();
        std::fs::create_dir_all(dir.path().join("pending")).unwrap();
        std::fs::write(dir.path().join("pending/model.json"), "{}").unwrap();
        dir
    }

    fn config(root: &Path, token: Option<&str>) -> ServerConfig {
        ServerConfig {
            root: root.to_path_buf(),
            token: token.map(str::to_string),
        }
    }

    #[test]
    fn lists_runs_with_completion_state() {
        let dir = fixture();
        let runs = list_runs(dir.path());
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].id, "owner__repo");
        assert!(runs[0].completed);
        assert_eq!(runs[0].findings, 2);
        assert!(!runs[1].completed);
    }

    #[test]
    fn filters_findings_by_verdict_level_and_cwe() {
        let dir = fixture();
        let all = load_findings(dir.path(), "owner__repo", &FindingFilter::default()).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].location.as_deref(), Some("app.py:12"));

        let by = |verdict: Option<&str>, level: Option<&str>, cwe: Option<&str>| {
            let filter = FindingFilter {
                verdict: verdict.map(str::to_string),
                level: level.map(str::to_string),
                cwe: cwe.map(str::to_string),
            };
            load_findings(dir.path(), "owner__repo", &filter).unwrap()
        };
        assert_eq!(by(Some("confirmed"), None, None)[0].rule_id, "SQLI");
        assert_eq!(by(None, Some("warning"), None)[0].rule_id, "XSS");
        assert_eq!(by(None, None, Some("cwe-79"))[0].rule_id, "XSS");
        assert!(by(Some("confirmed"), Some("warning"), None).is_empty());
    }

    #[test]
    fn rejects_unknown_or_traversal_run_ids() {
        let dir = fixture();
        assert!(load_findings(dir.path(), "../etc", &FindingFilter::default()).is_err());
        assert!(load_findings(dir.path(), "missing", &FindingFilter::default()).is_err());
    }

    #[test]
    fn non_loopback_bind_requires_token() {
        assert!(check_bind("127.0.0.1", None).is_ok());
        assert!(check_bind("localhost", None).is_ok());
        assert!(check_bind("::1", None).is_ok());
        assert!(check_bind("0.0.0.0", None).is_err());
        assert!(check_bind("0.0.0.0", Some("")).is_err());
        assert!(check_bind("0.0.0.0", Some("secret")).is_ok());
    }

    #[test]
    fn api_requires_bearer_token_when_configured() {
        let dir = fixture();
        let cfg = config(dir.path(), Some("secret"));
        let (status, _, _) = route("GET /api/runs HTTP/1.1\r\nHost: x\r\n\r\n", &cfg);
        assert_eq!(status, 401);
        let (status, _, _) = route(
            "GET /api/runs HTTP/1.1\r\nAuthorization: Bearer wrong\r\n\r\n",
            &cfg,
        );
        assert_eq!(status, 401);
        let (status, _, body) = route(
            "GET /api/runs HTTP/1.1\r\nauthorization: Bearer secret\r\n\r\n",
            &cfg,
        );
        assert_eq!(status, 200);
        assert!(String::from_utf8(body).unwrap().contains("owner__repo"));
    }

    #[test]
    fn serves_embedded_ui_and_findings_api() {
        let dir = fixture();
        let cfg = config(dir.path(), None);
        let (status, ct, body) = route("GET / HTTP/1.1\r\n\r\n", &cfg);
        assert_eq!(status, 200);
        assert!(ct.starts_with("text/html"));
        assert!(String::from_utf8(body).unwrap().contains("Findings"));
        let (status, _, body) = route(
            "GET /api/findings?run=owner__repo&verdict=confirmed HTTP/1.1\r\n\r\n",
            &cfg,
        );
        assert_eq!(status, 200);
        let findings: Vec<Value> = serde_json::from_slice(&body).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(route("GET /nope.js HTTP/1.1\r\n\r\n", &cfg).0, 404);
        assert_eq!(route("POST /api/runs HTTP/1.1\r\n\r\n", &cfg).0, 405);
    }
}
