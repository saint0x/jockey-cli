use crate::cli::{Commands, Config};
use crate::error::{Result, JockeyError};
use crate::output::{FileEntry, Repository, format_output};
use crate::tree::TreeBuilder;
use rayon::prelude::*;
use tokio::fs;
use chrono::Local;
use colored::*;

const OUTPUT_DIR: &str = "jockey-img";

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

async fn get_unique_filename(dir: &std::path::Path, base_name: &str, extension: &str) -> Result<String> {
    let mut counter = 0;
    let mut filename = format!("{}_{}.{}", base_name, Local::now().format("%m-%d-%y"), extension);
    
    while dir.join(&filename).exists() {
        counter += 1;
        filename = format!("{}_{}{}.{}", 
            base_name, 
            Local::now().format("%m-%d-%y"),
            format!("({})", counter),
            extension
        );
    }
    
    Ok(filename)
}

pub async fn process(config: Config) -> Result<()> {
    match &config.command {
        Commands::Generate { .. } => generate(&config).await,
    }
}

async fn generate(config: &Config) -> Result<()> {
    // Get project root and target directory
    let root_dir = Config::get_root_dir()?;
    let target_dir = config.get_target_dir()?;
    
    // Build directory tree and collect files
    let (tree, files) = TreeBuilder::process_directory(&target_dir, config.exclude_patterns())?;
    
    // Process files in parallel for better performance on large codebases
    let processed_files = files
        .par_iter()
        .map(|path| {
            let content = std::fs::read_to_string(path).map_err(|e| {
                JockeyError::Processing(format!("Failed to read file {}: {}", path.display(), e))
            })?;
            
            Ok(FileEntry {
                path: path.to_string_lossy().into_owned(),
                content,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // Create repository structure
    let repo = Repository {
        tree,
        files: processed_files,
    };

    // Format output
    let output = format_output(repo, config.format())?;
    
    // Ensure output directory exists
    let jockey_dir = root_dir.join(OUTPUT_DIR);
    fs::create_dir_all(&jockey_dir).await.map_err(|e| {
        JockeyError::Processing(format!("Failed to create {} directory: {}", OUTPUT_DIR, e))
    })?;

    // Generate output path with project name and date
    let project_name = root_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");
    let extension = match config.format() {
        crate::cli::OutputFormat::Md => "md",
        crate::cli::OutputFormat::Txt => "txt",
        crate::cli::OutputFormat::Json => "json",
        crate::cli::OutputFormat::Yaml => "yaml",
    };
    
    // Get unique filename
    let filename = get_unique_filename(&jockey_dir, project_name, extension).await?;
    let output_path = jockey_dir.join(&filename);
    
    // Write output in a single operation
    fs::write(&output_path, output).await.map_err(|e| {
        JockeyError::Processing(format!("Failed to write output file: {}", e))
    })?;

    // Get and format file size
    let file_size = fs::metadata(&output_path).await.map_err(|e| {
        JockeyError::Processing(format!("Failed to get file size: {}", e))
    })?.len();

    // Get relative path for display
    let relative_path = output_path.strip_prefix(&root_dir)
        .unwrap_or(&output_path)
        .to_string_lossy();

    // Print success message
    println!("\n{}", "Jockey image created successfully!".green().bold());
    println!("{} {} ({})", 
        "Location:".blue(),
        relative_path.yellow(),
        format_file_size(file_size).cyan()
    );
    
    Ok(())
} 