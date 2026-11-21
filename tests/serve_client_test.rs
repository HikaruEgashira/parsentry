use parsentry::serve_client::{SERVE_URL_ENV, ServeClient, format_event};
use parsentry::webhook::{EventBus, JobEvent, ServeConfig, serve};
use tokio::sync::mpsc;

async fn spawn_serve(
    token: Option<&str>,
) -> (
    String,
    EventBus,
    mpsc::UnboundedReceiver<parsentry::webhook::ScanRequest>,
    tempfile::TempDir,
) {
    let cache = tempfile::tempdir().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = mpsc::unbounded_channel();
    let config = ServeConfig {
        api_token: token.map(str::to_string),
        cache_base: cache.path().to_path_buf(),
        ..Default::default()
    };
    let events = config.events.clone();
    tokio::spawn(serve(listener, config, tx));
    (format!("http://{addr}"), events, rx, cache)
}

async fn closed_port_url() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    format!("http://{addr}")
}

fn parsentry(serve_url: &str, home: &std::path::Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_parsentry"))
        .args(args)
        .env(SERVE_URL_ENV, serve_url)
        .env("HOME", home)
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env(
            "PARSENTRY_AGENT_RUNTIME",
            "/nonexistent/parsentry-agent-runtime",
        )
        .env_remove("PARSENTRY_API_TOKEN")
        .output()
        .unwrap()
}

#[tokio::test]
async fn falls_back_when_serve_is_absent() {
    let url = closed_port_url().await;
    assert!(ServeClient::probe(&url, None).await.is_none());
    let home = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    let target = target.path().to_str().unwrap();

    // log -f keeps watching the cache in-process until --timeout.
    let out = parsentry(
        &url,
        home.path(),
        &["log", "-f", "--timeout", "1", "--no-color", target],
    );
    let stdout = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(stdout.contains("watching"), "{stdout}");
    assert!(!stdout.contains("/api/events"), "{stdout}");

    // scan runs in-process, so the missing sidecar is what fails.
    let out = parsentry(&url, home.path(), &["scan", "--runtime", "claude", target]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(!stderr.contains("Enqueued"), "{stderr}");
}

#[tokio::test]
async fn cli_uses_running_serve() {
    let (url, _events, mut rx, _cache) = spawn_serve(None).await;
    let home = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    let path = target.path().to_str().unwrap().to_string();
    let (h, u, p) = (home.path().to_path_buf(), url.clone(), path.clone());
    let out = tokio::task::spawn_blocking(move || {
        parsentry(&u, &h, &["scan", "--runtime", "openai", &p])
    })
    .await
    .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "{stderr}");
    assert!(stderr.contains("Enqueued"), "{stderr}");
    let scan = rx.recv().await.unwrap();
    assert_eq!(
        std::path::Path::new(&scan.repository),
        target.path().canonicalize().unwrap()
    );
    assert_eq!(scan.options.runtime.as_deref(), Some("openai"));

    let (h, u) = (home.path().to_path_buf(), url.clone());
    let out = tokio::task::spawn_blocking(move || {
        parsentry(&u, &h, &["log", "-f", "--timeout", "1", "--no-color"])
    })
    .await
    .unwrap();
    let stdout = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{stdout}");
    assert!(stdout.contains("/api/events"), "{stdout}");
}

#[tokio::test]
async fn probe_requires_valid_token() {
    let (url, _events, _rx, _cache) = spawn_serve(Some("s3cret")).await;
    assert!(ServeClient::probe(&url, None).await.is_none());
    assert!(
        ServeClient::probe(&url, Some("s3cret".into()))
            .await
            .is_some()
    );
}

#[tokio::test]
async fn enqueues_scan_on_running_serve() {
    let (url, _events, mut rx, _cache) = spawn_serve(Some("s3cret")).await;
    let client = ServeClient::probe(&url, Some("s3cret".into()))
        .await
        .unwrap();
    let id = client
        .enqueue_scan("owner/repo", Some("report.pdf"), Some("claude"), None)
        .await
        .unwrap();
    let scan = rx.recv().await.unwrap();
    assert_eq!(scan.id, id);
    assert_eq!(scan.event, "api");
    assert_eq!(scan.repository, "owner/repo");
    assert_eq!(scan.options.runtime.as_deref(), Some("claude"));
    let output = scan.options.output.unwrap();
    assert!(std::path::Path::new(&output).is_absolute(), "{output}");
    assert!(output.ends_with("report.pdf"));

    let err = client
        .enqueue_scan("x", None, Some("bogus"), None)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("400"), "{err}");
}

#[tokio::test]
async fn log_follow_consumes_sse_events() {
    let (url, events, _rx, _cache) = spawn_serve(None).await;
    let client = ServeClient::probe(&url, None).await.unwrap();
    let (tx, mut seen) = mpsc::unbounded_channel();
    let follower = tokio::spawn(async move {
        client
            .follow_events(|ev| {
                let done = ev["event"] == "finished";
                tx.send(ev).unwrap();
                !done
            })
            .await
    });
    // Re-emit until the subscriber is attached and has received the event.
    let first = loop {
        events.emit(JobEvent {
            id: 7,
            event: "started",
            target: Some("owner/repo".into()),
            line: None,
            error: None,
        });
        if let Ok(Some(ev)) =
            tokio::time::timeout(std::time::Duration::from_millis(100), seen.recv()).await
        {
            break ev;
        }
    };
    assert_eq!(format_event(&first), "job 7 started owner/repo");
    events.emit(JobEvent {
        id: 7,
        event: "finished",
        target: None,
        line: None,
        error: None,
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), follower)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let mut last = None;
    while let Ok(ev) = seen.try_recv() {
        last = Some(ev);
    }
    assert_eq!(format_event(&last.unwrap()), "job 7 finished");
}
