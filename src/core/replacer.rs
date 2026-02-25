//! Replacement strategies for emoji characters
//!
//! Defines different modes for handling detected emojis and provides
//! comprehensive ASCII mappings for common emoji characters.

// Allow str_to_string in this module due to extensive emoji mapping initialization
#![allow(clippy::str_to_string)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Replacement mode for emoji characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReplacementMode {
    /// Smart mode: replace functional emojis with ASCII, remove decorative ones (default)
    #[default]
    Smart,
    /// Remove emoji characters entirely
    Remove,
    /// Replace with ASCII alternatives (all mapped emojis)
    Replace,
    /// Replace with a configurable placeholder
    Placeholder,
}

/// Trait for emoji replacement strategies
pub trait EmojiReplacer {
    /// Replace an emoji character with its alternative representation
    ///
    /// # Arguments
    /// * `emoji` - The emoji character(s) to replace
    ///
    /// # Returns
    /// The replacement string, or None if the emoji should be removed
    fn replace(&self, emoji: &str) -> Option<String>;
}

/// Replacer that removes all emoji characters
#[derive(Debug, Default)]
pub struct RemoveReplacer;

impl EmojiReplacer for RemoveReplacer {
    fn replace(&self, _emoji: &str) -> Option<String> {
        None
    }
}

/// Replacer that uses ASCII alternatives from a predefined mapping
#[derive(Debug)]
pub struct AsciiReplacer {
    mapping: HashMap<String, String>,
}

impl Default for AsciiReplacer {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiReplacer {
    /// Creates a new ASCII replacer with the default emoji mapping
    pub fn new() -> Self {
        Self {
            mapping: Self::build_default_mapping(),
        }
    }

    /// Creates a new ASCII replacer with a custom mapping
    #[must_use]
    pub const fn with_mapping(mapping: HashMap<String, String>) -> Self {
        Self { mapping }
    }

    /// Builds the default emoji-to-ASCII mapping
    #[allow(clippy::too_many_lines)]
    fn build_default_mapping() -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Smileys and emotions - positive
        map.insert("😀".to_string(), ":D".to_string());
        map.insert("😁".to_string(), ":D".to_string());
        map.insert("😂".to_string(), ":'D".to_string());
        map.insert("🤣".to_string(), ":'D".to_string());
        map.insert("😃".to_string(), ":D".to_string());
        map.insert("😄".to_string(), ":D".to_string());
        map.insert("😅".to_string(), "^^;".to_string());
        map.insert("😆".to_string(), "XD".to_string());
        map.insert("😊".to_string(), ":)".to_string());
        map.insert("😇".to_string(), "O:)".to_string());
        map.insert("🙂".to_string(), ":)".to_string());
        map.insert("🙃".to_string(), "(:".to_string());
        map.insert("😉".to_string(), ";)".to_string());
        map.insert("😌".to_string(), ":)".to_string());
        map.insert("😍".to_string(), "<3".to_string());
        map.insert("🥰".to_string(), "<3".to_string());
        map.insert("😘".to_string(), ":*".to_string());
        map.insert("😗".to_string(), ":*".to_string());
        map.insert("😙".to_string(), ":*".to_string());
        map.insert("😚".to_string(), ":*".to_string());
        map.insert("😋".to_string(), ":P".to_string());
        map.insert("😛".to_string(), ":P".to_string());
        map.insert("😜".to_string(), ";P".to_string());
        map.insert("😝".to_string(), "XP".to_string());
        map.insert("🤪".to_string(), ":P".to_string());

        // Smileys and emotions - neutral/thinking
        map.insert("🤔".to_string(), ":-?".to_string());
        map.insert("🤨".to_string(), ":/".to_string());
        map.insert("😐".to_string(), ":|".to_string());
        map.insert("😑".to_string(), ":|".to_string());
        map.insert("😶".to_string(), ":-X".to_string());
        map.insert("🙄".to_string(), "-_-".to_string());
        map.insert("😏".to_string(), ":]".to_string());
        map.insert("😒".to_string(), ":/".to_string());
        map.insert("🤐".to_string(), ":-X".to_string());

        // Smileys and emotions - negative
        map.insert("😞".to_string(), ":(".to_string());
        map.insert("😔".to_string(), ":(".to_string());
        map.insert("😟".to_string(), ":(".to_string());
        map.insert("😕".to_string(), ":/".to_string());
        map.insert("🙁".to_string(), ":(".to_string());
        map.insert("☹️".to_string(), ":(".to_string());
        map.insert("☹".to_string(), ":(".to_string());
        map.insert("😣".to_string(), ">_<".to_string());
        map.insert("😖".to_string(), ">_<".to_string());
        map.insert("😫".to_string(), ">_<".to_string());
        map.insert("😩".to_string(), ":((".to_string());
        map.insert("🥺".to_string(), ":'(".to_string());
        map.insert("😢".to_string(), ":'(".to_string());
        map.insert("😭".to_string(), "T_T".to_string());
        map.insert("😤".to_string(), ">:(".to_string());
        map.insert("😠".to_string(), ">:(".to_string());
        map.insert("😡".to_string(), ">:(".to_string());
        map.insert("🤬".to_string(), "#$%!".to_string());
        map.insert("😱".to_string(), "D:".to_string());
        map.insert("😨".to_string(), "D:".to_string());
        map.insert("😰".to_string(), "D:".to_string());
        map.insert("😥".to_string(), ":'(".to_string());

        // Smileys and emotions - other
        map.insert("😴".to_string(), "zzz".to_string());
        map.insert("🤤".to_string(), "*drool*".to_string());
        map.insert("😪".to_string(), "zzz".to_string());
        map.insert("😵".to_string(), "X_X".to_string());
        map.insert("🤯".to_string(), "*mind blown*".to_string());
        map.insert("🤓".to_string(), "8)".to_string());
        map.insert("😎".to_string(), "8)".to_string());
        map.insert("🥳".to_string(), "\\o/".to_string());
        map.insert("😬".to_string(), ":-S".to_string());

