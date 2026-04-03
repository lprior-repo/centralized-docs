use contextual_chunker::{chunk, ChunkLevel, Document};

#[test]
fn test_adversarial_create_chunks_at_level() {
    let mut long_content = String::with_capacity(2_000_000);
    for _ in 0..2_000_000 {
        long_content.push('A');
    }

    let doc = Document::new("doc1".to_string(), "Title".to_string(), long_content);

    // This should not panic if properly fixed!
    let _result = chunk(&doc, ChunkLevel::Standard);
}
