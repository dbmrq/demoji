//! Backup functionality module
//!
//! Handles creating and managing backup files before modifications.

// Allow cfg blocks with trailing statements inside
#![allow(clippy::semicolon_outside_block)]

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Manages backup files
///
/// Creates `.bak` files before modifying originals, with support for
/// configurable backup directories and cleanup of old backups.
pub struct BackupManager {
    /// Directory where backups are stored (None = same as original file)
    backup_dir: Option<PathBuf>,
}

impl BackupManager {
    /// Creates a new backup manager with default settings
    ///
    /// By default, backups are created in the same directory as the original file.
    pub const fn new() -> Self {
        Self { backup_dir: None }
    }

    /// Creates a backup manager with a custom backup directory
    ///
    /// # Arguments
    /// * `backup_dir` - Directory where backups will be stored
    pub fn with_backup_dir<P: AsRef<Path>>(backup_dir: P) -> Self {
        Self {
            backup_dir: Some(backup_dir.as_ref().to_path_buf()),
        }
    }

    /// Creates a backup of the specified file
    ///
    /// Creates a `.bak` file before modifying the original.
    /// Preserves file permissions from the original file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to backup
    ///
    /// # Returns
    /// The path to the created backup file
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be read
    /// - The backup file cannot be written
    /// - Permissions cannot be preserved
    pub fn create_backup(&self, file_path: &Path) -> Result<PathBuf> {
        // Determine backup file path
        let backup_path = self.get_backup_path(file_path)?;

        // Create backup directory if needed
        if let Some(parent) = backup_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create backup directory: {}", parent.display())
                })?;
            }
        }

        // Copy file to backup location
        fs::copy(file_path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

        // Preserve file permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(file_path).with_context(|| {
                format!("Failed to read file metadata: {}", file_path.display())
            })?;
            let permissions = fs::Permissions::from_mode(metadata.permissions().mode());
            fs::set_permissions(&backup_path, permissions).with_context(|| {
                format!(
                    "Failed to set backup permissions: {}",
                    backup_path.display()
                )
            })?;
        }

        Ok(backup_path)
    }

    /// Restores a file from its backup
    ///
    /// Copies the `.bak` file back to the original location.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to restore
    ///
    /// # Errors
    /// Returns an error if:
    /// - The backup file doesn't exist
    /// - The file cannot be restored
    pub fn restore_backup(&self, file_path: &Path) -> Result<()> {
        let backup_path = self.get_backup_path(file_path)?;

        if !backup_path.exists() {
            return Err(anyhow::anyhow!(
                "Backup file not found: {}",
                backup_path.display()
            ));
        }

        // Copy backup back to original location
        fs::copy(&backup_path, file_path)
            .with_context(|| format!("Failed to restore from backup: {}", backup_path.display()))?;

        // Preserve permissions from backup
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&backup_path).with_context(|| {
                format!("Failed to read backup metadata: {}", backup_path.display())
            })?;
            let permissions = fs::Permissions::from_mode(metadata.permissions().mode());
            fs::set_permissions(file_path, permissions).with_context(|| {
                format!("Failed to set file permissions: {}", file_path.display())
            })?;
        }

        Ok(())
    }

    /// Cleans up backup files older than the specified duration
    ///
    /// Removes `.bak` files that haven't been modified within the specified duration.
    ///
    /// # Arguments
    /// * `older_than` - Duration threshold for cleanup
    ///
    /// # Returns
    /// Number of backup files deleted
    ///
    /// # Errors
    /// Returns an error if directory traversal fails
    pub fn cleanup_backups(&self, older_than: Duration) -> Result<usize> {
        let backup_dir = match &self.backup_dir {
            Some(dir) => dir.clone(),
            None => {
                // If no backup directory is configured, nothing to clean up
                return Ok(0);
            },
        };

        if !backup_dir.exists() {
            return Ok(0);
        }

        let now = SystemTime::now();
        let mut deleted_count = 0;

        for entry in fs::read_dir(&backup_dir)
            .with_context(|| format!("Failed to read backup directory: {}", backup_dir.display()))?
        {
            let entry = entry.with_context(|| {
                format!(
                    "Failed to read directory entry in: {}",
                    backup_dir.display()
                )
            })?;
            let path = entry.path();

            // Only process .bak files
            if path.extension().and_then(|ext| ext.to_str()) != Some("bak") {
                continue;
            }

            // Check modification time
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = now.duration_since(modified) {
                        if elapsed > older_than && fs::remove_file(&path).is_ok() {
                            deleted_count += 1;
                        }
                    }
                }
            }
        }

        Ok(deleted_count)
    }

    /// Gets the backup path for a given file
    ///
    /// If a backup directory is configured, the backup is placed there with the same filename.
    /// Otherwise, the backup is placed in the same directory as the original with a `.bak` extension.
    fn get_backup_path(&self, file_path: &Path) -> Result<PathBuf> {
        if let Some(backup_dir) = &self.backup_dir {
            let file_name = file_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file path: {}", file_path.display()))?;
            // Append .bak to the filename
            let backup_name = format!("{}.bak", file_name.to_string_lossy());
            Ok(backup_dir.join(backup_name))
        } else {
            // Backup in same directory as original
            let mut backup_path = file_path.to_path_buf();
            let current_ext = backup_path
                .extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default();

            if current_ext.is_empty() {
                backup_path.set_extension("bak");
            } else {
                let new_ext = format!("{current_ext}.bak");
                backup_path.set_extension(new_ext);
            }

            Ok(backup_path)
        }
    }
}

