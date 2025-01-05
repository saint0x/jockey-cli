use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::error::{Result, JockeyError};
use ignore::WalkBuilder;

const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    // Jockey-specific
    "jockey-img/", "jockey-img", ".jockey/", ".jockey-cache/",
    // Build directories
    "target/", "build/", "dist/", "out/",
    // Dependencies
    "node_modules/", "vendor/", "packages/", "bower_components/",
    // IDE and editor files
    ".idea/", ".vscode/", "*.iml", ".project", ".classpath",
    // Logs and caches
    "*.log", "logs/", ".cache/", "tmp/", "temp/",
    // Binary and compiled files
    "*.exe", "*.dll", "*.so", "*.dylib", "*.class", "*.o", "*.obj",
    // Package manager files
    "package-lock.json", "yarn.lock", "Cargo.lock", "Gemfile.lock",
    // Large media files
    "*.mp4", "*.mov", "*.avi", "*.mkv", "*.mp3", "*.wav",
    "*.iso", "*.dmg", "*.tar", "*.gz", "*.zip", "*.rar",
    // Database files
    "*.db", "*.sqlite", "*.sqlite3",
    // Environment and secrets
    ".env", ".env.*", "*.pem", "*.key",
];

#[derive(Clone)]
pub struct TreeBuilder {
    name: String,
    prefix: String,
    is_last: bool,
    children: Vec<TreeBuilder>,
}

impl TreeBuilder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            name: path.as_ref()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            prefix: String::new(),
            is_last: false,
            children: Vec::new(),
        }
    }

    pub fn process_directory(root: impl AsRef<Path>, exclude_patterns: Option<&String>) -> Result<(String, Vec<PathBuf>)> {
        let root = root.as_ref();
        let mut files = Vec::new();
        let mut nodes = HashMap::new();
        
        // Build walker with ignore patterns
        let mut walker = WalkBuilder::new(root);
        walker.hidden(true); // Skip hidden files by default
        
        // Add default ignore patterns
        for pattern in DEFAULT_IGNORE_PATTERNS {
            walker.add_ignore(pattern);
        }
        
        // Add user-specified patterns
        if let Some(patterns) = exclude_patterns {
            for pattern in patterns.split(',') {
                walker.add_ignore(pattern.trim());
            }
        }
        
        // First pass: collect all paths and create nodes
        for entry in walker.build() {
            let entry = entry.map_err(|e| {
                JockeyError::Processing(format!("Failed to read directory entry: {}", e))
            })?;
            
            let path = entry.path().to_path_buf();
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                files.push(path.clone());
            }
            nodes.insert(path, TreeBuilder::new(entry.path()));
        }
        
        // Second pass: build tree structure
        let mut root_node = TreeBuilder::new(root);
        let to_process = nodes.keys().cloned().collect::<Vec<_>>();
        
        for path in to_process {
            if let Some(parent) = path.parent() {
                if let Some(node) = nodes.remove(&path) {
                    if parent == root {
                        root_node.add_child(node);
                    } else {
                        let parent_path = parent.to_path_buf();
                        if let Some(parent_node) = nodes.get_mut(&parent_path) {
                            parent_node.add_child(node);
                        }
                    }
                }
            }
        }

        Ok((root_node.build(), files))
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn build(&self) -> String {
        let mut result = String::with_capacity(4096); // Pre-allocate buffer
        self.build_internal(&mut result, "");
        result
    }

    fn build_internal(&self, result: &mut String, prefix: &str) {
        result.push_str(prefix);
        if !self.prefix.is_empty() {
            result.push_str(&self.prefix);
        }
        result.push_str(&self.name);
        result.push('\n');

        let len = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last = i == len - 1;
            let child_prefix = if is_last { "└── " } else { "├── " };
            let child_continue = if is_last { "    " } else { "│   " };
            
            let mut child = child.clone();
            child.prefix = child_prefix.to_string();
            child.is_last = is_last;
            child.build_internal(result, &format!("{}{}", prefix, child_continue));
        }
    }
} 