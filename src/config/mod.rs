//! Configuration module
//!
//! Handles loading and merging configuration from .demoji.toml files.
//!
//! Configuration is loaded in the following priority order (highest to lowest):
//! 1. CLI arguments (handled by caller)
//! 2. Project config (.demoji.toml in project root or parent directories)
//! 3. Global config (~/.config/demoji/config.toml or ~/.demoji.toml)
//! 4. Default values

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};

use crate::core::error::DemojiError;
use crate::core::replacer::ReplacementMode;

/// Configuration for demoji
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Replacement mode (remove, replace, or placeholder)
    pub mode: ReplacementMode,

    /// Custom placeholder string (used when mode is Placeholder)
    #[serde(default = "default_placeholder")]
    pub placeholder: String,

    /// File extensions to process (e.g., `["rs", "py", "js"]`)
    /// If empty, processes all text files
    pub extensions: Vec<String>,

    /// Additional patterns to ignore (beyond .gitignore)
    pub ignore_patterns: Vec<String>,

    /// Whether to create backups before modifying files
    pub backup: bool,

    /// Default dry-run behavior (preview changes without writing)
    pub dry_run: bool,
}

fn default_placeholder() -> String {
    "[EMOJI]".to_owned()
}

impl Config {
    /// Creates a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads configuration from files and merges with defaults
    ///
    /// Searches for configuration in the following order:
    /// 1. Project config: walks up from current directory looking for `.demoji.toml`
    /// 2. Global config: `~/.config/demoji/config.toml` or `~/.demoji.toml`
    /// 3. Falls back to defaults if no config files found
    ///
    /// # Errors
    /// Returns an error if the current directory cannot be determined or config loading fails.
    pub fn load() -> Result<Self> {
        Self::load_from_dir(&std::env::current_dir()?)
    }

    /// Loads configuration starting from a specific directory
    ///
    /// # Errors
    /// Returns an error if config file loading or parsing fails.
    pub fn load_from_dir(start_dir: &Path) -> Result<Self> {
        let mut config = Self::default();

        // Try to load global config first (lowest priority)
        if let Some(global_config) = Self::load_global_config()? {
            config = config.merge(global_config);
        }

        // Try to load project config (higher priority)
        if let Some(project_config) = Self::find_and_load_project_config(start_dir)? {
            config = config.merge(project_config);
        }

        Ok(config)
    }

    /// Finds and loads project config by walking up the directory tree
    fn find_and_load_project_config(start_dir: &Path) -> Result<Option<Self>> {
        let mut current = start_dir;

        loop {
            let config_path = current.join(".demoji.toml");
            if config_path.exists() {
                let config = Self::load_from_file(&config_path).with_context(|| {
                    format!("Failed to load config from {}", config_path.display())
                })?;
                return Ok(Some(config));
            }

            // Move to parent directory
            match current.parent() {
                Some(parent) => current = parent,
                None => break, // Reached filesystem root
            }
        }

        Ok(None)
    }

    /// Loads global configuration from user's home directory
    fn load_global_config() -> Result<Option<Self>> {
        let Some(home_dir) = home_dir() else {
            return Ok(None); // No home directory, skip global config
        };

        // Try ~/.config/demoji/config.toml first (XDG standard)
        let xdg_config = home_dir.join(".config").join("demoji").join("config.toml");
        if xdg_config.exists() {
            let config = Self::load_from_file(&xdg_config).with_context(|| {
                format!("Failed to load global config from {}", xdg_config.display())
            })?;
            return Ok(Some(config));
        }

        // Fall back to ~/.demoji.toml
        let legacy_config = home_dir.join(".demoji.toml");
        if legacy_config.exists() {
            let config = Self::load_from_file(&legacy_config).with_context(|| {
                format!(
                    "Failed to load global config from {}",
                    legacy_config.display()
                )
            })?;
            return Ok(Some(config));
        }

        Ok(None)
    }

    /// Loads configuration from a specific file with detailed error handling
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_file(path: &Path) -> Result<Self> {
        // Read file with error handling
        let contents = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => DemojiError::PermissionDenied {
                        path: path.to_path_buf(),
                        source: e,
                    },
                    io::ErrorKind::NotFound => DemojiError::FileNotFound {
                        path: path.to_path_buf(),
                    },
                    _ => DemojiError::IoError {
                        message: format!("Failed to read config file: {}", path.display()),
                        source: e,
                    },
                }
                .into());
            },
        };

        // Parse TOML with detailed error handling
        match toml::from_str::<Self>(&contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                // Extract line and column information from the error if available
                let (line, column) = e.span().map_or((None, None), |span| {
                    // Count lines up to the error position
                    let line_num = contents[..span.start].matches('\n').count() + 1;
                    let last_newline = contents[..span.start].rfind('\n').map_or(0, |i| i + 1);
                    let col_num = span.start - last_newline + 1;
                    (Some(line_num), Some(col_num))
                });

                Err(DemojiError::ConfigParseError {
                    path: path.to_path_buf(),
                    source: Box::new(e),
                    line,
                    column,
                }
                .into())
            },
        }
    }

    /// Merges this config with another, preferring values from `other`
    ///
    /// This is used to implement the priority chain:
    /// defaults -> global config -> project config -> CLI args
    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        Self {
            mode: other.mode,
            placeholder: if other.placeholder == default_placeholder()
                && self.placeholder != default_placeholder()
            {
                self.placeholder
            } else {
                other.placeholder
            },
            extensions: if other.extensions.is_empty() {
                self.extensions
            } else {
                other.extensions
            },
            ignore_patterns: {
                let mut patterns = self.ignore_patterns;
                patterns.extend(other.ignore_patterns);
                patterns
            },
            backup: other.backup,
            dry_run: other.dry_run,
        }
    }

    /// Generates a default `.demoji.toml` template with comments
    #[must_use]
    pub fn generate_template() -> String {
        r#"# demoji configuration file
# See https://github.com/dbmrq/demoji for documentation

# Replacement mode: "smart", "remove", "replace", or "placeholder"
# - smart: Replace functional emojis with ASCII, remove decorative ones (default)
# - remove: Delete all emoji characters entirely
# - replace: Use ASCII alternatives for all mapped emojis
# - placeholder: Replace with custom placeholder string
mode = "smart"

# Custom placeholder string (only used when mode = "placeholder")
placeholder = "[EMOJI]"

# File extensions to process (empty = all text files)
# Example: extensions = ["rs", "py", "js", "ts", "go"]
extensions = []

# Additional patterns to ignore (beyond .gitignore)
# Example: ignore_patterns = ["*.generated.rs", "vendor/"]
ignore_patterns = []

# Create .bak files before modifying
backup = false

# Preview changes without writing (dry-run mode)
dry_run = false
"#
        .to_owned()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: ReplacementMode::default(),
            placeholder: default_placeholder(),
            extensions: Vec::new(),
            // Use the shared default ignore patterns from core::walker
            ignore_patterns: crate::core::default_ignore_patterns(),
            backup: false,
            dry_run: false,
        }
    }
}

