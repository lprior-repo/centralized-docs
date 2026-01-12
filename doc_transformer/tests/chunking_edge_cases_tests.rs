//! Comprehensive edge case tests for document chunking
//!
//! This module tests semantic chunking behavior with specific edge cases:
//! 1. NO H2 HEADERS: Documents with only H1 and content (no H2 boundaries)
//! 2. LONG DOCUMENTS: Single giant paragraph exceeding token limits
//! 3. UNICODE CONTENT: Emoji, RTL text, combining characters, CJK scripts
//! 4. SPECIAL CASES: Empty documents, whitespace-only, mixed structure
//!
//! Test strategy: Table-driven tests for each edge case category with
//! assertions on chunk count, content integrity, and metadata correctness.

use doc_transformer::chunk::{create_chunks_at_level, ChunkLevel};

// ============================================================================
// TEST CASE STRUCTURES
// ============================================================================

/// Test case for edge case chunking scenarios
#[derive(Debug, Clone)]
struct ChunkingEdgeCase {
    name: &'static str,
    content: &'static str,
    chunk_level: ChunkLevel,
    /// Expected minimum number of chunks
    expected_min_chunks: usize,
    /// Expected maximum number of chunks
    expected_max_chunks: usize,
    /// Should all chunks be non-empty?
    expect_non_empty: bool,
    /// Should summaries be generated?
    expect_summaries: bool,
}

// ============================================================================
// EDGE CASE 1: NO H2 HEADERS
// ============================================================================

