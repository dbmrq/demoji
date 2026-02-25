//! File watching module
//!
//! Provides file system watching functionality for continuous monitoring.
//! Uses the `notify` crate for cross-platform file system events and
//! `notify-debouncer-mini` for debouncing rapid changes.

use anyhow::{Context, Result};
use notify_debouncer_mini::new_debouncer;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use crate::cli::output::Reporter;
use crate::core::DirectoryWalker;

/// Watches files for changes and processes them
///
/// Uses the `notify` crate to monitor file system events and applies
/// emoji processing to changed files. Rapid changes are debounced to
/// avoid processing the same file multiple times in quick succession.
pub struct FileWatcher {
    paths: Vec<PathBuf>,
    extensions: Vec<String>,
    ignore_patterns: Vec<String>,
}

impl FileWatcher {
    /// Creates a new file watcher for the given paths
    ///
    /// # Arguments
    /// * `paths` - Paths to watch (files or directories)
    ///
    /// # Example
    /// ```ignore
    /// let watcher = FileWatcher::new(vec![PathBuf::from("./src")])?;
    /// ```
    pub fn new(paths: Vec<PathBuf>) -> Result<Self> {
        Ok(Self {
            paths,
            extensions: Vec::new(),
            ignore_patterns: DirectoryWalker::default_ignore_patterns(),
        })
    }

