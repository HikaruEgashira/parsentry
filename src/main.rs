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
            parsentry::sandbox::kill_all();
            std::process::exit(143);
        }
        ctrl_c_result = tokio::signal::ctrl_c() => {
            match ctrl_c_result {
                Ok(()) => {
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
