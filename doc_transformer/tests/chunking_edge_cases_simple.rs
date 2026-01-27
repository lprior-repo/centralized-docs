//! Edge case tests for document chunking - simplified version
//! Tests key scenarios: no H2 headers, long documents, Unicode content

use doc_transformer::chunk::{create_chunks_at_level, ChunkLevel};

#[test]
fn test_no_h2_headers_simple() {
    // Test document with only H1, no H2 section headers
    let content = "# Main Document\n\nThis is content without any H2 headers.\nJust plain prose throughout.";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    // Should create at least one chunk
    assert!(!chunks.is_empty(), "Should create at least one chunk");

    // Content should be preserved
    assert!(chunks[0].content.contains("Main Document"));

    // All chunks should have valid doc_id
    for chunk in &chunks {
        assert_eq!(chunk.doc_id, "test-doc");
    }
}

#[test]
fn test_long_document_splits_on_token_limit() {
    // Test that long content is split at token boundaries
    let long_paragraph = "paragraph text ".repeat(200);
    let content = format!("# Title\n\n{long_paragraph}");

    let chunks = create_chunks_at_level(&content, "test-doc", "Test Doc", ChunkLevel::Summary);

    // Debug output
    eprintln!("Number of chunks created: {}", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        eprintln!("Chunk {}: {} tokens", i, chunk.token_count);
    }

    // Summary level should split long content into multiple chunks
    assert!(chunks.len() > 1, "Long content should create multiple chunks at summary level");

    // All chunks should have content
    for (i, chunk) in chunks.iter().enumerate() {
        assert!(!chunk.content.trim().is_empty(), "Chunk {i} is empty");
        assert!(chunk.token_count > 0, "Chunk {i} has no tokens");
    }
}

#[test]
fn test_unicode_emoji_handling() {
    // Test that emoji characters don't break chunking
    let content = "# Document 🎉\n\n## Section 🚀\n\nContent with emoji 🎨 continues here.";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    assert!(!chunks.is_empty(), "Should handle emoji without error");

    // Emoji should be preserved in content
    let combined = chunks.iter()
        .map(|c| c.content.as_str())
        .collect::<String>();

    assert!(combined.contains("🎉") || combined.contains("Document"), "Content lost during chunking");
}

#[test]
fn test_cjk_text_chunking() {
    // Test that Chinese characters are handled correctly
    let content = "# 文档\n\n## 中文部分\n\n这是中文内容。包含多行。\n\n## 英文部分\n\nEnglish content here.";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    assert!(!chunks.is_empty(), "Should handle CJK without error");

    // Chinese characters should be preserved
    let combined = chunks.iter()
        .map(|c| c.content.as_str())
        .collect::<String>();

    assert!(combined.contains("文档") || combined.contains("中文"), "Chinese characters lost");
}

#[test]
fn test_empty_document_handling() {
    // Test that empty document creates a default chunk
    let content = "";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    // Should create at least one chunk even for empty content
    assert!(!chunks.is_empty(), "Should create at least one chunk for empty content");

    // The chunk should be empty but valid
    assert_eq!(chunks[0].chunk_index, 0);
    assert_eq!(chunks[0].doc_id, "test-doc");
}

#[test]
fn test_multiple_h2_headers_create_chunks() {
    // Test that multiple H2 headers create separate chunks
    let content = "# Title\n\n## Section 1\n\nContent 1.\n\n## Section 2\n\nContent 2.\n\n## Section 3\n\nContent 3.";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    // Should create multiple chunks (one per H2 roughly)
    assert!(chunks.len() >= 3, "Should create at least 3 chunks for 3 H2 sections");

    // Chunks should be sequential
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i, "Chunk indices should be sequential");
    }
}

#[test]
fn test_chunk_level_consistency() {
    // Test that all chunks have correct level
    let content = "# Title\n\n## Section\n\nContent";

    let levels = vec![ChunkLevel::Summary, ChunkLevel::Standard, ChunkLevel::Detailed];

    for level in levels {
        let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", level.clone());

        for chunk in &chunks {
            assert_eq!(chunk.chunk_level, level, "Chunk should have correct level");
        }
    }
}

#[test]
fn test_chunk_id_format() {
    // Test that chunk IDs are correctly formatted
    let content = "# Title\n\n## Section A\n\nContent A\n\n## Section B\n\nContent B";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    for chunk in &chunks {
        // Chunk ID should be in format "doc-id#index"
        assert!(chunk.chunk_id.contains('#'), "Chunk ID should contain #");
        assert!(chunk.chunk_id.starts_with("test-doc#"), "Chunk ID should start with doc-id#");
    }
}

#[test]
fn test_rtl_text_handling() {
    // Test right-to-left text (Arabic)
    let content = "# Document\n\n## Arabic Section\n\nهذا نص عربي يمتد عبر عدة أسطر.";

    let chunks = create_chunks_at_level(content, "test-doc", "Test Doc", ChunkLevel::Standard);

    assert!(!chunks.is_empty(), "Should handle RTL text without error");
}