        // Hearts and love
        map.insert("❤️".to_string(), "<3".to_string());
        map.insert("❤".to_string(), "<3".to_string());
        map.insert("🧡".to_string(), "<3".to_string());
        map.insert("💛".to_string(), "<3".to_string());
        map.insert("💚".to_string(), "<3".to_string());
        map.insert("💙".to_string(), "<3".to_string());
        map.insert("💜".to_string(), "<3".to_string());
        map.insert("🖤".to_string(), "</3".to_string());
        map.insert("🤍".to_string(), "<3".to_string());
        map.insert("🤎".to_string(), "<3".to_string());
        map.insert("💔".to_string(), "</3".to_string());
        map.insert("💕".to_string(), "<3<3".to_string());
        map.insert("💞".to_string(), "<3<3".to_string());
        map.insert("💓".to_string(), "<3".to_string());
        map.insert("💗".to_string(), "<3".to_string());
        map.insert("💖".to_string(), "<3".to_string());
        map.insert("💘".to_string(), "<3".to_string());
        map.insert("💝".to_string(), "<3".to_string());

        // Hand gestures
        map.insert("👍".to_string(), "[+1]".to_string());
        map.insert("👎".to_string(), "[-1]".to_string());
        map.insert("👌".to_string(), "[OK]".to_string());
        map.insert("✌️".to_string(), "[v]".to_string());
        map.insert("✌".to_string(), "[v]".to_string());
        map.insert("🤞".to_string(), "[fingers crossed]".to_string());
        map.insert("🤟".to_string(), "[love]".to_string());
        map.insert("🤘".to_string(), "[rock]".to_string());
        map.insert("🤙".to_string(), "[call me]".to_string());
        map.insert("👈".to_string(), "[<-]".to_string());
        map.insert("👉".to_string(), "[->]".to_string());
        map.insert("👆".to_string(), "[^]".to_string());
        map.insert("👇".to_string(), "[v]".to_string());
        map.insert("☝️".to_string(), "[^]".to_string());
        map.insert("☝".to_string(), "[^]".to_string());
        map.insert("👋".to_string(), "[wave]".to_string());
        map.insert("🤚".to_string(), "[hand]".to_string());
        map.insert("🖐️".to_string(), "[hand]".to_string());
        map.insert("🖐".to_string(), "[hand]".to_string());
        map.insert("✋".to_string(), "[hand]".to_string());
        map.insert("🖖".to_string(), "[vulcan]".to_string());
        map.insert("👏".to_string(), "[clap]".to_string());
        map.insert("🙌".to_string(), "[\\o/]".to_string());
        map.insert("🤲".to_string(), "[hands]".to_string());
        map.insert("🤝".to_string(), "[handshake]".to_string());
        map.insert("🙏".to_string(), "[pray]".to_string());
        map.insert("✍️".to_string(), "[write]".to_string());
        map.insert("✍".to_string(), "[write]".to_string());
        map.insert("💪".to_string(), "[strong]".to_string());
        map.insert("🦾".to_string(), "[strong]".to_string());

        // Common symbols and checkmarks
        map.insert("✓".to_string(), "[v]".to_string());
        map.insert("✔️".to_string(), "[v]".to_string());
        map.insert("✔".to_string(), "[v]".to_string());
        map.insert("✅".to_string(), "[✓]".to_string());
        map.insert("✗".to_string(), "[X]".to_string());
        map.insert("✘".to_string(), "[X]".to_string());
        map.insert("❌".to_string(), "[X]".to_string());
        map.insert("❎".to_string(), "[X]".to_string());
        map.insert("⭐".to_string(), "[*]".to_string());
        map.insert("🌟".to_string(), "[*]".to_string());
        map.insert("✨".to_string(), "[sparkle]".to_string());
        map.insert("💫".to_string(), "[dizzy]".to_string());
        map.insert("⚡".to_string(), "[!]".to_string());
        map.insert("🔥".to_string(), "[fire]".to_string());
        map.insert("💥".to_string(), "[boom]".to_string());
        map.insert("💯".to_string(), "[100]".to_string());
        map.insert("🎯".to_string(), "[target]".to_string());

        // Warning and alert symbols
        map.insert("⚠️".to_string(), "[!]".to_string());
        map.insert("⚠".to_string(), "[!]".to_string());
        map.insert("🚨".to_string(), "[!!]".to_string());
        map.insert("🚫".to_string(), "[no]".to_string());
        map.insert("⛔".to_string(), "[stop]".to_string());
        map.insert("🛑".to_string(), "[stop]".to_string());
        map.insert("❗".to_string(), "[!]".to_string());
        map.insert("❓".to_string(), "[?]".to_string());
        map.insert("❔".to_string(), "[?]".to_string());
        map.insert("❕".to_string(), "[!]".to_string());
        map.insert("⁉️".to_string(), "[!?]".to_string());
        map.insert("⁉".to_string(), "[!?]".to_string());

        // Arrows
        map.insert("➡️".to_string(), "->".to_string());
        map.insert("➡".to_string(), "->".to_string());
        map.insert("⬅️".to_string(), "<-".to_string());
        map.insert("⬅".to_string(), "<-".to_string());
        map.insert("⬆️".to_string(), "^".to_string());
        map.insert("⬆".to_string(), "^".to_string());
        map.insert("⬇️".to_string(), "v".to_string());
        map.insert("⬇".to_string(), "v".to_string());
        map.insert("↗️".to_string(), "^>".to_string());
        map.insert("↗".to_string(), "^>".to_string());
        map.insert("↘️".to_string(), "v>".to_string());
        map.insert("↘".to_string(), "v>".to_string());
        map.insert("↙️".to_string(), "v<".to_string());
        map.insert("↙".to_string(), "v<".to_string());
        map.insert("↖️".to_string(), "^<".to_string());
        map.insert("↖".to_string(), "^<".to_string());
        map.insert("↔️".to_string(), "<->".to_string());
        map.insert("↔".to_string(), "<->".to_string());
        map.insert("↕️".to_string(), "^v".to_string());
        map.insert("↕".to_string(), "^v".to_string());
        map.insert("🔄".to_string(), "[refresh]".to_string());
        map.insert("🔃".to_string(), "[reload]".to_string());
        map.insert("🔁".to_string(), "[repeat]".to_string());
        map.insert("🔂".to_string(), "[repeat-1]".to_string());

