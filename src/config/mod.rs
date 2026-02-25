//! Configuration module
//!
//! Handles loading and merging configuration from .demoji.toml files.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Configuration for demoji
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Will be implemented in Phase 4
}

impl Config {
    /// Creates a new config with default values
    pub fn new() -> Self {
        Self {}
    }

    /// Loads configuration from files and merges with defaults
    pub fn load() -> Result<Self> {
        // Will be implemented in Phase 4
        Ok(Self::new())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

