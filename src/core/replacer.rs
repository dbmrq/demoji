//! Replacement strategies for emoji characters
//!
//! Defines different modes for handling detected emojis.

use serde::{Deserialize, Serialize};

/// Replacement mode for emoji characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplacementMode {
    /// Remove emoji characters entirely
    Remove,
    /// Replace with ASCII alternatives
    Replace,
    /// Replace with a configurable placeholder
    Placeholder,
}

impl Default for ReplacementMode {
    fn default() -> Self {
        Self::Remove
    }
}

/// Trait for emoji replacement strategies
pub trait EmojiReplacer {
    // Will be implemented in Phase 2
}

