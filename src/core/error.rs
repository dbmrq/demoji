//! Custom error types for demoji
//!
//! Provides structured error handling with user-friendly messages and helpful suggestions.

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for demoji operations
pub type DemojiResult<T> = Result<T, DemojiError>;

/// Custom error type for demoji operations
#[derive(Error, Debug)]
pub enum DemojiError {
    /// Permission denied error with helpful suggestion
    #[error("Permission denied: {path}")]
    PermissionDenied {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// File not found error
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// File encoding error (non-UTF-8)
    #[error("File encoding error: {path} is not valid UTF-8")]
    EncodingError { path: PathBuf },

    /// Configuration parsing error with optional line/column info
    #[error("Configuration parsing error in {path}")]
    ConfigParseError {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        line: Option<usize>,
        column: Option<usize>,
    },

    /// Invalid replacement mode
    #[error("Invalid replacement mode: {mode}. Use 'remove', 'replace', or 'placeholder'")]
    InvalidMode { mode: String },

    /// Invalid path (doesn't exist or is invalid)
    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },

    /// Generic IO error
    #[error("IO error: {message}")]
    IoError {
        message: String,
        #[source]
        source: io::Error,
    },

    /// Directory walking error
    #[error("Error walking directory: {message}")]
    WalkError { message: String },

    /// Configuration not found (non-fatal)
    #[error("Configuration file not found")]
    ConfigNotFound,

    /// Generic error with context
    #[error("{message}")]
    Other { message: String },
}

impl DemojiError {
    /// Returns a user-friendly suggestion for fixing this error
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            DemojiError::PermissionDenied { .. } => {
                Some("Try running with elevated privileges (sudo) or check file permissions with 'ls -la'")
            }
            DemojiError::EncodingError { .. } => {
                Some("The file contains non-UTF-8 characters. Try converting it to UTF-8 or excluding it from processing.")
            }
            DemojiError::FileNotFound { .. } => {
                Some("Check that the path exists and is spelled correctly.")
            }
            DemojiError::ConfigParseError { .. } => {
                Some("Check the TOML syntax in your .demoji.toml file. Use 'demoji init' to generate a valid template.")
            }
            DemojiError::InvalidMode { .. } => {
                Some("Valid modes are: 'remove' (default), 'replace' (ASCII alternatives), or 'placeholder' (custom text).")
            }
            DemojiError::InvalidPath { .. } => {
                Some("Verify the path exists and is accessible.")
            }
            DemojiError::IoError { .. } => {
                Some("Check file permissions and disk space.")
            }
            DemojiError::WalkError { .. } => {
                Some("Check that the directory exists and is accessible.")
            }
            DemojiError::ConfigNotFound => {
                Some("Run 'demoji init' to create a configuration file, or use CLI flags to configure behavior.")
            }
            DemojiError::Other { .. } => None,
        }
    }

    /// Formats the error with suggestion for display to user
    pub fn user_message(&self) -> String {
        let mut msg = format!("Error: {}", self);
        if let Some(suggestion) = self.suggestion() {
            msg.push('\n');
            msg.push_str("Suggestion: ");
            msg.push_str(suggestion);
        }
        msg
    }
}

/// Convert from io::Error to DemojiError
impl From<io::Error> for DemojiError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::PermissionDenied => DemojiError::PermissionDenied {
                path: PathBuf::from("<unknown>"),
                source: err,
            },
            io::ErrorKind::NotFound => DemojiError::FileNotFound {
                path: PathBuf::from("<unknown>"),
            },
            _ => DemojiError::IoError {
                message: err.to_string(),
                source: err,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_denied_error() {
        let err = DemojiError::PermissionDenied {
            path: PathBuf::from("test.txt"),
            source: io::Error::new(io::ErrorKind::PermissionDenied, "access denied"),
        };
        assert!(err.to_string().contains("Permission denied"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_file_not_found_error() {
        let err = DemojiError::FileNotFound {
            path: PathBuf::from("missing.txt"),
        };
        assert!(err.to_string().contains("File not found"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_encoding_error() {
        let err = DemojiError::EncodingError {
            path: PathBuf::from("binary.bin"),
        };
        assert!(err.to_string().contains("not valid UTF-8"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_invalid_mode_error() {
        let err = DemojiError::InvalidMode {
            mode: "invalid".to_string(),
        };
        assert!(err.to_string().contains("Invalid replacement mode"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_invalid_path_error() {
        let err = DemojiError::InvalidPath {
            path: PathBuf::from("/nonexistent/path"),
        };
        assert!(err.to_string().contains("Invalid path"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_config_parse_error() {
        let source = Box::new(io::Error::new(io::ErrorKind::InvalidData, "invalid TOML"));
        let err = DemojiError::ConfigParseError {
            path: PathBuf::from(".demoji.toml"),
            source,
            line: Some(5),
            column: Some(10),
        };
        assert!(err.to_string().contains("Configuration parsing error"));
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_user_message_includes_suggestion() {
        let err = DemojiError::FileNotFound {
            path: PathBuf::from("test.txt"),
        };
        let msg = err.user_message();
        assert!(msg.contains("Error:"));
        assert!(msg.contains("Suggestion:"));
    }

    #[test]
    fn test_config_not_found_error() {
        let err = DemojiError::ConfigNotFound;
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let demoji_err: DemojiError = io_err.into();
        match demoji_err {
            DemojiError::PermissionDenied { .. } => {
                // Expected
            }
            _ => panic!("Expected PermissionDenied error"),
        }
    }

    #[test]
    fn test_walk_error() {
        let err = DemojiError::WalkError {
            message: "Failed to read directory".to_string(),
        };
        assert!(err.to_string().contains("Error walking directory"));
        assert!(err.suggestion().is_some());
    }
}
