//! Emoji detection module
//!
//! Provides functionality to identify emoji characters using Unicode ranges.
//! Handles single emojis, emoji sequences (ZWJ families), skin tone modifiers,
//! flag sequences, and keycap sequences.

/// Detects emoji characters in text
#[derive(Debug, Clone)]
pub struct EmojiDetector {
    // Currently stateless, but could be extended with configuration
}

impl EmojiDetector {
    /// Creates a new emoji detector
    pub fn new() -> Self {
        Self {}
    }

    /// Finds all emoji matches in the given text
    ///
    /// Returns a vector of `EmojiMatch` instances, each representing an emoji
    /// or emoji sequence found in the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use demoji::core::emoji::EmojiDetector;
    ///
    /// let detector = EmojiDetector::new();
    /// let matches = detector.find_all("Hello 👋 World 🌍");
    /// assert_eq!(matches.len(), 2);
    /// ```
    pub fn find_all(&self, text: &str) -> Vec<EmojiMatch> {
        let mut matches = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((start_idx, ch)) = chars.next() {
            if self.is_emoji_start(ch) || ch.is_ascii_digit() {
                // Collect the full emoji sequence
                let mut end_idx = start_idx + ch.len_utf8();
                let mut emoji_chars = vec![ch];
                let is_regional = self.is_regional_indicator(ch);
                let is_digit = ch.is_ascii_digit();

                // Look ahead for modifiers, ZWJ sequences, regional indicators, and keycap sequences
                while let Some(&(next_idx, next_ch)) = chars.peek() {
                    let should_continue = if is_regional && emoji_chars.len() == 1 {
                        // After first regional indicator, expect second one
                        self.is_regional_indicator(next_ch)
                    } else if is_digit && emoji_chars.len() == 1 {
                        // After digit, expect variation selector or keycap
                        self.is_emoji_modifier(next_ch)
                    } else if self.is_zwj(emoji_chars.last().copied().unwrap_or('\0')) {
                        // After ZWJ, the next character is part of the sequence
                        true
                    } else {
                        // Otherwise, check for modifiers and ZWJ
                        self.is_emoji_modifier(next_ch) || self.is_zwj(next_ch)
                    };

                    if should_continue {
                        emoji_chars.push(next_ch);
                        end_idx = next_idx + next_ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Only create a match if we have an actual emoji (not just a digit)
                if !is_digit || emoji_chars.len() > 1 {
                    let emoji_str: String = emoji_chars.iter().collect();
                    matches.push(EmojiMatch {
                        start: start_idx,
                        end: end_idx,
                        emoji: emoji_str,
                        line: text[..start_idx].matches('\n').count() + 1,
                        column: start_idx
                            - text[..start_idx].rfind('\n').map(|i| i + 1).unwrap_or(0)
                            + 1,
                    });
                }
            }
        }

        matches
    }

    /// Checks if the text contains any emojis
    ///
    /// # Examples
    ///
    /// ```
    /// use demoji::core::emoji::EmojiDetector;
    ///
    /// let detector = EmojiDetector::new();
    /// assert!(detector.contains_emoji("Hello 👋"));
    /// assert!(!detector.contains_emoji("Hello World"));
    /// ```
    pub fn contains_emoji(&self, text: &str) -> bool {
        text.chars().any(|ch| self.is_emoji_start(ch))
    }

    /// Checks if a character is the start of an emoji
    fn is_emoji_start(&self, ch: char) -> bool {
        self.is_emoji_presentation(ch) || self.is_regional_indicator(ch)
    }

    /// Checks if a character has emoji presentation
    ///
    /// This includes:
    /// - Emoji presentation characters (U+1F300..U+1F9FF)
    /// - Emoticons (U+1F600..U+1F64F)
    /// - Miscellaneous Symbols and Pictographs (U+1F300..U+1F5FF)
    /// - Transport and Map Symbols (U+1F680..U+1F6FF)
    /// - Supplemental Symbols and Pictographs (U+1F900..U+1F9FF)
    /// - Symbols and Pictographs Extended-A (U+1FA70..U+1FAFF)
    /// - Enclosed Alphanumeric Supplement (U+1F100..U+1F1FF)
    /// - Miscellaneous Symbols (U+2600..U+26FF)
    /// - Dingbats (U+2700..U+27BF)
    /// - Enclosed Alphanumerics (U+2460..U+24FF)
    /// - Geometric Shapes (U+25A0..U+25FF)
    /// - Miscellaneous Technical (U+2300..U+23FF)
    fn is_emoji_presentation(&self, ch: char) -> bool {
        matches!(ch,
            '\u{1F300}'..='\u{1F9FF}' |  // Main emoji blocks
            '\u{1FA00}'..='\u{1FAFF}' |  // Extended emoji
            '\u{2600}'..='\u{26FF}' |    // Miscellaneous Symbols
            '\u{2700}'..='\u{27BF}' |    // Dingbats
            '\u{2300}'..='\u{23FF}' |    // Miscellaneous Technical (includes watch, media controls, keyboard)
            '\u{2460}'..='\u{24FF}' |    // Enclosed Alphanumerics
            '\u{25A0}'..='\u{25FF}' |    // Geometric Shapes
            '\u{2B50}' |                 // Star
            '\u{2B55}' |                 // Heavy Large Circle
            '\u{FE0F}' |                 // Variation Selector-16 (emoji presentation)
            '\u{203C}' |                 // Double exclamation
            '\u{2049}' |                 // Exclamation question
            '\u{20E3}' |                 // Combining Enclosing Keycap
            '\u{2122}' |                 // Trademark
            '\u{2139}' |                 // Information
            '\u{2194}'..='\u{2199}' |    // Arrows
            '\u{21A9}'..='\u{21AA}' |    // More arrows
            '\u{2934}'..='\u{2935}' |    // Curved arrows
            '\u{2B05}'..='\u{2B07}' |    // Directional arrows
            '\u{3030}' |                 // Wavy dash
            '\u{303D}' |                 // Part alternation mark
            '\u{3297}' |                 // Circled ideograph congratulation
            '\u{3299}'                   // Circled ideograph secret
        )
    }

    /// Checks if a character is a regional indicator (for flag emojis)
    ///
    /// Regional indicators are used in pairs to create flag emojis.
    /// For example, 🇺🇸 is U+1F1FA U+1F1F8 (U + S)
    fn is_regional_indicator(&self, ch: char) -> bool {
        matches!(ch, '\u{1F1E6}'..='\u{1F1FF}')
    }

    /// Checks if a character is an emoji modifier (skin tone)
    ///
    /// Skin tone modifiers: U+1F3FB..U+1F3FF
    fn is_emoji_modifier(&self, ch: char) -> bool {
        matches!(ch, '\u{1F3FB}'..='\u{1F3FF}' | '\u{FE0F}' | '\u{20E3}')
    }

    /// Checks if a character is Zero Width Joiner (ZWJ)
    ///
    /// ZWJ is used to combine multiple emojis into a single glyph,
    /// such as family emojis (👨‍👩‍👧) or profession emojis (👨‍⚕️)
    fn is_zwj(&self, ch: char) -> bool {
        ch == '\u{200D}'
    }
}

impl Default for EmojiDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a detected emoji
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiMatch {
    /// Byte offset where the emoji starts in the source text
    pub start: usize,
    /// Byte offset where the emoji ends in the source text (exclusive)
    pub end: usize,
    /// The emoji string (may be multiple characters for sequences)
    pub emoji: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed, byte offset from start of line)
    pub column: usize,
}

impl EmojiMatch {
    /// Returns the length of the emoji in bytes
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if this is an empty match (should not happen in practice)
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns the emoji as a string slice
    pub fn as_str(&self) -> &str {
        &self.emoji
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_emoji() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("Hello 👋 World");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👋");
        assert_eq!(matches[0].start, 6);
    }