        // Common objects and tools
        map.insert("📝".to_string(), "[memo]".to_string());
        map.insert("📄".to_string(), "[doc]".to_string());
        map.insert("📃".to_string(), "[page]".to_string());
        map.insert("📋".to_string(), "[clipboard]".to_string());
        map.insert("📁".to_string(), "[folder]".to_string());
        map.insert("📂".to_string(), "[folder-open]".to_string());
        map.insert("🗂️".to_string(), "[files]".to_string());
        map.insert("🗂".to_string(), "[files]".to_string());
        map.insert("📌".to_string(), "[pin]".to_string());
        map.insert("📍".to_string(), "[pin]".to_string());
        map.insert("🔖".to_string(), "[bookmark]".to_string());
        map.insert("🏷️".to_string(), "[tag]".to_string());
        map.insert("🏷".to_string(), "[tag]".to_string());
        map.insert("💼".to_string(), "[briefcase]".to_string());
        map.insert("🔧".to_string(), "[wrench]".to_string());
        map.insert("🔨".to_string(), "[hammer]".to_string());
        map.insert("⚙️".to_string(), "[gear]".to_string());
        map.insert("⚙".to_string(), "[gear]".to_string());
        map.insert("🛠️".to_string(), "[tools]".to_string());
        map.insert("🛠".to_string(), "[tools]".to_string());
        map.insert("🔍".to_string(), "[search]".to_string());
        map.insert("🔎".to_string(), "[search]".to_string());
        map.insert("🔑".to_string(), "[key]".to_string());
        map.insert("🗝️".to_string(), "[key]".to_string());
        map.insert("🗝".to_string(), "[key]".to_string());
        map.insert("🔒".to_string(), "[lock]".to_string());
        map.insert("🔓".to_string(), "[unlock]".to_string());
        map.insert("🔐".to_string(), "[secure]".to_string());

        // Tech and development
        map.insert("💻".to_string(), "[computer]".to_string());
        map.insert("⌨️".to_string(), "[keyboard]".to_string());
        map.insert("⌨".to_string(), "[keyboard]".to_string());
        map.insert("🖥️".to_string(), "[desktop]".to_string());
        map.insert("🖥".to_string(), "[desktop]".to_string());
        map.insert("🖱️".to_string(), "[mouse]".to_string());
        map.insert("🖱".to_string(), "[mouse]".to_string());
        map.insert("🖨️".to_string(), "[printer]".to_string());
        map.insert("🖨".to_string(), "[printer]".to_string());
        map.insert("💾".to_string(), "[save]".to_string());
        map.insert("💿".to_string(), "[cd]".to_string());
        map.insert("📀".to_string(), "[dvd]".to_string());
        map.insert("🔌".to_string(), "[plug]".to_string());
        map.insert("🔋".to_string(), "[battery]".to_string());
        map.insert("📱".to_string(), "[phone]".to_string());
        map.insert("📲".to_string(), "[mobile]".to_string());

        // Common development-related emojis
        map.insert("🐛".to_string(), "[bug]".to_string());
        map.insert("🐞".to_string(), "[bug]".to_string());
        map.insert("🚀".to_string(), "[rocket]".to_string());
        map.insert("📦".to_string(), "[package]".to_string());
        map.insert("🎉".to_string(), "[party]".to_string());
        map.insert("🎊".to_string(), "[confetti]".to_string());
        map.insert("🎈".to_string(), "[balloon]".to_string());
        map.insert("🏁".to_string(), "[checkered flag]".to_string());
        map.insert("🚩".to_string(), "[flag]".to_string());
        map.insert("⚓".to_string(), "[anchor]".to_string());
        map.insert("🔗".to_string(), "[link]".to_string());
        map.insert("📊".to_string(), "[chart]".to_string());
        map.insert("📈".to_string(), "[trending up]".to_string());
        map.insert("📉".to_string(), "[trending down]".to_string());
        map.insert("💡".to_string(), "[bulb]".to_string());
        map.insert("🔔".to_string(), "[bell]".to_string());
        map.insert("🔕".to_string(), "[no bell]".to_string());
        map.insert("📢".to_string(), "[loudspeaker]".to_string());
        map.insert("📣".to_string(), "[megaphone]".to_string());
        map.insert("💬".to_string(), "[speech]".to_string());
        map.insert("💭".to_string(), "[thought]".to_string());
        map.insert("🗨️".to_string(), "[speech left]".to_string());
        map.insert("🗨".to_string(), "[speech left]".to_string());
        map.insert("🗯️".to_string(), "[anger]".to_string());
        map.insert("🗯".to_string(), "[anger]".to_string());

        map
    }

    /// Get the ASCII replacement for an emoji, if available
    pub fn get_replacement(&self, emoji: &str) -> Option<&str> {
        self.mapping.get(emoji).map(String::as_str)
    }
}

impl EmojiReplacer for AsciiReplacer {
    fn replace(&self, emoji: &str) -> Option<String> {
        self.get_replacement(emoji).map(ToString::to_string)
    }
}

/// Replacer that uses a configurable placeholder string
#[derive(Debug, Clone)]
pub struct PlaceholderReplacer {
    placeholder: String,
}

impl Default for PlaceholderReplacer {
    fn default() -> Self {
        Self::new("[EMOJI]")
    }
}

impl PlaceholderReplacer {
    /// Creates a new placeholder replacer with the given placeholder string
    pub fn new(placeholder: &str) -> Self {
        Self {
            placeholder: placeholder.to_string(),
        }
    }
}

