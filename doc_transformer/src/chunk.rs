use crate::analyze::Analysis;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use tap::Pipe;

// Lazy-initialized regex patterns for chunking
//
// SAFETY (BEAD-006): All regex patterns are hardcoded string literals verified to be valid.
// The `.expect()` calls will never panic - this is guaranteed by:
// 1. Patterns are compile-time constants (no user input)
// 2. All patterns are tested in tests/bead_006_regex_initialization_tests.rs
// 3. If a pattern were invalid, tests would fail immediately
//
// Using `.expect()` here is acceptable per BEAD-006 Option A: "Keep LazyLock + Add Compile-Time Test"
static H2_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^## (.+)$").expect("valid H2 regex"));

static TABLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|.*\|").expect("valid table regex"));

/// Chunk level for hierarchical retrieval
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChunkLevel {
    /// Summary level: ~128 tokens, high-level overview
    Summary,
    /// Standard level: ~512 tokens, balanced detail
    Standard,
    /// Detailed level: ~1024 tokens, full context
    Detailed,
}

impl ChunkLevel {
    pub fn target_tokens(&self) -> usize {
        match self {
            ChunkLevel::Summary => 128,
            ChunkLevel::Standard => 512,
            ChunkLevel::Detailed => 1024,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ChunkLevel::Summary => "summary",
            ChunkLevel::Standard => "standard",
            ChunkLevel::Detailed => "detailed",
        }
    }
}

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
    /// Hierarchical chunk level (summary/standard/detailed)
    pub chunk_level: ChunkLevel,
    /// Parent chunk ID (for hierarchical navigation)
    pub parent_chunk_id: Option<String>,
    /// Child chunk IDs (for hierarchical navigation)
    pub child_chunk_ids: Vec<String>,
}

pub struct ChunksResult {
    pub total_chunks: usize,
    pub document_count: usize,
    pub chunks_metadata: Vec<Chunk>,
    /// Count of chunks by level
    pub summary_chunks: usize,
    pub standard_chunks: usize,
    pub detailed_chunks: usize,
}

/// Smart chunking that preserves semantic boundaries with hierarchical levels
/// Implements contextual retrieval: each chunk knows its place in the document
/// Creates chunks at three levels: summary (~128 tokens), standard (~512), detailed (~1024)
pub fn chunk_all(analyses: &[Analysis], output_dir: &Path) -> Result<ChunksResult> {
    let chunks_dir = output_dir.join("chunks");
    fs::create_dir_all(&chunks_dir)?;

    // Use functional fold to collect all chunks with counts
    let (all_chunks, summary_chunks, standard_chunks, detailed_chunks) = analyses.iter().fold(
        (Vec::new(), 0usize, 0usize, 0usize),
        |(mut chunks, sum_count, std_count, det_count), analysis| {
            let doc_id = slugify(&analysis.source_path);

            // Create chunks at ALL THREE levels for hierarchical retrieval
            let summary = create_chunks_at_level(
                &analysis.content,
                &doc_id,
                &analysis.title,
                ChunkLevel::Summary,
            );
            let standard = create_chunks_at_level(
                &analysis.content,
                &doc_id,
                &analysis.title,
                ChunkLevel::Standard,
            );
            let detailed = create_chunks_at_level(
                &analysis.content,
                &doc_id,
                &analysis.title,
                ChunkLevel::Detailed,
            );

            // Link parent-child relationships between levels
            let summary_ids: Vec<String> = summary.iter().map(|c| c.chunk_id.clone()).collect();
            let standard_ids: Vec<String> = standard.iter().map(|c| c.chunk_id.clone()).collect();
            let detailed_ids: Vec<String> = detailed.iter().map(|c| c.chunk_id.clone()).collect();

            let new_sum_count = summary.len();
            let new_std_count = standard.len();
            let new_det_count = detailed.len();

            // Add summary chunks with standard as children
            chunks.extend(summary.into_iter().map(|mut chunk| {
                chunk.child_chunk_ids = standard_ids.clone();
                chunk
            }));

            // Add standard chunks with relationships
            chunks.extend(standard.into_iter().map(|mut chunk| {
                chunk.parent_chunk_id = summary_ids.first().cloned();
                chunk.child_chunk_ids = detailed_ids.clone();
                chunk
            }));

            // Add detailed chunks with parent
            chunks.extend(detailed.into_iter().map(|mut chunk| {
                chunk.parent_chunk_id = standard_ids.first().cloned();
                chunk
            }));

            (
                chunks,
                sum_count.saturating_add(new_sum_count),
                std_count.saturating_add(new_std_count),
                det_count.saturating_add(new_det_count),
            )
        },
    );

    // Add navigation links between chunks (same level, same doc)
    let mut all_chunks = all_chunks;
    link_chunks(&mut all_chunks);

    // Write chunks to disk using functional for_each
    all_chunks.iter().try_for_each(|chunk| {
        let level_suffix = chunk.chunk_level.as_str();
        let chunk_filename = format!("{}-{}.md", chunk.chunk_id.replace(['/', '#'], "-"), level_suffix);
        let chunk_file = chunks_dir.join(&chunk_filename);

        let frontmatter = format!(
            "---\ndoc_id: {}\nchunk_id: {}\nchunk_level: {}\nchunk_type: {}\nheading: {}\ntoken_count: {}\nsummary: {}\n---\n",
            chunk.doc_id,
            chunk.chunk_id,
            level_suffix,
            chunk.chunk_type,
            chunk.heading.as_ref().unwrap_or(&"Introduction".to_string()),
            chunk.token_count,
            escape_frontmatter(&chunk.summary)
        );

        let content = format!("{}\n{}", frontmatter, chunk.content);
        fs::write(chunk_file, content)
    })?;

    let total_chunks = all_chunks.len();

    Ok(ChunksResult {
        total_chunks,
        document_count: analyses.len(),
        chunks_metadata: all_chunks,
        summary_chunks,
        standard_chunks,
        detailed_chunks,
    })
}

