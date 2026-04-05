//! Token estimation, content analysis, and BPE tokenizer caching.
//!
//! All BPE tokenization routes through a single `LazyLock`-cached
//! `Arc<CoreBPE>` — zero redundant initializations.

use crate::chunk::{table_regex, ChunkType};
use std::sync::LazyLock;
use text_splitter::ChunkSizer;

/// Global cached BPE tokenizer — initialized exactly once via `LazyLock`.
///
/// `Arc<CoreBPE>` is cloned cheaply (atomic refcount) and shared across
/// all threads. The `Result` wrapper makes init infallible at the type
/// level — callers use `.as_ref()` without `unwrap` or `panic`.
static CACHED_BPE: LazyLock<std::result::Result<std::sync::Arc<tiktoken_rs::CoreBPE>, String>> =
    LazyLock::new(|| {
        tiktoken_rs::cl100k_base()
            .map(std::sync::Arc::new)
            .map_err(|e| format!("BPE tokenizer init failed: {e}"))
    });

/// Retrieve a reference to the globally cached BPE tokenizer.
///
/// Returns `&'static Arc<CoreBPE>` — cheap to clone, safe to share,
/// zero mutation. Callers match on `Ok`/`Err` — no panics.
pub(crate) fn shared_bpe(
) -> std::result::Result<&'static std::sync::Arc<tiktoken_rs::CoreBPE>, &'static str> {
    CACHED_BPE
        .as_ref()
        .map_err(|_| "BPE tokenizer initialization failed")
}

#[derive(Clone)]
pub(crate) struct FastTokenizer {
    bpe: std::sync::Arc<tiktoken_rs::CoreBPE>,
}

impl FastTokenizer {
    /// Create a FastTokenizer using the globally cached BPE instance.
    ///
    /// Zero-cost after first call — just an `Arc::clone` (atomic refcount).
    pub(crate) fn new() -> std::result::Result<Self, String> {
        shared_bpe()
            .map(std::sync::Arc::clone)
            .map(|bpe| Self { bpe })
            .map_err(|e: &str| e.to_string())
    }
}

impl ChunkSizer for FastTokenizer {
    fn size(&self, text: &str) -> usize {
        // Pathological strings (minified code, base64, adversarial repeated chars)
        // cause regex-based tokenizers to exhibit O(N^2) behavior.
        // Use a fast approximation for extremely low space-density strings.
        let space_count = text.bytes().filter(|&b| b == b' ' || b == b'\n').count();
        if text.len() > 1000 && space_count < text.len() / 100 {
            return (text.len() / 4).max(1);
        }
        self.bpe.encode_with_special_tokens(text).len()
    }
}

/// Estimate token count using the globally cached BPE tokenizer.
///
/// Falls back to character approximation (len/4) for pathological strings
/// or if the global tokenizer failed to initialize.
pub(crate) fn estimate_tokens(text: &str) -> usize {
    // Fast path for adversarial/pathological strings with no whitespace.
    if text.len() > 1000 && !text.contains(|c: char| c.is_whitespace()) {
        return (text.len() / 4).max(1);
    }

    shared_bpe().map_or((text.len() / 4).max(1), |arc| {
        arc.encode_with_special_tokens(text).len()
    })
}

/// Create a summary from chunk content (extractive, first ~200 chars).
///
/// Truncates at the last whitespace boundary to avoid mangling markdown.
pub(crate) fn create_summary(content: &str) -> String {
    let clean = content.trim();
    if clean.is_empty() {
        return String::new();
    }

    let mut chars = clean.chars();
    let truncated: String = chars.by_ref().take(200).collect();

    if chars.next().is_none() {
        return clean.to_string();
    }

    truncated.rfind(char::is_whitespace).map_or_else(
        || format!("{truncated}..."),
        |pos| format!("{}...", &truncated[..pos]),
    )
}

/// Detect chunk content type based on code block count and table patterns.
pub(crate) fn detect_chunk_type(content: &str) -> ChunkType {
    let code_block_count = content.matches("```").count() / 2;
    let has_table =
        content.contains('|') && table_regex().is_ok_and(|regex| regex.is_match(content));

    match (code_block_count > 5, has_table) {
        (true, _) => ChunkType::Code,
        (_, true) => ChunkType::Table,
        _ => ChunkType::Prose,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]

    use super::*;
    use crate::chunk::ChunkType;

    #[test]
    fn test_estimate_tokens() {
        let text = "This is a test";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!((3..=4).contains(&tokens));
    }

    #[test]
    fn test_create_summary_ascii() {
        let content = "This is a test. This is another sentence.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
        assert!(summary.contains("This is a test"));
    }

    #[test]
    fn test_create_summary_unicode_emoji() {
        let content = "This is a test with emoji 🎉 and more content here.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_create_summary_unicode_cjk() {
        let content = "这是一个测试。这是另一个句子。More content after Chinese.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_chunk_type_detection() {
        let code = "```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n\
                     ```
code\n```\n```\ncode\n```\n```\ncode\n```";
        assert_eq!(detect_chunk_type(code), ChunkType::Code);

        let table = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        assert_eq!(detect_chunk_type(table), ChunkType::Table);

        let prose = "This is just regular prose content with no tables or code blocks.";
        assert_eq!(detect_chunk_type(prose), ChunkType::Prose);
    }
}
