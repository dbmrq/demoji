# Distribution Testing Guide

This document describes how to manually test the demoji distribution packages before release.

## Table of Contents

1. [Testing `cargo install`](#testing-cargo-install)
2. [Testing Homebrew Installation](#testing-homebrew-installation)
3. [Testing npm Package](#testing-npm-package)
4. [Automated CI Testing](#automated-ci-testing)

---

## Testing `cargo install`

### From Local Path

Test that the binary can be installed from the local repository:

```bash
# Install from current directory
cargo install --path .

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt
```

### From Published Crate

Once published to crates.io, test installation from the registry:

```bash
# Install from crates.io
cargo install demoji

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt
```

### Smoke Test Checklist

- [ ] Binary installs without errors
- [ ] `demoji --version` displays version number
- [ ] `demoji --help` shows usage information
- [ ] `demoji --dry-run <file>` works with emoji-containing files
- [ ] `demoji run --help` shows run subcommand help
- [ ] `demoji watch --help` shows watch subcommand help
- [ ] `demoji init --help` shows init subcommand help
- [ ] Exit codes are correct (0 for success, 1 for emojis found, 2 for errors)

---

## Testing Homebrew Installation

### Prerequisites

- Homebrew installed on macOS
- The Homebrew formula file at `homebrew/demoji.rb`

### Testing Local Formula

```bash
# Create a local tap for testing
mkdir -p ~/homebrew-demoji-test
cp homebrew/demoji.rb ~/homebrew-demoji-test/demoji.rb

# Install from local formula
brew install ~/homebrew-demoji-test/demoji.rb

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt

# Uninstall
brew uninstall demoji
```

### Testing from Tap (After Publishing)

Once the formula is published to a Homebrew tap:

```bash
# Add the tap
brew tap yourusername/demoji

# Install from tap
brew install demoji

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt

# Uninstall
brew uninstall demoji
brew untap yourusername/demoji
```

### Homebrew Smoke Test Checklist

- [ ] Formula installs without errors
- [ ] Binary is placed in `/usr/local/bin/demoji` (or equivalent)
- [ ] `demoji --version` displays version number
- [ ] `demoji --help` shows usage information
- [ ] `demoji --dry-run <file>` works with emoji-containing files
- [ ] All subcommands are available
- [ ] Uninstall removes binary cleanly

### Testing Source Build Option

If the formula supports building from source:

```bash
# Install from source
brew install demoji --with-from-source

# Verify installation
which demoji
demoji --version

# Uninstall
brew uninstall demoji
```

---

## Testing npm Package

### Prerequisites

- Node.js 12+ installed
- npm installed
- The npm package files in `npm/` directory

### Testing Local Installation

```bash
# Navigate to npm directory
cd npm

# Install dependencies (if any)
npm install

# Test local installation
npm install -g .

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt

# Uninstall
npm uninstall -g demoji
```

### Testing from npm Registry (After Publishing)

Once published to npm:

```bash
# Install from npm registry
npm install -g demoji

# Verify installation
which demoji
demoji --version
demoji --help

# Test basic functionality
echo "Hello 👋 World 🌍" > test.txt
demoji --dry-run test.txt

# Uninstall
npm uninstall -g demoji
```

### npm Smoke Test Checklist

- [ ] Package installs without errors
- [ ] Binary is available in PATH
- [ ] `demoji --version` displays version number
- [ ] `demoji --help` shows usage information
- [ ] `demoji --dry-run <file>` works with emoji-containing files
- [ ] All subcommands are available
- [ ] Uninstall removes binary cleanly
- [ ] Works on different platforms (macOS, Linux, Windows)
- [ ] Works on different architectures (x64, arm64)

### Testing Platform-Specific Installation

The npm package includes platform detection. Test on different systems:

```bash
# On macOS (Intel)
npm install -g demoji
demoji --version

# On macOS (Apple Silicon)
npm install -g demoji
demoji --version

# On Linux (x64)
npm install -g demoji
demoji --version

# On Linux (ARM64)
npm install -g demoji
demoji --version

# On Windows (x64)
npm install -g demoji
demoji --version
```

---

## Automated CI Testing

The CI workflow includes automated smoke tests for `cargo install`:

### CI Test Job

The `.github/workflows/ci.yml` includes a `cargo-install` job that:

1. Checks out the repository
2. Sets up the Rust toolchain
3. Runs `cargo build --release` to verify the binary can be built
4. Runs the compiled binary with `--version` to verify it works

This job runs on all three platforms (Linux, macOS, Windows) to ensure cross-platform compatibility.

### Running CI Tests Locally

To simulate the CI tests locally:

```bash
# Run all tests
cargo test --verbose

# Build release binary
cargo build --release

# Test the release binary
./target/release/demoji --version
./target/release/demoji --help

# Test with a sample file
echo "Hello 👋 World 🌍" > test.txt
./target/release/demoji --dry-run test.txt
```

---

## Pre-Release Checklist

Before releasing a new version, verify:

### Code Quality
- [ ] All tests pass: `cargo test --verbose`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt -- --check`
- [ ] No unused dependencies: `cargo tree`

### Functionality
- [ ] `demoji --version` shows correct version
- [ ] `demoji --help` shows all subcommands
- [ ] `demoji run --help` works
- [ ] `demoji watch --help` works
- [ ] `demoji init --help` works
- [ ] `demoji --dry-run <file>` works
- [ ] Exit codes are correct

### Distribution
- [ ] `cargo install --path .` works
- [ ] Homebrew formula installs correctly (if applicable)
- [ ] npm package installs correctly (if applicable)
- [ ] Binary works on all supported platforms
- [ ] Binary works on all supported architectures

### Documentation
- [ ] README.md is up to date
- [ ] CONTRIBUTING.md is up to date
- [ ] Inline code documentation is complete
- [ ] Man page is generated (if applicable)

---

## Troubleshooting

### `cargo install` fails

```bash
# Check Rust toolchain
rustc --version
cargo --version

# Clean and rebuild
cargo clean
cargo build --release

# Try installing again
cargo install --path .
```

### Homebrew installation fails

```bash
# Check formula syntax
brew audit homebrew/demoji.rb

# Check dependencies
brew deps demoji

# Try installing with verbose output
brew install -v demoji
```

### npm installation fails

```bash
# Check Node.js version
node --version
npm --version

# Clear npm cache
npm cache clean --force

# Try installing again
npm install -g demoji
```

### Binary doesn't work after installation

```bash
# Verify binary location
which demoji

# Check binary permissions
ls -la $(which demoji)

# Try running with full path
/usr/local/bin/demoji --version

# Check for missing dependencies
ldd $(which demoji)  # On Linux
otool -L $(which demoji)  # On macOS
```

---

## Additional Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [npm Package Publishing Guide](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [demoji GitHub Repository](https://github.com/yourusername/demoji)
