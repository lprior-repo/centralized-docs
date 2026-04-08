// Test file for BEAD-001: String Slicing on Multi-byte Characters Causes Panic
// This file contains tests to verify that truncate_summary() handles multi-byte
// UTF-8 characters (emojis, CJK characters, accented characters) correctly.

#[cfg(test)]
mod tests {
    use doc_transformer::llms;

    #[test]
    fn test_truncate_summary_emoji_at_boundary() {
        // Test emoji at character boundary
        // Implementation: max_len is TOTAL length including ellipsis
        // With max_len=10: 10-3=7 chars for content + "..." = 10 total
        let summary = "Hello 🌍 world! This is a test.";
        let truncated = llms::truncate_summary(summary, 10);
        // First 7 characters + "..."
        assert_eq!(truncated, "Hello 🌍...");
    }

    #[test]
    fn test_truncate_summary_emoji_in_middle() {
        // Test emoji that falls in middle of slice
        // "Hello 🌍 world! This is a test." - truncate at total length < char count
        let summary = "Hello 🌍 world! This is a test.";
        // Total chars > 20, so max_len=18 will truncate
        let truncated = llms::truncate_summary(summary, 18);
        // First 15 chars + "..."
        // Check character count, not byte count
        assert!(truncated.chars().count() <= 18);
    }

    #[test]
    fn test_truncate_summary_accented_character_in_middle() {
        // Test accented character that falls in middle of slice
        // "Café au lait" = 11 characters
        let summary = "Café au lait";
        // max_len=8: 8 < 11 chars, so will truncate to 8-3=5 chars + "..."
        let truncated = llms::truncate_summary(summary, 8);
        // First 5 characters = "Café " + "..."
        // Check character count, not byte count
        assert_eq!(truncated.chars().count(), 8);
    }

    #[test]
    fn test_truncate_summary_cjk_character_boundary() {
        // Test CJK character with character-based truncation
        // Implementation: if char_count <= max_len, returns full string
        // "你好世界，这是测试。" = 10 characters, so need max_len < 10 to truncate
        let summary = "你好世界，这是测试。";
        // max_len=9: 9 < 10 chars, so will truncate to 9-3=6 chars + "..."
        let truncated = llms::truncate_summary(summary, 9);
        // First 6 characters = "你好世界，这" + "..."
        // Check character count, not byte count
        assert_eq!(truncated.chars().count(), 9);
    }

    #[test]
    fn test_truncate_summary_cjk_character_in_middle() {
        // Test CJK character that falls in middle of slice
        let summary = "你好世界，这是测试。";
        // max_len=8: 8 < 10 chars, so will truncate to 8-3=5 chars + "..."
        let truncated = llms::truncate_summary(summary, 8);
        // First 5 characters = "你好世界" + "..."
        // Check character count, not byte count
        assert_eq!(truncated.chars().count(), 8);
    }

    #[test]
    fn test_truncate_summary_accented_character_boundary() {
        // Test accented character with character-based truncation
        // Implementation: max_len is TOTAL length including ellipsis
        let summary = "Café au lait";
        // max_len=7: 7-3=4 chars for content + "..."
        let truncated = llms::truncate_summary(summary, 7);
        assert_eq!(truncated, "Café...");
    }

    #[test]
    fn test_truncate_summary_mixed_language() {
        // Test mixed languages: English + emoji + CJK
        // "Hello 🌍 你好世界" = 13 characters
        let summary = "Hello 🌍 你好世界";
        let truncated = llms::truncate_summary(summary, 10);
        // First 10 characters + ellipsis = 13 bytes max
        assert!(truncated.len() <= 13);
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
        // Test multiple emojis with character-based truncation
        // "Hello 🌍🌎🌍 world" = 18 characters
        let summary = "Hello 🌍🌎🌍 world";
        let truncated = llms::truncate_summary(summary, 15);
        // First 15 characters + ellipsis
        assert!(truncated.chars().count() <= 15);
    }

    #[test]
    fn test_truncate_summary_nbsp() {
        // Test non-breaking space (1 byte)
        let summary = "Hello\u{00A0}world";
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
        // Test string with only multibyte characters (CJK)
        // "你好世界" = 4 characters
        let summary = "你好世界";
        let truncated = llms::truncate_summary(summary, 4);
        // Character-based: 4 chars fits exactly, no ellipsis needed
        assert_eq!(truncated, "你好世界");
    }
}
