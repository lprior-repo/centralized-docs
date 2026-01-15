---
doc_id: ralph-loop-final-report
chunk_id: ralph-loop-final-report#26
chunk_level: summary
chunk_type: prose
heading: Innovation Verified
token_count: 131
summary: **Implementation:**. fn create_context_prefix(prev_chunk: &str, target_tokens: usize) -> String {
---


---




**Implementation:**
```rust
fn create_context_prefix(prev_chunk: &str, target_tokens: usize) -> String {
    let words: Vec<&str> = prev_chunk.split_whitespace().collect();
    let mut prefix = String::new();
    let mut tokens = 0;

    for word in words.iter().rev() {
        tokens += estimate_word_tokens(word);
        if tokens > target_tokens { break; }
        prefix.insert_str(0, word);
        prefix.insert(0, ' ');
    }
    prefix.trim().to_string()
}
```

**Status:** ✅ Fully implemented and tested
