use anyhow::Result;
use dotenvy::dotenv;

use parsentry::cli::RootCommand;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Handle Ctrl+C / SIGTERM gracefully, removing any live verify sandboxes.
    tokio::select! {
        result = RootCommand::execute() => result,
        _ = sigterm() => {
            stop_serve().await;
            parsentry::sandbox::kill_all();
            std::process::exit(143);
        }
        ctrl_c_result = tokio::signal::ctrl_c() => {
            match ctrl_c_result {
                Ok(()) => {
                    stop_serve().await;
                    parsentry::sandbox::kill_all();
                    eprintln!("\nInterrupted by user");
                    std::process::exit(130);
                }
                Err(e) => {
                    eprintln!("Failed to listen for ctrl-c signal: {}", e);
                    Err(anyhow::anyhow!("Signal handler setup failed: {}", e))
                }
            }
        }
    }
}

/// Close the `serve` listener and terminate its scan processes, if running.
async fn stop_serve() {
    let _ = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        parsentry::webhook::shutdown(),
    )
    .await;
}

#[cfg(unix)]
async fn sigterm() {
    use tokio::signal::unix::{SignalKind, signal};
    match signal(SignalKind::terminate()) {
        Ok(mut s) => {
            s.recv().await;
        }
        Err(_) => std::future::pending().await,
    }
}

#[cfg(not(unix))]
async fn sigterm() {
    std::future::pending().await
}
