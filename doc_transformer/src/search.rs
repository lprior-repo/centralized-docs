use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkSearchResult {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub heading: Option<String>,
    pub chunk_level: String,
    pub score: f32,
    pub snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSearchResult {
    pub id: String,
    pub title: String,
    pub category: String,
    pub score: f32,
    pub summary: String,
}

/// Calculate BM25 score for a document
///
/// BM25 is a ranking function that scores documents based on term frequency (TF)
/// and inverse document frequency (IDF). The formula is:
///
/// score = Σ IDF(qi) * (f(qi, D) * (k1 + 1)) / (f(qi, D) + k1 * (1 - b + b * |D| / avgdl))
///
/// where:
/// - qi: query term i
/// - f(qi, D): frequency of qi in document D
/// - |D|: length of document D
/// - avgdl: average document length
/// - k1, b: tuning parameters (typically k1=1.5, b=0.75)
pub fn bm25_score(
    query: &str,
    document_text: &str,
    avg_doc_length: f32,
    k1: f32,
    b: f32,
) -> f32 {
    // Guard against zero or negative avg_doc_length to prevent division by zero
    if avg_doc_length <= 0.0 {
        return 0.0;
    }

    let query_lower = query.to_lowercase();
    let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
    let doc_lower = document_text.to_lowercase();
    let doc_length = doc_lower.split_whitespace().count() as f32;

    let mut score = 0.0;

    for term in query_terms {
        // Count term frequency in document
        let tf = doc_lower.matches(term).count() as f32;

        if tf > 0.0 {
            // Simplified IDF (we don't have corpus stats, so use a constant)
            let idf = 1.0;

            // BM25 formula
            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_length / avg_doc_length));

            score += idf * (numerator / denominator);
        }
    }

    score
}

