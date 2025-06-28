use anyhow::Result;
use dotenvy::dotenv;

use parsentry::cli::RootCommand;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    RootCommand::execute().await
}