//! Core functionality module
//!
//! Contains emoji detection, replacement strategies, file processing,
//! directory traversal, and backup functionality.

pub mod backup;
pub mod emoji;
pub mod processor;
pub mod replacer;
pub mod walker;

pub use backup::BackupManager;
pub use emoji::EmojiDetector;
pub use processor::FileProcessor;
pub use replacer::{EmojiReplacer, ReplacementMode};
pub use walker::DirectoryWalker;