/// Search within document summaries
pub fn search_documents(
    index_path: &Path,
    query: &str,
    limit: usize,
) -> Result<Vec<DocumentSearchResult>> {
    let index_content = fs::read_to_string(index_path)
        .with_context(|| format!("Failed to read index file: {}", index_path.display()))?;

    let index: Value = serde_json::from_str(&index_content)
        .context("Failed to parse INDEX.json")?;

    let documents = index["documents"]
        .as_array()
        .context("INDEX.json missing 'documents' array")?;

    if documents.is_empty() {
        return Ok(Vec::new());
    }

    // Calculate average document length
    let total_length: usize = documents
        .iter()
        .filter_map(|doc| doc["summary"].as_str())
        .map(|s| s.split_whitespace().count())
        .sum();
    let avg_doc_length = if documents.is_empty() {
        100.0
    } else {
        // Use max(1.0) to prevent division by zero in bm25_score
        (total_length as f32 / documents.len() as f32).max(1.0)
    };

    // Score each document
    let mut results: Vec<DocumentSearchResult> = documents
        .iter()
        .filter_map(|doc| {
            let summary = doc["summary"].as_str()?;
            let title = doc["title"].as_str()?;

            // Search in title and summary
            let searchable_text = format!("{} {}", title, summary);
            let score = bm25_score(query, &searchable_text, avg_doc_length, 1.5, 0.75);

            if score > 0.0 {
                Some(DocumentSearchResult {
                    id: doc["id"].as_str()?.to_string(),
                    title: title.to_string(),
                    category: doc["category"].as_str()?.to_string(),
                    score,
                    summary: summary.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Take top N results
    results.truncate(limit);

    Ok(results)
}

/// Search within chunk content
pub fn search_chunks(
    index_path: &Path,
    query: &str,
    limit: usize,
) -> Result<Vec<ChunkSearchResult>> {
    let index_content = fs::read_to_string(index_path)
        .with_context(|| format!("Failed to read index file: {}", index_path.display()))?;

    let index: Value = serde_json::from_str(&index_content)
        .context("Failed to parse INDEX.json")?;

    let chunks = index["chunks"]
        .as_array()
        .context("INDEX.json missing 'chunks' array")?;

    if chunks.is_empty() {
        return Ok(Vec::new());
    }

    // Load chunk content from files
    let output_dir = index_path.parent()
        .context("Failed to get index directory")?;

    let mut chunks_with_content: Vec<(Value, String)> = Vec::new();

    for chunk in chunks {
        let chunk_path = chunk["path"].as_str()
            .context("Chunk missing 'path' field")?;
        let full_path = output_dir.join(chunk_path);

        if let Ok(content) = fs::read_to_string(&full_path) {
            // Extract actual content (skip frontmatter)
            let content_lines: Vec<&str> = content.lines().collect();
            let mut actual_content = String::new();
            let mut in_frontmatter = false;
            let mut frontmatter_closed = false;

            for line in content_lines {
                if line.trim() == "---" {
                    if !in_frontmatter {
                        in_frontmatter = true;
                    } else {
                        frontmatter_closed = true;
                    }
                    continue;
                }

                if frontmatter_closed || !in_frontmatter {
                    actual_content.push_str(line);
                    actual_content.push('\n');
                }
            }

            chunks_with_content.push((chunk.clone(), actual_content));
        }
    }

    if chunks_with_content.is_empty() {
        return Ok(Vec::new());
    }

    // Calculate average chunk length
    let total_length: usize = chunks_with_content
        .iter()
        .map(|(_, content)| content.split_whitespace().count())
        .sum();
    // Use max(1.0) to prevent division by zero in bm25_score
    let avg_chunk_length = (total_length as f32 / chunks_with_content.len() as f32).max(1.0);

    // Score each chunk
    let mut results: Vec<ChunkSearchResult> = chunks_with_content
        .iter()
        .filter_map(|(chunk, content)| {
            let heading = chunk["heading"].as_str().unwrap_or("").to_string();

            // Search in heading and content
            let searchable_text = format!("{} {}", heading, content);
            let score = bm25_score(query, &searchable_text, avg_chunk_length, 1.5, 0.75);

            if score > 0.0 {
                // Create snippet (first 100 bytes of content)
                let snippet = if content.len() > 100 {
                    format!("{}...", safe_truncate(content, 100).trim())
                } else {
                    content.trim().to_string()
                };

                // Determine chunk level
                let chunk_type = chunk["chunk_type"].as_str().unwrap_or("unknown");
                let chunk_level = match chunk_type {
                    "h2_section" => "Section".to_string(),
                    "h3_section" => "Subsection".to_string(),
                    "introduction" => "Introduction".to_string(),
                    _ => "Chunk".to_string(),
                };

                Some(ChunkSearchResult {
                    chunk_id: chunk["chunk_id"].as_str()?.to_string(),
                    doc_id: chunk["doc_id"].as_str()?.to_string(),
                    doc_title: chunk["doc_title"].as_str()?.to_string(),
                    heading: if heading.is_empty() { None } else { Some(heading) },
                    chunk_level,
                    score,
                    snippet,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Take top N results
    results.truncate(limit);

    Ok(results)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bm25_score_basic() {
        let query = "rust programming";
        let document = "Rust is a systems programming language that runs blazingly fast.";
        let avg_doc_length = 100.0;

        let score = bm25_score(query, document, avg_doc_length, 1.5, 0.75);

        assert!(score > 0.0, "Score should be positive when terms match");
    }

    #[test]
    fn test_bm25_score_no_match() {
        let query = "python django";
        let document = "Rust is a systems programming language that runs blazingly fast.";
        let avg_doc_length = 100.0;

        let score = bm25_score(query, document, avg_doc_length, 1.5, 0.75);

        assert_eq!(score, 0.0, "Score should be zero when no terms match");
    }

    #[test]
    fn test_bm25_score_partial_match() {
        let query = "rust python";
        let document = "Rust is a systems programming language that runs blazingly fast.";
        let avg_doc_length = 100.0;

        let score = bm25_score(query, document, avg_doc_length, 1.5, 0.75);

        assert!(score > 0.0, "Score should be positive when some terms match");
    }

    #[test]
    fn test_bm25_score_case_insensitive() {
        let query = "RUST";
        let document = "rust is a systems programming language";
        let avg_doc_length = 100.0;

        let score = bm25_score(query, document, avg_doc_length, 1.5, 0.75);

        assert!(score > 0.0, "Search should be case insensitive");
    }

    #[test]
    fn test_bm25_score_multiple_occurrences() {
        let query = "rust";
        let doc1 = "rust rust rust rust";  // High frequency
        let doc2 = "rust programming language";  // Low frequency
        let avg_doc_length = 10.0;

        let score1 = bm25_score(query, doc1, avg_doc_length, 1.5, 0.75);
        let score2 = bm25_score(query, doc2, avg_doc_length, 1.5, 0.75);

        assert!(score1 > score2, "Higher frequency should yield higher score");
    }

    #[test]
    fn test_bm25_score_zero_avg_length() {
        let query = "test content";
        let document = "test content here";

        // This should not panic or produce inf/nan
        let score = bm25_score(query, document, 0.0, 1.5, 0.75);

        assert_eq!(score, 0.0, "Score should be 0.0 when avg_doc_length is 0.0");
        assert!(score.is_finite(), "Score must be finite");
    }

    #[test]
    fn test_bm25_score_negative_avg_length() {
        let query = "test";
        let document = "test content";

        // Negative avg_doc_length should be treated same as zero
        let score = bm25_score(query, document, -1.0, 1.5, 0.75);

        assert_eq!(score, 0.0, "Score should be 0.0 when avg_doc_length is negative");
        assert!(score.is_finite(), "Score must be finite");
    }

    #[test]
    fn test_bm25_score_very_small_avg_length() {
        let query = "test";
        let document = "test content here";

        // Very small avg_doc_length should still work
        let score = bm25_score(query, document, 0.001, 1.5, 0.75);

        assert!(score > 0.0, "Score should be positive with very small avg_doc_length");
        assert!(score.is_finite(), "Score must be finite");
    }

    #[test]
    fn test_bm25_score_empty_document() {
        let query = "test";
        let document = "";

        // Empty document with zero avg_doc_length
        let score = bm25_score(query, document, 0.0, 1.5, 0.75);

        assert_eq!(score, 0.0, "Empty document should have zero score");
        assert!(score.is_finite(), "Score must be finite");
    }

    #[test]
    fn test_bm25_score_all_edge_cases_together() {
        // Test multiple edge cases don't cause issues
        let test_cases = vec![
            ("", "", 0.0),
            ("test", "", 0.0),
            ("", "content", 0.0),
            ("test", "test", 0.0),
            ("test", "test", -5.0),
        ];

        for (query, doc, avg_len) in test_cases {
            let score = bm25_score(query, doc, avg_len, 1.5, 0.75);
            assert!(score.is_finite(), "Score must always be finite: query={}, doc={}, avg_len={}", query, doc, avg_len);
            assert!(score >= 0.0, "Score must be non-negative: query={}, doc={}, avg_len={}", query, doc, avg_len);
        }
    }
}
