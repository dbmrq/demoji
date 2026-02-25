//! # demoji
//!
//! A fast, cross-platform library and CLI tool to remove or replace emoji characters
//! from source code files.
//!
//! ## Overview
//!
//! `demoji` helps you maintain clean, portable source code by detecting and removing
//! or replacing emoji characters. It's designed to be fast, safe, and respectful of
//! your project structure.
//!
//! ## Features
//!
//! - **Fast**: Parallel file processing with Rayon
//! - **Smart**: Respects `.gitignore` patterns automatically
//! - **Safe**: Dry-run mode, backup options, and atomic file writes
//! - **Flexible**: Multiple replacement strategies (remove, replace, placeholder)
//! - **Configurable**: Per-project settings via `.demoji.toml`
//!
//! ## Quick Start
//!
//! ### As a CLI tool
//!
//! ```bash
//! # Check for emojis (dry-run)
//! demoji run
//!
//! # Remove emojis from files
//! demoji run --write
//!
//! # Watch for changes
//! demoji watch --write
//! ```
//!
//! ### As a library
//!
//! ```rust
//! use demoji::{EmojiDetector, ReplacementMode};
//!
//! // Detect emojis in text
//! let detector = EmojiDetector::new();
//! let text = "Hello 👋 World 🌍";
//! // Process text...
//!
//! // Use different replacement modes
//! let mode = ReplacementMode::Remove;
//! // Apply replacement...
//! ```
//!
//! ## Modules
//!
//! - [`cli`] - Command-line interface and argument parsing
//! - [`config`] - Configuration file handling (`.demoji.toml`)
//! - [`core`] - Core functionality (emoji detection, file processing, replacement)
//! - [`watch`] - File system watching for continuous mode
//!
//! ## Replacement Modes
//!
//! ### Remove (default)
//!
//! Removes emoji characters entirely:
//!
//! ```text
//! Before: const msg = "Hello 👋 World";
//! After:  const msg = "Hello  World";
//! ```
//!
//! ### Replace
//!
//! Replaces emojis with ASCII alternatives:
//!
//! ```text
//! Before: const status = "✅ Success";
//! After:  const status = "[OK] Success";
//! ```
//!
//! ### Placeholder
//!
//! Replaces all emojis with a custom placeholder:
//!
//! ```text
//! Before: const msg = "Hello 👋 World 🌍";
//! After:  const msg = "Hello [EMOJI] World [EMOJI]";
//! ```
//!
//! ## Configuration
//!
//! Create a `.demoji.toml` file in your project root:
//!
//! ```toml
//! mode = "remove"
//! backup = true
//! include = ["**/*.rs", "**/*.js"]
//! exclude = ["vendor/**"]
//! ```
//!
//! Configuration is merged from:
//! 1. Command-line arguments (highest priority)
//! 2. Project `.demoji.toml`
//! 3. Global `~/.demoji.toml`
//! 4. Built-in defaults
//!
//! ## Examples
//!
//! ### Process a single file
//!
//! ```rust,no_run
//! use demoji::{FileProcessor, ReplacementMode};
//!
//! let processor = FileProcessor::new(ReplacementMode::Remove);
//! // Process file...
//! ```
//!
//! ### Walk a directory
//!
//! ```rust,no_run
//! use demoji::core::walker::DirectoryWalker;
//!
//! let walker = DirectoryWalker::new("./src");
//! // Walk and process files...
//! ```
//!
//! ## Safety Features
//!
//! - **Dry-run by default**: Preview changes before modifying files
//! - **Backup support**: Create `.bak` files before modifying
//! - **Atomic writes**: Write to temp file then rename (prevents corruption)
//! - **Gitignore respect**: Honors `.gitignore` patterns automatically
//! - **Binary file detection**: Skips binary files to avoid corruption
//!
//! ## Performance
//!
//! `demoji` is designed for speed:
//!
//! - Parallel file processing using Rayon
//! - Streaming processing for large files (doesn't load entire file into memory)
//! - Efficient Unicode handling
//! - Respects `.gitignore` to skip unnecessary files
//!
//! ## Exit Codes
//!
//! When used as a CLI tool:
//!
//! - `0`: Success (no emojis found, or emojis successfully processed)
//! - `1`: Emojis were found (useful with `--check` flag in CI)
//! - `2`: Error occurred (IO error, permission denied, invalid config, etc.)

// Public API modules
pub mod cli;
pub mod config;
pub mod core;
pub mod watch;

// Re-export commonly used types
pub use core::emoji::EmojiDetector;
pub use core::processor::FileProcessor;
pub use core::replacer::ReplacementMode;
pub use config::Config;