impl EmojiReplacer for PlaceholderReplacer {
    fn replace(&self, _emoji: &str) -> Option<String> {
        Some(self.placeholder.clone())
    }
}

/// Replacer that replaces functional emojis with ASCII and removes decorative ones
///
/// "Functional" emojis are those that convey meaning in code contexts:
/// - Checkmarks, X marks, and status indicators
/// - Warnings and alerts
/// - Arrows and directional indicators
/// - Common development symbols (bug, rocket, etc.)
///
/// Decorative emojis (faces, animals, food, etc.) are simply removed.
#[derive(Debug)]
pub struct SmartReplacer {
    mapping: HashMap<String, String>,
}

impl Default for SmartReplacer {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartReplacer {
    /// Creates a new smart replacer with functional emoji mappings
    pub fn new() -> Self {
        Self {
            mapping: Self::build_functional_mapping(),
        }
    }

    /// Builds the functional emoji-to-ASCII mapping
    ///
    /// Only includes emojis that have clear functional meaning in code:
    /// - Status indicators (checkmarks, X marks)
    /// - Warnings and alerts
    /// - Arrows and directions
    /// - Common dev symbols
    fn build_functional_mapping() -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Checkmarks and success indicators
        map.insert("✅".to_string(), "[OK]".to_string());
        map.insert("✓".to_string(), "[v]".to_string());
        map.insert("✔".to_string(), "[v]".to_string());
        map.insert("☑".to_string(), "[v]".to_string());
        map.insert("🗹".to_string(), "[v]".to_string());
        map.insert("👍".to_string(), "[+1]".to_string());

        // X marks and failure indicators
        map.insert("❌".to_string(), "[X]".to_string());
        map.insert("✗".to_string(), "[X]".to_string());
        map.insert("✘".to_string(), "[X]".to_string());
        map.insert("☒".to_string(), "[X]".to_string());
        map.insert("👎".to_string(), "[-1]".to_string());

        // Warning and alert symbols
        map.insert("⚠".to_string(), "[!]".to_string());
        map.insert("⚠️".to_string(), "[!]".to_string());
        map.insert("‼".to_string(), "[!!]".to_string());
        map.insert("‼️".to_string(), "[!!]".to_string());
        map.insert("❗".to_string(), "[!]".to_string());
        map.insert("❕".to_string(), "[!]".to_string());
        map.insert("❓".to_string(), "[?]".to_string());
        map.insert("❔".to_string(), "[?]".to_string());
        map.insert("⁉".to_string(), "[!?]".to_string());
        map.insert("⁉️".to_string(), "[!?]".to_string());
        map.insert("🚫".to_string(), "[no]".to_string());
        map.insert("⛔".to_string(), "[stop]".to_string());
        map.insert("🛑".to_string(), "[stop]".to_string());

        // Arrows
        map.insert("➡".to_string(), "->".to_string());
        map.insert("➡️".to_string(), "->".to_string());
        map.insert("⬅".to_string(), "<-".to_string());
        map.insert("⬅️".to_string(), "<-".to_string());
        map.insert("⬆".to_string(), "^".to_string());
        map.insert("⬆️".to_string(), "^".to_string());
        map.insert("⬇".to_string(), "v".to_string());
        map.insert("⬇️".to_string(), "v".to_string());
        map.insert("↔".to_string(), "<->".to_string());
        map.insert("↔️".to_string(), "<->".to_string());
        map.insert("↕".to_string(), "^v".to_string());
        map.insert("↕️".to_string(), "^v".to_string());
        map.insert("🔄".to_string(), "[refresh]".to_string());
        map.insert("🔃".to_string(), "[reload]".to_string());

        // Common development symbols
        map.insert("🐛".to_string(), "[bug]".to_string());
        map.insert("🐞".to_string(), "[bug]".to_string());
        map.insert("🚀".to_string(), "[rocket]".to_string());
        map.insert("💡".to_string(), "[idea]".to_string());
        map.insert("🔥".to_string(), "[fire]".to_string());
        map.insert("💥".to_string(), "[boom]".to_string());
        map.insert("⚡".to_string(), "[zap]".to_string());
        map.insert("⚡️".to_string(), "[zap]".to_string());
        map.insert("📝".to_string(), "[memo]".to_string());
        map.insert("📋".to_string(), "[clipboard]".to_string());
        map.insert("🔧".to_string(), "[wrench]".to_string());
        map.insert("🔨".to_string(), "[hammer]".to_string());
        map.insert("⚙".to_string(), "[gear]".to_string());
        map.insert("⚙️".to_string(), "[gear]".to_string());
        map.insert("🔗".to_string(), "[link]".to_string());
        map.insert("🔒".to_string(), "[lock]".to_string());
        map.insert("🔓".to_string(), "[unlock]".to_string());
        map.insert("🔑".to_string(), "[key]".to_string());
        map.insert("🔍".to_string(), "[search]".to_string());
        map.insert("📦".to_string(), "[package]".to_string());
        map.insert("🏷".to_string(), "[tag]".to_string());
        map.insert("🏷️".to_string(), "[tag]".to_string());
        map.insert("📌".to_string(), "[pin]".to_string());
        map.insert("📍".to_string(), "[pin]".to_string());
        map.insert("🎯".to_string(), "[target]".to_string());
        map.insert("💯".to_string(), "[100]".to_string());
        map.insert("⭐".to_string(), "[*]".to_string());
        map.insert("🌟".to_string(), "[*]".to_string());
        map.insert("✨".to_string(), "[sparkle]".to_string());

        // Info and communication
        map.insert("ℹ".to_string(), "[i]".to_string());
        map.insert("ℹ️".to_string(), "[i]".to_string());
        map.insert("🔔".to_string(), "[bell]".to_string());
        map.insert("🔕".to_string(), "[no bell]".to_string());
        map.insert("📢".to_string(), "[announce]".to_string());
        map.insert("📣".to_string(), "[megaphone]".to_string());

