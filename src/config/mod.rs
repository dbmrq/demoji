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
use std::path::{Path, PathBuf};

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

    /// File extensions to process (e.g., ["rs", "py", "js"])
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
    "[EMOJI]".to_string()
}

impl Config {
    /// Creates a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads configuration from files and merges with defaults
    ///
    /// Searches for configuration in the following order:
    /// 1. Project config: walks up from current directory looking for .demoji.toml
    /// 2. Global config: ~/.config/demoji/config.toml or ~/.demoji.toml
    /// 3. Falls back to defaults if no config files found
    pub fn load() -> Result<Self> {
        Self::load_from_dir(&std::env::current_dir()?)
    }

    /// Loads configuration starting from a specific directory
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
                let config = Self::load_from_file(&config_path)
                    .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
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
        let home_dir = match home_dir() {
            Some(dir) => dir,
            None => return Ok(None), // No home directory, skip global config
        };

        // Try ~/.config/demoji/config.toml first (XDG standard)
        let xdg_config = home_dir.join(".config").join("demoji").join("config.toml");
        if xdg_config.exists() {
            let config = Self::load_from_file(&xdg_config)
                .with_context(|| format!("Failed to load global config from {}", xdg_config.display()))?;
            return Ok(Some(config));
        }

        // Fall back to ~/.demoji.toml
        let legacy_config = home_dir.join(".demoji.toml");
        if legacy_config.exists() {
            let config = Self::load_from_file(&legacy_config)
                .with_context(|| format!("Failed to load global config from {}", legacy_config.display()))?;
            return Ok(Some(config));
        }

        Ok(None)
    }

    /// Loads configuration from a specific file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Self = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML config: {}", path.display()))?;

        Ok(config)
    }

    /// Merges this config with another, preferring values from `other`
    ///
    /// This is used to implement the priority chain:
    /// defaults -> global config -> project config -> CLI args
    pub fn merge(self, other: Self) -> Self {
        Self {
            mode: other.mode,
            placeholder: if other.placeholder == default_placeholder() && self.placeholder != default_placeholder() {
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

    /// Generates a default .demoji.toml template with comments
    pub fn generate_template() -> String {
        r#"# demoji configuration file
# See https://github.com/yourusername/demoji for full documentation

# Replacement mode: "remove", "replace", or "placeholder"
# - remove: Delete emoji characters entirely (default)
# - replace: Use ASCII alternatives (e.g., 😊 → :), ❌ → [X])
# - placeholder: Replace with custom placeholder string
mode = "remove"

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
"#.to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: ReplacementMode::default(),
            placeholder: default_placeholder(),
            extensions: Vec::new(),
            ignore_patterns: vec![
                // Binary file extensions
                "*.png".to_string(),
                "*.jpg".to_string(),
                "*.jpeg".to_string(),
                "*.gif".to_string(),
                "*.ico".to_string(),
                "*.woff".to_string(),
                "*.woff2".to_string(),
                "*.ttf".to_string(),
                "*.otf".to_string(),
                "*.exe".to_string(),
                "*.dll".to_string(),
                "*.so".to_string(),
                "*.dylib".to_string(),
                "*.zip".to_string(),
                "*.tar".to_string(),
                "*.gz".to_string(),
                "*.bz2".to_string(),
                "*.xz".to_string(),
                "*.pdf".to_string(),
                "*.mp4".to_string(),
                "*.mp3".to_string(),
                "*.wav".to_string(),
                // Common directories to ignore
                ".git/".to_string(),
                "node_modules/".to_string(),
                "target/".to_string(),
                "build/".to_string(),
                "dist/".to_string(),
                ".next/".to_string(),
                "__pycache__/".to_string(),
                ".venv/".to_string(),
                "venv/".to_string(),
                "vendor/".to_string(),
                ".idea/".to_string(),
                ".vscode/".to_string(),
            ],
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
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(userprofile));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.mode, ReplacementMode::Remove);
        assert_eq!(config.placeholder, "[EMOJI]");
        assert!(config.extensions.is_empty());
        assert!(!config.ignore_patterns.is_empty()); // Should have default ignores
        assert!(!config.backup);
        assert!(!config.dry_run);
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");

        let toml_content = r#"
mode = "placeholder"
placeholder = "[REMOVED]"
extensions = ["rs", "py"]
ignore_patterns = ["*.test.rs"]
backup = true
dry_run = true
"#;

        std::fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.mode, ReplacementMode::Placeholder);
        assert_eq!(config.placeholder, "[REMOVED]");
        assert_eq!(config.extensions, vec!["rs", "py"]);
        assert_eq!(config.ignore_patterns, vec!["*.test.rs"]);
        assert!(config.backup);
        assert!(config.dry_run);
    }

    #[test]
    fn test_merge_configs() {
        let base = Config {
            mode: ReplacementMode::Remove,
            placeholder: "[EMOJI]".to_string(),
            extensions: vec!["rs".to_string()],
            ignore_patterns: vec!["*.bak".to_string()],
            backup: false,
            dry_run: false,
        };

        let override_config = Config {
            mode: ReplacementMode::Placeholder,
            placeholder: "[REMOVED]".to_string(),
            extensions: vec!["py".to_string()],
            ignore_patterns: vec!["*.tmp".to_string()],
            backup: true,
            dry_run: true,
        };

        let merged = base.merge(override_config);
        assert_eq!(merged.mode, ReplacementMode::Placeholder);
        assert_eq!(merged.placeholder, "[REMOVED]");
        assert_eq!(merged.extensions, vec!["py"]);
        assert_eq!(merged.ignore_patterns, vec!["*.bak", "*.tmp"]);
        assert!(merged.backup);
        assert!(merged.dry_run);
    }

    #[test]
    fn test_find_project_config() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let subdir = root.join("src").join("nested");
        std::fs::create_dir_all(&subdir).unwrap();

        // Create config in root
        let config_path = root.join(".demoji.toml");
        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(file, "mode = \"placeholder\"").unwrap();
        writeln!(file, "placeholder = \"[TEST]\"").unwrap();

        // Load from nested directory - should find root config
        let config = Config::load_from_dir(&subdir).unwrap();
        assert_eq!(config.mode, ReplacementMode::Placeholder);
        assert_eq!(config.placeholder, "[TEST]");
    }

    #[test]
    fn test_generate_template() {
        let template = Config::generate_template();
        assert!(template.contains("mode = \"remove\""));
        assert!(template.contains("placeholder = \"[EMOJI]\""));
        assert!(template.contains("extensions = []"));
        assert!(template.contains("# demoji configuration file"));
    }
}

