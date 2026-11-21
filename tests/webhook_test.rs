use parsentry::webhook::{
    EventBus, Request, ScanRequest, ServeConfig, WEBHOOK_PATH, handle, run_worker_with, serve,
    verify_signature,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

const PUSH: &[u8] = include_bytes!("fixtures/webhooks/push.json");
const PULL_REQUEST: &[u8] = include_bytes!("fixtures/webhooks/pull_request.json");

fn post(event: &str, body: &[u8]) -> Request {
    Request {
        method: "POST".into(),
        path: WEBHOOK_PATH.into(),
        headers: vec![("X-GitHub-Event".into(), event.into())],
        body: body.to_vec(),
    }
}

#[test]
fn push_fixture_enqueues_scan() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (status, _) = handle(&post("push", PUSH), None, &tx, &EventBus::default());
    assert_eq!(status, 202);
    let scan = rx.try_recv().unwrap();
    assert!(scan.id > 0);
    assert_eq!(
        scan,
        ScanRequest {
            id: scan.id,
            event: "push".into(),
            repository: "octocat/Hello-World".into(),
            git_ref: Some("refs/heads/main".into()),
            sha: Some("6dcb09b5b57875f334f61aebed695e2e4193db5e".into()),
            ..Default::default()
        }
    );
}

#[test]
fn pull_request_fixture_enqueues_scan() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (status, _) = handle(
        &post("pull_request", PULL_REQUEST),
        None,
        &tx,
        &EventBus::default(),
    );
    assert_eq!(status, 202);
    let scan = rx.try_recv().unwrap();
    assert_eq!(scan.repository, "octocat/Hello-World");
    assert_eq!(scan.git_ref.as_deref(), Some("feature"));
}

#[test]
fn closed_pull_request_and_other_events_are_ignored() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let closed = String::from_utf8_lossy(PULL_REQUEST).replace("synchronize", "closed");
    assert_eq!(
        handle(
            &post("pull_request", closed.as_bytes()),
            None,
            &tx,
            &EventBus::default()
        )
        .0,
        202
    );
    assert_eq!(
        handle(&post("issues", b"{}"), None, &tx, &EventBus::default()).0,
        202
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn rejects_bad_requests() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut get = post("push", PUSH);
    get.method = "GET".into();
    assert_eq!(handle(&get, None, &tx, &EventBus::default()).0, 405);
    let mut other = post("push", PUSH);
    other.path = "/other".into();
    assert_eq!(handle(&other, None, &tx, &EventBus::default()).0, 404);
    assert_eq!(
        handle(&post("push", b"not json"), None, &tx, &EventBus::default()).0,
        400
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn signature_is_enforced_when_secret_set() {
    // Example from GitHub's "Validating webhook deliveries" docs.
    let sig = "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";
    assert!(verify_signature(
        "It's a Secret to Everybody",
        b"Hello, World!",
        Some(sig)
    ));
    assert!(!verify_signature("wrong", b"Hello, World!", Some(sig)));
    assert!(!verify_signature(
        "It's a Secret to Everybody",
        b"Hello, World!",
        None
    ));

    let (tx, mut rx) = mpsc::unbounded_channel();
    assert_eq!(
        handle(
            &post("push", PUSH),
            Some("s3cret"),
            &tx,
            &EventBus::default()
        )
        .0,
        401
    );
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn http_server_enqueues_scan() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(serve(listener, ServeConfig::default(), tx));

    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let head = format!(
        "POST {WEBHOOK_PATH} HTTP/1.1\r\nHost: localhost\r\nX-GitHub-Event: push\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
        PUSH.len()
    );
    stream.write_all(head.as_bytes()).await.unwrap();
    stream.write_all(PUSH).await.unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).await.unwrap();
    assert!(resp.starts_with("HTTP/1.1 202"), "{resp}");
    assert_eq!(rx.recv().await.unwrap().repository, "octocat/Hello-World");
}

#[tokio::test]
async fn http_api_requires_bearer_token() {
    let cache = tempfile::tempdir().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, _rx) = mpsc::unbounded_channel();
    let config = ServeConfig {
        api_token: Some("s3cret".into()),
        cache_base: cache.path().to_path_buf(),
        ..Default::default()
    };
    tokio::spawn(serve(listener, config, tx));

    async fn get(addr: std::net::SocketAddr, auth: &str) -> String {
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let req = format!("GET /api/runs HTTP/1.1\r\nHost: localhost\r\n{auth}\r\n");
        stream.write_all(req.as_bytes()).await.unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).await.unwrap();
        resp
    }
    let denied = get(addr, "").await;
    assert!(denied.starts_with("HTTP/1.1 401"), "{denied}");
    assert!(denied.contains("WWW-Authenticate: Bearer"), "{denied}");
    let ok = get(addr, "Authorization: Bearer s3cret\r\n").await;
    assert!(ok.starts_with("HTTP/1.1 200"), "{ok}");
    assert!(ok.ends_with("[]"), "{ok}");
}