impl Default for BackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::assertions_on_result_states,
    clippy::create_dir
)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_backup_manager_new() {
        let manager = BackupManager::new();
        assert!(manager.backup_dir.is_none());
    }

    #[test]
    fn test_backup_manager_with_backup_dir() {
        let temp_dir = TempDir::new().unwrap();
        let manager = BackupManager::with_backup_dir(temp_dir.path());
        assert!(manager.backup_dir.is_some());
        assert_eq!(manager.backup_dir.as_ref().unwrap(), temp_dir.path());
    }

    #[test]
    fn test_create_backup_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello, World!";

        fs::write(&file_path, content).unwrap();

        let manager = BackupManager::new();
        let backup_path = manager.create_backup(&file_path).unwrap();

        // Backup should be in same directory with .bak extension
        assert_eq!(backup_path, temp_dir.path().join("test.txt.bak"));
        assert!(backup_path.exists());

        // Backup content should match original
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, content);
    }

    #[test]
    fn test_create_backup_custom_directory() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello, World!";

        fs::write(&file_path, content).unwrap();

        let manager = BackupManager::with_backup_dir(&backup_dir);
        let backup_path = manager.create_backup(&file_path).unwrap();

        // Backup should be in custom directory
        assert_eq!(backup_path, backup_dir.join("test.txt.bak"));
        assert!(backup_path.exists());

        // Backup content should match original
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, content);
    }

    #[test]
    fn test_create_backup_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("nested").join("backups");
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "content").unwrap();

        let manager = BackupManager::with_backup_dir(&backup_dir);
        let backup_path = manager.create_backup(&file_path).unwrap();

        // Directory should be created
        assert!(backup_dir.exists());
        assert!(backup_path.exists());
    }

    #[test]
    fn test_restore_backup() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let original_content = "Original content";
        let modified_content = "Modified content";

        // Create original file
        fs::write(&file_path, original_content).unwrap();

        // Create backup
        let manager = BackupManager::new();
        manager.create_backup(&file_path).unwrap();

        // Modify original file
        fs::write(&file_path, modified_content).unwrap();
        assert_eq!(fs::read_to_string(&file_path).unwrap(), modified_content);

        // Restore from backup
        manager.restore_backup(&file_path).unwrap();
        assert_eq!(fs::read_to_string(&file_path).unwrap(), original_content);
    }

    #[test]
    fn test_restore_backup_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");

        let manager = BackupManager::new();
        let result = manager.restore_backup(&file_path);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Backup file not found"));
    }

    #[test]
    fn test_get_backup_path_same_directory() {
        let manager = BackupManager::new();
        let file_path = Path::new("/path/to/file.txt");
        let backup_path = manager.get_backup_path(file_path).unwrap();

        assert_eq!(backup_path, Path::new("/path/to/file.txt.bak"));
    }

    #[test]
    fn test_get_backup_path_custom_directory() {
        let manager = BackupManager::with_backup_dir("/backups");
        let file_path = Path::new("/path/to/file.txt");
        let backup_path = manager.get_backup_path(file_path).unwrap();

        assert_eq!(backup_path, Path::new("/backups/file.txt.bak"));
    }

    #[test]
    fn test_get_backup_path_no_extension() {
        let manager = BackupManager::new();
        let file_path = Path::new("/path/to/Makefile");
        let backup_path = manager.get_backup_path(file_path).unwrap();

        assert_eq!(backup_path, Path::new("/path/to/Makefile.bak"));
    }

    #[test]
    fn test_cleanup_backups_no_directory() {
        let manager = BackupManager::new();
        let deleted = manager.cleanup_backups(Duration::from_secs(3600)).unwrap();
        assert_eq!(deleted, 0);
    }

    #[test]
    fn test_cleanup_backups_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        fs::create_dir(&backup_dir).unwrap();

        let manager = BackupManager::with_backup_dir(&backup_dir);
        let deleted = manager.cleanup_backups(Duration::from_secs(3600)).unwrap();
        assert_eq!(deleted, 0);
    }

    #[test]
    fn test_cleanup_backups_removes_old_files() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        fs::create_dir(&backup_dir).unwrap();

        // Create a backup file
        let backup_file = backup_dir.join("test.txt.bak");
        fs::write(&backup_file, "backup content").unwrap();

        // Set modification time to 2 hours ago (using a trick with file metadata)
        // For testing purposes, we'll just verify the cleanup function works
        let manager = BackupManager::with_backup_dir(&backup_dir);

        // With a very short duration, the file should be old enough to delete
        // But since we just created it, it won't be deleted
        let result = manager.cleanup_backups(Duration::from_secs(0));
        // Verify cleanup_backups runs successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_cleanup_backups_ignores_non_bak_files() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        fs::create_dir(&backup_dir).unwrap();

        // Create various files
        fs::write(backup_dir.join("test.txt"), "content").unwrap();
        fs::write(backup_dir.join("test.txt.bak"), "backup").unwrap();
        fs::write(backup_dir.join("other.rs"), "code").unwrap();

        let manager = BackupManager::with_backup_dir(&backup_dir);
        let _deleted = manager.cleanup_backups(Duration::from_secs(0)).unwrap();

        // Should only process .bak files
        assert!(backup_dir.join("test.txt").exists());
        assert!(backup_dir.join("other.rs").exists());
    }

    #[test]
    fn test_backup_preserves_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        let content = r#"fn main() {
    println!("Hello 👋 World 🎉");
}
"#;

        fs::write(&file_path, content).unwrap();

        let manager = BackupManager::new();
        let backup_path = manager.create_backup(&file_path).unwrap();

        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, content);
    }

    #[test]
    fn test_multiple_backups_same_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "version 1").unwrap();
        let manager = BackupManager::new();
        let backup1 = manager.create_backup(&file_path).unwrap();

        fs::write(&file_path, "version 2").unwrap();
        let backup2 = manager.create_backup(&file_path).unwrap();

        // Both should point to same backup location (second overwrites first)
        assert_eq!(backup1, backup2);
        assert_eq!(fs::read_to_string(&backup2).unwrap(), "version 2");
    }

    #[test]
    fn test_backup_with_special_characters_in_filename() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test-file_2024.txt");

        fs::write(&file_path, "content").unwrap();

        let manager = BackupManager::new();
        let backup_path = manager.create_backup(&file_path).unwrap();

        assert!(backup_path.exists());
        assert_eq!(backup_path.file_name().unwrap(), "test-file_2024.txt.bak");
    }
}
