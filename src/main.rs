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
        _ = tokio::signal::ctrl_c() => {
            eprintln!("\nInterrupted by user");
            std::process::exit(130);
        }
    }
}