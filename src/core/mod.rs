//! Core functionality module
//!
//! Contains emoji detection, replacement strategies, file processing,
//! directory traversal, backup functionality, and error handling.

pub mod backup;
pub mod emoji;
pub mod error;
pub mod processor;
pub mod replacer;
pub mod walker;

pub use backup::BackupManager;
pub use emoji::EmojiDetector;
pub use error::{DemojiError, DemojiResult};
pub use processor::{FileProcessor, ProcessingResult};
pub use replacer::{EmojiReplacer, ReplacementMode};
pub use walker::DirectoryWalker;
