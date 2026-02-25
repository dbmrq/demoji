# demoji - npm wrapper

This is an npm wrapper package for the [demoji](https://github.com/yourusername/demoji) CLI tool.

## Installation

```bash
npm install -g demoji
```

## Usage

```bash
demoji --help                    # Show help
demoji --dry-run src/            # Preview changes
demoji src/                      # Remove emojis from source files
demoji init                      # Create .demoji.toml config
demoji watch src/                # Watch for file changes
```

## About demoji

demoji is a fast, cross-platform CLI tool to remove or replace emoji characters from source code files.

### Features

- 🚀 **Fast**: Built in Rust for maximum performance
- 🔍 **Smart**: Respects `.gitignore` patterns automatically
- 🛡️ **Safe**: Dry-run mode, backup options, and atomic file writes
- 🎯 **Flexible**: Multiple replacement strategies
- 👀 **Watch mode**: Continuous monitoring for file changes
- ⚙️ **Configurable**: Per-project settings via `.demoji.toml`
- 🌍 **Cross-platform**: Works on Linux, macOS, and Windows

## Installation Methods

### npm (this package)

```bash
npm install -g demoji
```

### Homebrew

```bash
brew install demoji
```

### Cargo

```bash
cargo install demoji
```

### From source

```bash
git clone https://github.com/yourusername/demoji.git
cd demoji
cargo install --path .
```

## Documentation

For full documentation, visit: https://github.com/yourusername/demoji

## License

MIT
