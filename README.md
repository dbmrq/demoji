# demoji

A fast, cross-platform CLI tool to remove or replace emoji characters from source code files.

## Features

- 🚀 **Fast**: Built in Rust for maximum performance
- 🔍 **Smart**: Respects `.gitignore` patterns automatically
- 🛡️ **Safe**: Dry-run mode and backup options
- 🎯 **Flexible**: Multiple replacement strategies (remove, replace, placeholder)
- 👀 **Watch mode**: Continuous monitoring for file changes
- ⚙️ **Configurable**: Per-project settings via `.demoji.toml`

## Installation

### From source (requires Rust)

```bash
cargo install --path .
```

### Package managers (coming soon)

- Homebrew: `brew install demoji`
- Cargo: `cargo install demoji`
- npm: `npm install -g demoji`

## Quick Start

```bash
# Check for emojis in current directory
demoji

# Remove emojis from all source files
demoji --write

# Preview changes without modifying files
demoji --dry-run

# Watch for changes and process automatically
demoji watch

# Initialize configuration file
demoji init
```

## Usage

```bash
demoji [OPTIONS] [PATH]

Options:
  --dry-run         Preview changes without modifying files
  --backup          Create .bak files before modifying
  --mode <MODE>     Replacement mode: remove, replace, placeholder [default: remove]
  --pattern <GLOB>  File pattern to process
  --exclude <GLOB>  Pattern to exclude
  --verbose         Show detailed output
  --quiet           Suppress output
  -h, --help        Print help
  -V, --version     Print version
```

## Configuration

Create a `.demoji.toml` file in your project root:

```toml
# Replacement mode: "remove", "replace", or "placeholder"
mode = "remove"

# File patterns to include
include = ["**/*.rs", "**/*.js", "**/*.py"]

# Patterns to exclude (in addition to .gitignore)
exclude = ["vendor/**", "third_party/**"]

# Backup files before modifying
backup = false

# Custom placeholder text (when mode = "placeholder")
placeholder = "[EMOJI]"
```

## Development Status

This project is currently under active development. See the implementation plan in `.villalobos/context/todo.md` for details.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see CONTRIBUTING.md for guidelines (coming soon).

