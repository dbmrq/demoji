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
//! let processor = FileProcessor::new()
//!     .with_replacer(Box::new(demoji::core::replacer::RemoveReplacer));
//! // Process file...
//! ```
//!
//! ### Walk a directory
//!
//! ```rust,no_run
//! use demoji::core::walker::DirectoryWalker;
//!
//! let walker = DirectoryWalker::new(std::path::Path::new("./src"));
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
pub use config::Config;
pub use core::emoji::EmojiDetector;
pub use core::processor::FileProcessor;
pub use core::replacer::ReplacementMode;

use anyhow::Result;
use cli::args::{Args, Command};
use cli::output::{create_reporter, VerbosityLevel};
use core::error::DemojiError;
use core::replacer::create_replacer;
use core::DirectoryWalker;
use std::path::PathBuf;

/// Main entry point for the demoji application
///
/// Orchestrates the entire workflow:
/// 1. Parses CLI arguments and loads configuration
/// 2. Determines paths to process (uses current directory if none specified)
/// 3. Creates appropriate reporter based on verbosity level
/// 4. Walks directories and processes files
/// 5. Reports progress and summary
/// 6. Returns exit code: 0=success, 1=emojis found, 2=error
///
/// # Exit Codes
/// - `0`: Success (no emojis found or successfully processed)
/// - `1`: Emojis were found (useful for CI checks)
/// - `2`: Error occurred (IO, permission, config parsing, etc.)
pub fn run(args: Args, config: Config) -> Result<i32> {
    // Determine verbosity level
    let verbosity = if args.quiet {
        VerbosityLevel::Quiet
    } else if args.verbose {
        VerbosityLevel::Verbose
    } else {
        VerbosityLevel::Normal
    };

    // Create appropriate reporter
    let mut reporter = create_reporter(verbosity);

    // Handle subcommands
    match args.command {
        Some(Command::Run {
            paths: cmd_paths,
            dry_run: cmd_dry_run,
            backup: cmd_backup,
            mode: cmd_mode,
            extensions: cmd_extensions,
            exclude: cmd_exclude,
            verbose: cmd_verbose,
            quiet: cmd_quiet,
            placeholder: cmd_placeholder,
        }) => {
            // Merge command-specific args with global args
            let merged_args = Args {
                command: None,
                paths: if cmd_paths.is_empty() {
                    args.paths
                } else {
                    cmd_paths
                },
                dry_run: cmd_dry_run || args.dry_run,
                backup: cmd_backup || args.backup,
                mode: cmd_mode.or(args.mode),
                extensions: cmd_extensions.or(args.extensions),
                exclude: cmd_exclude.or(args.exclude),
                verbose: cmd_verbose || args.verbose,
                quiet: cmd_quiet || args.quiet,
                placeholder: cmd_placeholder.or(args.placeholder),
            };
            run_on_paths(merged_args, config, &mut *reporter)
        }
        None => {
            // Default run command
            run_on_paths(args, config, &mut *reporter)
        }
        Some(Command::Watch { .. }) => {
            // Watch command - not implemented yet
            eprintln!("Watch mode is not yet implemented");
            Ok(2)
        }
        Some(Command::Init { .. }) => {
            // Init command - not implemented yet
            eprintln!("Init command is not yet implemented");
            Ok(2)
        }
    }
}

