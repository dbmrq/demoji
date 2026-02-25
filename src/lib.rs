//! demoji - A fast CLI tool to remove or replace emoji characters from source code files
//!
//! This library provides the core functionality for detecting and processing emoji
//! characters in text files, with support for various replacement strategies.

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

