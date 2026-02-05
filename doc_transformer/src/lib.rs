// Library exports for doc_transformer
//
// This module re-exports all public modules for use in integration tests and as a library.

pub mod analyze;
pub mod assign;
pub mod chunk;
pub mod chunking_adapter;
pub mod config;
pub mod discover;
pub mod embeddings;
pub mod errors;
#[cfg(feature = "enhanced")]
pub mod features;
pub mod filter;
pub mod graph;
pub mod highlight;
pub mod index;
pub mod llms;

#[cfg(test)]
mod llms_test_multibyte {
    use super::llms::truncate_summary;

    #[test]
    fn test_truncate_multibyte_emoji() {
        let emoji_content = "Hello 🦀 Rust is awesome! 🎉";
        let result = truncate_summary(emoji_content, 20);
        assert_eq!(result.chars().count(), 20);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_truncate_multibyte_cjk() {
        let cjk_content = "日本語のドキュメントをテストします。";
        let result = truncate_summary(cjk_content, 10);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 10);
    }

    #[test]
    fn test_truncate_multibyte_accented() {
        let accented_content = "Café naïve résumé";
        let result = truncate_summary(accented_content, 5);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 5);
    }

    #[test]
    fn test_truncate_exact_boundary() {
        let text = "Hello 🦀 World";
        let result = truncate_summary(text, 6);
        assert_eq!(result.chars().count(), 6);
    }

    #[test]
    fn test_truncate_empty_string() {
        let result = truncate_summary("", 10);
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_short_string() {
        let text = "Hi";
        let result = truncate_summary(text, 10);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_truncate_no_truncation_needed() {
        let text = "This is a short text";
        let result = truncate_summary(text, 100);
        assert_eq!(result, "This is a short text");
    }

    #[test]
    fn test_truncate_preserves_newlines() {
        let text = "Line 1\nLine 2\nLine 3";
        let result = truncate_summary(text, 10);
        assert_eq!(result.chars().count(), 10);
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_truncate_preserves_whitespace() {
        let text = "   Hello World   ";
        let result = truncate_summary(text, 10);
        assert_eq!(result.chars().count(), 10);
        assert!(!result.starts_with(' '));
    }

    #[test]
    fn test_truncate_mixed_language() {
        let mixed = "Hello 世界 🌍 How are you?";
        let result = truncate_summary(mixed, 15);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 15);
    }

    #[test]
    fn test_truncate_very_long_multibyte() {
        let long_text = "日本語テスト".repeat(100);
        let result = truncate_summary(&long_text, 50);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 50);
    }

    #[test]
    fn test_truncate_very_long_ascii() {
        let long_text = "a".repeat(1000);
        let result = truncate_summary(&long_text, 50);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 50);
    }

    #[test]
    fn test_truncate_preserves_ellipsis() {
        let text = "This is a very long text that needs truncation";
        let result = truncate_summary(text, 20);
        assert_eq!(result.chars().count(), 20);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_preserves_content_before_ellipsis() {
        let text = "Hello 🦀 Rust is awesome";
        let result = truncate_summary(text, 15);
        assert!(!result.ends_with('🦀'));
        assert_eq!(result.chars().count(), 15);
    }

    #[test]
    fn test_truncate_zero_max_len() {
        let text = "Hello World";
        let result = truncate_summary(text, 0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_minimal_max_len() {
        let text = "Hello World";
        let result = truncate_summary(text, 1);
        assert_eq!(result.chars().count(), 1);
    }

    #[test]
    fn test_truncate_max_len_3() {
        let text = "Hello World";
        let result = truncate_summary(text, 3);
        // When max_len <= 3, we can't fit "..." so we just truncate
        assert_eq!(result.chars().count(), 3);
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_truncate_non_breaking_space() {
        let text = "Hello\u{A0}World";
        let result = truncate_summary(text, 5);
        assert!(!result.is_empty());
        assert_eq!(result.chars().count(), 5);
    }

    #[test]
    fn test_truncate_combining_chars() {
        let text = "cafe\u{0301}";
        let result = truncate_summary(text, 1);
        assert_eq!(result.chars().count(), 1);
    }
}

pub mod scrape;
pub mod search;
pub mod similarity;
pub mod transform;
pub mod types;
pub mod validate;
