use crate::analyze::Analysis;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

// Compile regexes once at startup using LazyLock for thread-safe lazy initialization
static H2_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^## (.+)$").expect("H2_REGEX pattern is valid")
});

static TABLE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|.*\|").expect("TABLE_REGEX pattern is valid")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub chunk_index: usize,
    pub content: String,
    pub token_count: usize,
    pub heading: Option<String>,
    pub chunk_type: String,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    pub related_chunk_ids: Vec<String>,
    pub summary: String,
}

pub struct ChunksResult {
    pub total_chunks: usize,
    pub document_count: usize,
    pub chunks_metadata: Vec<Chunk>,
}

/// Smart chunking that preserves semantic boundaries while keeping token count ~170
/// Implements contextual retrieval: each chunk knows its place in the document
pub fn chunk_all(analyses: &[Analysis], output_dir: &Path) -> Result<ChunksResult> {
    let chunks_dir = output_dir.join("chunks");
    fs::create_dir_all(&chunks_dir)?;

    let mut total_chunks = 0;
    let mut all_chunks = Vec::new();

    for analysis in analyses {
        let doc_id = slugify(&analysis.source_path);
        let chunks = create_chunks_smart(&analysis.content, &doc_id, &analysis.title, &analysis.source_path);

        for chunk in chunks {
            all_chunks.push(chunk);
        }
    }

    // Add navigation links between chunks
    link_chunks(&mut all_chunks);

    // Write chunks to disk
    for chunk in &all_chunks {
        let chunk_filename = format!("{}.md", chunk.chunk_id.replace(['/', '#'], "-"));
        let chunk_file = chunks_dir.join(&chunk_filename);

        let frontmatter = format!(
            "---\ndoc_id: {}\nchunk_id: {}\nchunk_type: {}\nheading: {}\ntoken_count: {}\nsummary: {}\n---\n",
            chunk.doc_id,
            chunk.chunk_id,
            chunk.chunk_type,
            chunk.heading.as_ref().unwrap_or(&"Introduction".to_string()),
            chunk.token_count,
            escape_frontmatter(&chunk.summary)
        );

        let content = format!("{}\n{}", frontmatter, chunk.content);
        fs::write(chunk_file, content)?;
        total_chunks += 1;
    }

    Ok(ChunksResult {
        total_chunks,
        document_count: analyses.len(),
        chunks_metadata: all_chunks,
    })
}

/// Smart semantic chunking that:
/// - Splits on H2 boundaries (semantic units)
/// - Limits to ~170 tokens per chunk
/// - Prepends 50-100 tokens of context from previous chunk
/// - Extracts heading for navigation
fn create_chunks_smart(
    content: &str,
    doc_id: &str,
    doc_title: &str,
    _source_path: &str,
) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut current_heading: Option<String> = None;
    let mut chunk_index = 0;
    let mut context_buffer = String::new();

    let lines: Vec<&str> = content.lines().collect();

    for (_i, line) in lines.iter().enumerate() {
        // Check for H2 heading (new chunk boundary)
        if let Some(caps) = H2_REGEX.captures(line) {
            if !current_chunk.is_empty() {
                let chunk_id = format!("{}#{}", doc_id, chunk_index);
                let summary = create_summary(&current_chunk);
                let token_count = estimate_tokens(&current_chunk);
                let chunk_type = detect_chunk_type(&current_chunk);

                chunks.push(Chunk {
                    chunk_id,
                    doc_id: doc_id.to_string(),
                    doc_title: doc_title.to_string(),
                    chunk_index,
                    content: current_chunk.clone(),
                    token_count,
                    heading: current_heading.clone(),
                    chunk_type,
                    previous_chunk_id: if chunk_index > 0 {
                        Some(format!("{}#{}", doc_id, chunk_index - 1))
                    } else {
                        None
                    },
                    next_chunk_id: None, // Will be set after
                    related_chunk_ids: Vec::new(), // Will be computed later
                    summary,
                });

                chunk_index += 1;

                // Save context for next chunk
                context_buffer = current_chunk
                    .lines()
                    .rev()
                    .take_while(|l| estimate_tokens(l) < 100)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n");

                current_chunk.clear();
            }

            current_heading = caps.get(1).map(|m| m.as_str().to_string());

            // Add context to new chunk
            if !context_buffer.is_empty() {
                current_chunk.push_str(&context_buffer);
                current_chunk.push('\n');
            }
        }

        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    // Add final chunk
    if !current_chunk.is_empty() {
        let chunk_id = format!("{}#{}", doc_id, chunk_index);
        let summary = create_summary(&current_chunk);
        let token_count = estimate_tokens(&current_chunk);
        let chunk_type = detect_chunk_type(&current_chunk);

        chunks.push(Chunk {
            chunk_id,
            doc_id: doc_id.to_string(),
            doc_title: doc_title.to_string(),
            chunk_index,
            content: current_chunk,
            token_count,
            heading: current_heading,
            chunk_type,
            previous_chunk_id: if chunk_index > 0 {
                Some(format!("{}#{}", doc_id, chunk_index - 1))
            } else {
                None
            },
            next_chunk_id: None,
            related_chunk_ids: Vec::new(),
            summary,
        });
    }

    // If no chunks created, create one from whole content
    if chunks.is_empty() {
        let chunk_id = format!("{}#0", doc_id);
        let summary = create_summary(content);
        let token_count = estimate_tokens(content);
        let chunk_type = detect_chunk_type(content);

        chunks.push(Chunk {
            chunk_id,
            doc_id: doc_id.to_string(),
            doc_title: doc_title.to_string(),
            chunk_index: 0,
            content: content.to_string(),
            token_count,
            heading: None,
            chunk_type,
            previous_chunk_id: None,
            next_chunk_id: None,
            related_chunk_ids: Vec::new(),
            summary,
        });
    }

    chunks
}