/// Process files in the given paths
///
/// Handles the core logic of walking directories and processing files.
/// Returns exit code based on results.
fn run_on_paths(
    args: Args,
    config: Config,
    reporter: &mut dyn cli::output::Reporter,
) -> Result<i32> {
    // Determine paths to process
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    // Determine replacement mode from CLI args or config with better error handling
    let mode = if let Some(mode_str) = &args.mode {
        match mode_str.to_lowercase().as_str() {
            "remove" => ReplacementMode::Remove,
            "replace" => ReplacementMode::Replace,
            "placeholder" => ReplacementMode::Placeholder,
            _ => {
                let err = DemojiError::InvalidMode {
                    mode: mode_str.clone(),
                };
                eprintln!("{}", err.user_message());
                return Ok(2);
            }
        }
    } else {
        config.mode
    };

    // Determine placeholder from CLI args or config
    let placeholder = args.placeholder.as_deref().or(Some(&config.placeholder));

    // Determine dry-run mode
    let dry_run = args.dry_run || config.dry_run;

    // Determine backup mode from CLI args or config
    let backup = args.backup || config.backup;

    // Create file processor with appropriate replacer
    let replacer = create_replacer(mode, placeholder);
    let mut processor = FileProcessor::new()
        .with_replacer(replacer)
        .with_dry_run(dry_run);

    // Add backup manager if backup is enabled
    if backup {
        let backup_manager = crate::core::backup::BackupManager::new();
        processor = processor.with_backup(backup_manager);
    }

    // Determine extensions to process
    let extensions = if let Some(ext_str) = &args.extensions {
        ext_str.split(',').map(|s| s.trim().to_string()).collect()
    } else if !config.extensions.is_empty() {
        config.extensions.clone()
    } else {
        Vec::new()
    };

    // Determine ignore patterns
    let mut ignore_patterns = config.ignore_patterns.clone();
    if let Some(exclude_str) = &args.exclude {
        ignore_patterns.extend(exclude_str.split(',').map(|s| s.trim().to_string()));
    }

    // Process all paths
    let mut total_files = 0;
    let mut files_with_emojis = 0;
    let mut total_emojis = 0;
    let mut file_count = 0;

    for path in paths {
        if path.is_file() {
            // Process single file
            file_count += 1;
            total_files += 1;

            match processor.process_file(&path) {
                Ok(result) => {
                    if result.has_emojis() {
                        files_with_emojis += 1;
                        total_emojis += result.emojis_found;

                        reporter.report_file(path.to_string_lossy().as_ref(), file_count);

                        for emoji_match in &result.emoji_matches {
                            let context = extract_context(
                                &result.original_content,
                                emoji_match.start,
                                emoji_match.end,
                                10,
                            );

                            reporter.report_match(
                                emoji_match.line,
                                emoji_match.column,
                                &emoji_match.emoji,
                                &context,
                            );
                        }
                    }
                }
                Err(e) => {
                    // Check if it's a DemojiError for better messaging
                    if let Some(demoji_err) = e.downcast_ref::<DemojiError>() {
                        eprintln!("{}", demoji_err.user_message());
                    } else {
                        eprintln!("Error processing file {}: {}", path.display(), e);
                    }
                    return Ok(2);
                }
            }
        } else if path.is_dir() {
            // Process directory
            let walker = DirectoryWalker::new(&path)
                .with_extensions(extensions.clone())
                .with_ignore_patterns(ignore_patterns.clone());

            for file_result in walker.walk() {
                match file_result {
                    Ok(file_path) => {
                        file_count += 1;
                        total_files += 1;

                        match processor.process_file(&file_path) {
                            Ok(result) => {
                                if result.has_emojis() {
                                    files_with_emojis += 1;
                                    total_emojis += result.emojis_found;

                                    reporter.report_file(
                                        file_path.to_string_lossy().as_ref(),
                                        file_count,
                                    );

                                    for emoji_match in &result.emoji_matches {
                                        let context = extract_context(
                                            &result.original_content,
                                            emoji_match.start,
                                            emoji_match.end,
                                            10,
                                        );

                                        reporter.report_match(
                                            emoji_match.line,
                                            emoji_match.column,
                                            &emoji_match.emoji,
                                            &context,
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                // Check if it's a DemojiError for better messaging
                                if let Some(demoji_err) = e.downcast_ref::<DemojiError>() {
                                    eprintln!("{}", demoji_err.user_message());
                                } else {
                                    eprintln!(
                                        "Error processing file {}: {}",
                                        file_path.display(),
                                        e
                                    );
                                }
                                return Ok(2);
                            }
                        }
                    }
                    Err(e) => {
                        // Check if it's a DemojiError for better messaging
                        if let Some(demoji_err) = e.downcast_ref::<DemojiError>() {
                            eprintln!("{}", demoji_err.user_message());
                        } else {
                            eprintln!("Error walking directory: {}", e);
                        }
                        return Ok(2);
                    }
                }
            }
        } else {
            let err = DemojiError::InvalidPath { path: path.clone() };
            eprintln!("{}", err.user_message());
            return Ok(2);
        }
    }

    // Report summary
    reporter.report_summary(total_files, files_with_emojis, total_emojis);

    // Return appropriate exit code
    if files_with_emojis > 0 {
        Ok(1) // Emojis were found
    } else {
        Ok(0) // Success, no emojis found
    }
}

/// Extract context around a byte position, respecting UTF-8 boundaries
fn extract_context(content: &str, start: usize, end: usize, context_width: usize) -> String {
    // Find safe UTF-8 boundaries
    let mut context_start = start.saturating_sub(context_width);
    let mut context_end = (end + context_width).min(content.len());

    // Move context_start forward to a valid UTF-8 boundary
    while context_start < start && !is_char_boundary(content, context_start) {
        context_start += 1;
    }

    // Move context_end backward to a valid UTF-8 boundary
    while context_end > end && !is_char_boundary(content, context_end) {
        context_end -= 1;
    }

    // Extract and return the context
    content[context_start..context_end].to_string()
}

/// Check if a position is a valid UTF-8 character boundary
fn is_char_boundary(s: &str, index: usize) -> bool {
    if index > s.len() {
        return false;
    }
    if index == 0 || index == s.len() {
        return true;
    }
    s.is_char_boundary(index)
}
