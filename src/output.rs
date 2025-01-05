use crate::cli::OutputFormat;
use crate::error::{Result, JockeyError};
use serde::Serialize;
use chrono::Local;

const FOOTER: &str = "\n\n---\n\n> 📸 Generated with [Jockey CLI](https://github.com/saint0x/jockey-cli)\n";

#[derive(Serialize)]
pub struct FileEntry {
    pub path: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct Repository {
    pub tree: String,
    pub files: Vec<FileEntry>,
}

pub fn format_output(repo: Repository, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Md => format_markdown(repo),
        OutputFormat::Txt => format_text(repo),
        OutputFormat::Json => format_json(repo),
        OutputFormat::Yaml => format_yaml(repo),
    }
}

fn format_markdown(repo: Repository) -> Result<String> {
    let timestamp = format!("{} at {}", 
        Local::now().format("%m-%d-%Y"),
        Local::now().format("%H:%M:%S")
    );
    let mut output = String::new();
    output.push_str("# Jockey Image\n\n");
    output.push_str(&format!("Generated: {}\n\n", timestamp));
    output.push_str("## Repository Structure\n\n");
    output.push_str("```\n");
    output.push_str(&repo.tree);
    output.push_str("```\n\n");

    for file in repo.files {
        output.push_str(&format!("## File: {}\n\n", file.path));
        
        // Determine the language for syntax highlighting
        let extension = std::path::Path::new(&file.path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        // Use an extra backtick if the content contains triple backticks
        let (start_block, end_block) = if file.content.contains("```") {
            ("````", "````")
        } else {
            ("```", "```")
        };

        // Add language hint for syntax highlighting
        if !extension.is_empty() {
            output.push_str(&format!("{}{}\n", start_block, extension));
        } else {
            output.push_str(start_block);
            output.push('\n');
        }

        // Fix line wrapping by ensuring each line ends with a newline
        for line in file.content.lines() {
            output.push_str(line);
            output.push('\n');
        }
        
        output.push_str(end_block);
        output.push_str("\n\n");
    }

    output.push_str(FOOTER);
    Ok(output)
}

fn format_text(repo: Repository) -> Result<String> {
    let timestamp = format!("{} at {}", 
        Local::now().format("%m-%d-%Y"),
        Local::now().format("%H:%M:%S")
    );
    let mut output = String::new();
    output.push_str("Jockey Image\n============\n\n");
    output.push_str(&format!("Generated: {}\n\n", timestamp));
    output.push_str("Repository Structure:\n\n");
    output.push_str(&repo.tree);
    output.push_str("\n");

    for file in repo.files {
        output.push_str(&format!("File: {}\n", file.path));
        output.push_str("----------------------------------------\n");
        // Fix line wrapping
        for line in file.content.lines() {
            output.push_str(line);
            output.push('\n');
        }
        output.push_str("\n");
    }

    output.push_str(&FOOTER.replace("```", ""));
    Ok(output)
}

fn format_json(repo: Repository) -> Result<String> {
    #[derive(Serialize)]
    struct JockeyImage {
        timestamp: String,
        repository: Repository,
        footer: String,
    }

    let image = JockeyImage {
        timestamp: format!("{} at {}", 
            Local::now().format("%m-%d-%Y"),
            Local::now().format("%H:%M:%S")
        ),
        repository: repo,
        footer: "📸 Generated with Jockey CLI (github.com/saint0x/jockey-cli)".to_string(),
    };

    serde_json::to_string_pretty(&image).map_err(|e| {
        JockeyError::InvalidFormat(format!("Failed to serialize to JSON: {}", e))
    })
}

fn format_yaml(repo: Repository) -> Result<String> {
    #[derive(Serialize)]
    struct JockeyImage {
        timestamp: String,
        repository: Repository,
        footer: String,
    }

    let image = JockeyImage {
        timestamp: format!("{} at {}", 
            Local::now().format("%m-%d-%Y"),
            Local::now().format("%H:%M:%S")
        ),
        repository: repo,
        footer: "📸 Generated with Jockey CLI (github.com/saint0x/jockey-cli)".to_string(),
    };

    serde_yaml::to_string(&image).map_err(|e| {
        JockeyError::InvalidFormat(format!("Failed to serialize to YAML: {}", e))
    })
} 