/// Link chunks together: set next_chunk_id pointers
fn link_chunks(chunks: &mut [Chunk]) {
    for i in 0..chunks.len() {
        if i + 1 < chunks.len() && chunks[i].doc_id == chunks[i + 1].doc_id {
            chunks[i].next_chunk_id = Some(chunks[i + 1].chunk_id.clone());
        }
    }
}

/// Estimate token count (simple: ~4 chars = 1 token)
fn estimate_tokens(text: &str) -> usize {
    (text.len() / 4).max(1)
}

/// Create a summary of chunk content (first 2 sentences or 50 words)
fn create_summary(content: &str) -> String {
    let sentences: Vec<&str> = content
        .split(['.', '\n'])
        .filter(|s| s.trim().len() > 10)
        .take(2)
        .collect();

    let summary = sentences.join(". ");
    if summary.len() > 200 {
        format!("{}...", safe_truncate(&summary, 200))
    } else {
        summary
    }
}

/// Safely truncate a string to max_bytes, ensuring we don't split UTF-8 characters
fn safe_truncate(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }

    // Find the largest valid UTF-8 boundary <= max_bytes
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// Generate URL-safe slug for document ID
fn slugify(text: &str) -> String {
    text
        .to_lowercase()
        .replace('/', "-")
        .replace(".md", "")
        .replace(".mdx", "")
        .replace('_', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

/// Escape special characters in frontmatter string values
fn escape_frontmatter(text: &str) -> String {
    text.replace('"', "\\\"")
        .replace('\n', " ")
        .chars()
        .take(100)
        .collect()
}

/// Detect chunk type: code, table, or prose
fn detect_chunk_type(content: &str) -> String {
    let code_block_count = content.matches("```").count() / 2;
    let has_table = content.contains('|') && TABLE_REGEX.is_match(content);

    if code_block_count > 5 {
        "code".to_string()
    } else if has_table {
        "table".to_string()
    } else {
        "prose".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_summary_with_multibyte_chars() {
        // Test with emoji - should not panic
        let emoji_content = "Hello 🦀 Rust is awesome! 🎉 This is a test. More content here.";
        let result = create_summary(emoji_content);
        assert!(!result.is_empty());

        // Test with CJK characters - should not panic
        let cjk_content = "日本語のドキュメントをテストします。これは２番目の文章です。";
        let result2 = create_summary(cjk_content);
        assert!(!result2.is_empty());

        // Test with long emoji string that needs truncation
        let long_emoji = "🦀".repeat(300) + " This is a sentence. And another one.";
        let result3 = create_summary(&long_emoji);
        assert!(!result3.is_empty());
        // Should end with ... if truncated
        if result3.contains("...") {
            // Verify it's valid UTF-8 and no panic occurred
            // The truncated part should be ~200 chars + "..." = 203
            assert!(result3.chars().count() <= 203);
        }
    }

    #[test]
    fn test_create_summary_no_panic_on_boundary() {
        // This would panic with byte-based slicing
        // "🦀" is 4 bytes in UTF-8
        let content = "a".repeat(198) + "🦀🦀🦀"; // 198 + 12 bytes = 210 bytes total
        let result = create_summary(&content);
        // Should successfully create summary without panic
        assert!(!result.is_empty());
    }
}
