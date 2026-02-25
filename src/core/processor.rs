//! File processing module
//!
//! Handles reading files, applying emoji detection/replacement, and writing results.

use crate::core::backup::BackupManager;
use crate::core::emoji::{EmojiDetector, EmojiMatch};
use crate::core::replacer::EmojiReplacer;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Processes individual files for emoji detection and replacement
pub struct FileProcessor {
    detector: EmojiDetector,
    replacer: Box<dyn EmojiReplacer>,
    dry_run: bool,
    backup_manager: Option<BackupManager>,
}

impl FileProcessor {
    /// Creates a new file processor with default settings
    pub fn new() -> Self {
        Self {
            detector: EmojiDetector::new(),
            replacer: Box::new(crate::core::replacer::RemoveReplacer),
            dry_run: true,
            backup_manager: None,
        }
    }

    /// Sets the replacer strategy
    #[must_use]
    pub fn with_replacer(mut self, replacer: Box<dyn EmojiReplacer>) -> Self {
        self.replacer = replacer;
        self
    }

    /// Sets dry-run mode
    #[must_use]
    pub const fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the backup manager
    #[must_use]
    pub fn with_backup(mut self, backup_manager: BackupManager) -> Self {
        self.backup_manager = Some(backup_manager);
        self
    }

    /// Processes a single file for emoji detection and replacement
    ///
    /// Reads the file, detects emojis, applies replacements, and optionally writes back.
    /// Returns a ProcessingResult with statistics about the processing.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file is not valid UTF-8
    /// - Writing the file fails (when not in dry-run mode)
    pub fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<ProcessingResult> {
        let path = path.as_ref();
        let file_path = path.to_path_buf();

        // Read file content
        let original_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        // Process the content
        let result = self.process_content(&original_content)?;

        // Update the result with the file path
        let mut result = result;
        result.file_path = file_path;

        // Write back to file if not in dry-run mode and content changed
        if !self.dry_run && result.original_content != result.processed_content {
            // Create backup if backup manager is configured
            if let Some(backup_manager) = &self.backup_manager {
                backup_manager.create_backup(&result.file_path)?;
            }
            Self::write_file(&result.file_path, &result.processed_content)?;
        }

        Ok(result)
    }

    /// Processes string content directly for emoji detection and replacement
    ///
    /// This method doesn't read or write files, just processes the content.
    /// Useful for testing or processing content from other sources.
    ///
    /// # Errors
    /// This method is infallible, but returns Result for API consistency.
    pub fn process_content(&self, content: &str) -> Result<ProcessingResult> {
        // Find all emojis in the content
        let emoji_matches = self.detector.find_all(content);

        // Track which lines have emojis
        let mut lines_with_emojis = HashSet::new();
        for emoji_match in &emoji_matches {
            lines_with_emojis.insert(emoji_match.line);
        }

        // Apply replacements to build processed content
        let mut processed_content = String::new();
        let mut last_end = 0;

        for emoji_match in &emoji_matches {
            // Add content before this emoji
            processed_content.push_str(&content[last_end..emoji_match.start]);

            // Add replacement for this emoji
            if let Some(replacement) = self.replacer.replace(&emoji_match.emoji) {
                processed_content.push_str(&replacement);
            }

            last_end = emoji_match.end;
        }

        // Add remaining content after the last emoji
        processed_content.push_str(&content[last_end..]);

        Ok(ProcessingResult {
            file_path: PathBuf::new(),
            emojis_found: emoji_matches.len(),
            lines_with_emojis: lines_with_emojis.len(),
            emoji_matches,
            original_content: content.to_owned(),
            processed_content,
        })
    }

    /// Writes processed content back to a file
    fn write_file(path: &Path, content: &str) -> Result<()> {
        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        Ok(())
    }
}

impl Default for FileProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of processing a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessingResult {
    /// Path to the processed file
    pub file_path: PathBuf,
    /// Number of emojis found
    pub emojis_found: usize,
    /// Number of lines with emojis
    pub lines_with_emojis: usize,
    /// Detailed emoji matches
    pub emoji_matches: Vec<EmojiMatch>,
    /// Original file content
    pub original_content: String,
    /// Processed file content
    pub processed_content: String,
}

impl ProcessingResult {
    /// Returns true if any emojis were found
    #[must_use]
    pub const fn has_emojis(&self) -> bool {
        self.emojis_found > 0
    }

    /// Returns true if the content was modified
    pub fn was_modified(&self) -> bool {
        self.original_content != self.processed_content
    }

