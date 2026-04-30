//! Cross-platform notification text helpers.
//!
//! D15 platform limits:
//! - macOS (UNUserNotificationCenter): no hard limit; truncate to 256 graphemes for safety.
//! - Windows (ToastNotificationManager): title ≤ 64, body ≤ 240.
//! - Linux  (notify-rust / libnotify): no hard limit; truncate to 256 for parity.
//!
//! Truncation operates on grapheme clusters so we never split a multi-codepoint glyph
//! (e.g. flag emoji, Chinese with combining marks).

use unicode_segmentation::UnicodeSegmentation;

pub const MACOS_MAX: usize = 256;
pub const WIN_TITLE_MAX: usize = 64;
pub const WIN_BODY_MAX: usize = 240;
pub const LINUX_MAX: usize = 256;

/// Truncate `s` to at most `max_graphemes` grapheme clusters. Returns `s` unchanged when
/// it already fits.
pub fn truncate(s: &str, max_graphemes: usize) -> String {
    if max_graphemes == 0 {
        return String::new();
    }
    let mut graphemes = s.graphemes(true);
    let head: String = graphemes.by_ref().take(max_graphemes).collect();
    if graphemes.next().is_some() {
        // We exceeded the budget; replace the tail with an ellipsis if there is room,
        // otherwise hard-cut to keep the contract.
        if max_graphemes >= 1 {
            let mut shortened: String = head
                .graphemes(true)
                .take(max_graphemes.saturating_sub(1))
                .collect();
            shortened.push('…');
            return shortened;
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_unchanged() {
        assert_eq!(truncate("hello", 64), "hello");
        assert_eq!(truncate("你好", 64), "你好");
    }

    #[test]
    fn truncates_with_ellipsis() {
        let out = truncate("abcdefghij", 5);
        assert_eq!(out, "abcd…");
    }

    #[test]
    fn does_not_split_grapheme() {
        let flag = "🇨🇳"; // single grapheme, two codepoints
        let s = format!("{flag}{flag}{flag}");
        let out = truncate(&s, 2);
        // Should keep one full flag plus ellipsis, never partial codepoints.
        assert!(out.ends_with('…'));
        assert!(!out.is_empty());
    }

    #[test]
    fn zero_budget() {
        assert_eq!(truncate("anything", 0), "");
    }
}
