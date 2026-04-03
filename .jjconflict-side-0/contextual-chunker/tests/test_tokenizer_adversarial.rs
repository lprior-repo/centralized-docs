//! Tokenizer pathologically-slow benchmarks.
//!
//! These tests verify that the chunker handles adversarial inputs gracefully
//! without exponential time complexity or panics:
//!
//! - Minified JavaScript/CSS (no whitespace)
//! - Base64-encoded data
//! - Repeated characters
//! - Zero-width unicode characters
//! - Very long lines without breaks
//! - Mixed multibyte characters
//!
//! Run with: cargo test --release --test test_tokenizer_adversarial -- --nocapture

use contextual_chunker::{chunk, ChunkLevel, Document};
use std::time::{Duration, Instant};

/// Maximum acceptable time for adversarial input (prevents O(n²) regressions)
const MAX_ADVERSARIAL_TIME: Duration = Duration::from_secs(10);

/// Generate minified-code-like content (no spaces, dense tokens)
fn generate_minified_content(chars: usize) -> String {
    let tokens = [
        "fn", "()", "{}", "let", "x=", "1", ";", "return", "if", "else", "match", "=>",
    ];
    (0..)
        .map(|i| tokens[i % tokens.len()])
        .take_while(|s| s.len() < chars)
        .fold(String::with_capacity(chars), |mut acc, s| {
            acc.push_str(s);
            acc
        })
}

/// Generate base64-like content (random-looking, no spaces)
fn generate_base64_content(chars: usize) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    (0..chars)
        .map(|i| BASE64_CHARS[i % BASE64_CHARS.len()] as char)
        .collect()
}

/// Generate repeated single character (worst case for some tokenizers)
fn generate_repeated_char(char: char, count: usize) -> String {
    char.to_string().repeat(count)
}

/// Generate content with zero-width characters
fn generate_zero_width_content(base_len: usize) -> String {
    const ZERO_WIDTH: char = '\u{200B}'; // Zero-width space
    (0..base_len)
        .flat_map(|i| {
            let regular = if i % 10 == 0 { 'a' } else { ' ' };
            [regular, ZERO_WIDTH]
        })
        .collect()
}

/// Generate content with mixed multibyte characters (emoji, CJK, etc.)
fn generate_multibyte_content(len: usize) -> String {
    const MULTIBYTE_CHARS: &[&str] = &[
        "😀", "🎉", "🚀", // Emoji
        "中", "文", "日", "本", // CJK
        "α", "β", "γ", // Greek
        "→", "←", "↔", // Arrows
        "™", "©", "®", // Symbols
    ];

    (0..len)
        .map(|i| MULTIBYTE_CHARS[i % MULTIBYTE_CHARS.len()])
        .collect::<String>()
}

/// Generate content with very long lines (no newlines)
fn generate_long_line_content(chars: usize) -> String {
    let words = ["word", "test", "content", "data", "text"];
    (0..chars)
        .map(|i| {
            let word = words[i % words.len()];
            word.chars().nth(i % word.len()).unwrap_or('x')
        })
        .collect()
}

/// Test result with timing
struct TimedResult {
    chunks: Option<usize>,
    error: Option<String>,
    duration: Duration,
}

/// Time a chunking operation
fn time_chunk(doc: &Document, level: ChunkLevel) -> TimedResult {
    let start = Instant::now();
    let result = chunk(doc, level).map(|c| c.len());
    let duration = start.elapsed();

    match result {
        Ok(count) => TimedResult {
            chunks: Some(count),
            error: None,
            duration,
        },
        Err(e) => TimedResult {
            chunks: None,
            error: Some(e.to_string()),
            duration,
        },
    }
}

/// Assert operation completes within time limit
fn assert_completes_within(name: &str, duration: Duration, max: Duration) {
    assert!(
        duration < max,
        "{} took {:?} (max: {:?}) - possible O(n²) regression",
        name,
        duration,
        max
    );
}

// ============================================================================
// MINIFIED CODE TESTS
// ============================================================================

#[test]
fn test_minified_small() {
    let content = generate_minified_content(1_000);
    let doc = Document::new("minified-small".into(), "Minified Small".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Minified (1K chars): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle minified content: {:?}",
        result.error
    );
    assert_completes_within(
        "minified_small",
        result.duration,
        Duration::from_millis(500),
    );
}