    /// Returns the number of emojis that were replaced/removed
    #[must_use]
    pub const fn emojis_processed(&self) -> usize {
        self.emojis_found
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::redundant_closure_for_method_calls)]
mod tests {
    use super::*;
    use crate::core::replacer::{AsciiReplacer, PlaceholderReplacer};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_process_content_no_emojis() {
        let processor = FileProcessor::new();
        let content = "Hello World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 0);
        assert_eq!(result.lines_with_emojis, 0);
        assert_eq!(result.original_content, result.processed_content);
    }

    #[test]
    fn test_process_content_with_single_emoji() {
        let processor = FileProcessor::new();
        let content = "Hello 👋 World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
        assert_eq!(result.original_content, "Hello 👋 World");
        assert_eq!(result.processed_content, "Hello  World");
    }

    #[test]
    fn test_process_content_with_multiple_emojis() {
        let processor = FileProcessor::new();
        let content = "Hello 👋 World 🎉";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 2);
        assert_eq!(result.processed_content, "Hello  World ");
    }

    #[test]
    fn test_process_content_with_replacer() {
        let processor = FileProcessor::new().with_replacer(Box::new(AsciiReplacer::new()));
        let content = "Hello 👋 World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
        assert!(result.processed_content.contains("wave"));
    }

    #[test]
    fn test_process_content_with_placeholder() {
        let processor =
            FileProcessor::new().with_replacer(Box::new(PlaceholderReplacer::new("[EMOJI]")));
        let content = "Hello 👋 World 🎉";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 2);
        assert_eq!(result.processed_content, "Hello [EMOJI] World [EMOJI]");
    }

    #[test]
    fn test_process_file_dry_run() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello 👋 World";

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(true);
        let result = processor.process_file(&file_path).unwrap();

        assert_eq!(result.emojis_found, 1);
        // File should not be modified in dry-run mode
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_process_file_write_mode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello 👋 World";

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let result = processor.process_file(&file_path).unwrap();

        assert_eq!(result.emojis_found, 1);
        // File should be modified
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, "Hello  World");
    }

    #[test]
    fn test_process_content_emoji_at_start() {
        let processor = FileProcessor::new();
        let content = "👋 Hello World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
        assert_eq!(result.processed_content, " Hello World");
    }

    #[test]
    fn test_process_content_emoji_at_end() {
        let processor = FileProcessor::new();
        let content = "Hello World 👋";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
        assert_eq!(result.processed_content, "Hello World ");
    }

    #[test]
    fn test_process_content_consecutive_emojis() {
        let processor = FileProcessor::new();
        let content = "Hello 👋🎉🦀 World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 3);
        assert_eq!(result.processed_content, "Hello  World");
    }

    #[test]
    fn test_process_content_emoji_only() {
        let processor = FileProcessor::new();
        let content = "👋";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
        assert_eq!(result.processed_content, "");
    }

    #[test]
    fn test_process_content_with_skin_tone() {
        let processor = FileProcessor::new();
        let content = "Hello 👋🏻 World";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
    }

    #[test]
    fn test_process_content_with_zwj_sequence() {
        let processor = FileProcessor::new();
        let content = "Family 👨‍👩‍👧‍👦 emoji";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
    }

    #[test]
    fn test_process_content_with_flag() {
        let processor = FileProcessor::new();
        let content = "Flag 🇺🇸 emoji";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 1);
    }

    #[test]
    fn test_process_content_lines_with_emojis() {
        let processor = FileProcessor::new();
        let content = "Line 1\nLine 2 👋\nLine 3 🎉\nLine 4";
        let result = processor.process_content(content).unwrap();

        assert_eq!(result.emojis_found, 2);
        assert_eq!(result.lines_with_emojis, 2);
    }

    #[test]
    fn test_processing_result_has_emojis() {
        let processor = FileProcessor::new();
        let result = processor.process_content("Hello 👋").unwrap();
        assert!(result.has_emojis());

        let result = processor.process_content("Hello World").unwrap();
        assert!(!result.has_emojis());
    }

    #[test]
    fn test_processing_result_was_modified() {
        let processor = FileProcessor::new();
        let result = processor.process_content("Hello 👋").unwrap();
        assert!(result.was_modified());

        let result = processor.process_content("Hello World").unwrap();
        assert!(!result.was_modified());
    }

    #[test]
    fn test_processing_result_emojis_processed() {
        let processor = FileProcessor::new();
        let result = processor.process_content("Hello 👋 World 🎉").unwrap();
        assert_eq!(result.emojis_processed(), 2);
    }

    #[test]
    fn test_process_file_preserves_non_emoji_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Line 1\nLine 2 with 👋 emoji\nLine 3";

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let _result = processor.process_file(&file_path).unwrap();

        let modified = fs::read_to_string(&file_path).unwrap();
        assert!(modified.contains("Line 1"));
        assert!(modified.contains("Line 2 with"));
        assert!(modified.contains("Line 3"));
        assert!(!modified.contains("👋"));
    }

    #[test]
    fn test_integration_complex_multiline_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("complex.rs");

        let content = r#"// File with emojis 🎉
