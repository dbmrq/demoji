# demoji

A fast CLI tool to remove or replace emoji characters from source code files. Works as a standalone tool or integrates with IDEs like Xcode and VS Code to show emoji locations as compiler warnings.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

```bash
# Homebrew (macOS)
brew install dbmrq/tap/demoji

# Cargo (cross-platform)
cargo install demoji

# From source
git clone https://github.com/dbmrq/demoji.git
cd demoji && cargo install --path .
```

## Quick Start

```bash
# Clean emojis from source files
demoji src/

# Check for emojis without modifying (for CI/linting)
demoji --check src/

# Preview changes without modifying
demoji --dry-run src/
```

## IDE Integration

### Getting Compiler Warnings in Your IDE

Use `--format gcc` to output in the standard compiler warning format. IDEs like Xcode and VS Code automatically parse this format and show **clickable warnings** at the exact emoji locations.

```bash
demoji --check --format gcc src/
```

Output:
```
/path/to/File.swift:42:15: warning: emoji found: 🎉
/path/to/File.swift:87:22: warning: emoji found: 👍
```

### Xcode Build Phase

Add a **Run Script** build phase to see emoji warnings in Xcode's Issue Navigator:

1. Select your target → Build Phases → + → New Run Script Phase
2. Add this script:

```bash
if command -v demoji &> /dev/null; then
    demoji --format gcc --extensions swift,m,h,mm "$SRCROOT"
fi
```

**Result:** Emojis appear as ⚠️ warnings in the Issue Navigator. Click to jump directly to the line!

To **fail the build** if emojis are found (without auto-fixing):

```bash
if command -v demoji &> /dev/null; then
    demoji --check --format gcc --extensions swift,m,h,mm "$SRCROOT"
fi
```

### VS Code

Add to `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Check Emojis",
      "type": "shell",
      "command": "demoji",
      "args": ["--check", "--format", "gcc", "${workspaceFolder}"],
      "problemMatcher": {
        "pattern": {
          "regexp": "^(.*):(\\d+):(\\d+): warning: (.*)$",
          "file": 1,
          "line": 2,
          "column": 3,
          "message": 4
        }
      }
    }
  ]
}
```

Run the task (Cmd+Shift+P → "Run Task" → "Check Emojis") to see issues in the Problems panel.

### GitHub Actions

Use `--format github` to get inline annotations in pull requests:

```yaml
- name: Check for emojis
  run: |
    cargo install demoji
    demoji --check --format github src/
```

Emojis appear as annotations directly in the PR diff!

## Usage

```bash
demoji [OPTIONS] [PATH]...    # Process files (writes changes by default)
demoji --check [PATH]...      # Check only, exit 1 if emojis found
demoji --dry-run [PATH]...    # Preview changes without writing
demoji watch [PATH]...        # Watch and auto-clean on save
demoji init                   # Create .demoji.toml config
```

### Options

| Option | Description |
|--------|-------------|
| `--write` | Write changes to files (default behavior) |
| `--check` | Check mode - report emojis, exit 1 if found, don't modify |
| `--dry-run` | Preview changes without modifying files |
| `--format <FORMAT>` | Output format: `human`, `gcc`, `json`, `github` |
| `--mode <MODE>` | Replacement mode: `smart`, `remove`, `replace`, `placeholder` |
| `--extensions <EXT>` | File extensions to process (e.g., `swift,m,h`) |
| `--exclude <PATTERN>` | Patterns to exclude |
| `--backup` | Create .bak files before modifying |
| `--placeholder <TEXT>` | Custom placeholder text |
| `-v, --verbose` | Detailed output |
| `-q, --quiet` | Suppress output |

### Output Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| `human` | Colored, human-readable (default) | Terminal use |
| `gcc` | `file:line:col: warning: message` | Xcode, VS Code, any IDE |
| `github` | GitHub Actions annotations | CI with inline PR comments |
| `json` | Machine-readable JSON | Scripting, custom tooling |

### Replacement Modes

| Mode | Description |
|------|-------------|
| `smart` | Replace functional emojis (✅→[OK], ❌→[X]), remove decorative ones |
| `remove` | Delete all emojis entirely |
| `replace` | Replace all emojis with ASCII equivalents |
| `placeholder` | Replace with custom text (use with `--placeholder`) |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (no emojis found, or emojis cleaned successfully) |
| 1 | Emojis found (in `--check` or `--dry-run` mode) |
| 2 | Error (invalid path, permission denied, etc.) |

## Configuration

Create `.demoji.toml` in your project root:

```toml
mode = "smart"               # smart | remove | replace | placeholder
backup = false
placeholder = "[EMOJI]"
extensions = ["swift", "m", "h", "rs", "py", "js"]
ignore_patterns = ["vendor/**", "*.min.js", "Pods/**"]
```

## More Integration Examples

### Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: demoji
        name: demoji
        entry: demoji --check
        language: system
        types: [text]
```

### Makefile

```makefile
lint:
	demoji --check --format gcc src/

fix:
	demoji src/
```

### Git Hook

```bash
# .git/hooks/pre-commit
#!/bin/sh
demoji --check . || {
    echo "Emojis found. Run 'demoji' to clean them."
    exit 1
}
```

## License

MIT

