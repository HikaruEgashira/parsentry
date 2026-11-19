//! Local web UI: a live agent map (updated over SSE) and a threat model view
//! listing each surface with finding counts per verdict.

use anyhow::Result;
use parsentry_core::ThreatModel;
use serde::Serialize;
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const INDEX_HTML: &str = include_str!("web_index.html");

/// SSE refresh interval while a client is connected.
const SSE_POLL: Duration = Duration::from_secs(1);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct VerdictCounts {
    pub confirmed: usize,
    pub ruled_out: usize,
    pub indeterminate: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SurfaceState {
    pub id: String,
    pub kind: String,
    pub identifier: String,
    pub description: String,
    /// `pending` (no prompt yet), `running` (prompt issued), or `complete` (SARIF written)
    pub status: &'static str,
    pub verdicts: VerdictCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScanState {
    pub target: String,
    /// `model`, `surface`, or `complete`
    pub phase: &'static str,
    pub app_type: String,
    pub summary: String,
    pub surfaces: Vec<SurfaceState>,
}

/// Count findings per verdict in a SARIF document. Missing verdicts are
/// `indeterminate`, matching `evidence::normalize`.
pub fn count_verdicts(sarif: &serde_json::Value) -> VerdictCounts {
    let mut counts = VerdictCounts::default();
    let runs = sarif.get("runs").and_then(|r| r.as_array());
    for run in runs.into_iter().flatten() {
        let results = run.get("results").and_then(|r| r.as_array());
        for result in results.into_iter().flatten() {
            let verdict = result
                .pointer("/properties/verdict")
                .and_then(|v| v.as_str())
                .unwrap_or("indeterminate");
            match verdict {
                "confirmed" => counts.confirmed += 1,
                "ruled_out" => counts.ruled_out += 1,
                _ => counts.indeterminate += 1,
            }
        }
    }
    counts
}

/// Build a snapshot of the scan from the target's cache directory.
pub fn load_state(target: &str, cache_dir: &Path) -> ScanState {
    let model = std::fs::read_to_string(cache_dir.join("model.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<ThreatModel>(&s).ok());
    let Some(model) = model else {
        return ScanState {
            target: target.to_string(),
            phase: "model",
            app_type: String::new(),
            summary: String::new(),
            surfaces: Vec::new(),
        };
    };

    let reports = cache_dir.join("reports");
    let surfaces: Vec<SurfaceState> = model
        .surfaces
        .iter()
        .map(|s| {
            let dir = reports.join(&s.id);
            let sarif = std::fs::read_to_string(dir.join("result.sarif.json"))
                .ok()
                .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok());
            let (status, verdicts) = match sarif {
                Some(v) => ("complete", count_verdicts(&v)),
                None if dir.join("prompt.md").exists() => ("running", VerdictCounts::default()),
                None => ("pending", VerdictCounts::default()),
            };
            SurfaceState {
                id: s.id.clone(),
                kind: s.kind.clone(),
                identifier: s.identifier.clone(),
                description: s.description.clone(),
                status,
                verdicts,
            }
        })
        .collect();

    let phase = if !surfaces.is_empty() && surfaces.iter().all(|s| s.status == "complete") {
        "complete"
    } else {
        "surface"
    };
    ScanState {
        target: target.to_string(),
        phase,
        app_type: model.app_type,
        summary: model.summary,
        surfaces,
    }
}

pub async fn run_web_command(target: &str, host: &str, port: u16) -> Result<()> {
    let cache_dir = super::common::cache_dir_for(target);
    let listener = TcpListener::bind((host, port)).await?;
    eprintln!("parsentry web UI: http://{}", listener.local_addr()?);
    loop {
        let (stream, _) = listener.accept().await?;
        let target = target.to_string();
        let cache_dir = cache_dir.clone();
        tokio::spawn(async move {
            let _ = handle(stream, &target, &cache_dir).await;
        });
    }
}

async fn handle(mut stream: TcpStream, target: &str, cache_dir: &Path) -> Result<()> {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("/");
    let path = path.split('?').next().unwrap_or("/");

    match path {
        "/" | "/index.html" => {
            respond(
                &mut stream,
                "200 OK",
                "text/html; charset=utf-8",
                INDEX_HTML,
            )
            .await
        }
        "/api/state" => {
            let body = serde_json::to_string(&load_state(target, cache_dir))?;
            respond(&mut stream, "200 OK", "application/json", &body).await
        }
        "/events" => stream_events(stream, target, cache_dir).await,
        _ => respond(&mut stream, "404 Not Found", "text/plain", "not found").await,
    }
}

async fn respond(stream: &mut TcpStream, status: &str, ctype: &str, body: &str) -> Result<()> {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).await?;
    stream.write_all(body.as_bytes()).await?;
    Ok(())
}

/// Push a `state` event whenever the snapshot changes, plus keep-alives.
async fn stream_events(mut stream: TcpStream, target: &str, cache_dir: &Path) -> Result<()> {
    stream
        .write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n",
        )
        .await?;
    let mut last: Option<ScanState> = None;
    loop {
        let state = load_state(target, cache_dir);
        if last.as_ref() != Some(&state) {
            let data = serde_json::to_string(&state)?;
            stream
                .write_all(format!("event: state\ndata: {data}\n\n").as_bytes())
                .await?;
            last = Some(state);
        } else {
            stream.write_all(b": keep-alive\n\n").await?;
        }
        tokio::time::sleep(SSE_POLL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn write_model(dir: &Path) {
        let model = json!({
            "app_type": "web_application",
            "summary": "demo",
            "surfaces": [
                {"id": "S1", "kind": "endpoint", "identifier": "GET /a", "locations": [], "description": "a"},
                {"id": "S2", "kind": "endpoint", "identifier": "GET /b", "locations": [], "description": "b"},
                {"id": "S3", "kind": "endpoint", "identifier": "GET /c", "locations": [], "description": "c"}
            ]
        });
        std::fs::write(dir.join("model.json"), model.to_string()).unwrap();
    }

    #[test]
    fn counts_verdicts_with_indeterminate_default() {
        let sarif = json!({"runs": [{"results": [
            {"properties": {"verdict": "confirmed"}},
            {"properties": {"verdict": "ruled_out"}},
            {"properties": {}},
            {}
        ]}]});
        assert_eq!(
            count_verdicts(&sarif),
            VerdictCounts {
                confirmed: 1,
                ruled_out: 1,
                indeterminate: 2
            }
        );
    }

    #[test]
    fn no_model_is_model_phase() {
        let tmp = tempfile::tempdir().unwrap();
        let state = load_state("t", tmp.path());
        assert_eq!(state.phase, "model");
        assert!(state.surfaces.is_empty());
    }

    #[test]
    fn in_progress_scan_shows_surface_statuses() {
        let tmp = tempfile::tempdir().unwrap();
        write_model(tmp.path());
        let reports = tmp.path().join("reports");
        std::fs::create_dir_all(reports.join("S1")).unwrap();
        std::fs::create_dir_all(reports.join("S2")).unwrap();
        std::fs::write(reports.join("S1/prompt.md"), "p").unwrap();
        std::fs::write(reports.join("S2/prompt.md"), "p").unwrap();
        std::fs::write(
            reports.join("S1/result.sarif.json"),
            json!({"runs": [{"results": [{"properties": {"verdict": "confirmed"}}]}]}).to_string(),
        )
        .unwrap();

        let state = load_state("t", tmp.path());
        assert_eq!(state.phase, "surface");
        let statuses: Vec<_> = state.surfaces.iter().map(|s| s.status).collect();
        assert_eq!(statuses, ["complete", "running", "pending"]);
        assert_eq!(state.surfaces[0].verdicts.confirmed, 1);
    }

    #[tokio::test]
    async fn serves_index_state_and_sse() {
        let tmp = tempfile::tempdir().unwrap();
        write_model(tmp.path());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let cache = tmp.path().to_path_buf();
        tokio::spawn(async move {
            loop {
                let (s, _) = listener.accept().await.unwrap();
                let cache = cache.clone();
                tokio::spawn(async move {
                    let _ = handle(s, "t", &cache).await;
                });
            }
        });

        async fn get(addr: std::net::SocketAddr, path: &str) -> String {
            let mut s = TcpStream::connect(addr).await.unwrap();
            s.write_all(format!("GET {path} HTTP/1.1\r\nHost: x\r\n\r\n").as_bytes())
                .await
                .unwrap();
            let mut out = String::new();
            let mut buf = [0u8; 8192];
            // SSE never closes, so stop once the first event has arrived.
            while !(path == "/events" && out.contains("data: ")) {
                let n = s.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                out.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
            out
        }

        assert!(get(addr, "/").await.contains("EventSource"));
        assert!(get(addr, "/api/state").await.contains("\"S3\""));
        let sse = get(addr, "/events").await;
        assert!(sse.contains("text/event-stream"));
        assert!(sse.contains("event: state"));
        assert!(sse.contains("\"pending\""));
    }
}
