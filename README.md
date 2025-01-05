# Jockey CLI

A high-performance CLI tool that converts repositories into structured text formats optimized for LLM ingestion. The tool recursively processes repositories and generates a human-readable output with an ASCII directory tree.

## Features

- 🚀 **High Performance**: Parallel file processing for large codebases
- 🌲 **Directory Tree**: Visual ASCII representation of repository structure
- 📄 **Multiple Formats**: Support for Markdown, Text, JSON, and YAML output
- 🎯 **Smart Exclusions**: Automatically excludes common large directories and binary files
- 📸 **Versioned Output**: Automatically handles multiple snapshots with versioning

## Installation

```bash
cargo install --git https://github.com/saint0x/jockey-cli
```

Or build from source:

```bash
git clone https://github.com/saint0x/jockey-cli
cd jockey-cli
cargo install --path .
```

## Usage

Basic usage (outputs in Markdown format):

```bash
jockey generate
```

### Options

- `--json`: Output in JSON format
- `--txt`: Output in plain text format
- `--yaml`: Output in YAML format
- `--path <PATH>`: Process specific subdirectory (relative to project root)
- `--exclude <PATTERN>`: Additional exclude patterns (comma-separated)

### Examples

Generate JSON output:
```bash
jockey generate --json
```

Process specific subdirectory:
```bash
jockey generate --path src
```

Process subdirectory with specific format:
```bash
jockey generate --path src/lib --json
```

Exclude specific patterns:
```bash
jockey generate --exclude "*.log,temp"
```

## Output

The tool generates a structured output in your chosen format, containing:

1. Timestamp and metadata
2. ASCII directory tree
3. File contents with syntax highlighting
4. Generated files are saved in `jockey-img/` with automatic versioning

Check out a [sample output](jockey-img.md) to see what the generated documentation looks like.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License 