#[tokio::test]
async fn http_serves_embedded_ui_and_findings() {
    let cache = tempfile::tempdir().unwrap();
    let run = cache.path().join("owner__app");
    std::fs::create_dir_all(&run).unwrap();
    std::fs::write(
        run.join("merged.sarif.json"),
        r#"{"version":"2.1.0","runs":[{"results":[{"ruleId":"sqli","level":"error","properties":{"verdict":"confirmed","cwe":["CWE-89"]}}]}]}"#,
    )
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, _rx) = mpsc::unbounded_channel();
    let config = ServeConfig {
        cache_base: cache.path().to_path_buf(),
        ..Default::default()
    };
    tokio::spawn(serve(listener, config, tx));

    async fn get(addr: std::net::SocketAddr, path: &str) -> String {
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let req = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n");
        stream.write_all(req.as_bytes()).await.unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).await.unwrap();
        resp
    }
    let index = get(addr, "/").await;
    assert!(index.starts_with("HTTP/1.1 200"), "{index}");
    assert!(index.contains("Content-Type: text/html"), "{index}");
    assert!(index.contains("Content-Security-Policy:"), "{index}");
    assert!(index.contains("findings-view"), "{index}");
    let js = get(addr, "/app.js").await;
    assert!(js.contains("sessionStorage"), "{js}");

    let findings = get(addr, "/api/findings?verdict=confirmed&level=error&cwe=89").await;
    assert!(findings.starts_with("HTTP/1.1 200"), "{findings}");
    assert!(findings.contains("\"rule_id\":\"sqli\""), "{findings}");
    assert!(get(addr, "/nope").await.starts_with("HTTP/1.1 404"));
}

async fn request(addr: std::net::SocketAddr, raw: &str) -> String {
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    stream.write_all(raw.as_bytes()).await.unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).await.unwrap();
    resp
}

