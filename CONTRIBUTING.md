# Contributing to demoji

Thank you for your interest in contributing to demoji! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Building and Testing](#building-and-testing)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Project Structure](#project-structure)

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build something useful together.

## Getting Started

### Prerequisites

- **Rust toolchain** (1.70.0 or later)
  - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Or via Homebrew: `brew install rust`
- **Git** for version control
- A text editor or IDE (VS Code with rust-analyzer recommended)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/demoji.git
   cd demoji
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/yourusername/demoji.git
   ```

## Development Setup

### Install Dependencies

Dependencies are managed by Cargo and will be installed automatically when you build:

```bash
cargo build
```

### Verify Installation

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

## Building and Testing

### Build Commands

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Build and run
cargo run -- [ARGS]

# Example: Run on test fixtures
cargo run -- tests/fixtures --dry-run
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests only
cargo test --test '*'

# Run with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Test Structure

- `src/*/tests.rs` or `src/*_tests.rs` - Unit tests (inline with modules)
- `tests/integration/` - Integration tests
- `tests/fixtures/` - Test files with emoji characters

### Adding Tests

When adding new functionality:

1. **Unit tests**: Add tests in the same file or a `tests` submodule
2. **Integration tests**: Add to `tests/integration/` for end-to-end scenarios
3. **Fixtures**: Add test files to `tests/fixtures/` if needed

Example unit test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_detection() {
        let detector = EmojiDetector::new();
        assert!(detector.is_emoji('👋'));
        assert!(!detector.is_emoji('a'));
    }
}
```

## Code Style

### Formatting

We use `rustfmt` for consistent code formatting:

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Linting

We use `clippy` for linting:

```bash
# Run clippy
cargo clippy

# Run clippy with strict settings
cargo clippy -- -D warnings

# Fix automatically fixable issues
cargo clippy --fix
```

### Code Quality Standards

- **All code must pass `cargo fmt` and `cargo clippy`** before submitting PR
- Write clear, self-documenting code with meaningful variable names
- Add doc comments (`///`) for public APIs
- Keep functions focused and small (prefer < 50 lines)
- Handle errors properly (use `Result` and `?` operator)
- Avoid `unwrap()` and `expect()` in library code (OK in tests and examples)

### Documentation

- Add doc comments for all public items (modules, structs, functions, traits)
- Include examples in doc comments where helpful
- Update README.md if adding user-facing features
- Use `cargo doc --open` to preview documentation

Example:

```rust
/// Detects emoji characters in text.
///
/// # Examples
///
/// ```
/// use demoji::EmojiDetector;
///
/// let detector = EmojiDetector::new();
/// assert!(detector.is_emoji('👋'));
/// ```
pub struct EmojiDetector {
    // ...
}
```

## Pull Request Process

### Before Submitting

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines

3. **Run the full test suite**:
   ```bash
   cargo test
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

4. **Update documentation** if needed (README.md, doc comments)

5. **Commit with clear messages**:
   ```bash
   git commit -m "Add feature: brief description"
   ```
   
   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `test:` - Adding or updating tests
   - `refactor:` - Code refactoring
   - `perf:` - Performance improvements
   - `chore:` - Maintenance tasks

### Submitting the PR

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub

3. **Fill out the PR template** with:
   - Description of changes
   - Related issue number (if applicable)
   - Testing performed
   - Screenshots (if UI changes)

4. **Respond to review feedback** promptly

5. **Ensure CI passes** - All tests and checks must pass

### PR Review Process

- Maintainers will review your PR within a few days
- Address any requested changes
- Once approved, a maintainer will merge your PR
- Your contribution will be included in the next release!

## Project Structure

Understanding the codebase organization:

```
demoji/
├── Cargo.toml              # Project manifest and dependencies
├── LICENSE                 # MIT license
├── README.md               # User documentation
├── CONTRIBUTING.md         # This file
├── src/
│   ├── main.rs            # CLI entry point (minimal)
│   ├── lib.rs             # Public API and re-exports
│   ├── cli/               # Command-line interface
│   │   ├── mod.rs
│   │   ├── args.rs        # Argument parsing (clap)
│   │   └── output.rs      # Output formatting and reporting
│   ├── core/              # Core functionality
│   │   ├── mod.rs
│   │   ├── emoji.rs       # Emoji detection logic
│   │   ├── replacer.rs    # Replacement strategies
│   │   ├── processor.rs   # File processing
│   │   ├── walker.rs      # Directory traversal
│   │   └── backup.rs      # Backup management
│   ├── config/            # Configuration handling
│   │   └── mod.rs         # .demoji.toml parsing
│   └── watch/             # File watching
│       └── mod.rs         # Watch mode implementation
└── tests/
    ├── integration/       # Integration tests
    │   └── basic_test.rs
    └── fixtures/          # Test files with emojis
```

### Module Responsibilities

- **`cli/`**: Handles all command-line interaction (parsing args, formatting output)
- **`core/`**: Core business logic (emoji detection, file processing, replacement)
- **`config/`**: Configuration file loading and merging
- **`watch/`**: File system watching for continuous mode

### Key Design Principles

1. **Separation of concerns**: CLI, core logic, and config are independent
2. **Testability**: Core logic has no I/O dependencies (uses traits/dependency injection)
3. **Error handling**: Use `Result` and custom error types (via `thiserror`)
4. **Performance**: Parallel processing with `rayon`, streaming for large files
5. **Safety**: Atomic writes, backup support, dry-run by default

## Development Workflow

### Typical Development Cycle

1. **Pick an issue** or create one for discussion
2. **Create a branch**: `git checkout -b feature/issue-123-description`
3. **Write tests first** (TDD approach recommended)
4. **Implement the feature**
5. **Run tests**: `cargo test`
6. **Format and lint**: `cargo fmt && cargo clippy`
7. **Test manually**: `cargo run -- [args]`
8. **Commit and push**
9. **Open PR**

### Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo run -- [args]

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run -- [args]

# Use rust-gdb or rust-lldb for debugging
rust-gdb target/debug/demoji
```

### Performance Profiling

```bash
# Build with profiling symbols
cargo build --release --profile profiling

# Use perf (Linux)
perf record target/release/demoji [args]
perf report

# Use Instruments (macOS)
# Or use cargo-flamegraph
cargo install flamegraph
cargo flamegraph -- [args]
```

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/yourusername/demoji/discussions)
- **Bugs**: Open a [GitHub Issue](https://github.com/yourusername/demoji/issues)
- **Chat**: Join our community (link TBD)

## Recognition

Contributors will be:
- Listed in release notes
- Added to the contributors list (automatically via GitHub)
- Credited in the README for significant contributions

Thank you for contributing to demoji! 🎉

