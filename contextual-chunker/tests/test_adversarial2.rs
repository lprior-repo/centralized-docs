use contextual_chunker::{chunk, ChunkLevel, Document};

#[test]
fn test_adversarial_create_chunks_at_level() {
    let mut long_content = String::with_capacity(2_000_000);
    for _ in 0..2_000_000 {
        long_content.push('A');
    }

    let doc = Document::new("doc1".to_string(), "Title".to_string(), long_content);

    let result = chunk(&doc, ChunkLevel::Standard);
    assert!(
        result.is_ok(),
        "chunk should succeed on large input: {:?}",
        result.err()
    );
    let chunks = result.unwrap();
    assert!(
        !chunks.is_empty(),
        "chunk should produce at least one chunk for 2M chars of input"
    );
}
