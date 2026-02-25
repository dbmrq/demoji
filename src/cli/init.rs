//! Init subcommand implementation
//!
//! Handles the `demoji init` command to create a `.demoji.toml` configuration file.

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::Config;

/// Run the init subcommand
///
/// Creates a `.demoji.toml` configuration file in the specified directory.
/// If no path is provided, uses the current directory.
///
/// # Arguments
/// * `path` - Optional path where to create the config file
/// * `verbose` - Whether to print verbose output
/// * `quiet` - Whether to suppress output
///
/// # Errors
/// Returns an error if:
/// - The directory cannot be created
/// - The config file cannot be written
/// - Current directory cannot be determined
///
/// # Returns
/// * `Ok(())` if the config file was created successfully
/// * `Err` if there was an error (e.g., file already exists, permission denied)
#[allow(clippy::print_stdout, clippy::print_stderr)]
pub fn run_init(path: Option<PathBuf>, verbose: bool, quiet: bool) -> Result<()> {
    let target_dir = path.map_or_else(std::env::current_dir, Ok)?;

    // Ensure the directory exists
    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir)
            .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;
    }

    let config_path = target_dir.join(".demoji.toml");

    // Check if config file already exists
    if config_path.exists() {
        if !quiet {
            eprintln!("  Config file already exists at: {}", config_path.display());
        }
        return Ok(());
    }

    // Generate template content
    let template = Config::generate_template();

    // Write the template to file
    std::fs::write(&config_path, template)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    // Print success message
    if !quiet {
        println!(" Created .demoji.toml at: {}", config_path.display());
        if verbose {
            println!("   You can now customize the configuration to match your project's needs.");
            println!("   See the comments in the file for detailed option descriptions.");
        }
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");

        assert!(!config_path.exists());

        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        assert!(config_path.exists());
    }

    #[test]
    fn test_init_creates_valid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");

        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        // Should be able to parse the generated file
        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.mode, crate::core::ReplacementMode::Smart);
        assert_eq!(config.placeholder, "[EMOJI]");
    }

    #[test]
    fn test_init_skips_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".demoji.toml");

        // Create initial file
        std::fs::write(&config_path, "mode = \"placeholder\"").unwrap();

        // Try to init again - should skip
        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        // File should still have original content
        let content = std::fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, "mode = \"placeholder\"");
    }

    #[test]
    fn test_init_with_current_directory() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = run_init(None, false, true);

        // Restore original directory
        std::env::set_current_dir(&original_dir).unwrap();

        result.unwrap();

        let config_path = temp_dir.path().join(".demoji.toml");
        assert!(config_path.exists());
    }

    #[test]
    fn test_init_creates_directory_if_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir");

        assert!(!nested_path.exists());

        run_init(Some(nested_path.clone()), false, true).unwrap();

        assert!(nested_path.exists());
        assert!(nested_path.join(".demoji.toml").exists());
    }

    #[test]
    fn test_init_template_contains_comments() {
        let temp_dir = TempDir::new().unwrap();

        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        let content = std::fs::read_to_string(temp_dir.path().join(".demoji.toml")).unwrap();
        assert!(content.contains("# demoji configuration file"));
        assert!(content.contains("# Replacement mode"));
        assert!(content.contains("# Custom placeholder"));
        assert!(content.contains("# File extensions"));
        assert!(content.contains("# Additional patterns"));
    }

    #[test]
    fn test_init_template_has_all_options() {
        let temp_dir = TempDir::new().unwrap();

        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        let content = std::fs::read_to_string(temp_dir.path().join(".demoji.toml")).unwrap();
        assert!(content.contains("mode = \"smart\""));
        assert!(content.contains("placeholder = \"[EMOJI]\""));
        assert!(content.contains("extensions = []"));
        assert!(content.contains("ignore_patterns = []"));
        assert!(content.contains("backup = false"));
        assert!(content.contains("dry_run = false"));
    }

    #[test]
    fn test_init_verbose_mode() {
        let temp_dir = TempDir::new().unwrap();

        // Should not panic with verbose=true
        run_init(Some(temp_dir.path().to_path_buf()), true, false).unwrap();

        assert!(temp_dir.path().join(".demoji.toml").exists());
    }

    #[test]
    fn test_init_quiet_mode() {
        let temp_dir = TempDir::new().unwrap();

        // Should not panic with quiet=true
        run_init(Some(temp_dir.path().to_path_buf()), false, true).unwrap();

        assert!(temp_dir.path().join(".demoji.toml").exists());
    }
}