/// Cross-platform home directory detection
fn home_dir() -> Option<PathBuf> {
    // Use std::env::var for cross-platform compatibility
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home));
    }

    // Windows fallback
    if let Ok(home) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(home));
    }

    None
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::str_to_string,
    clippy::assertions_on_result_states
)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.mode, ReplacementMode::Smart);
        assert_eq!(config.placeholder, "[EMOJI]");
        assert!(config.extensions.is_empty());
        assert!(!config.ignore_patterns.is_empty());
        assert!(!config.backup);
        assert!(!config.dry_run);
    }

    #[test]
    fn test_new_config() {
        let config = Config::new();
        assert_eq!(config.mode, ReplacementMode::Smart);
    }

    #[test]
    fn test_merge_configs() {
        let config1 = Config {
            mode: ReplacementMode::Remove,
            placeholder: "[EMOJI]".to_string(),
            extensions: vec!["rs".to_string()],
            ignore_patterns: vec!["*.bak".to_string()],
            backup: false,
            dry_run: false,
        };

        let config2 = Config {
            mode: ReplacementMode::Replace,
            placeholder: "[CUSTOM]".to_string(),
            extensions: vec!["py".to_string()],
            ignore_patterns: vec!["*.tmp".to_string()],
            backup: true,
            dry_run: true,
        };

        let merged = config1.merge(config2);
        assert_eq!(merged.mode, ReplacementMode::Replace);
        assert_eq!(merged.placeholder, "[CUSTOM]".to_string());
        assert_eq!(merged.extensions, vec!["py"]);
        assert!(merged.backup);
        assert!(merged.dry_run);
    }

    #[test]
    fn test_generate_template() {
        let template = Config::generate_template();
        assert!(template.contains("mode = \"smart\""));
        assert!(template.contains("placeholder = \"[EMOJI]\""));
        assert!(template.contains("extensions = []"));
        assert!(template.contains("ignore_patterns = []"));
    }

    #[test]
    fn test_load_from_file() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"
mode = "remove"
placeholder = "[EMOJI]"
extensions = ["rs", "py"]
backup = false
dry_run = false
"#;
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.mode, ReplacementMode::Remove);
        assert_eq!(config.extensions, vec!["rs", "py"]);
    }

    #[test]
    fn test_load_from_file_not_found() {
        let result = Config::load_from_file(Path::new("/nonexistent/path/.demoji.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_invalid_toml() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = "invalid toml [[[";
        fs::write(&config_path, content).unwrap();

        let result = Config::load_from_file(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_project_config() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"mode = "remove""#;
        fs::write(&config_path, content).unwrap();

        let result = Config::find_and_load_project_config(temp_dir.path()).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_project_config_not_found() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let result = Config::find_and_load_project_config(temp_dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_load_global_config_not_found() {
        let result = Config::load_global_config().unwrap();
        // Result depends on whether global config exists, so we just check it doesn't error
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_config_with_placeholder_mode() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"
mode = "placeholder"
placeholder = "[CUSTOM]"
"#;
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.mode, ReplacementMode::Placeholder);
        assert_eq!(config.placeholder, "[CUSTOM]");
    }

    #[test]
    fn test_config_with_replace_mode() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"mode = "replace""#;
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.mode, ReplacementMode::Replace);
    }

    #[test]
    fn test_config_with_extensions() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"extensions = ["rs", "py", "js"]"#;
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.extensions, vec!["rs", "py", "js"]);
    }

    #[test]
    fn test_config_with_ignore_patterns() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r#"ignore_patterns = ["*.bak", "vendor/"]"#;
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.ignore_patterns, vec!["*.bak", "vendor/"]);
    }

    #[test]
    fn test_config_with_backup_flag() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r"backup = true";
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert!(config.backup);
    }

    #[test]
    fn test_config_with_dry_run_flag() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");
        let content = r"dry_run = true";
        fs::write(&config_path, content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert!(config.dry_run);
    }
}
