# demoji

A fast CLI tool to remove or replace emoji characters from text files.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

```bash
# Homebrew
brew install dbmrq/tap/demoji

# Cargo
cargo install demoji

# From source
git clone https://github.com/dbmrq/demoji.git
cd demoji && cargo install --path .
```

## Usage

```bash
demoji [OPTIONS] [PATH]...        # Scan files (dry-run by default)
demoji run [OPTIONS] [PATH]...    # Same as above
demoji watch [OPTIONS] [PATH]...  # Watch for changes
demoji init [PATH]                # Create .demoji.toml config
```

### Options

```
    --dry-run                 Preview changes without modifying files (default)
    --backup                  Create .bak files before modifying
    --mode <MODE>             remove | replace | placeholder [default: remove]
    --placeholder <TEXT>      Custom placeholder text
    --extensions <EXT>        File extensions to process (e.g., rs,py,js)
    --exclude <PATTERN>       Patterns to exclude
-v, --verbose                 Detailed output
-q, --quiet                   Suppress output
-h, --help                    Print help
-V, --version                 Print version
```

### Examples

```bash
# Preview what would change
demoji src/

# Actually remove emojis (removes --dry-run default)
demoji --mode remove src/

# Replace emojis with ASCII equivalents (👍 → [+1], ✅ → [OK], etc.)
demoji --mode replace src/

# Replace with custom placeholder
demoji --mode placeholder --placeholder "[EMOJI]" src/

# Only process specific file types
demoji --extensions rs,py,js src/

# Watch mode
demoji watch src/
```

## Exit Codes

- **0**: No emojis found
- **1**: Emojis found (useful for CI)
- **2**: Error

## Configuration

Create `.demoji.toml` in your project root:

```toml
mode = "remove"              # remove | replace | placeholder
backup = true
placeholder = "[EMOJI]"
extensions = ["rs", "py", "js"]
exclude = ["vendor/**", "*.min.js"]
```

## License

MIT

