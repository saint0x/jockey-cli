use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use crate::error::{Result, JockeyError};

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Md,
    Txt,
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
#[command(name = "jockey", about = "A high-performance CLI tool for converting repositories into structured text formats optimized for LLM ingestion", version)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a structured representation of the repository
    Generate {
        /// Specific subdirectory to process (optional, uses fuzzy matching)
        #[arg(long)]
        path: Option<String>,

        /// Output in markdown format (default)
        #[arg(long, default_value_t = true, conflicts_with_all = &["json", "txt", "yaml"])]
        md: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Output in plain text format
        #[arg(long)]
        txt: bool,

        /// Output in YAML format
        #[arg(long)]
        yaml: bool,

        /// Exclude patterns (comma-separated)
        #[arg(long)]
        exclude: Option<String>,

        /// Enable parallel processing
        #[arg(long)]
        parallel: bool,

        /// Enable verbose logging
        #[arg(long)]
        verbose: bool,
    },
}

impl Config {
    pub fn format(&self) -> OutputFormat {
        match &self.command {
            Commands::Generate { json, txt, yaml, .. } => {
                if *json {
                    OutputFormat::Json
                } else if *txt {
                    OutputFormat::Txt
                } else if *yaml {
                    OutputFormat::Yaml
                } else {
                    OutputFormat::Md
                }
            }
        }
    }

    pub fn get_root_dir() -> Result<PathBuf> {
        // Start from current directory
        let mut current = std::env::current_dir().map_err(|e| {
            JockeyError::Config(format!("Failed to get current directory: {}", e))
        })?;

        // Keep going up until we find a .git directory or reach the root
        while !current.join(".git").exists() {
            if !current.pop() {
                return Err(JockeyError::Config(
                    "Not in a git repository. Please run from within a git project.".to_string(),
                ));
            }
        }

        Ok(current)
    }

    pub fn get_target_dir(&self) -> Result<PathBuf> {
        let root = Self::get_root_dir()?;
        
        match &self.command {
            Commands::Generate { path, .. } => {
                if let Some(path) = path {
                    // TODO: Implement fuzzy matching for subdirectory
                    let target = root.join(path);
                    if !target.exists() {
                        return Err(JockeyError::Config(format!(
                            "Directory '{}' does not exist",
                            path
                        )));
                    }
                    Ok(target)
                } else {
                    Ok(root)
                }
            }
        }
    }

    pub fn is_parallel(&self) -> bool {
        match &self.command {
            Commands::Generate { parallel, .. } => *parallel
        }
    }

    pub fn exclude_patterns(&self) -> Option<&String> {
        match &self.command {
            Commands::Generate { exclude, .. } => exclude.as_ref()
        }
    }
}

pub fn parse_args() -> Result<Config> {
    Ok(Config::parse())
} 