/// Create chunks at a specific hierarchical level
///
/// - Summary (~128 tokens): High-level overview for quick retrieval
/// - Standard (~512 tokens): Balanced detail for most use cases
/// - Detailed (~1024 tokens): Full context for deep understanding
pub fn create_chunks_at_level(
    content: &str,
    doc_id: &str,
    doc_title: &str,
    level: ChunkLevel,
) -> Vec<Chunk> {
    let target_tokens = level.target_tokens();

    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut current_heading: Option<String> = None;
    let mut chunk_index = 0;
    let mut context_buffer = String::new();

    let lines: Vec<&str> = content.lines().collect();

    for line in lines.iter() {
        // Check for H2 heading (new chunk boundary) or token limit reached
        let current_tokens = estimate_tokens(&current_chunk);
        let should_split = H2_REGEX.captures(line).is_some()
            || (current_tokens >= target_tokens && !current_chunk.is_empty());

        if should_split && !current_chunk.is_empty() {
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
                previous_chunk_id: chunk_index
                    .checked_sub(1)
                    .map(|prev| format!("{}#{}", doc_id, prev)),
                next_chunk_id: None,
                related_chunk_ids: Vec::new(),
                summary,
                chunk_level: level.clone(),
                parent_chunk_id: None,
                child_chunk_ids: Vec::new(),
            });

            chunk_index = chunk_index.saturating_add(1);

            // Context buffer size varies by level
            let context_tokens = match level {
                ChunkLevel::Summary => 30,
                ChunkLevel::Standard => 100,
                ChunkLevel::Detailed => 200,
            };

            context_buffer = get_context_tail(&current_chunk, context_tokens);
            current_chunk.clear();
        }

        // Update heading if this is an H2
        if let Some(caps) = H2_REGEX.captures(line) {
            current_heading = caps.get(1).map(|m| m.as_str().to_string());

            // Add context to new chunk
            if !context_buffer.is_empty() {
                current_chunk.push_str(&context_buffer);
                current_chunk.push('\n');
                context_buffer.clear();
            }
        }

        // Handle long lines that would exceed token limit by splitting on word boundaries
        let line_tokens = estimate_tokens(line);
        if line_tokens > target_tokens {
            // Split long line into chunks at word boundaries
            for word in line.split_whitespace() {
                // Check if adding this word would exceed limit
                let word_with_space = if current_chunk.is_empty() {
                    word.to_string()
                } else {
                    format!(" {}", word)
                };

                let new_tokens = estimate_tokens(&current_chunk)
                    .saturating_add(estimate_tokens(&word_with_space));

                if new_tokens >= target_tokens && !current_chunk.is_empty() {
                    // Emit current chunk
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
                        previous_chunk_id: chunk_index
                            .checked_sub(1)
                            .map(|prev| format!("{}#{}", doc_id, prev)),
                        next_chunk_id: None,
                        related_chunk_ids: Vec::new(),
                        summary,
                        chunk_level: level.clone(),
                        parent_chunk_id: None,
                        child_chunk_ids: Vec::new(),
                    });

                    chunk_index = chunk_index.saturating_add(1);
                    context_buffer = get_context_tail(&current_chunk, match level {
                        ChunkLevel::Summary => 30,
                        ChunkLevel::Standard => 100,
                        ChunkLevel::Detailed => 200,
                    });
                    current_chunk.clear();

                    // Start new chunk with context
                    if !context_buffer.is_empty() {
                        current_chunk.push_str(&context_buffer);
                        current_chunk.push(' ');
                        context_buffer.clear();
                    }
                }

                // Add word to current chunk
                if !current_chunk.is_empty() && !current_chunk.ends_with(' ') && !current_chunk.ends_with('\n') {
                    current_chunk.push(' ');
                }
                current_chunk.push_str(word);
            }
            current_chunk.push('\n');
        } else {
            current_chunk.push_str(line);
            current_chunk.push('\n');
        }
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
            previous_chunk_id: chunk_index
                .checked_sub(1)
                .map(|prev| format!("{}#{}", doc_id, prev)),
            next_chunk_id: None,
            related_chunk_ids: Vec::new(),
            summary,
            chunk_level: level.clone(),
            parent_chunk_id: None,
            child_chunk_ids: Vec::new(),
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
            chunk_level: level,
            parent_chunk_id: None,
            child_chunk_ids: Vec::new(),
        });
    }

    chunks
}

