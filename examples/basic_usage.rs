use jockey_cli::{Config, Command, OutputFormat, process_repository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create configuration programmatically
    let config = Config {
        command: Command::Generate,
        format: OutputFormat::Json,  // Using JSON format for this example
        exclude: Some("target,node_modules".to_string()),
        compressed: true,
        parallel: true,
        verbose: true,
        input_dir: ".".to_string(),  // Current directory
    };
    
    // Process the repository
    process_repository(config).await?;
    println!("Repository processing complete! Check output.zip");
    
    Ok(())
} 