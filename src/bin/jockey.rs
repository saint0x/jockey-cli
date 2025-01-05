use jockey_cli::cli;
use jockey_cli::error::Result;
use jockey_cli::process;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse command line arguments
    let config = cli::parse_args()?;

    // Process repository
    process(config).await?;

    Ok(())
} 