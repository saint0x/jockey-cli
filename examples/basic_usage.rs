use jockey_cli::{cli::{Config, Commands}, process};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create configuration programmatically
    let config = Config {
        command: Commands::Generate {
            path: None,
            md: false,
            json: true,
            txt: false,
            yaml: false,
            exclude: Some("target,node_modules".to_string()),
            parallel: true,
            verbose: true,
        },
    };
    
    // Process the repository
    process(config).await?;
    
    Ok(())
} 