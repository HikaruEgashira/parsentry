use anyhow::Result;
use dotenvy::dotenv;

use parsentry::cli::RootCommand;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    // Handle Ctrl+C gracefully
    tokio::select! {
        result = RootCommand::execute() => result,
        ctrl_c_result = tokio::signal::ctrl_c() => {
            match ctrl_c_result {
                Ok(()) => {
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