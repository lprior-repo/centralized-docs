// Test file for BEAD-001: String Slicing on Multi-byte Characters Causes Panic
// This file contains tests to verify that truncate_summary() handles multi-byte
// UTF-8 characters (emojis, CJK characters, accented characters) correctly.

#[cfg(test)]
mod tests {
    use doc_transformer::llms;

    #[test]
    fn test_truncate_summary_emoji_at_boundary() {
        // Test emoji that starts at byte boundary
        let summary = "Hello 🌍 world! This is a test.";
        let truncated = llms::truncate_summary(summary, 5);
        assert_eq!(truncated, "Hello 🌍");
    }

    #[test]
    fn test_truncate_summary_emoji_in_middle() {
        // Test emoji that falls in middle of slice
        let summary = "Hello 🌍 world! This is a test.";
        let truncated = llms::truncate_summary(summary, 15);
        // "Hello 🌍" is 12 bytes (including emoji), so next char is ' '
        assert_eq!(truncated, "Hello 🌍");
    }

    #[test]
    fn test_truncate_summary_cjk_character_boundary() {
        // Test CJK character (2 bytes in UTF-8)
        let summary = "你好世界，这是测试。";
        let truncated = llms::truncate_summary(summary, 4);
        // "你好" is 4 bytes, so truncate at 4 bytes -> "你好"
        assert_eq!(truncated, "你好");
    }

    #[test]
    fn test_truncate_summary_cjk_character_in_middle() {
        // Test CJK character that falls in middle of slice
        let summary = "你好世界，这是测试。";
        let truncated = llms::truncate_summary(summary, 6);
        // "你好世" is 6 bytes, but "世" (世界) is 2 bytes, so we can't cut in middle
        // Should either keep full "世" or cut before it
        // Using chars().take() should handle this correctly
        assert_eq!(truncated.len(), 4); // "你好" (4 bytes)
    }

    #[test]
    fn test_truncate_summary_accented_character_boundary() {
        // Test accented character (multiple bytes in UTF-8)
        let summary = "Café au lait";
        let truncated = llms::truncate_summary(summary, 4);
        // "Café" is 4 bytes (é is 2 bytes in UTF-8)
        assert_eq!(truncated, "Café");
    }

    #[test]
    fn test_truncate_summary_accented_character_in_middle() {
        // Test accented character that falls in middle of slice
        let summary = "Café au lait";
        let truncated = llms::truncate_summary(summary, 6);
        // "Café " is 6 bytes, but "é" is 2 bytes, so we can't cut in middle
        assert_eq!(truncated.len(), 4); // "Café" (4 bytes)
    }

    #[test]
    fn test_truncate_summary_mixed_language() {
        // Test mixed languages: English + emoji + CJK
        let summary = "Hello 🌍 你好世界";
        let truncated = llms::truncate_summary(summary, 10);
        // "Hello 🌍" is 12 bytes, but we want 10 bytes
        // This should safely truncate without panic
        assert_eq!(truncated.len(), 10);
        assert!(truncated.chars().count() <= 10);
    }

    #[test]
    fn test_truncate_summary_very_long_string() {
        // Test very long string with emoji
        let summary = "A".repeat(1000) + " 🌍 " + &"B".repeat(1000);
        let truncated = llms::truncate_summary(&summary, 50);
        assert_eq!(truncated.len(), 50);
    }

    #[test]
    fn test_truncate_summary_empty_string() {
        // Test empty string
        let summary = "";
        let truncated = llms::truncate_summary(summary, 100);
        assert_eq!(truncated, "");
    }

    #[test]
    fn test_truncate_summary_zero_max_len() {
        // Test zero max_len
        let summary = "Hello world";
        let truncated = llms::truncate_summary(summary, 0);
        assert_eq!(truncated, "");
    }

    #[test]
    fn test_truncate_summary_shorter_than_max_len() {
        // Test string shorter than max_len
        let summary = "Hello";
        let truncated = llms::truncate_summary(summary, 100);
        assert_eq!(truncated, "Hello");
    }

    #[test]
    fn test_truncate_summary_exact_boundary() {
        // Test exact boundary at byte position
        let summary = "Hello 🌍";
        let truncated = llms::truncate_summary(summary, 12); // "Hello 🌍" is 12 bytes
        assert_eq!(truncated, "Hello 🌍");
    }

    #[test]
    fn test_truncate_summary_multiple_emojis() {
        // Test multiple emojis
        let summary = "Hello 🌍🌎🌍 world";
        let truncated = llms::truncate_summary(summary, 15);
        // "Hello 🌍🌎" is 15 bytes
        assert_eq!(truncated.len(), 15);
    }

    #[test]
    fn test_truncate_summary_nbsp() {
        // Test non-breaking space (1 byte)
        let summary = "Hello&nbsp;world";
        let truncated = llms::truncate_summary(summary, 6);
        assert_eq!(truncated.len(), 6);
    }

    #[test]
    fn test_truncate_summary_combining_characters() {
        // Test combining characters
        let summary = "e\u{0301}"; // e + combining acute accent
        let truncated = llms::truncate_summary(summary, 1);
        // Should handle combining characters correctly
        assert_eq!(truncated.len(), 1);
    }

    #[test]
    fn test_truncate_summary_very_short_string() {
        // Test string that's just one character
        let summary = "A";
        let truncated = llms::truncate_summary(summary, 100);
        assert_eq!(truncated, "A");
    }

    #[test]
    fn test_truncate_summary_only_emoji() {
        // Test string that's only an emoji
        let summary = "🌍";
        let truncated = llms::truncate_summary(summary, 100);
        assert_eq!(truncated, "🌍");
    }

    #[test]
    fn test_truncate_summary_only_multibyte_chars() {
        // Test string with only multibyte characters
        let summary = "你好世界";
        let truncated = llms::truncate_summary(summary, 4);
        // "你好" is 4 bytes
        assert_eq!(truncated.len(), 4);
    }
}