        map
    }

    /// Get the ASCII replacement for a functional emoji, if available
    pub fn get_replacement(&self, emoji: &str) -> Option<&str> {
        self.mapping.get(emoji).map(String::as_str)
    }
}

impl EmojiReplacer for SmartReplacer {
    fn replace(&self, emoji: &str) -> Option<String> {
        // If it's a functional emoji, replace it; otherwise remove it (return None)
        self.get_replacement(emoji).map(ToString::to_string)
    }
}

/// Creates an emoji replacer based on the replacement mode
pub fn create_replacer(mode: ReplacementMode, placeholder: Option<&str>) -> Box<dyn EmojiReplacer> {
    match mode {
        ReplacementMode::Smart => Box::new(SmartReplacer::new()),
        ReplacementMode::Remove => Box::new(RemoveReplacer),
        ReplacementMode::Replace => Box::new(AsciiReplacer::new()),
        ReplacementMode::Placeholder => {
            let replacer: Box<dyn EmojiReplacer> = placeholder.map_or_else(
                || Box::new(PlaceholderReplacer::default()),
                |p| Box::new(PlaceholderReplacer::new(p)),
            );
            replacer
        }
    }
}

#[cfg(test)]
#[allow(
    let_underscore_drop,
    clippy::manual_string_new,
    clippy::uninlined_format_args
)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_replacer() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("❤️"), None);
        assert_eq!(replacer.replace("🔥"), None);
    }

    #[test]
    fn test_ascii_replacer_smileys() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("😀"), Some(":D".to_string()));
        assert_eq!(replacer.replace("😊"), Some(":)".to_string()));
        assert_eq!(replacer.replace("😂"), Some(":'D".to_string()));
        assert_eq!(replacer.replace("😢"), Some(":'(".to_string()));
        assert_eq!(replacer.replace("😭"), Some("T_T".to_string()));
        assert_eq!(replacer.replace("😡"), Some(">:(".to_string()));
        assert_eq!(replacer.replace("😎"), Some("8)".to_string()));
        assert_eq!(replacer.replace("🤔"), Some(":-?".to_string()));
    }

    #[test]
    fn test_ascii_replacer_symbols() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("✓"), Some("[v]".to_string()));
        assert_eq!(replacer.replace("✅"), Some("[✓]".to_string()));
        assert_eq!(replacer.replace("❌"), Some("[X]".to_string()));
        assert_eq!(replacer.replace("⭐"), Some("[*]".to_string()));
        assert_eq!(replacer.replace("🔥"), Some("[fire]".to_string()));
        assert_eq!(replacer.replace("💯"), Some("[100]".to_string()));
        assert_eq!(replacer.replace("⚠️"), Some("[!]".to_string()));
    }

    #[test]
    fn test_ascii_replacer_hands() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
        assert_eq!(replacer.replace("👎"), Some("[-1]".to_string()));
        assert_eq!(replacer.replace("👋"), Some("[wave]".to_string()));
        assert_eq!(replacer.replace("👏"), Some("[clap]".to_string()));
        assert_eq!(replacer.replace("🙏"), Some("[pray]".to_string()));
        assert_eq!(replacer.replace("💪"), Some("[strong]".to_string()));
    }

    #[test]
    fn test_ascii_replacer_hearts() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("❤️"), Some("<3".to_string()));
        assert_eq!(replacer.replace("❤"), Some("<3".to_string()));
        assert_eq!(replacer.replace("💔"), Some("</3".to_string()));
        assert_eq!(replacer.replace("💙"), Some("<3".to_string()));
        assert_eq!(replacer.replace("💚"), Some("<3".to_string()));
    }

    #[test]
    fn test_ascii_replacer_arrows() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("➡️"), Some("->".to_string()));
        assert_eq!(replacer.replace("⬅️"), Some("<-".to_string()));
        assert_eq!(replacer.replace("⬆️"), Some("^".to_string()));
        assert_eq!(replacer.replace("⬇️"), Some("v".to_string()));
        assert_eq!(replacer.replace("↔️"), Some("<->".to_string()));
    }

    #[test]
    fn test_ascii_replacer_dev_emojis() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("🐛"), Some("[bug]".to_string()));
        assert_eq!(replacer.replace("🚀"), Some("[rocket]".to_string()));
        assert_eq!(replacer.replace("📦"), Some("[package]".to_string()));
        assert_eq!(replacer.replace("💡"), Some("[bulb]".to_string()));
        assert_eq!(replacer.replace("🔗"), Some("[link]".to_string()));
    }

    #[test]
    fn test_ascii_replacer_tech() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("💻"), Some("[computer]".to_string()));
        assert_eq!(replacer.replace("🔧"), Some("[wrench]".to_string()));
        assert_eq!(replacer.replace("🔨"), Some("[hammer]".to_string()));
        assert_eq!(replacer.replace("⚙️"), Some("[gear]".to_string()));
        assert_eq!(replacer.replace("🔍"), Some("[search]".to_string()));
        assert_eq!(replacer.replace("🔒"), Some("[lock]".to_string()));
    }

    #[test]
    fn test_ascii_replacer_unknown_emoji() {
        let replacer = AsciiReplacer::new();
        // Unknown emoji should return None
        assert_eq!(replacer.replace("🦸"), None);
        assert_eq!(replacer.replace("🧙"), None);
    }

    #[test]
    fn test_ascii_replacer_with_variation_selector() {
        let replacer = AsciiReplacer::new();
        // Test both with and without variation selector (U+FE0F)
        assert_eq!(replacer.replace("❤️"), Some("<3".to_string()));
        assert_eq!(replacer.replace("❤"), Some("<3".to_string()));
        assert_eq!(replacer.replace("⚠️"), Some("[!]".to_string()));
        assert_eq!(replacer.replace("⚠"), Some("[!]".to_string()));
    }

    #[test]
    fn test_placeholder_replacer_default() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("❤️"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("🔥"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("unknown"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_placeholder_replacer_custom() {
        let replacer = PlaceholderReplacer::new("***");
        assert_eq!(replacer.replace("😀"), Some("***".to_string()));
        assert_eq!(replacer.replace("❤️"), Some("***".to_string()));
        assert_eq!(replacer.replace("🔥"), Some("***".to_string()));
    }

    #[test]
    fn test_placeholder_replacer_empty() {
        let replacer = PlaceholderReplacer::new("");
        assert_eq!(replacer.replace("😀"), Some("".to_string()));
    }

    #[test]
    fn test_create_replacer_remove() {
        let replacer = create_replacer(ReplacementMode::Remove, None);
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("❤️"), None);
    }

    #[test]
    fn test_create_replacer_replace() {
        let replacer = create_replacer(ReplacementMode::Replace, None);
        assert_eq!(replacer.replace("😀"), Some(":D".to_string()));
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
    }

    #[test]
    fn test_create_replacer_placeholder_default() {
        let replacer = create_replacer(ReplacementMode::Placeholder, None);
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("❤️"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_create_replacer_placeholder_custom() {
        let replacer = create_replacer(ReplacementMode::Placeholder, Some("XXX"));
        assert_eq!(replacer.replace("😀"), Some("XXX".to_string()));
        assert_eq!(replacer.replace("❤️"), Some("XXX".to_string()));
    }

    #[test]
    fn test_replacement_mode_default() {
        assert_eq!(ReplacementMode::default(), ReplacementMode::Smart);
    }

    #[test]
    fn test_custom_mapping() {
        let mut custom_map = HashMap::new();
        custom_map.insert("😀".to_string(), "SMILE".to_string());
        custom_map.insert("❤️".to_string(), "HEART".to_string());

        let replacer = AsciiReplacer::with_mapping(custom_map);
        assert_eq!(replacer.replace("😀"), Some("SMILE".to_string()));
        assert_eq!(replacer.replace("❤️"), Some("HEART".to_string()));
        assert_eq!(replacer.replace("🔥"), None); // Not in custom mapping
    }
    // ===== COMPREHENSIVE REPLACEMENT MODE TESTS =====

    // ===== Remove Mode with Various Inputs =====

    #[test]
    fn test_remove_mode_single_emoji() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("😀"), None);
    }

    #[test]
    fn test_remove_mode_multiple_emojis() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("🎉"), None);
        assert_eq!(replacer.replace("🚀"), None);
    }

    #[test]
    fn test_remove_mode_emoji_with_skin_tone() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("👍🏽"), None);
    }

    #[test]
    fn test_remove_mode_zwj_sequence() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("👨‍👩‍👧"), None);
    }

    #[test]
    fn test_remove_mode_flag() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("🇺🇸"), None);
    }

    // ===== Replace Mode with Various Inputs =====

    #[test]
    fn test_replace_mode_face_emojis() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("😀"), Some(":D".to_string()));
        assert_eq!(replacer.replace("😊"), Some(":)".to_string()));
        assert_eq!(replacer.replace("😢"), Some(":'(".to_string()));
    }

    #[test]
    fn test_replace_mode_hand_gestures() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
        assert_eq!(replacer.replace("👎"), Some("[-1]".to_string()));
        assert_eq!(replacer.replace("👋"), Some("[wave]".to_string()));
    }

    #[test]
    fn test_replace_mode_symbols() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("✅"), Some("[✓]".to_string()));
        assert_eq!(replacer.replace("❌"), Some("[X]".to_string()));
        assert_eq!(replacer.replace("⭐"), Some("[*]".to_string()));
    }

    #[test]
    fn test_replace_mode_hearts() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("❤️"), Some("<3".to_string()));
        assert_eq!(replacer.replace("💔"), Some("</3".to_string()));
        assert_eq!(replacer.replace("💕"), Some("<3<3".to_string()));
    }

    #[test]
    fn test_replace_mode_dev_emojis() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("🐛"), Some("[bug]".to_string()));
        assert_eq!(replacer.replace("🚀"), Some("[rocket]".to_string()));
        assert_eq!(replacer.replace("📦"), Some("[package]".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("[party]".to_string()));
    }

    #[test]
    fn test_replace_mode_arrows() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.replace("➡️"), Some("->".to_string()));
        assert_eq!(replacer.replace("⬅️"), Some("<-".to_string()));
        assert_eq!(replacer.replace("⬆️"), Some("^".to_string()));
        assert_eq!(replacer.replace("⬇️"), Some("v".to_string()));
    }

    #[test]
    fn test_replace_mode_unmapped_emoji() {
        let replacer = AsciiReplacer::new();
        // Emojis not in the mapping should return None
        assert_eq!(replacer.replace("🦸"), None);
        assert_eq!(replacer.replace("🧙"), None);
    }

    #[test]
    fn test_replace_mode_with_variation_selector() {
        let replacer = AsciiReplacer::new();
        // Test emojis with and without variation selector
        assert_eq!(replacer.replace("❤️"), Some("<3".to_string()));
        assert_eq!(replacer.replace("❤"), Some("<3".to_string()));
    }

    #[test]
    fn test_replace_mode_emoji_with_skin_tone() {
        let replacer = AsciiReplacer::new();
        // Skin tone modifiers are part of the emoji string
        // The mapping should handle the base emoji
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
    }

    // ===== Placeholder Mode with Various Inputs =====

    #[test]
    fn test_placeholder_mode_default_single() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_placeholder_mode_default_multiple() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("🚀"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_placeholder_mode_custom_single_char() {
        let replacer = PlaceholderReplacer::new("*");
        assert_eq!(replacer.replace("😀"), Some("*".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("*".to_string()));
    }

    #[test]
    fn test_placeholder_mode_custom_word() {
        let replacer = PlaceholderReplacer::new("EMOJI");
        assert_eq!(replacer.replace("😀"), Some("EMOJI".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("EMOJI".to_string()));
    }

    #[test]
    fn test_placeholder_mode_custom_brackets() {
        let replacer = PlaceholderReplacer::new("<emoji>");
        assert_eq!(replacer.replace("😀"), Some("<emoji>".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("<emoji>".to_string()));
    }

    #[test]
    fn test_placeholder_mode_empty_string() {
        let replacer = PlaceholderReplacer::new("");
        assert_eq!(replacer.replace("😀"), Some("".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("".to_string()));
    }

    #[test]
    fn test_placeholder_mode_long_placeholder() {
        let replacer = PlaceholderReplacer::new("[REPLACED_EMOJI_HERE]");
        assert_eq!(
            replacer.replace("😀"),
            Some("[REPLACED_EMOJI_HERE]".to_string())
        );
    }

    #[test]
    fn test_placeholder_mode_special_chars() {
        let replacer = PlaceholderReplacer::new("@#$%");
        assert_eq!(replacer.replace("😀"), Some("@#$%".to_string()));
    }

    #[test]
    fn test_placeholder_mode_with_skin_tone() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("👍🏽"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_placeholder_mode_with_zwj() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("👨‍👩‍👧"), Some("[EMOJI]".to_string()));
    }

    // ===== Create Replacer Factory Tests =====

    #[test]
    fn test_create_replacer_remove_mode() {
        let replacer = create_replacer(ReplacementMode::Remove, None);
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("🎉"), None);
    }

    #[test]
    fn test_create_replacer_replace_mode() {
        let replacer = create_replacer(ReplacementMode::Replace, None);
        assert_eq!(replacer.replace("😀"), Some(":D".to_string()));
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
    }

    #[test]
    fn test_create_replacer_placeholder_mode_default() {
        let replacer = create_replacer(ReplacementMode::Placeholder, None);
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_create_replacer_placeholder_mode_custom() {
        let replacer = create_replacer(ReplacementMode::Placeholder, Some("***"));
        assert_eq!(replacer.replace("😀"), Some("***".to_string()));
        assert_eq!(replacer.replace("🎉"), Some("***".to_string()));
    }

    #[test]
    fn test_create_replacer_placeholder_mode_empty() {
        let replacer = create_replacer(ReplacementMode::Placeholder, Some(""));
        assert_eq!(replacer.replace("😀"), Some("".to_string()));
    }

    // ===== Edge Cases and Special Scenarios =====

    #[test]
    fn test_ascii_replacer_consistency() {
        let replacer1 = AsciiReplacer::new();
        let replacer2 = AsciiReplacer::new();

        // Same emoji should produce same replacement
        assert_eq!(replacer1.replace("😀"), replacer2.replace("😀"));
        assert_eq!(replacer1.replace("🎉"), replacer2.replace("🎉"));
    }

    #[test]
    fn test_placeholder_replacer_consistency() {
        let replacer1 = PlaceholderReplacer::new("TEST");
        let replacer2 = PlaceholderReplacer::new("TEST");

        // Same emoji should produce same replacement
        assert_eq!(replacer1.replace("😀"), replacer2.replace("😀"));
        assert_eq!(replacer1.replace("🎉"), replacer2.replace("🎉"));
    }

    #[test]
    fn test_ascii_replacer_get_replacement() {
        let replacer = AsciiReplacer::new();
        assert_eq!(replacer.get_replacement("😀"), Some(":D"));
        assert_eq!(replacer.get_replacement("🎉"), Some("[party]"));
        assert_eq!(replacer.get_replacement("🦸"), None);
    }

    #[test]
    fn test_custom_mapping_empty() {
        let custom_map = HashMap::new();
        let replacer = AsciiReplacer::with_mapping(custom_map);

        // All emojis should return None with empty mapping
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("🎉"), None);
    }

    #[test]
    fn test_custom_mapping_partial() {
        let mut custom_map = HashMap::new();
        custom_map.insert("😀".to_string(), "HAPPY".to_string());
        custom_map.insert("😢".to_string(), "SAD".to_string());

        let replacer = AsciiReplacer::with_mapping(custom_map);
        assert_eq!(replacer.replace("😀"), Some("HAPPY".to_string()));
        assert_eq!(replacer.replace("😢"), Some("SAD".to_string()));
        assert_eq!(replacer.replace("🎉"), None); // Not in custom mapping
    }

    #[test]
    fn test_custom_mapping_override() {
        let mut custom_map = HashMap::new();
        custom_map.insert("😀".to_string(), "CUSTOM_SMILE".to_string());

        let replacer = AsciiReplacer::with_mapping(custom_map);
        // Custom mapping should override default
        assert_eq!(replacer.replace("😀"), Some("CUSTOM_SMILE".to_string()));
    }

    #[test]
    fn test_replacement_mode_serialization() {
        // Test that ReplacementMode can be used with serde
        assert_eq!(ReplacementMode::Remove, ReplacementMode::Remove);
        assert_eq!(ReplacementMode::Replace, ReplacementMode::Replace);
        assert_eq!(ReplacementMode::Placeholder, ReplacementMode::Placeholder);
    }

    #[test]
    fn test_placeholder_replacer_clone() {
        let replacer1 = PlaceholderReplacer::new("TEST");
        let replacer2 = replacer1.clone();

        assert_eq!(replacer1.replace("😀"), replacer2.replace("😀"));
    }

    #[test]
    fn test_remove_replacer_always_none() {
        let replacer = RemoveReplacer;

        // RemoveReplacer should always return None
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("🎉"), None);
        assert_eq!(replacer.replace("🚀"), None);
        assert_eq!(replacer.replace("❤️"), None);
        assert_eq!(replacer.replace("👍🏽"), None);
        assert_eq!(replacer.replace("👨‍👩‍👧"), None);
        assert_eq!(replacer.replace("🇺🇸"), None);
    }

    #[test]
    fn test_placeholder_replacer_always_some() {
        let replacer = PlaceholderReplacer::default();

        // PlaceholderReplacer should always return Some
        assert!(replacer.replace("😀").is_some());
        assert!(replacer.replace("🎉").is_some());
        assert!(replacer.replace("🚀").is_some());
        assert!(replacer.replace("unknown").is_some());
    }

    #[test]
    fn test_ascii_replacer_comprehensive_coverage() {
        let replacer = AsciiReplacer::new();

        // Test a variety of emoji categories
        // Faces
        assert!(replacer.replace("😀").is_some());
        assert!(replacer.replace("😢").is_some());

        // Hands
        assert!(replacer.replace("👍").is_some());
        assert!(replacer.replace("👋").is_some());

        // Symbols
        assert!(replacer.replace("✅").is_some());
        assert!(replacer.replace("❌").is_some());

        // Hearts
        assert!(replacer.replace("❤️").is_some());

        // Dev-related
        assert!(replacer.replace("🐛").is_some());
        assert!(replacer.replace("🚀").is_some());
    }

    #[test]
    fn test_replacement_mode_default_is_smart() {
        let default_mode = ReplacementMode::default();
        assert_eq!(default_mode, ReplacementMode::Smart);
    }

    // ===== Smart Mode Tests =====

    #[test]
    fn test_smart_replacer_functional_checkmarks() {
        let replacer = SmartReplacer::new();
        assert_eq!(replacer.replace("✅"), Some("[OK]".to_string()));
        assert_eq!(replacer.replace("✓"), Some("[v]".to_string()));
        assert_eq!(replacer.replace("✔"), Some("[v]".to_string()));
        assert_eq!(replacer.replace("👍"), Some("[+1]".to_string()));
    }

    #[test]
    fn test_smart_replacer_functional_x_marks() {
        let replacer = SmartReplacer::new();
        assert_eq!(replacer.replace("❌"), Some("[X]".to_string()));
        assert_eq!(replacer.replace("✗"), Some("[X]".to_string()));
        assert_eq!(replacer.replace("👎"), Some("[-1]".to_string()));
    }

    #[test]
    fn test_smart_replacer_functional_warnings() {
        let replacer = SmartReplacer::new();
        assert_eq!(replacer.replace("⚠️"), Some("[!]".to_string()));
        assert_eq!(replacer.replace("❗"), Some("[!]".to_string()));
        assert_eq!(replacer.replace("❓"), Some("[?]".to_string()));
        assert_eq!(replacer.replace("🚫"), Some("[no]".to_string()));
    }

    #[test]
    fn test_smart_replacer_functional_arrows() {
        let replacer = SmartReplacer::new();
        assert_eq!(replacer.replace("➡️"), Some("->".to_string()));
        assert_eq!(replacer.replace("⬅️"), Some("<-".to_string()));
        assert_eq!(replacer.replace("⬆️"), Some("^".to_string()));
        assert_eq!(replacer.replace("⬇️"), Some("v".to_string()));
    }

    #[test]
    fn test_smart_replacer_functional_dev_symbols() {
        let replacer = SmartReplacer::new();
        assert_eq!(replacer.replace("🐛"), Some("[bug]".to_string()));
        assert_eq!(replacer.replace("🚀"), Some("[rocket]".to_string()));
        assert_eq!(replacer.replace("💡"), Some("[idea]".to_string()));
        assert_eq!(replacer.replace("🔥"), Some("[fire]".to_string()));
        assert_eq!(replacer.replace("📦"), Some("[package]".to_string()));
    }

    #[test]
    fn test_smart_replacer_removes_decorative_emojis() {
        let replacer = SmartReplacer::new();
        // Faces should be removed (return None)
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("😊"), None);
        assert_eq!(replacer.replace("🤣"), None);
        // Animals should be removed
        assert_eq!(replacer.replace("🐱"), None);
        assert_eq!(replacer.replace("🐶"), None);
        // Food should be removed
        assert_eq!(replacer.replace("🍕"), None);
        assert_eq!(replacer.replace("🎉"), None);
    }

    #[test]
    fn test_create_replacer_smart_mode() {
        let replacer = create_replacer(ReplacementMode::Smart, None);
        // Functional emojis are replaced
        assert_eq!(replacer.replace("✅"), Some("[OK]".to_string()));
        assert_eq!(replacer.replace("🐛"), Some("[bug]".to_string()));
        // Decorative emojis are removed
        assert_eq!(replacer.replace("😀"), None);
        assert_eq!(replacer.replace("🎉"), None);
    }

    #[test]
    fn test_placeholder_replacer_default_is_emoji() {
        let replacer = PlaceholderReplacer::default();
        assert_eq!(replacer.replace("😀"), Some("[EMOJI]".to_string()));
    }

    #[test]
    fn test_ascii_replacer_default() {
        let replacer = AsciiReplacer::default();
        assert_eq!(replacer.replace("😀"), Some(":D".to_string()));
    }

    #[test]
    fn test_remove_replacer_default() {
        let replacer = RemoveReplacer;
        assert_eq!(replacer.replace("😀"), None);
    }

    #[test]
    fn test_emoji_replacer_trait_object() {
        let replacers: Vec<Box<dyn EmojiReplacer>> = vec![
            Box::new(RemoveReplacer),
            Box::new(AsciiReplacer::new()),
            Box::new(PlaceholderReplacer::default()),
        ];

        // All should handle the same emoji
        for replacer in replacers {
            let _ = replacer.replace("😀");
        }
    }

    #[test]
    fn test_ascii_replacer_all_mapped_emojis() {
        let replacer = AsciiReplacer::new();

        // Test a sample of all mapped emojis
        let test_emojis = vec![
            "😀", "😊", "😂", "😢", "😡", "❤️", "👍", "👎", "👋", "✅", "❌", "⭐", "🔥", "💯",
            "🐛", "🚀", "📦", "🎉",
        ];

        for emoji in test_emojis {
            assert!(
                replacer.replace(emoji).is_some(),
                "Emoji {} should be mapped",
                emoji
            );
        }
    }
}