fn main() {
    println!("Hello 👋");
    // TODO: Fix bug 🐛
    let rocket = "🚀";
    assert_eq!(100, 💯);
}
"#;

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let result = processor.process_file(&file_path).unwrap();

        // Should find 5 emojis
        assert_eq!(result.emojis_found, 5);
        assert_eq!(result.lines_with_emojis, 5);

        // Verify file was modified
        let modified = fs::read_to_string(&file_path).unwrap();
        assert!(!modified.contains("🎉"));
        assert!(!modified.contains("🐛"));
        assert!(!modified.contains("🚀"));
        assert!(!modified.contains("💯"));
        assert!(!modified.contains("👋"));
    }

    #[test]
    fn test_integration_process_multiple_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        fs::write(&file1, "Hello 👋").unwrap();
        fs::write(&file2, "World 🌍").unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let result1 = processor.process_file(&file1).unwrap();
        let result2 = processor.process_file(&file2).unwrap();

        assert_eq!(result1.emojis_found, 1);
        assert_eq!(result2.emojis_found, 1);

        let content1 = fs::read_to_string(&file1).unwrap();
        let content2 = fs::read_to_string(&file2).unwrap();

        assert_eq!(content1, "Hello ");
        assert_eq!(content2, "World ");
    }

    #[test]
    fn test_integration_dry_run_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        fs::write(&file1, "Hello 👋").unwrap();
        fs::write(&file2, "World 🌍").unwrap();

        let processor = FileProcessor::new().with_dry_run(true);
        let result1 = processor.process_file(&file1).unwrap();
        let result2 = processor.process_file(&file2).unwrap();

        assert_eq!(result1.emojis_found, 1);
        assert_eq!(result2.emojis_found, 1);

        // Files should not be modified
        let content1 = fs::read_to_string(&file1).unwrap();
        let content2 = fs::read_to_string(&file2).unwrap();

        assert_eq!(content1, "Hello 👋");
        assert_eq!(content2, "World 🌍");
    }

    #[test]
    fn test_integration_different_replacement_modes() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Test Remove mode
        fs::write(&file_path, "Hello 👋").unwrap();
        let processor = FileProcessor::new().with_dry_run(false);
        processor.process_file(&file_path).unwrap();
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello ");

        // Test Placeholder mode
        fs::write(&file_path, "Hello 👋").unwrap();
        let processor = FileProcessor::new()
            .with_replacer(Box::new(PlaceholderReplacer::new("[X]")))
            .with_dry_run(false);
        processor.process_file(&file_path).unwrap();
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello [X]");
    }

    #[test]
    fn test_integration_various_file_types() {
        let temp_dir = TempDir::new().unwrap();

        let files = vec![
            ("test.rs", "fn main() { 👋 }"),
            ("test.py", "def hello(): 🎉"),
            ("test.js", "console.log('👋');"),
            ("test.txt", "Hello 🌍"),
            ("test.json", r#"{"emoji": "🚀"}"#),
        ];

        let processor = FileProcessor::new().with_dry_run(false);

        for (filename, content) in files {
            let file_path = temp_dir.path().join(filename);
            fs::write(&file_path, content).unwrap();
            let result = processor.process_file(&file_path).unwrap();
            assert!(result.emojis_found > 0);
        }
    }

    #[test]
    fn test_integration_file_with_no_emojis() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("clean.txt");
        let content = "This file has no emojis";

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let result = processor.process_file(&file_path).unwrap();

        assert_eq!(result.emojis_found, 0);
        // File should not be modified
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_integration_mixed_emoji_types() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("mixed.txt");

        let content = "Single: 😀, Skin tone: 👍🏽, ZWJ: 👨‍👩‍👧, Flag: 🇺🇸, Heart: ❤️";

        fs::write(&file_path, content).unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        let result = processor.process_file(&file_path).unwrap();

        assert!(result.emojis_found >= 5);
    }

    #[test]
    fn test_integration_atomic_write_safety() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "Hello 👋").unwrap();

        let processor = FileProcessor::new().with_dry_run(false);
        processor.process_file(&file_path).unwrap();

        // Check that no temp files are left behind
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].file_name(), "test.txt");
    }
}
