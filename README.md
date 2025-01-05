# Jockey CLI

A high-performance CLI tool that converts Git repositories into structured text formats optimized for LLM ingestion. The tool recursively processes repositories, compresses file contents, and generates a human-readable output with an ASCII directory tree.

## Features

- 🚀 **High Performance**: Parallel file processing and efficient compression
- 🌲 **Directory Tree**: Visual ASCII representation of repository structure
- 🗜️ **Compression**: Automatic lossless compression of file contents
- 📄 **Multiple Formats**: Support for Markdown, Text, JSON, and YAML output
- 🎯 **Selective Processing**: Exclude files/directories using .gitignore patterns
- 📦 **ZIP Output**: Optional ZIP archive output

## Installation

```bash
cargo install jockey-cli
```

Or build from source:

```bash
git clone https://github.com/yourusername/jockey-cli
cd jockey-cli
cargo build --release
```

## Usage

Basic usage (outputs in Markdown format):

```bash
jockey generate
```

### Options

- `--format <FORMAT>`: Output format (md, txt, json, yaml)
- `--exclude <PATTERN>`: Exclude files/directories (comma-separated)
- `--compressed`: Output as ZIP archive
- `--parallel`: Enable parallel processing
- `--verbose`: Enable verbose logging

### Examples

Generate JSON output:
```bash
jockey generate --format json
```

Exclude node_modules and target directories:
```bash
jockey generate --exclude "node_modules,target"
```

Generate compressed output:
```bash
jockey generate --compressed
```

Enable parallel processing with verbose logging:
```bash
jockey generate --parallel --verbose
```

## Output Format

The tool generates a structured output containing:

1. ASCII directory tree
2. File contents with paths
3. Compressed content (when enabled)

Example Markdown output:

```markdown
# Repository Structure

```
/project-root
├── src
│   ├── main.rs
│   └── utils.rs
├── README.md
└── Cargo.toml
```

## File: src/main.rs

```rust
// File contents here
```

## File: src/utils.rs

```rust
// File contents here
```
```

## Performance

- Parallel processing for large repositories
- Efficient compression using flate2
- O(1) memory usage for file processing
- Optimized for repositories up to 10GB

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License 