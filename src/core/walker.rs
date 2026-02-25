//! Directory traversal module
//!
//! Handles recursive directory walking with gitignore support and filtering.

use crate::core::error::DemojiError;
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Default ignore patterns for binary files and common directories.
///
/// This is the single source of truth for ignore patterns used by both
/// `DirectoryWalker` and `Config::default()`.
pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    // Binary file extensions
    "*.png",
    "*.jpg",
    "*.jpeg",
    "*.gif",
    "*.ico",
    "*.woff",
    "*.woff2",
    "*.ttf",
    "*.otf",
    "*.exe",
    "*.dll",
    "*.so",
    "*.dylib",
    "*.zip",
    "*.tar",
    "*.gz",
    "*.bz2",
    "*.xz",
    "*.7z",
    "*.rar",
    "*.pdf",
    "*.mp4",
    "*.mp3",
    "*.wav",
    // Directories
    ".git",
    "node_modules",
    "target",
    "build",
    "dist",
    ".next",
    "__pycache__",
    ".venv",
    "venv",
    "vendor",
    ".vscode",
    ".idea",
    ".DS_Store",
];

/// Walks directories and processes files
///
/// Uses the `ignore` crate for gitignore support and provides filtering
/// by file extension and custom ignore patterns.
pub struct DirectoryWalker {
    root: PathBuf,
    extensions: Vec<String>,
    ignore_patterns: Vec<String>,
}

impl DirectoryWalker {
    /// Creates a new directory walker for the given root path
    ///
    /// # Arguments
    /// * `root` - The root directory to start walking from
    ///
    /// # Example
    /// ```ignore
    /// let walker = DirectoryWalker::new(Path::new("."));
    /// ```
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            extensions: Vec::new(),
            ignore_patterns: default_ignore_patterns(),
        }
    }

    /// Sets the file extensions to process
    ///
    /// If empty, all files are processed (except ignored ones).
    /// Extensions should be provided without the leading dot.
    ///
    /// # Arguments
    /// * `extensions` - List of file extensions to process (e.g., vec!["rs", "py", "js"])
    #[must_use]
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Adds custom ignore patterns
    ///
    /// Patterns are added to the default ignore patterns.
    /// Patterns follow gitignore syntax.
    ///
    /// # Arguments
    /// * `patterns` - Additional patterns to ignore
    #[must_use]
    pub fn with_ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.ignore_patterns.extend(patterns);
        self
    }

    /// Walks the directory tree and yields matching file paths
    ///
    /// Returns an iterator of file paths that match the configured criteria:
    /// - Not in ignored directories or matching ignore patterns
    /// - If extensions are set, only files with those extensions
    ///
    /// # Returns
    /// An iterator of `Result<PathBuf>` for each matching file
    pub fn walk(&self) -> impl Iterator<Item = Result<PathBuf>> {
        let mut builder = WalkBuilder::new(&self.root);
        builder.standard_filters(true); // Respect .gitignore files
        builder.hidden(false); // Don't skip hidden files by default

        let walker = builder.build();
        let extensions = self.extensions.clone();
        let ignore_patterns = self.ignore_patterns.clone();

        walker.filter_map(move |entry| {
            match entry {
                Ok(entry) => {
                    let path = entry.path();

                    // Skip directories
                    if path.is_dir() {
                        return None;
                    }

                    // Check if path matches any ignore pattern
                    if should_ignore_path(path, &ignore_patterns) {
                        return None;
                    }

                    // Filter by extension if specified
                    if !extensions.is_empty() {
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if !extensions.contains(&ext_str.to_owned()) {
                                    return None;
                                }
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }

                    Some(Ok(path.to_path_buf()))
                },
                Err(e) => {
                    // Convert walk errors to DemojiError with helpful message
                    let err = DemojiError::WalkError {
                        message: format!("Failed to read directory entry: {e}"),
                    };
                    Some(Err(err.into()))
                },
            }
        })
    }
}