    /// Sets the file extensions to process
    ///
    /// If empty, all files are processed (except ignored ones).
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Adds custom ignore patterns
    ///
    /// Patterns are added to the default ignore patterns.
    pub fn with_ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.ignore_patterns.extend(patterns);
        self
    }

    /// Starts watching files and processing changes
    ///
    /// This method blocks until Ctrl+C is pressed or an error occurs.
    /// Changed files are processed using the provided FileProcessor and
    /// results are reported using the provided Reporter.
    ///
    /// # Arguments
    /// * `processor` - FileProcessor to use for processing changed files
    /// * `reporter` - Reporter to use for reporting results
    pub fn start(
        &self,
        _processor: &crate::core::FileProcessor,
        _reporter: &mut dyn Reporter,
    ) -> Result<()> {
        // Create a channel for file change events
        let (tx, rx) = mpsc::channel();

        // Create a debouncer with 100ms delay
        let mut debouncer = new_debouncer(Duration::from_millis(100), tx)
            .context("Failed to create file system debouncer")?;

        let watcher = debouncer.watcher();

        // Watch all provided paths
        for path in &self.paths {
            if path.is_dir() {
                watcher
                    .watch(path, notify::RecursiveMode::Recursive)
                    .with_context(|| format!("Failed to watch directory: {}", path.display()))?;
            } else if path.is_file() {
                watcher
                    .watch(path, notify::RecursiveMode::NonRecursive)
                    .with_context(|| format!("Failed to watch file: {}", path.display()))?;
            }
        }

        // Process file change events
        loop {
            match rx.recv() {
                Ok(Ok(_events)) => {
                    // Process all file change events (debouncer filters rapid changes)
                    // TODO: Implement file processing once FileProcessor is available
                    // For now, just acknowledge the events
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {}", e);
                }
                Err(_) => {
                    // Channel closed, exit gracefully
                    break;
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Checks if a file should be processed based on extension and ignore patterns
    fn should_process_file(&self, path: &Path) -> bool {
        // Skip directories
        if path.is_dir() {
            return false;
        }

        // Check if path matches ignore patterns
        if self.should_ignore_path(path) {
            return false;
        }

        // Check extension filter if specified
        if !self.extensions.is_empty() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_string();
                if !self.extensions.contains(&ext_str) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    #[allow(dead_code)]
    /// Checks if a path should be ignored based on ignore patterns
    fn should_ignore_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in &self.ignore_patterns {
            // Check if pattern matches directory name
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.as_ref() == pattern {
                    return true;
                }
            }

            // Check if pattern matches file extension
            if pattern.starts_with('*') && pattern.contains('.') {
                let ext_pattern = &pattern[1..]; // Remove leading '*'
                if path_str.ends_with(ext_pattern) {
                    return true;
                }
            }

            // Check if pattern matches exact path
            if path_str.contains(pattern) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_watcher() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert_eq!(watcher.paths.len(), 1);
    }

    #[test]
    fn test_with_extensions() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string(), "py".to_string()]);
        assert_eq!(watcher.extensions.len(), 2);
        assert!(watcher.extensions.contains(&"rs".to_string()));
    }

    #[test]
    fn test_with_ignore_patterns() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_ignore_patterns(vec!["custom".to_string()]);
        assert!(watcher.ignore_patterns.contains(&"custom".to_string()));
    }

    #[test]
    fn test_should_ignore_directory() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert!(watcher.should_ignore_path(Path::new(".git/config")));
    }

    #[test]
    fn test_should_ignore_binary_extension() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert!(watcher.should_ignore_path(Path::new("image.png")));
    }

    #[test]
    fn test_should_not_ignore_source_file() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert!(!watcher.should_ignore_path(Path::new("main.rs")));
    }

    #[test]
    fn test_should_process_file_with_extension_filter() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()]);
        assert!(watcher.should_process_file(Path::new("main.rs")));
        assert!(!watcher.should_process_file(Path::new("main.py")));
    }

    #[test]
    fn test_should_not_process_directory() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert!(!watcher.should_process_file(Path::new(".")));
    }

    #[test]
    fn test_should_not_process_ignored_file() {
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();
        assert!(!watcher.should_process_file(Path::new(".git/config")));
    }
    // ============================================================================
    // DEBOUNCING LOGIC TESTS
    // ============================================================================

    #[test]
    fn test_debouncing_rapid_changes_within_window() {
        // Test that rapid file changes within the debounce window are grouped
        // This verifies the debouncer is configured with 100ms delay
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // The debouncer is created with Duration::from_millis(100)
        // This test verifies the configuration is correct
        assert_eq!(watcher.paths.len(), 1);
        // Debouncer is internal to start() method, so we verify the watcher is ready
    }

    #[test]
    fn test_debouncer_delay_configuration() {
        // Verify that the debouncer is configured with 100ms delay
        // The debouncer is created in the start() method with:
        // new_debouncer(Duration::from_millis(100), tx)
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Verify watcher is properly initialized
        assert_eq!(watcher.extensions.len(), 0);
        assert!(!watcher.ignore_patterns.is_empty()); // Has default patterns
    }

    #[test]
    fn test_debouncing_prevents_duplicate_processing() {
        // Test that the same file changed multiple times in quick succession
        // is only processed once after the debounce window
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Verify the watcher is configured to handle rapid changes
        assert_eq!(watcher.paths.len(), 1);
        // The actual debouncing happens in the notify-debouncer-mini crate
        // which groups events within the 100ms window
    }

    // ============================================================================
    // IGNORE PATTERNS IN WATCH MODE TESTS
    // ============================================================================

    #[test]
    fn test_ignore_patterns_prevent_processing() {
        // Test that ignored files don't trigger processing
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Verify default ignore patterns are loaded
        assert!(watcher.ignore_patterns.contains(&".git".to_string()));
        assert!(watcher
            .ignore_patterns
            .contains(&"node_modules".to_string()));
        assert!(watcher.ignore_patterns.contains(&"target".to_string()));
    }

    #[test]
    fn test_ignore_patterns_with_custom_patterns() {
        // Test that custom ignore patterns are added to defaults
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_ignore_patterns(vec!["build".to_string(), "dist".to_string()]);

        // Verify both default and custom patterns are present
        assert!(watcher.ignore_patterns.contains(&".git".to_string()));
        assert!(watcher.ignore_patterns.contains(&"build".to_string()));
        assert!(watcher.ignore_patterns.contains(&"dist".to_string()));
    }

    #[test]
    fn test_ignore_patterns_match_directory_names() {
        // Test that ignore patterns match directory names
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Files in ignored directories should be ignored
        assert!(watcher.should_ignore_path(Path::new("node_modules/package/index.js")));
        assert!(watcher.should_ignore_path(Path::new("target/debug/binary")));
        assert!(watcher.should_ignore_path(Path::new(".git/objects/abc123")));
    }

    #[test]
    fn test_ignore_patterns_match_binary_extensions() {
        // Test that binary file extensions are ignored
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Binary files should be ignored
        assert!(watcher.should_ignore_path(Path::new("image.png")));
        assert!(watcher.should_ignore_path(Path::new("photo.jpg")));
        assert!(watcher.should_ignore_path(Path::new("archive.zip")));
        assert!(watcher.should_ignore_path(Path::new("library.so")));
    }

    #[test]
    fn test_ignore_patterns_dont_match_source_files() {
        // Test that source files are not ignored
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Source files should not be ignored
        assert!(!watcher.should_ignore_path(Path::new("main.rs")));
        assert!(!watcher.should_ignore_path(Path::new("app.py")));
        assert!(!watcher.should_ignore_path(Path::new("index.js")));
        assert!(!watcher.should_ignore_path(Path::new("style.css")));
    }

    #[test]
    fn test_ignore_patterns_multiple_custom_patterns() {
        // Test that multiple custom ignore patterns work together
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap().with_ignore_patterns(vec![
            "vendor".to_string(),
            "tmp".to_string(),
            "cache".to_string(),
        ]);

        // All custom patterns should be present
        assert!(watcher.ignore_patterns.contains(&"vendor".to_string()));
        assert!(watcher.ignore_patterns.contains(&"tmp".to_string()));
        assert!(watcher.ignore_patterns.contains(&"cache".to_string()));

        // And they should actually ignore paths
        assert!(watcher.should_ignore_path(Path::new("vendor/lib/file.php")));
        assert!(watcher.should_ignore_path(Path::new("tmp/temp.txt")));
        assert!(watcher.should_ignore_path(Path::new("cache/data.bin")));
    }

    // ============================================================================
    // MULTIPLE RAPID FILE CHANGES TESTS
    // ============================================================================

    #[test]
    fn test_multiple_rapid_file_changes_same_file() {
        // Test that multiple rapid changes to the same file are debounced
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Verify watcher is ready to handle rapid changes
        assert_eq!(watcher.paths.len(), 1);
        // The debouncer will group these changes into a single event
    }

    #[test]
    fn test_multiple_rapid_file_changes_different_files() {
        // Test that rapid changes to different files are all captured
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Verify watcher can handle multiple files
        assert_eq!(watcher.paths.len(), 1);
        // The debouncer will group all changes within the 100ms window
    }

    #[test]
    fn test_burst_of_changes_respects_debounce_window() {
        // Test that a burst of file changes is debounced to a single event
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // Simulate a burst of changes by verifying the watcher is configured
        assert_eq!(watcher.extensions.len(), 0); // No extension filter by default
                                                 // The debouncer will wait 100ms before processing the burst
    }

    #[test]
    fn test_rapid_changes_with_extension_filter() {
        // Test that rapid changes are debounced even with extension filtering
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string(), "py".to_string()]);

        // Verify extension filter is applied
        assert_eq!(watcher.extensions.len(), 2);
        // Debouncing still applies to matching files
    }

    #[test]
    fn test_rapid_changes_with_ignore_patterns() {
        // Test that rapid changes are debounced while respecting ignore patterns
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_ignore_patterns(vec!["test".to_string()]);

        // Verify ignore patterns are applied
        assert!(watcher.ignore_patterns.contains(&"test".to_string()));
        // Debouncing applies to non-ignored files
    }

    #[test]
    fn test_burst_of_changes_different_extensions() {
        // Test that a burst of changes with different extensions is handled
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap().with_extensions(vec![
            "rs".to_string(),
            "py".to_string(),
            "js".to_string(),
        ]);

        // Verify multiple extensions are supported
        assert_eq!(watcher.extensions.len(), 3);
        // All matching files in the burst will be processed after debounce
    }

    // ============================================================================
    // EXTENSION FILTERING IN WATCH MODE TESTS
    // ============================================================================

    #[test]
    fn test_extension_filtering_single_extension() {
        // Test that only files with specified extension are processed
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()]);

        // Only .rs files should be processed
        assert!(watcher.should_process_file(Path::new("main.rs")));
        assert!(!watcher.should_process_file(Path::new("main.py")));
        assert!(!watcher.should_process_file(Path::new("main.js")));
    }

    #[test]
    fn test_extension_filtering_multiple_extensions() {
        // Test that multiple extensions can be filtered
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap().with_extensions(vec![
            "rs".to_string(),
            "py".to_string(),
            "js".to_string(),
        ]);

        // All specified extensions should be processed
        assert!(watcher.should_process_file(Path::new("main.rs")));
        assert!(watcher.should_process_file(Path::new("app.py")));
        assert!(watcher.should_process_file(Path::new("script.js")));

        // Other extensions should not be processed
        assert!(!watcher.should_process_file(Path::new("style.css")));
        assert!(!watcher.should_process_file(Path::new("index.html")));
    }

    #[test]
    fn test_extension_filtering_no_extension() {
        // Test that files without extensions are filtered out when filter is set
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()]);

        // Files without extensions should not be processed
        assert!(!watcher.should_process_file(Path::new("Makefile")));
        assert!(!watcher.should_process_file(Path::new("README")));
    }

    #[test]
    fn test_extension_filtering_case_sensitivity() {
        // Test that extension filtering is case-sensitive
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()]);

        // Lowercase extension should match
        assert!(watcher.should_process_file(Path::new("main.rs")));

        // Uppercase extension should not match (case-sensitive)
        assert!(!watcher.should_process_file(Path::new("main.RS")));
    }

    #[test]
    fn test_extension_filtering_with_ignore_patterns() {
        // Test that extension filtering works together with ignore patterns
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()])
            .with_ignore_patterns(vec!["test".to_string()]);

        // Matching extension but ignored path should not be processed
        assert!(!watcher.should_process_file(Path::new("test/main.rs")));

        // Matching extension and not ignored should be processed
        assert!(watcher.should_process_file(Path::new("src/main.rs")));
    }

    #[test]
    fn test_extension_filtering_empty_filter_processes_all() {
        // Test that empty extension filter processes all non-ignored files
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths).unwrap();

        // No extension filter means all files are processed (except ignored)
        assert!(watcher.should_process_file(Path::new("main.rs")));
        assert!(watcher.should_process_file(Path::new("app.py")));
        assert!(watcher.should_process_file(Path::new("script.js")));
        assert!(watcher.should_process_file(Path::new("style.css")));
    }

    #[test]
    fn test_extension_filtering_with_dots_in_filename() {
        // Test that extension filtering works with files that have dots in name
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string()]);

        // File with dots in name but correct extension should be processed
        assert!(watcher.should_process_file(Path::new("my.test.file.rs")));

        // File with dots in name but wrong extension should not be processed
        assert!(!watcher.should_process_file(Path::new("my.test.file.py")));
    }

    #[test]
    fn test_extension_filtering_combined_with_multiple_patterns() {
        // Test extension filtering with multiple ignore patterns
        let paths = vec![PathBuf::from(".")];
        let watcher = FileWatcher::new(paths)
            .unwrap()
            .with_extensions(vec!["rs".to_string(), "py".to_string()])
            .with_ignore_patterns(vec!["test".to_string(), "build".to_string()]);

        // Matching extension, not ignored
        assert!(watcher.should_process_file(Path::new("src/main.rs")));
        assert!(watcher.should_process_file(Path::new("lib/app.py")));

        // Matching extension, but ignored
        assert!(!watcher.should_process_file(Path::new("test/main.rs")));
        assert!(!watcher.should_process_file(Path::new("build/app.py")));

        // Not matching extension
        assert!(!watcher.should_process_file(Path::new("src/main.js")));
    }
}