fn post_scan(body: &str, auth: &str) -> String {
    format!(
        "POST /api/scans HTTP/1.1\r\nHost: localhost\r\n{auth}Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
}

#[tokio::test]
async fn api_scan_is_enqueued_and_streams_events() {
    use tokio::io::AsyncBufReadExt;
    let cache = tempfile::tempdir().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = mpsc::unbounded_channel();
    let config = ServeConfig {
        api_token: Some("s3cret".into()),
        cache_base: cache.path().to_path_buf(),
        ..Default::default()
    };
    // `sh scan -- <target>` fails, exercising started/log/failed without a real scan.
    tokio::spawn(run_worker_with("sh".into(), rx, config.events.clone()));
    tokio::spawn(serve(listener, config, tx));
    let auth = "Authorization: Bearer s3cret\r\n";

    let denied = request(addr, "GET /api/events HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
    assert!(denied.starts_with("HTTP/1.1 401"), "{denied}");
    let denied = request(addr, &post_scan(r#"{"target":"x"}"#, "")).await;
    assert!(denied.starts_with("HTTP/1.1 401"), "{denied}");

    let mut sse = tokio::net::TcpStream::connect(addr).await.unwrap();
    sse.write_all(format!("GET /api/events HTTP/1.1\r\nHost: localhost\r\n{auth}\r\n").as_bytes())
        .await
        .unwrap();
    let mut sse = tokio::io::BufReader::new(sse);
    let mut line = String::new();
    loop {
        line.clear();
        sse.read_line(&mut line).await.unwrap();
        assert!(!line.contains("401"), "{line}");
        if line.starts_with(": connected") {
            break;
        }
    }

    let bad = request(
        addr,
        &post_scan(r#"{"target":"x","runtime":"bogus"}"#, auth),
    )
    .await;
    assert!(bad.starts_with("HTTP/1.1 400"), "{bad}");
    let bad = request(addr, &post_scan(r#"{"nope":1}"#, auth)).await;
    assert!(bad.starts_with("HTTP/1.1 400"), "{bad}");

    let resp = request(
        addr,
        &post_scan(r#"{"target":"owner/repo","runtime":"pipe"}"#, auth),
    )
    .await;
    assert!(resp.starts_with("HTTP/1.1 202"), "{resp}");
    let body = resp.split("\r\n\r\n").nth(1).unwrap();
    let id = serde_json::from_str::<serde_json::Value>(body).unwrap()["id"]
        .as_u64()
        .unwrap();

    let mut kinds = Vec::new();
    let read = async {
        loop {
            line.clear();
            assert!(sse.read_line(&mut line).await.unwrap() > 0, "stream closed");
            let Some(data) = line.strip_prefix("data: ") else {
                continue;
            };
            let ev: serde_json::Value = serde_json::from_str(data).unwrap();
            if ev["id"] != id {
                continue;
            }
            let kind = ev["event"].as_str().unwrap().to_string();
            if kind == "queued" || kind == "started" {
                assert_eq!(ev["target"], "owner/repo");
            }
            kinds.push(kind.clone());
            if kind == "finished" || kind == "failed" {
                break;
            }
        }
    };
    tokio::time::timeout(std::time::Duration::from_secs(30), read)
        .await
        .expect("timed out waiting for job events");
    assert_eq!(kinds[..2], ["queued", "started"]);
    assert_eq!(kinds.last().unwrap(), "failed");
    assert!(kinds.contains(&"log".to_string()), "{kinds:?}");
}

#[cfg(unix)]
#[tokio::test]
async fn shutdown_closes_listener_and_kills_running_scan() {
    use parsentry::webhook::{run_worker_until, serve_until};
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let pid_file = dir.path().join("pid");
    let bin = dir.path().join("fake-parsentry");
    std::fs::write(
        &bin,
        format!(
            "#!/bin/sh\necho $$ > {}\nexec sleep 300\n",
            pid_file.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (stop, stop_rx) = tokio::sync::watch::channel(false);
    let (tx, rx) = mpsc::unbounded_channel();
    let config = ServeConfig::default();
    let worker = tokio::spawn(run_worker_until(
        bin,
        rx,
        config.events.clone(),
        stop_rx.clone(),
    ));
    let server = tokio::spawn(serve_until(listener, config, tx.clone(), stop_rx));
    tx.send(ScanRequest {
        repository: "owner/repo".into(),
        ..Default::default()
    })
    .unwrap();

    let pid = loop {
        if let Ok(pid) = std::fs::read_to_string(&pid_file)
            && let Ok(pid) = pid.trim().parse::<i32>()
        {
            break pid;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    };
    let alive = |pid: i32| {
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .unwrap()
            .success()
    };
    assert!(alive(pid));

    stop.send(true).unwrap();
    let timeout = std::time::Duration::from_secs(10);
    tokio::time::timeout(timeout, server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    tokio::time::timeout(timeout, worker)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(tokio::net::TcpStream::connect(addr).await.is_err());
    assert!(!alive(pid), "scan process {pid} still running");
}