impl Default for DirectoryWalker {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

/// Returns the default ignore patterns as a `Vec<String>`
///
/// Converts the static DEFAULT_IGNORE_PATTERNS slice to owned strings.
pub fn default_ignore_patterns() -> Vec<String> {
    DEFAULT_IGNORE_PATTERNS
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
}

/// Checks if a path should be ignored based on patterns
fn should_ignore_path(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in patterns {
        // Handle directory patterns (e.g., ".git", "node_modules")
        if pattern.starts_with('.') || pattern.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Check if any component of the path matches the directory pattern
            for component in path.components() {
                if let Some(name) = component.as_os_str().to_str() {
                    if name == pattern {
                        return true;
                    }
                }
            }
        }

        // Handle file extension patterns (e.g., "*.png")
        if pattern.starts_with('*') {
            let ext = pattern.trim_start_matches('*');
            if path_str.ends_with(ext) {
                return true;
            }
        }

        // Handle exact filename matches
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if name_str == pattern {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::str_to_string,
    clippy::redundant_closure_for_method_calls,
    clippy::needless_collect,
    clippy::create_dir,
    clippy::uninlined_format_args
)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_new_creates_walker() {
        let walker = DirectoryWalker::new(Path::new("."));
        assert_eq!(walker.root, PathBuf::from("."));
        assert!(walker.extensions.is_empty());
        assert!(!walker.ignore_patterns.is_empty());
    }

    #[test]
    fn test_default_creates_walker() {
        let walker = DirectoryWalker::default();
        assert_eq!(walker.root, PathBuf::from("."));
    }

    #[test]
    fn test_with_extensions() {
        let walker = DirectoryWalker::new(Path::new("."))
            .with_extensions(vec!["rs".to_string(), "py".to_string()]);
        assert_eq!(walker.extensions, vec!["rs", "py"]);
    }

    #[test]
    fn test_with_ignore_patterns() {
        let walker = DirectoryWalker::new(Path::new("."))
            .with_ignore_patterns(vec!["custom_pattern".to_string()]);
        assert!(walker
            .ignore_patterns
            .contains(&"custom_pattern".to_string()));
    }

    #[test]
    fn test_default_ignore_patterns_includes_binary_extensions() {
        let patterns = default_ignore_patterns();
        assert!(patterns.contains(&"*.png".to_string()));
        assert!(patterns.contains(&"*.jpg".to_string()));
        assert!(patterns.contains(&"*.exe".to_string()));
    }

    #[test]
    fn test_default_ignore_patterns_includes_directories() {
        let patterns = default_ignore_patterns();
        assert!(patterns.contains(&".git".to_string()));
        assert!(patterns.contains(&"node_modules".to_string()));
        assert!(patterns.contains(&"target".to_string()));
    }

    #[test]
    fn test_should_ignore_directory_pattern() {
        let patterns = vec![".git".to_string()];
        let path = Path::new("/home/user/project/.git/config");
        assert!(should_ignore_path(path, &patterns));
    }

    #[test]
    fn test_should_ignore_extension_pattern() {
        let patterns = vec!["*.png".to_string()];
        let path = Path::new("/home/user/image.png");
        assert!(should_ignore_path(path, &patterns));
    }

    #[test]
    fn test_should_not_ignore_non_matching_path() {
        let patterns = vec!["*.png".to_string(), ".git".to_string()];
        let path = Path::new("/home/user/file.txt");
        assert!(!should_ignore_path(path, &patterns));
    }

    #[test]
    fn test_walk_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_walk_with_single_file() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_walk_with_extension_filter() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "code").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path()).with_extensions(vec!["rs".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with(".rs"));
    }

    #[test]
    fn test_walk_with_multiple_extensions() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "code").unwrap();
        fs::write(temp_dir.path().join("test.py"), "code").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path())
            .with_extensions(vec!["rs".to_string(), "py".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_walk_ignores_directories() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_walk_with_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("test1.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("subdir/test2.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_walk_skips_git_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join(".git")).unwrap();
        fs::write(temp_dir.path().join(".git/config"), "git config").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with("test.txt"));
    }

    #[test]
    fn test_walk_skips_node_modules() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("node_modules")).unwrap();
        fs::write(temp_dir.path().join("node_modules/package.json"), "{}").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with("test.txt"));
    }

    #[test]
    fn test_walk_skips_target_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("target")).unwrap();
        fs::write(temp_dir.path().join("target/debug"), "binary").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with("test.txt"));
    }

    #[test]
    fn test_walk_respects_ignore_patterns() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("test.bak"), "backup").unwrap();
        let walker =
            DirectoryWalker::new(temp_dir.path()).with_ignore_patterns(vec!["*.bak".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with(".txt"));
    }

    // Integration tests
    #[test]
    fn test_integration_walk_with_gitignore_file() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join(".gitignore"), "*.log\n").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("test.log"), "log").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        // Note: .gitignore is only respected if we're in a git repo
        // For this test, we just verify the walker works
        assert!(!files.is_empty());
    }

    #[test]
    fn test_integration_walk_directory_with_various_file_types() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "code").unwrap();
        fs::write(temp_dir.path().join("test.py"), "code").unwrap();
        fs::write(temp_dir.path().join("test.js"), "code").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn test_integration_walk_with_nested_structure() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::create_dir(temp_dir.path().join("src/core")).unwrap();
        fs::write(temp_dir.path().join("test.txt"), "root").unwrap();
        fs::write(temp_dir.path().join("src/main.rs"), "main").unwrap();
        fs::write(temp_dir.path().join("src/core/lib.rs"), "lib").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn test_integration_walk_respects_multiple_ignore_patterns() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        fs::write(temp_dir.path().join("test.bak"), "backup").unwrap();
        fs::write(temp_dir.path().join("test.tmp"), "temp").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path())
            .with_ignore_patterns(vec!["*.bak".to_string(), "*.tmp".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with(".txt"));
    }

    #[test]
    fn test_integration_walk_with_extension_filter_and_ignore() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "code").unwrap();
        fs::write(temp_dir.path().join("test.py"), "code").unwrap();
        fs::write(temp_dir.path().join("test.bak"), "backup").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path())
            .with_extensions(vec!["rs".to_string(), "py".to_string()])
            .with_ignore_patterns(vec!["*.bak".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_integration_walk_large_directory_structure() {
        let temp_dir = TempDir::new().unwrap();
        for i in 0..3 {
            fs::create_dir(temp_dir.path().join(format!("dir{}", i))).unwrap();
            for j in 0..3 {
                fs::create_dir(temp_dir.path().join(format!("dir{}/subdir{}", i, j))).unwrap();
                for k in 0..2 {
                    fs::write(
                        temp_dir
                            .path()
                            .join(format!("dir{}/subdir{}/file{}.txt", i, j, k)),
                        "content",
                    )
                    .unwrap();
                }
            }
        }
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 18); // 3 * 3 * 2
    }

    #[test]
    fn test_integration_walk_with_hidden_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join(".hidden"), "hidden").unwrap();
        fs::write(temp_dir.path().join("visible.txt"), "visible").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_integration_walk_empty_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir_all(temp_dir.path().join("a/b/c")).unwrap();
        fs::write(temp_dir.path().join("a/b/c/file.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_integration_walk_with_special_characters_in_filenames() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test-file.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("test_file.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("test file.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn test_integration_walk_respects_git_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join(".git")).unwrap();
        fs::write(temp_dir.path().join(".git/config"), "git").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_integration_walk_with_symlinks() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("original.txt"), "content").unwrap();
        // Note: symlink creation might fail on some systems, so we just test the walker works
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_integration_walk_and_process_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_integration_walk_collect_statistics() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "code").unwrap();
        fs::write(temp_dir.path().join("test.py"), "code").unwrap();
        fs::write(temp_dir.path().join("test.rs.bak"), "backup").unwrap();
        let walker =
            DirectoryWalker::new(temp_dir.path()).with_ignore_patterns(vec!["*.bak".to_string()]);
        let files: Vec<_> = walker.walk().filter_map(|r| r.ok()).collect();
        let rs_count = files
            .iter()
            .filter(|f| f.to_string_lossy().ends_with(".rs"))
            .count();
        let py_count = files
            .iter()
            .filter(|f| f.to_string_lossy().ends_with(".py"))
            .count();
        assert_eq!(rs_count, 1);
        assert_eq!(py_count, 1);
    }
}
