//! Replacement strategies for emoji characters
//!
//! Defines different modes for handling detected emojis and provides
//! comprehensive ASCII mappings for common emoji characters.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn with_mapping(mapping: HashMap<String, String>) -> Self {
        Self { mapping }
    }

    /// Builds the default emoji-to-ASCII mapping
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
        self.mapping.get(emoji).map(|s| s.as_str())
    }
}

impl EmojiReplacer for AsciiReplacer {
    fn replace(&self, emoji: &str) -> Option<String> {
        self.get_replacement(emoji).map(|s| s.to_string())
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

/// Creates an emoji replacer based on the replacement mode
pub fn create_replacer(mode: ReplacementMode, placeholder: Option<&str>) -> Box<dyn EmojiReplacer> {
    match mode {
        ReplacementMode::Remove => Box::new(RemoveReplacer),
        ReplacementMode::Replace => Box::new(AsciiReplacer::new()),
        ReplacementMode::Placeholder => {
            if let Some(p) = placeholder {
                Box::new(PlaceholderReplacer::new(p))
            } else {
                Box::new(PlaceholderReplacer::default())
            }
        }
    }
}


#[cfg(test)]
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
        assert_eq!(ReplacementMode::default(), ReplacementMode::Remove);
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
}