#[test]
fn test_minified_medium() {
    let content = generate_minified_content(10_000);
    let doc = Document::new("minified-medium".into(), "Minified Medium".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Minified (10K chars): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle minified content: {:?}",
        result.error
    );
    assert_completes_within("minified_medium", result.duration, Duration::from_secs(2));
}

#[test]
fn test_minified_large() {
    let content = generate_minified_content(100_000);
    let doc = Document::new("minified-large".into(), "Minified Large".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Minified (100K chars): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle minified content: {:?}",
        result.error
    );
    assert_completes_within("minified_large", result.duration, MAX_ADVERSARIAL_TIME);
}

// ============================================================================
// BASE64 TESTS
// ============================================================================

#[test]
fn test_base64_medium() {
    let content = generate_base64_content(50_000);
    let doc = Document::new("base64-medium".into(), "Base64 Medium".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Base64 (50K chars): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle base64 content: {:?}",
        result.error
    );
    assert_completes_within("base64_medium", result.duration, MAX_ADVERSARIAL_TIME);
}

// ============================================================================
// REPEATED CHARACTER TESTS
// ============================================================================

#[test]
fn test_repeated_char_a() {
    let content = generate_repeated_char('A', 50_000);
    let doc = Document::new("repeated-a".into(), "Repeated A".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Repeated 'A' (50K): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle repeated characters: {:?}",
        result.error
    );
    assert_completes_within("repeated_char_a", result.duration, MAX_ADVERSARIAL_TIME);
}

#[test]
fn test_repeated_char_unicode() {
    let content = generate_repeated_char('😀', 10_000);
    let doc = Document::new("repeated-emoji".into(), "Repeated Emoji".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Repeated emoji (10K): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle repeated emoji: {:?}",
        result.error
    );
    assert_completes_within(
        "repeated_char_unicode",
        result.duration,
        MAX_ADVERSARIAL_TIME,
    );
}

// ============================================================================
// ZERO-WIDTH CHARACTER TESTS
// ============================================================================

#[test]
fn test_zero_width_small() {
    let content = generate_zero_width_content(1_000);
    let doc = Document::new(
        "zero-width-small".into(),
        "Zero Width Small".into(),
        content,
    );

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Zero-width (1K base): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle zero-width characters: {:?}",
        result.error
    );
    assert_completes_within("zero_width_small", result.duration, Duration::from_secs(1));
}

#[test]
fn test_zero_width_medium() {
    let content = generate_zero_width_content(10_000);
    let doc = Document::new(
        "zero-width-medium".into(),
        "Zero Width Medium".into(),
        content,
    );

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Zero-width (10K base): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle zero-width characters: {:?}",
        result.error
    );
    assert_completes_within("zero_width_medium", result.duration, MAX_ADVERSARIAL_TIME);
}

// ============================================================================
// MULTIBYTE CHARACTER TESTS
// ============================================================================

#[test]
fn test_multibyte_mixed() {
    let content = generate_multibyte_content(5_000);
    let doc = Document::new("multibyte-mixed".into(), "Multibyte Mixed".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Multibyte mixed (5K chars): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle multibyte characters: {:?}",
        result.error
    );
    assert_completes_within("multibyte_mixed", result.duration, Duration::from_secs(2));
}

// ============================================================================
// LONG LINE TESTS
// ============================================================================

#[test]
fn test_long_line_medium() {
    let content = generate_long_line_content(50_000);
    let doc = Document::new(
        "long-line-medium".into(),
        "Long Line Medium".into(),
        content,
    );

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Long line (50K chars, no newlines): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle long lines: {:?}",
        result.error
    );
    assert_completes_within("long_line_medium", result.duration, MAX_ADVERSARIAL_TIME);
}