/// Get trailing context from a chunk for the next chunk's prefix using fold
fn get_context_tail(content: &str, max_tokens: usize) -> String {
    content
        .lines()
        .rev()
        .fold((Vec::new(), 0usize), |(mut lines, count), line| {
            let line_tokens = estimate_tokens(line);
            if count.saturating_add(line_tokens) <= max_tokens {
                lines.push(line);
                (lines, count.saturating_add(line_tokens))
            } else {
                (lines, count)
            }
        })
        .0
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

/// Link chunks together: set next_chunk_id pointers (same level, same doc only)
fn link_chunks(chunks: &mut [Chunk]) {
    for i in 0..chunks.len() {
        if let Some(next_i) = i.checked_add(1) {
            if next_i < chunks.len()
                && chunks[i].doc_id == chunks[next_i].doc_id
                && chunks[i].chunk_level == chunks[next_i].chunk_level
            {
                chunks[i].next_chunk_id = Some(chunks[next_i].chunk_id.clone());
            }
        }
    }
}

/// Estimate token count (simple: ~4 chars = 1 token)
fn estimate_tokens(text: &str) -> usize {
    (text.len() / 4).max(1)
}

/// Create a summary of chunk content using functional composition
fn create_summary(content: &str) -> String {
    // Try to get meaningful fragments (>10 chars)
    let fragments: Vec<&str> = content
        .split(['.', '\n'])
        .filter(|s| s.trim().len() > 10)
        .take(2)
        .collect();

    // If no long fragments, use any non-empty trimmed content
    let summary = if fragments.is_empty() {
        content
            .split(['.', '\n'])
            .map(|s| s.trim())
            .find(|s| !s.is_empty())
            .unwrap_or("")
            .to_string()
    } else {
        fragments.join(". ")
    };

    // Truncate if too long
    let char_count = summary.chars().count();
    if char_count > 200 {
        let truncated: String = summary.chars().take(197).collect();
        format!("{}...", truncated)
    } else {
        summary
    }
}

/// Generate URL-safe slug for document ID using functional composition
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .pipe(|s| s.replace('/', "-"))
        .pipe(|s| s.replace(".md", ""))
        .pipe(|s| s.replace(".mdx", ""))
        .pipe(|s| s.replace('_', "-"))
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

/// Escape special characters in frontmatter string values using functional composition
fn escape_frontmatter(text: &str) -> String {
    text.replace('"', "\\\"")
        .pipe(|s| s.replace('\n', " "))
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
        // Should not panic on emoji
        assert!(summary.contains("test") || summary.contains("emoji"));
    }

    #[test]
    fn test_create_summary_unicode_cjk() {
        let content = "这是一个测试。这是另一个句子。More content after Chinese.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
        // Should handle Chinese characters without panicking
        assert!(summary.len() > 0);
    }

    #[test]
    fn test_create_summary_long_with_special_chars() {
        let long_text = "This is a long document with special characters like em-dashes — and ellipses … and other unicode like 'smart quotes' and naïve. ".repeat(5);
        let summary = create_summary(&long_text);
        assert!(!summary.is_empty());
        // Should be truncated properly without panic
        assert!(summary.len() <= 210); // 197 chars + "..."
    }

    #[test]
    fn test_escape_frontmatter_unicode() {
        let text = "Unicode text with 🎉 emoji and é accent";
        let escaped = escape_frontmatter(text);
        assert!(escaped.contains("emoji"));
        assert!(escaped.contains("é"));
    }

    #[test]
    fn test_chunk_type_detection() {
        // Code: requires > 5 code blocks (i.e., > 10 triple backticks)
        let code = "```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```";
        assert_eq!(detect_chunk_type(code), "code");

        let table = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        assert_eq!(detect_chunk_type(table), "table");

        let prose = "This is just regular prose content with no tables or code blocks.";
        assert_eq!(detect_chunk_type(prose), "prose");
    }

    #[test]
    fn test_slugify_special_chars() {
        let text = "Path/To/File.md with Special_Chars";
        let slug = slugify(text);
        assert!(!slug.contains('/'));
        assert!(!slug.contains('.'));
        assert!(!slug.contains('_'));
        assert!(slug.contains('-'));
    }

    #[test]
    fn test_estimate_tokens() {
        let text = "This is a test";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        // Roughly 4 chars per token
        assert!(tokens >= 3 && tokens <= 4);
    }

    #[test]
    fn test_empty_chunk_content() {
        let chunks = create_chunks_at_level("", "doc1", "Empty Doc", ChunkLevel::Standard);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "");
        assert_eq!(chunks[0].summary, "");
    }
}
