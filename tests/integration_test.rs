use assert_fs::prelude::*;
use predicates::prelude::*;
use jockey_cli::{Config, Command, OutputFormat, process_repository};
use std::path::PathBuf;

#[tokio::test]
async fn test_basic_markdown_generation() {
    // Create a temporary directory with some test files
    let temp = assert_fs::TempDir::new().unwrap();
    
    // Create a simple file structure
    temp.child("file1.txt").write_str("Hello, World!").unwrap();
    temp.child("dir1").create_dir_all().unwrap();
    temp.child("dir1/file2.txt").write_str("Test content").unwrap();
    
    // Create test configuration
    let config = Config {
        command: Command::Generate,
        format: OutputFormat::Md,
        exclude: None,
        compressed: false,
        parallel: false,
        verbose: false,
        input_dir: temp.path().to_string_lossy().into_owned(),
    };
    
    // Process the repository
    process_repository(config).await.unwrap();
    
    // Verify output file exists and contains expected content
    let output = PathBuf::from("output.md");
    assert!(output.exists());
    
    let content = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("Repository Structure"));
    assert!(content.contains("file1.txt"));
    assert!(content.contains("dir1"));
    assert!(content.contains("file2.txt"));
    assert!(content.contains("Hello, World!"));
    assert!(content.contains("Test content"));
    
    // Cleanup
    std::fs::remove_file(output).unwrap();
    temp.close().unwrap();
}

#[tokio::test]
async fn test_compressed_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("test.txt").write_str("Compressed content test").unwrap();
    
    let config = Config {
        command: Command::Generate,
        format: OutputFormat::Txt,
        exclude: None,
        compressed: true,
        parallel: false,
        verbose: false,
        input_dir: temp.path().to_string_lossy().into_owned(),
    };
    
    process_repository(config).await.unwrap();
    
    // Verify zip file exists
    let output = PathBuf::from("output.zip");
    assert!(output.exists());
    
    // Cleanup
    std::fs::remove_file(output).unwrap();
    temp.close().unwrap();
}

#[tokio::test]
async fn test_exclude_patterns() {
    let temp = assert_fs::TempDir::new().unwrap();
    
    // Create files and directories
    temp.child("include.txt").write_str("Include this").unwrap();
    temp.child("exclude.tmp").write_str("Exclude this").unwrap();
    temp.child("node_modules/test.js").create_dir_all().unwrap();
    
    let config = Config {
        command: Command::Generate,
        format: OutputFormat::Md,
        exclude: Some("*.tmp,node_modules".to_string()),
        compressed: false,
        parallel: false,
        verbose: false,
        input_dir: temp.path().to_string_lossy().into_owned(),
    };
    
    process_repository(config).await.unwrap();
    
    let output = PathBuf::from("output.md");
    let content = std::fs::read_to_string(&output).unwrap();
    
    // Verify exclusions
    assert!(content.contains("include.txt"));
    assert!(!content.contains("exclude.tmp"));
    assert!(!content.contains("node_modules"));
    
    // Cleanup
    std::fs::remove_file(output).unwrap();
    temp.close().unwrap();
} 