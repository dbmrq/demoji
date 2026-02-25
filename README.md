# demoji

A fast, cross-platform CLI tool to remove or replace emoji characters from source code files.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/demoji.svg)](https://crates.io/crates/demoji)

## Why demoji?

Emoji characters in source code can cause issues with:
- Legacy systems that don't support Unicode properly
- Code review tools and diff viewers
- Terminal emulators with limited Unicode support
- Automated code analysis tools
- Cross-platform compatibility

`demoji` helps you maintain clean, portable source code by detecting and removing or replacing emoji characters while respecting your project's structure and `.gitignore` patterns.

## Features

- 🚀 **Fast**: Built in Rust for maximum performance with parallel file processing
- 🔍 **Smart**: Respects `.gitignore` patterns automatically
- 🛡️ **Safe**: Dry-run mode, backup options, and atomic file writes
- 🎯 **Flexible**: Multiple replacement strategies (remove, replace with ASCII, custom placeholder)
- 👀 **Watch mode**: Continuous monitoring for file changes
- ⚙️ **Configurable**: Per-project settings via `.demoji.toml`
- 🌍 **Cross-platform**: Works on Linux, macOS, and Windows
- 📊 **Detailed reporting**: Shows exactly what was found and where

## Installation

### Homebrew (macOS/Linux)

```bash
brew install demoji
```

### Cargo (from crates.io)

```bash
cargo install demoji
```

### From source

```bash
git clone https://github.com/yourusername/demoji.git
cd demoji
cargo install --path .
```

### Binary downloads

Download pre-built binaries from the [releases page](https://github.com/yourusername/demoji/releases).

### npm (wrapper)

```bash
npm install -g demoji
```

## Quick Start

```bash
# Check for emojis in current directory (dry-run by default)
demoji

# Remove emojis from all source files
demoji run --write

# Preview changes without modifying files
demoji run --dry-run

# Watch for changes and process automatically
demoji watch

# Initialize configuration file
demoji init
```

## Usage

### Basic Commands

#### `demoji run` (default)

Process files once and exit.

```bash
# Check current directory
demoji run

# Process specific directory
demoji run ./src

# Remove emojis (modify files)
demoji run --write

# Create backups before modifying
demoji run --write --backup

# Use different replacement mode
demoji run --write --mode replace
demoji run --write --mode placeholder --placeholder "[EMOJI_REMOVED]"
```

#### `demoji watch`

Continuously monitor files for changes.

```bash
# Watch current directory
demoji watch

# Watch specific directory
demoji watch ./src

# Watch with automatic processing
demoji watch --write
```

#### `demoji init`

Create a `.demoji.toml` configuration file.

```bash
# Interactive mode (if terminal supports it)
demoji init

# Create with defaults
demoji init --defaults
```

### Command-Line Options

```
demoji [SUBCOMMAND] [OPTIONS] [PATH]

SUBCOMMANDS:
    run         Process files once (default)
    watch       Watch for file changes and process automatically
    init        Create a .demoji.toml configuration file

OPTIONS:
    -w, --write              Modify files (default is dry-run)
    -b, --backup             Create .bak files before modifying
    -m, --mode <MODE>        Replacement mode [default: remove]
                             Values: remove, replace, placeholder
    -p, --placeholder <TEXT> Custom placeholder text (for placeholder mode)
    --pattern <GLOB>         File pattern to include (can be repeated)
    --exclude <GLOB>         Pattern to exclude (can be repeated)
    -v, --verbose            Show detailed output
    -q, --quiet              Suppress output (only errors)
    --check                  Exit with code 1 if emojis found (CI mode)
    -h, --help               Print help
    -V, --version            Print version

ARGUMENTS:
    [PATH]                   Directory or file to process [default: .]
```

### Replacement Modes

#### Remove (default)

Removes emoji characters entirely.

```bash
demoji run --write --mode remove
```

**Example:**
```
Before: const greeting = "Hello 👋 World 🌍";
After:  const greeting = "Hello  World ";
```

#### Replace

Replaces emojis with ASCII alternatives.

```bash
demoji run --write --mode replace
```

**Example:**
```
Before: const status = "✅ Success";
After:  const status = "[OK] Success";

Before: const error = "❌ Failed";
After:  const error = "[X] Failed";
```

**Built-in mappings:**
- 👍 → `[+1]`
- 👎 → `[-1]`
- ✅ → `[OK]`
- ❌ → `[X]`
- ⚠️ → `[!]`
- 🔥 → `[FIRE]`
- 💡 → `[IDEA]`
- And many more...

#### Placeholder

Replaces all emojis with a custom placeholder.

```bash
demoji run --write --mode placeholder --placeholder "[EMOJI]"
```

**Example:**
```
Before: const msg = "Hello 👋 World 🌍";
After:  const msg = "Hello [EMOJI] World [EMOJI]";
```

## Configuration

Create a `.demoji.toml` file in your project root or home directory (`~/.demoji.toml`) for persistent settings.

### Example Configuration

```toml
# Replacement mode: "remove", "replace", or "placeholder"
mode = "remove"

# Automatically write changes (disable dry-run)
write = false

# Create backups before modifying files
backup = true

# Custom placeholder text (when mode = "placeholder")
placeholder = "[EMOJI_REMOVED]"

# File patterns to include (glob patterns)
include = [
    "**/*.rs",
    "**/*.js",
    "**/*.ts",
    "**/*.py",
    "**/*.go",
    "**/*.java",
]

# Patterns to exclude (in addition to .gitignore)
exclude = [
    "vendor/**",
    "third_party/**",
    "node_modules/**",
    "*.min.js",
]

# Follow symbolic links (default: false)
follow_links = false
```

### Configuration Priority

Configuration is merged in this order (highest priority first):

1. Command-line arguments
2. Project `.demoji.toml` (in current directory or parent directories)
3. Global `~/.demoji.toml`
4. Built-in defaults

### Default Ignore Patterns

`demoji` automatically ignores:

**Directories:**
- `.git`, `.svn`, `.hg`
- `node_modules`, `vendor`, `third_party`
- `target`, `build`, `dist`, `.next`
- `__pycache__`, `.venv`, `venv`

**File extensions:**
- Binary files: `.png`, `.jpg`, `.gif`, `.ico`, `.pdf`
- Archives: `.zip`, `.tar`, `.gz`, `.7z`
- Executables: `.exe`, `.dll`, `.so`, `.dylib`
- Fonts: `.woff`, `.woff2`, `.ttf`, `.otf`

## Exit Codes

`demoji` uses standard exit codes for scripting and CI integration:

- **0**: Success (no emojis found, or emojis successfully processed)
- **1**: Emojis were found (useful with `--check` flag in CI)
- **2**: Error occurred (IO error, permission denied, invalid config, etc.)

### CI Integration Example

```bash
# Fail CI build if emojis are found
demoji run --check || exit 1

# Or with GitHub Actions
- name: Check for emojis
  run: demoji run --check
```

## Examples

### Clean a specific directory

```bash
demoji run --write ./src
```

### Process only Python files

```bash
demoji run --write --pattern "**/*.py"
```

### Exclude test files

```bash
demoji run --write --exclude "**/*_test.rs" --exclude "**/test_*.py"
```

### Watch mode with backups

```bash
demoji watch --write --backup
```

### CI mode (fail if emojis found)

```bash
demoji run --check
```

### Verbose output

```bash
demoji run --verbose
```

**Output example:**
```
Scanning directory: ./src
Found emoji at src/main.rs:42:15 - 👋 (U+1F44B)
Found emoji at src/lib.rs:10:8 - 🚀 (U+1F680)

Summary:
  Files scanned: 15
  Files with emojis: 2
  Total emojis found: 2

Run with --write to modify files.
```

## Watch Mode

Watch mode continuously monitors your project for file changes and processes them automatically.

```bash
# Start watching
demoji watch --write

# Watch with verbose output
demoji watch --write --verbose
```

**Features:**
- Debounces rapid changes (100ms)
- Respects same ignore patterns as batch mode
- Graceful shutdown on Ctrl+C
- Only processes changed files (efficient)

## Development Status

This project is under active development. Current implementation status:

- ✅ Phase 1: Project scaffolding
- 🚧 Phase 2-9: Core functionality (in progress)
- 📝 Phase 10: Documentation (this file)

See the [implementation plan](.villalobos/context/todo.md) for details.

## Performance

`demoji` is designed for speed:

- Parallel file processing using Rayon
- Streaming processing for large files (doesn't load entire file into memory)
- Efficient Unicode handling
- Respects `.gitignore` to skip unnecessary files

**Benchmarks** (coming soon)

## Troubleshooting

### "Permission denied" errors

Make sure you have write permissions for the files you're trying to modify. Use `--dry-run` first to preview changes.

### Binary files being processed

`demoji` should automatically skip binary files. If you encounter issues, use `--exclude` to explicitly exclude them:

```bash
demoji run --exclude "**/*.png" --exclude "**/*.jpg"
```

### Emoji not detected

Some emoji sequences (especially newer ones with ZWJ sequences or skin tone modifiers) may not be detected. Please [open an issue](https://github.com/yourusername/demoji/issues) with examples.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) - Gitignore handling
- [notify](https://github.com/notify-rs/notify) - File system watching
- [rayon](https://github.com/rayon-rs/rayon) - Parallel processing