fn no_header_test_cases() -> Vec<ChunkingEdgeCase> {
    vec![
        // Case 1a: Only H1, no H2 sections - should create single chunk
        ChunkingEdgeCase {
            name: "no_h2_headers_simple",
            content: "# Document Title\n\nThis is the content of the document.\nIt has no H2 headers.\nJust plain prose.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 1b: H1 with H3/H4 but no H2 - should not split on H3/H4
        ChunkingEdgeCase {
            name: "no_h2_with_h3_h4",
            content: "# Main Title\n\n### Subsection\n\nContent under H3.\n\n#### Details\n\nMore content.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 1c: Only H1, very long content (should split on token limit, not headers)
        ChunkingEdgeCase {
            name: "no_h2_long_content",
            content: &format!(
                "# Very Long Document\n\n{}\n\n{}",
                "This is a long paragraph. ".repeat(200),
                "And more content continues here. ".repeat(200)
            ),
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,  // Will split due to token limit
            expected_max_chunks: 10,  // But not too many chunks
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 1d: Multiple H1s, no H2s - each H1 might get its own chunk
        ChunkingEdgeCase {
            name: "multiple_h1_no_h2",
            content: "# First Document\n\nContent of first.\n\n# Second Document\n\nContent of second.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 2,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 1e: H1 with inline code and lists but no H2
        ChunkingEdgeCase {
            name: "no_h2_with_code_lists",
            content: "# Documentation\n\n- Item 1\n- Item 2\n- Item 3\n\n```\ncode block\n```\n\nMore text here.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: true,
            expect_summaries: true,
        },
    ]
}

// ============================================================================
// EDGE CASE 2: LONG DOCUMENTS (SINGLE GIANT PARAGRAPH)
// ============================================================================

fn long_document_test_cases() -> Vec<ChunkingEdgeCase> {
    vec![
        // Case 2a: Single paragraph exceeding all token limits
        ChunkingEdgeCase {
            name: "single_giant_paragraph",
            content: &format!(
                "# Title\n\n{}",
                "This is an extremely long paragraph that continues without any line breaks or natural boundaries. ".repeat(500)
            ),
            chunk_level: ChunkLevel::Summary,
            expected_min_chunks: 1,
            expected_max_chunks: 100,  // Will split on token boundaries
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 2b: Multiple long paragraphs (no H2) at different levels
        ChunkingEdgeCase {
            name: "long_paragraphs_summary_level",
            content: &format!(
                "# Title\n\n{}\n\n{}",
                "Paragraph 1: ".repeat(300) + "content.",
                "Paragraph 2: ".repeat(300) + "more content."
            ),
            chunk_level: ChunkLevel::Summary,
            expected_min_chunks: 2,
            expected_max_chunks: 20,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 2c: Single paragraph at Standard level
        ChunkingEdgeCase {
            name: "single_long_paragraph_standard",
            content: &format!(
                "# Title\n\n{}",
                "Standard level paragraph: ".repeat(400) + "end of content."
            ),
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 20,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 2d: Single paragraph at Detailed level
        ChunkingEdgeCase {
            name: "single_long_paragraph_detailed",
            content: &format!(
                "# Title\n\n{}",
                "Detailed level paragraph: ".repeat(400) + "end of content."
            ),
            chunk_level: ChunkLevel::Detailed,
            expected_min_chunks: 1,
            expected_max_chunks: 10,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 2e: Long content with sentences but no paragraph breaks
        ChunkingEdgeCase {
            name: "long_sentences_no_breaks",
            content: "# Title\n\nFirst sentence with lots of words. Second sentence continues. Third sentence adds more. Fourth sentence extends further. Fifth sentence goes on. Sixth continues the pattern. Seventh adds information. Eighth extends it further. Ninth keeps going. Tenth wraps up.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: true,
            expect_summaries: true,
        },
    ]
}

// ============================================================================
// EDGE CASE 3: UNICODE CONTENT
// ============================================================================

fn unicode_test_cases() -> Vec<ChunkingEdgeCase> {
    vec![
        // Case 3a: Emoji characters throughout
        ChunkingEdgeCase {
            name: "emoji_content",
            content: "# Emoji Document 🎉\n\n## Features\n\n✨ Feature 1\n🚀 Feature 2\n⭐ Feature 3\n\nContent: 🎨🎭🎪🎬🎯",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3b: RTL text (Arabic, Hebrew)
        ChunkingEdgeCase {
            name: "rtl_text",
            content: "# Document\n\n## Arabic Section\n\nهذا نص عربي يمتد عبر عدة أسطر. يحتوي على محتوى مهم.\n\n## Hebrew Section\n\nזה טקסט בעברית החוזר על עצמו כמה פעמים.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3c: CJK (Chinese, Japanese, Korean)
        ChunkingEdgeCase {
            name: "cjk_content",
            content: "# 文档\n\n## 中文部分\n\n这是一份包含中文内容的文档。它包含多行文本。\n\n## 日本語セクション\n\nこれは日本語のテキストです。複数の行があります。\n\n## 한국어 섹션\n\n이것은 한국어 텍스트입니다.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 3,
            expected_max_chunks: 4,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3d: Combining characters and diacritics
        ChunkingEdgeCase {
            name: "combining_diacritics",
            content: "# Café au Lait\n\n## Naïve Résumé\n\nThis section contains combining marks: é, ñ, ü, ç.\n\nMoré tëxt with vàrious diacritical marks.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3e: Mixed scripts (Latin + Cyrillic + Greek + Arabic)
        ChunkingEdgeCase {
            name: "mixed_scripts",
            content: "# Multilingual\n\n## English\nEnglish text here.\n\n## Русский\nРусский текст здесь.\n\n## Ελληνικά\nΕλληνικό κείμενο εδώ.\n\n## العربية\nالنص العربي هنا.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 4,
            expected_max_chunks: 5,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3f: Zero-width characters and invisible marks
        ChunkingEdgeCase {
            name: "zero_width_chars",
            content: "# Title\u{200B}\n\nContent\u{200C}with\u{200D}zero\u{FEFF}width\u{200B}chars\n\n## Section\n\nMore content here.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 3g: Emoji with variation selectors and skin tone modifiers
        ChunkingEdgeCase {
            name: "emoji_modifiers",
            content: "# Emoji Variants 👨‍👩‍👧‍👦\n\n👋 Wave\n👋🏻 Light skin tone\n👋🏿 Dark skin tone\n\n🏳️‍🌈 Rainbow flag\n⚪ Variations\n\nContent continues here.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 2,
            expect_non_empty: true,
            expect_summaries: true,
        },
    ]
}

// ============================================================================
// EDGE CASE 4: SPECIAL CASES
// ============================================================================

fn special_case_test_cases() -> Vec<ChunkingEdgeCase> {
    vec![
        // Case 4a: Empty document
        ChunkingEdgeCase {
            name: "empty_document",
            content: "",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,  // Should create a default chunk
            expected_max_chunks: 1,
            expect_non_empty: false,
            expect_summaries: false,
        },
        // Case 4b: Whitespace only
        ChunkingEdgeCase {
            name: "whitespace_only",
            content: "   \n\n   \n\n   ",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: false,
            expect_summaries: false,
        },
        // Case 4c: Single character document
        ChunkingEdgeCase {
            name: "single_character",
            content: "x",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 1,
            expect_non_empty: true,
            expect_summaries: false,
        },
        // Case 4d: Only headings, no content
        ChunkingEdgeCase {
            name: "only_headings",
            content: "# Title\n\n## Section 1\n\n## Section 2\n\n## Section 3",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 3,
            expected_max_chunks: 4,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 4e: Headings with special characters
        ChunkingEdgeCase {
            name: "special_chars_in_headers",
            content: "# Title with <special> & \"quoted\" characters\n\n## Section [with] {brackets}\n\nContent here.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 2,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 4f: Very deeply nested headings
        ChunkingEdgeCase {
            name: "deeply_nested_headings",
            content: "# Level 1\n\nContent.\n\n## Level 2\n\nMore content.\n\n### Level 3\n\n#### Level 4\n\n##### Level 5\n\nDeep content.",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 1,
            expected_max_chunks: 3,
            expect_non_empty: true,
            expect_summaries: true,
        },
        // Case 4g: Multiple consecutive H2 headers
        ChunkingEdgeCase {
            name: "consecutive_h2_headers",
            content: "# Title\n\n## Section 1\n\n## Section 2\n\n## Section 3\n\n## Section 4\n\n## Section 5",
            chunk_level: ChunkLevel::Standard,
            expected_min_chunks: 5,
            expected_max_chunks: 6,
            expect_non_empty: true,
            expect_summaries: true,
        },
    ]
}

// ============================================================================
// MACRO FOR GENERATING TESTS
// ============================================================================

macro_rules! test_edge_cases {
    ($test_name:ident, $cases_fn:ident) => {
        #[test]
        fn $test_name() {
            let test_cases = $cases_fn();

            for test_case in test_cases {
                println!(
                    "\n=== Testing Edge Case: {} (Level: {:?}) ===",
                    test_case.name, test_case.chunk_level
                );

                let chunks = create_chunks_at_level(
                    test_case.content,
                    "test-doc",
                    "Test Document",
                    test_case.chunk_level.clone(),
                );

                // Assertion 1: Chunk count within expected range
                assert!(
                    chunks.len() >= test_case.expected_min_chunks,
                    "Test '{}': Expected at least {} chunks, got {}",
                    test_case.name,
                    test_case.expected_min_chunks,
                    chunks.len()
                );

                assert!(
                    chunks.len() <= test_case.expected_max_chunks,
                    "Test '{}': Expected at most {} chunks, got {}",
                    test_case.name,
                    test_case.expected_max_chunks,
                    chunks.len()
                );

                // Assertion 2: Check if chunks are non-empty (if expected)
                if test_case.expect_non_empty && !test_case.content.is_empty() {
                    for (i, chunk) in chunks.iter().enumerate() {
                        assert!(
                            !chunk.content.trim().is_empty(),
                            "Test '{}': Chunk {} is empty",
                            test_case.name,
                            i
                        );
                    }
                }

                // Assertion 3: Check if summaries exist (if expected)
                if test_case.expect_summaries && !test_case.content.is_empty() {
                    for (i, chunk) in chunks.iter().enumerate() {
                        assert!(
                            !chunk.summary.is_empty(),
                            "Test '{}': Chunk {} has no summary",
                            test_case.name,
                            i
                        );
                    }
                }

                // Assertion 4: All chunks have valid IDs
                for chunk in &chunks {
                    assert!(
                        chunk.chunk_id.contains('#'),
                        "Test '{}': Chunk ID malformed: {}",
                        test_case.name,
                        chunk.chunk_id
                    );
                    assert_eq!(
                        chunk.doc_id, "test-doc",
                        "Test '{}': Wrong doc_id",
                        test_case.name
                    );
                    assert_eq!(
                        chunk.doc_title, "Test Document",
                        "Test '{}': Wrong doc_title",
                        test_case.name
                    );
                }

                // Assertion 5: Chunk indices are sequential
                for (expected_index, chunk) in chunks.iter().enumerate() {
                    assert_eq!(
                        chunk.chunk_index, expected_index,
                        "Test '{}': Chunk indices not sequential",
                        test_case.name
                    );
                }

                // Assertion 6: Token counts are positive for non-empty chunks
                for (i, chunk) in chunks.iter().enumerate() {
                    if !chunk.content.is_empty() {
                        assert!(
                            chunk.token_count > 0,
                            "Test '{}': Chunk {} has zero token count",
                            test_case.name,
                            i
                        );
                    }
                }

                // Assertion 7: Chunk level consistency
                for chunk in &chunks {
                    assert_eq!(
                        chunk.chunk_level, test_case.chunk_level,
                        "Test '{}': Chunk has wrong level",
                        test_case.name
                    );
                }

                println!("  ✓ Test '{}' passed with {} chunks", test_case.name, chunks.len());
            }
        }
    };
}

// ============================================================================
// ACTUAL TEST INVOCATIONS
// ============================================================================

test_edge_cases!(test_no_h2_headers_chunking, no_header_test_cases);
test_edge_cases!(test_long_documents_chunking, long_document_test_cases);
test_edge_cases!(test_unicode_content_chunking, unicode_test_cases);
test_edge_cases!(test_special_cases_chunking, special_case_test_cases);

// ============================================================================
// ADDITIONAL PROPERTY-BASED TESTS
// ============================================================================

#[test]
fn test_chunking_all_levels() {
    // Test that the same content produces chunks at all three levels
    let content = "# Main Title\n\n## Section 1\n\nThis is section 1 content.\n\n## Section 2\n\nThis is section 2 content.";

    let summary = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Summary);
    let standard = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);
    let detailed = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Detailed);

    // Summary chunks should typically be fewer or equal to standard
    println!(
        "Summary: {} chunks, Standard: {} chunks, Detailed: {} chunks",
        summary.len(),
        standard.len(),
        detailed.len()
    );

    // All levels should produce at least one chunk
    assert!(!summary.is_empty(), "Summary level produced no chunks");
    assert!(!standard.is_empty(), "Standard level produced no chunks");
    assert!(!detailed.is_empty(), "Detailed level produced no chunks");

    // Token counts should increase with level (approximately)
    let avg_summary_tokens: usize = summary.iter().map(|c| c.token_count).sum::<usize>() / summary.len();
    let avg_standard_tokens: usize = standard.iter().map(|c| c.token_count).sum::<usize>() / standard.len();
    let avg_detailed_tokens: usize = detailed.iter().map(|c| c.token_count).sum::<usize>() / detailed.len();

    println!(
        "Average tokens - Summary: {}, Standard: {}, Detailed: {}",
        avg_summary_tokens, avg_standard_tokens, avg_detailed_tokens
    );

    // Detailed level should have higher target than summary
    assert!(
        ChunkLevel::Detailed.target_tokens() > ChunkLevel::Summary.target_tokens(),
        "Detailed level target not greater than summary"
    );
}

#[test]
fn test_chunk_content_integrity() {
    // Ensure chunking doesn't lose content
    let content = "# Title\n\n## Section A\n\nContent A\n\n## Section B\n\nContent B\n\n## Section C\n\nContent C";

    let chunks = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);

    // Reconstruct by concatenating all chunk contents
    let reconstructed: String = chunks
        .iter()
        .map(|c| &c.content)
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join("\n");

    // Should contain all major sections
    assert!(
        reconstructed.contains("Title"),
        "Reconstructed content missing 'Title'"
    );
    assert!(
        reconstructed.contains("Section A"),
        "Reconstructed content missing 'Section A'"
    );
    assert!(
        reconstructed.contains("Content A"),
        "Reconstructed content missing 'Content A'"
    );
}

#[test]
fn test_unicode_integrity_in_chunks() {
    // Verify Unicode content is preserved correctly during chunking
    let content = "# 文档标题\n\n## 第一部分\n\n这是中文内容。\n\n## 第二部分\n\nMore 中文 混合 English。";

    let chunks = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);

    let combined = chunks
        .iter()
        .map(|c| c.content.clone())
        .collect::<String>();

    // Verify Chinese characters are preserved
    assert!(combined.contains("文档标题"), "Chinese title lost");
    assert!(combined.contains("第一部分"), "Chinese section 1 lost");
    assert!(combined.contains("这是中文内容"), "Chinese content lost");
}

#[test]
fn test_emoji_handling_in_summaries() {
    // Test that emoji don't break summary generation
    let content = "# Title 🎉\n\n## Section 🚀\n\nContent with emoji 🎨 and more text 🎭 here.";

    let chunks = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);

    for chunk in chunks {
        // Summary should exist and not crash
        assert!(
            !chunk.summary.is_empty() || chunk.content.is_empty(),
            "Summary generation failed with emoji content"
        );
    }
}

#[test]
fn test_chunk_type_detection_mixed_content() {
    // Test chunk type detection with mixed content (code, tables, prose)
    let content = "# Document\n\n## Code Section\n\n```rust\nfn main() {}\n```\n\n## Table Section\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n## Prose\n\nRegular text.";

    let chunks = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);

    assert!(chunks.len() >= 3, "Should have at least 3 chunks");

    // Each chunk should have a chunk_type (code, table, or prose)
    for chunk in chunks {
        assert!(
            chunk.chunk_type == "code" || chunk.chunk_type == "table" || chunk.chunk_type == "prose",
            "Invalid chunk type: {}",
            chunk.chunk_type
        );
    }
}

#[test]
fn test_heading_extraction_no_h2() {
    // Test heading extraction when there are no H2 headers
    let content = "# Main Title\n\nContent without H2 sections.";

    let chunks = create_chunks_at_level(content, "doc", "Doc", ChunkLevel::Standard);

    assert_eq!(chunks.len(), 1, "Should create single chunk");

    // Heading should be None since no H2 found
    assert!(
        chunks[0].heading.is_none(),
        "Should have no heading for H1-only document"
    );
}
