use parsentry::webhook::{
    Request, ScanRequest, ServeConfig, WEBHOOK_PATH, handle, serve, verify_signature,
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
    let (status, _) = handle(&post("push", PUSH), None, &tx);
    assert_eq!(status, 202);
    assert_eq!(
        rx.try_recv().unwrap(),
        ScanRequest {
            event: "push".into(),
            repository: "octocat/Hello-World".into(),
            git_ref: Some("refs/heads/main".into()),
            sha: Some("6dcb09b5b57875f334f61aebed695e2e4193db5e".into()),
        }
    );
}

#[test]
fn pull_request_fixture_enqueues_scan() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (status, _) = handle(&post("pull_request", PULL_REQUEST), None, &tx);
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
        handle(&post("pull_request", closed.as_bytes()), None, &tx).0,
        202
    );
    assert_eq!(handle(&post("issues", b"{}"), None, &tx).0, 202);
    assert!(rx.try_recv().is_err());
}

#[test]
fn rejects_bad_requests() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut get = post("push", PUSH);
    get.method = "GET".into();
    assert_eq!(handle(&get, None, &tx).0, 405);
    let mut other = post("push", PUSH);
    other.path = "/other".into();
    assert_eq!(handle(&other, None, &tx).0, 404);
    assert_eq!(handle(&post("push", b"not json"), None, &tx).0, 400);
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
    assert_eq!(handle(&post("push", PUSH), Some("s3cret"), &tx).0, 401);
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