    #[test]
    fn test_multiple_emojis() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👋 🌍 🎉");
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].emoji, "👋");
        assert_eq!(matches[1].emoji, "🌍");
        assert_eq!(matches[2].emoji, "🎉");
    }

    #[test]
    fn test_emoji_with_skin_tone() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👋🏽");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👋🏽");
    }

    #[test]
    fn test_zwj_sequence_family() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👨‍👩‍👧");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👨‍👩‍👧");
    }

    #[test]
    fn test_flag_sequence() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🇺🇸");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🇺🇸");
    }

    #[test]
    fn test_keycap_sequence() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("1️⃣");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "1️⃣");
    }

    #[test]
    fn test_no_emoji() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("Hello World");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_contains_emoji() {
        let detector = EmojiDetector::new();
        assert!(detector.contains_emoji("Hello 👋"));
        assert!(!detector.contains_emoji("Hello World"));
    }

    #[test]
    fn test_line_and_column() {
        let detector = EmojiDetector::new();
        let text = "Line 1\nLine 2 👋\nLine 3";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, 2);
        assert_eq!(matches[0].column, 8);
    }

    #[test]
    fn test_emoji_in_code() {
        let detector = EmojiDetector::new();
        let code = r#"
fn main() {
    println!("Hello 👋");
    let x = "🎉"; // celebration
}
"#;
        let matches = detector.find_all(code);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].emoji, "👋");
        assert_eq!(matches[1].emoji, "🎉");
    }

    #[test]
    fn test_various_emoji_types() {
        let detector = EmojiDetector::new();
        let text = "😀 ❤️ ⭐ ✅ ⚠️ 🔥 💯";
        let matches = detector.find_all(text);
        assert!(matches.len() >= 5, "Should detect multiple emoji types");
    }
    // ===== COMPREHENSIVE EMOJI TESTS =====

    // ===== Single Emojis of Various Types =====

    #[test]
    fn test_single_emoji_face() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("😀");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "😀");
    }

    #[test]
    fn test_single_emoji_animal() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🐶");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🐶");
    }

    #[test]
    fn test_single_emoji_object() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🎸");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🎸");
    }

    #[test]
    fn test_single_emoji_symbol() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("⭐");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "⭐");
    }

    #[test]
    fn test_single_emoji_heart() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("❤️");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "❤️");
    }

    #[test]
    fn test_single_emoji_checkmark() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("✅");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "✅");
    }

    // ===== Emoji Sequences (ZWJ Families) =====

    #[test]
    fn test_zwj_sequence_family_four_members() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👨‍👩‍👧‍👦");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👨‍👩‍👧‍👦");
    }

    #[test]
    fn test_zwj_sequence_couple() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👨‍❤️‍👨");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👨‍❤️‍👨");
    }

    #[test]
    fn test_zwj_sequence_profession() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👨‍⚕️");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👨‍⚕️");
    }

    // ===== Flag Sequences =====

    #[test]
    fn test_flag_us() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🇺🇸");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🇺🇸");
    }

    #[test]
    fn test_flag_gb() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🇬🇧");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🇬🇧");
    }

    #[test]
    fn test_flag_japan() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🇯🇵");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🇯🇵");
    }

    #[test]
    fn test_multiple_flags() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("🇺🇸 🇬🇧 🇯🇵");
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].emoji, "🇺🇸");
        assert_eq!(matches[1].emoji, "🇬🇧");
        assert_eq!(matches[2].emoji, "🇯🇵");
    }

    // ===== Skin Tone Modifiers =====

    #[test]
    fn test_thumbs_up_light_skin_tone() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👍🏻");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👍🏻");
    }

    #[test]
    fn test_thumbs_up_medium_skin_tone() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👍🏽");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👍🏽");
    }

    #[test]
    fn test_thumbs_up_dark_skin_tone() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👍🏿");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "👍🏿");
    }

    #[test]
    fn test_wave_with_skin_tones() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("👋🏻 👋🏽 👋🏿");
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].emoji, "👋🏻");
        assert_eq!(matches[1].emoji, "👋🏽");
        assert_eq!(matches[2].emoji, "👋🏿");
    }

    #[test]
    fn test_pointing_finger_with_skin_tone() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("☝🏻");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "☝🏻");
    }

    // ===== Emojis in Real Source Code Contexts =====

    #[test]
    fn test_emoji_in_rust_comment() {
        let detector = EmojiDetector::new();
        let code = "// TODO: Fix this 🐛 bug";
        let matches = detector.find_all(code);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🐛");
    }

    #[test]
    fn test_emoji_in_string_literal() {
        let detector = EmojiDetector::new();
        let code = r#"let msg = "Success! 🎉";"#;
        let matches = detector.find_all(code);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🎉");
    }

    #[test]
    fn test_emoji_in_multiline_comment() {
        let detector = EmojiDetector::new();
        let code = r#"
/* This function is broken 💔
   and needs fixing 🔧
*/"#;
        let matches = detector.find_all(code);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].emoji, "💔");
        assert_eq!(matches[1].emoji, "🔧");
    }

    #[test]
    fn test_emoji_in_python_code() {
        let detector = EmojiDetector::new();
        let code = r#"
def process():
    # This is a rocket 🚀
    print("Status: ✅")
"#;
        let matches = detector.find_all(code);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].emoji, "🚀");
        assert_eq!(matches[1].emoji, "✅");
    }

    #[test]
    fn test_emoji_in_json_string() {
        let detector = EmojiDetector::new();
        let json = r#"{"status": "ready 🚀", "error": "failed ❌"}"#;
        let matches = detector.find_all(json);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].emoji, "🚀");
        assert_eq!(matches[1].emoji, "❌");
    }

    #[test]
    fn test_emoji_in_markdown() {
        let detector = EmojiDetector::new();
        let markdown = r#"
# Project Status 📊

- [x] Feature A ✅
- [ ] Feature B 🚧
- [x] Bug fix 🐛
"#;
        let matches = detector.find_all(markdown);
        assert_eq!(matches.len(), 4);
    }

    // ===== Edge Cases =====

    #[test]
    fn test_empty_string() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_only_whitespace() {
        let detector = EmojiDetector::new();
        let matches = detector.find_all("   \n\t  ");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_mixed_text_and_emojis() {
        let detector = EmojiDetector::new();
        let text = "Start 🎉 middle 🚀 end";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].emoji, "🎉");
        assert_eq!(matches[1].emoji, "🚀");
    }

    #[test]
    fn test_consecutive_emojis() {
        let detector = EmojiDetector::new();
        let text = "🎉🚀💯";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].emoji, "🎉");
        assert_eq!(matches[1].emoji, "🚀");
        assert_eq!(matches[2].emoji, "💯");
    }

    #[test]
    fn test_emoji_at_start() {
        let detector = EmojiDetector::new();
        let text = "🎉 celebration";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].start, 0);
    }

    #[test]
    fn test_emoji_at_end() {
        let detector = EmojiDetector::new();
        let text = "celebration 🎉";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🎉");
    }

    #[test]
    fn test_emoji_only() {
        let detector = EmojiDetector::new();
        let text = "🎉";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].emoji, "🎉");
    }

    #[test]
    fn test_emoji_match_positions() {
        let detector = EmojiDetector::new();
        let text = "Hello 👋 World 🌍";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 2);

        // First emoji at position 6
        assert_eq!(matches[0].start, 6);
        assert_eq!(matches[0].end, 6 + "👋".len());

        // Second emoji after "World "
        assert!(matches[1].start > matches[0].end);
    }

    #[test]
    fn test_emoji_line_tracking_multiple_lines() {
        let detector = EmojiDetector::new();
        let text = "Line 1 🎉\nLine 2 🚀\nLine 3 💯";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].line, 1);
        assert_eq!(matches[1].line, 2);
        assert_eq!(matches[2].line, 3);
    }

    #[test]
    fn test_emoji_column_tracking() {
        let detector = EmojiDetector::new();
        let text = "abc 🎉 def";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].column, 5); // "abc " is 4 chars, emoji at position 5
    }

    #[test]
    fn test_emoji_match_len() {
        let detector = EmojiDetector::new();
        let text = "👋";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].len(), "👋".len());
    }

    #[test]
    fn test_emoji_match_is_empty() {
        let detector = EmojiDetector::new();
        let text = "👋";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert!(!matches[0].is_empty());
    }

    #[test]
    fn test_emoji_match_as_str() {
        let detector = EmojiDetector::new();
        let text = "👋";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].as_str(), "👋");
    }

    #[test]
    fn test_contains_emoji_with_multiple() {
        let detector = EmojiDetector::new();
        assert!(detector.contains_emoji("🎉 🚀 💯"));
    }

    #[test]
    fn test_contains_emoji_false() {
        let detector = EmojiDetector::new();
        assert!(!detector.contains_emoji("no emojis here"));
    }

    #[test]
    fn test_emoji_detector_default() {
        let detector = EmojiDetector::default();
        let matches = detector.find_all("Hello 👋");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_emoji_with_variation_selector() {
        let detector = EmojiDetector::new();
        // Some emojis have variation selectors (U+FE0F)
        let matches = detector.find_all("❤️");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_keycap_sequences() {
        let detector = EmojiDetector::new();
        let text = "1️⃣ 2️⃣ 3️⃣";
        let matches = detector.find_all(text);
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_mixed_emoji_types_in_one_string() {
        let detector = EmojiDetector::new();
        let text = "Face: 😀, Flag: 🇺🇸, Family: 👨‍👩‍👧, Skin tone: 👍🏽";
        let matches = detector.find_all(text);
        assert!(matches.len() >= 4);
    }
}
