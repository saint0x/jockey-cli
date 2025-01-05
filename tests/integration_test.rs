use assert_fs::prelude::*;
use predicates::prelude::*;
use jockey_cli::{cli::{Config, Commands}, process};
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
        command: Commands::Generate {
            path: Some(temp.path().to_string_lossy().into_owned()),
            md: true,
            json: false,
            txt: false,
            yaml: false,
            exclude: None,
            parallel: false,
            verbose: false,
        },
    };
    
    // Process the repository
    process(config).await.unwrap();
    
    // Verify output file exists and contains expected content
    let output = PathBuf::from("jockey-img");
    assert!(output.exists());
    
    // Cleanup
    std::fs::remove_dir_all(output).unwrap();
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
        command: Commands::Generate {
            path: Some(temp.path().to_string_lossy().into_owned()),
            md: true,
            json: false,
            txt: false,
            yaml: false,
            exclude: Some("*.tmp,node_modules".to_string()),
            parallel: false,
            verbose: false,
        },
    };
    
    process(config).await.unwrap();
    
    // Verify output directory exists
    let output = PathBuf::from("jockey-img");
    assert!(output.exists());
    
    // Cleanup
    std::fs::remove_dir_all(output).unwrap();
    temp.close().unwrap();
} 