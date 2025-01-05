use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use crate::error::{Result, JockeyError};
use glob;

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

const ROOT_INDICATORS: &[&str] = &[
    // Common Project Root Folders
    "src/", "app/", "lib/", "test/", "tests/", "docs/", "scripts/",
    "packages/", "internal/", "cmd/", "bin/", "include/",
    
    // Version Control
    ".git", ".hg", ".svn", ".bzr",
    
    // JavaScript/TypeScript/Node.js
    "package.json", "package-lock.json", "yarn.lock", "pnpm-lock.yaml",
    "tsconfig.json", "next.config.js", "nuxt.config.js", "angular.json",
    
    // Rust
    "Cargo.toml", "Cargo.lock",
    
    // Python
    "requirements.txt", "setup.py", "pyproject.toml", "Pipfile", "poetry.lock",
    
    // Ruby
    "Gemfile", "Gemfile.lock", ".ruby-version", "config.ru", "Rakefile",
    
    // PHP
    "composer.json", "composer.lock", "artisan", ".env.example",
    
    // Java/Kotlin/Scala
    "pom.xml", "build.gradle", "settings.gradle", "gradlew", "build.sbt",
    "mvnw", ".mvn", ".gradle",
    
    // Go
    "go.mod", "go.sum", "Gopkg.toml", "Gopkg.lock",
    
    // C/C++
    "CMakeLists.txt", "Makefile", "configure", "meson.build",
    "conanfile.txt", "vcpkg.json", "compile_commands.json",
    
    // .NET/C#
    "*.sln", "*.csproj", "*.fsproj", "*.vbproj", "packages.config",
    
    // Swift/iOS
    "Package.swift", "*.xcodeproj", "*.xcworkspace", "Podfile",
    
    // Elixir/Erlang
    "mix.exs", "rebar.config", "erlang.mk",
    
    // Docker/Containers
    "Dockerfile", "docker-compose.yml", "docker-compose.yaml",
    
    // Web
    "index.html", "webpack.config.js", "vite.config.js", "rollup.config.js",
    
    // Generic Project Files
    "README.md", "LICENSE", ".editorconfig", ".gitlab-ci.yml", ".travis.yml",
    "netlify.toml", "vercel.json", ".env", "Procfile",
    
    // Workspace/IDE Config
    ".vscode", ".idea", ".eclipse", ".project"
];

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

        // Keep going up until we find a project root indicator or reach the root
        loop {
            // First check if current directory has any subdirectories
            if let Ok(entries) = std::fs::read_dir(&current) {
                let has_subdirs = entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false));
                
                if has_subdirs {
                    return Ok(current);
                }
            }

            // If no subdirs found, check for specific root indicators as fallback
            let found = ROOT_INDICATORS.iter().any(|&indicator| {
                if indicator.ends_with('/') {
                    // Handle directory indicators
                    current.join(&indicator[..indicator.len()-1]).is_dir()
                } else if indicator.contains('*') {
                    // Handle glob patterns
                    if let Ok(entries) = std::fs::read_dir(&current) {
                        let pattern = glob::Pattern::new(indicator).unwrap_or_else(|_| glob::Pattern::new("").unwrap());
                        entries
                            .filter_map(|e| e.ok())
                            .any(|e| {
                                if let Some(name) = e.file_name().to_str() {
                                    pattern.matches(name)
                                } else {
                                    false
                                }
                            })
                    } else {
                        false
                    }
                } else {
                    // Direct file check
                    current.join(indicator).exists()
                }
            });

            if found {
                return Ok(current);
            }

            // Go up one level
            if !current.pop() {
                return Err(JockeyError::Config(
                    "Not in a project directory. Please run from within a project directory.".to_string(),
                ));
            }
        }
    }

    pub fn get_target_dir(&self) -> Result<PathBuf> {
        let root = Self::get_root_dir()?;
        
        match &self.command {
            Commands::Generate { path, .. } => {
                if let Some(path) = path {
                    // Normalize and resolve the path
                    let target = if path.starts_with("/") {
                        PathBuf::from(path)
                    } else {
                        root.join(path)
                    };

                    // Canonicalize to resolve any '..' or '.' components
                    let canonical = target.canonicalize().map_err(|e| {
                        JockeyError::Config(format!(
                            "Failed to resolve path '{}': {}",
                            path, e
                        ))
                    })?;

                    // Ensure the path is within the project root
                    if !canonical.starts_with(&root) {
                        return Err(JockeyError::Config(format!(
                            "Path '{}' is outside the project root",
                            path
                        )));
                    }

                    // Ensure it's a directory
                    if !canonical.is_dir() {
        return Err(JockeyError::Config(format!(
                            "Path '{}' is not a directory",
                            path
                        )));
                    }

                    Ok(canonical)
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