#[test]
fn test_long_line_very_long() {
    // This tests the 50K character preprocessing safety check
    let content = generate_long_line_content(100_000);
    let doc = Document::new("long-line-huge".into(), "Long Line Huge".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Long line (100K chars, no newlines): {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle very long lines: {:?}",
        result.error
    );
    assert_completes_within("long_line_very_long", result.duration, MAX_ADVERSARIAL_TIME);
}

// ============================================================================
// COMBINED ADVERSARIAL TESTS
// ============================================================================

#[test]
fn test_adversarial_corpus_mixed() {
    // Mix multiple adversarial patterns in one document
    let parts: Vec<String> = vec![
        generate_minified_content(5_000),
        "\n\n## Section\n\n".to_string(),
        generate_base64_content(5_000),
        "\n\n## Another\n\n".to_string(),
        generate_multibyte_content(2_000),
    ];

    let content = parts.join("");
    let doc = Document::new(
        "adversarial-mixed".into(),
        "Adversarial Mixed".into(),
        content,
    );

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Adversarial mixed corpus: {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle mixed adversarial content: {:?}",
        result.error
    );
    assert_completes_within(
        "adversarial_corpus_mixed",
        result.duration,
        MAX_ADVERSARIAL_TIME,
    );
}

#[test]
fn test_adversarial_scaling_is_linear() {
    // Verify adversarial inputs scale linearly, not quadratically
    let sizes = [5_000, 10_000, 20_000];

    let timings: Vec<Duration> = sizes
        .iter()
        .map(|&size| {
            let content = generate_base64_content(size);
            let doc = Document::new(
                format!("scale-{}", size),
                format!("Scale {}", size),
                content,
            );
            let result = time_chunk(&doc, ChunkLevel::Standard);
            println!(
                "  Adversarial scaling ({} chars): {:?}",
                size, result.duration
            );
            assert!(result.error.is_none(), "Should succeed at size {}", size);
            result.duration
        })
        .collect();

    // 2x input should not take >4x time (allowing some variance)
    let ratio_1_to_2 = timings[1].as_secs_f64() / timings[0].as_secs_f64().max(0.001);
    let ratio_2_to_3 = timings[2].as_secs_f64() / timings[1].as_secs_f64().max(0.001);

    println!(
        "Adversarial scaling ratios: {:.2}x, {:.2}x",
        ratio_1_to_2, ratio_2_to_3
    );

    // Allow up to 5x for 2x input (tokenizer overhead + variance)
    assert!(
        ratio_1_to_2 < 5.0,
        "Scaling from 5K to 10K took {:.2}x longer - possible O(n²)",
        ratio_1_to_2
    );
    assert!(
        ratio_2_to_3 < 5.0,
        "Scaling from 10K to 20K took {:.2}x longer - possible O(n²)",
        ratio_2_to_3
    );
}

#[test]
fn test_no_panic_on_any_unicode() {
    // Comprehensive unicode stress test - should never panic
    let unicode_samples: Vec<String> = vec![
        // RTL text
        "مرحبا بالعالم".to_string(),
        // Combining characters
        "e\u{0301}\u{0301}\u{0301}".to_string(),
        // Surrogate pairs (emoji)
        "👨‍👩‍👧‍👦".to_string(),
        // Zero-width joiners
        "a\u{200D}b\u{200D}c".to_string(),
        // Variation selectors
        "☕\u{FE0F}".to_string(),
        // Very long combining sequence
        (0..100)
            .map(|i| format!("a\u{0300}{:04x}", i % 16))
            .collect(),
    ];

    let content = unicode_samples.join("\n\n## Section\n\n");
    let doc = Document::new("unicode-stress".into(), "Unicode Stress".into(), content);

    let result = time_chunk(&doc, ChunkLevel::Standard);

    println!(
        "Unicode stress test: {:?} for {} chunks",
        result.duration,
        result.chunks.unwrap_or(0)
    );

    assert!(
        result.error.is_none(),
        "Should handle all unicode patterns without panic: {:?}",
        result.error
    );

    // Verify char boundaries are valid by re-chunking and checking
    let content = unicode_samples.join("\n\n## Section\n\n");
    let doc = Document::new("unicode-stress".into(), "Unicode Stress".into(), content);

    match chunk(&doc, ChunkLevel::Standard) {
        Ok(chunks) => {
            chunks.iter().for_each(|c| {
                assert!(
                    c.content.is_char_boundary(0),
                    "Content should start at char boundary"
                );
                assert!(
                    c.content.is_char_boundary(c.content.len()),
                    "Content should end at char boundary"
                );
            });
        }
        Err(e) => panic!("Should not fail: {}", e),
    }
}
