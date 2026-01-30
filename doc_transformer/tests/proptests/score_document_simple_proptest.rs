//! Property-based tests for `score_document_simple`
//!
//! These tests verify fundamental properties of the scoring function using
//! proptest to generate random inputs. This helps catch edge cases that
//! unit tests might miss.

#![allow(clippy::uninlined_format_args)] // Test code with clearer format placeholders
#![allow(clippy::useless_format)] // Test code may use format! for clarity

use proptest::prelude::*;

fn score_document_simple(title: &str, summary: &str, query: &str, word_count: f32) -> f32 {
    const BM25_K1: f32 = 1.2;
    const BM25_B: f32 = 0.75;

    let document = format!("{title} {summary}");
    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    // Avoid division by zero
    let avg_doc_length = word_count.max(1.0);

    query
        .split_whitespace()
        .map(|term| {
            let term_lower = term.to_lowercase();
            doc_words
                .iter()
                .filter(|w| w.to_lowercase() == term_lower)
                .count() as f32
        })
        .filter(|&tf| tf > 0.0)
        .map(|tf| {
            let idf = (10.0_f32).ln();
            let numerator = tf * (BM25_K1 + 1.0);
            let denominator =
                tf + BM25_K1 * (1.0 - BM25_B + BM25_B * (doc_length / avg_doc_length));
            idf * (numerator / denominator.max(0.0001))
        })
        .sum()
}

proptest! {
    /// Property 1: Bounds - Score must always be finite and non-negative
    ///
    /// The score should never be NaN, infinite, or negative for any valid input.
    #[test]
    fn prop_score_is_finite_and_non_negative(
        title in "[a-zA-Z0-9 ]{0,500}",
        summary in "[a-zA-Z0-9 ]{0,500}",
        query in "[a-zA-Z0-9 ]{0,100}",
        word_count in 0.0f32..10000.0f32
    ) {
        let score = score_document_simple(&title, &summary, &query, word_count);

        // Score must be finite (not NaN or infinite)
        prop_assert!(score.is_finite(), "Score must be finite: {}", score);

        // Score must be non-negative
        prop_assert!(score >= 0.0, "Score must be non-negative: {}", score);
    }

    /// Property 2: Empty query returns zero
    ///
    /// When the query is empty or contains no terms, the score should be zero.
    #[test]
    fn prop_empty_query_returns_zero(
        title in "[a-zA-Z0-9 ]{0,500}",
        summary in "[a-zA-Z0-9 ]{0,500}",
        word_count in 0.0f32..10000.0f32
    ) {
        // Test various forms of empty queries
        let score1 = score_document_simple(&title, &summary, "", word_count);
        let score2 = score_document_simple(&title, &summary, "   ", word_count);
        let score3 = score_document_simple(&title, &summary, "\t\t", word_count);

        prop_assert_eq!(score1, 0.0, "Empty string query should return 0");
        prop_assert_eq!(score2, 0.0, "Whitespace query should return 0");
        prop_assert_eq!(score3, 0.0, "Tab query should return 0");
    }

    /// Property 3: Determinism - Same inputs produce same outputs
    ///
    /// Calling the function twice with identical inputs must produce identical results.
    #[test]
    fn prop_determinism(
        title in "[a-zA-Z0-9 ]{0,500}",
        summary in "[a-zA-Z0-9 ]{0,500}",
        query in "[a-zA-Z0-9 ]{0,100}",
        word_count in 0.0f32..10000.0f32
    ) {
        let score1 = score_document_simple(&title, &summary, &query, word_count);
        let score2 = score_document_simple(&title, &summary, &query, word_count);

        prop_assert_eq!(score1, score2, "Function must be deterministic");
    }

    /// Property 4: Zero word count handling
    ///
    /// Zero or near-zero word counts should not cause division by zero or NaN.
    #[test]
    fn prop_zero_word_count_handling(
        title in "[a-zA-Z0-9 ]{0,500}",
        summary in "[a-zA-Z0-9 ]{0,500}",
        query in "[a-zA-Z0-9 ]{0,100}"
    ) {
        // Test with exact zero
        let score_zero = score_document_simple(&title, &summary, &query, 0.0);
        prop_assert!(score_zero.is_finite(), "Zero word_count must produce finite score");

        // Test with very small values
        let score_small = score_document_simple(&title, &summary, &query, 0.001);
        prop_assert!(score_small.is_finite(), "Tiny word_count must produce finite score");

        // Test with very large values
        let score_large = score_document_simple(&title, &summary, &query, 1000000.0);
        prop_assert!(score_large.is_finite(), "Large word_count must produce finite score");
    }

    /// Property 5: Case insensitivity
    ///
    /// Query term case should not affect scoring.
    #[test]
    fn prop_case_insensitivity(
        title in "[a-zA-Z ]{10,100}",
        summary in "[a-zA-Z ]{10,100}",
        query in "[a-zA-Z]{3,20}",
        word_count in 10.0f32..1000.0f32
    ) {
        // Score with lowercase query
        let score_lower = score_document_simple(&title, &summary, &query.to_lowercase(), word_count);

        // Score with uppercase query
        let score_upper = score_document_simple(&title, &summary, &query.to_uppercase(), word_count);

        // Score with mixed case query
        let score_mixed = score_document_simple(&title, &summary, &query, word_count);

        // All should be equal (within floating point tolerance)
        let tolerance = 0.0001;
        prop_assert!(
            (score_lower - score_upper).abs() < tolerance,
            "Lower and upper case scores differ: {} vs {}",
            score_lower,
            score_upper
        );
        prop_assert!(
            (score_lower - score_mixed).abs() < tolerance,
            "Lower and mixed case scores differ: {} vs {}",
            score_lower,
            score_mixed
        );
    }

    /// Property 6: Monotonicity - More matches should generally mean higher scores
    ///
    /// If document A contains all query terms that document B contains, plus additional ones,
    /// A should score at least as high as B (assuming equal document length).
    #[test]
    fn prop_monotonicity_more_terms(
        base_doc in "[a-zA-Z ]{50,200}",
        extra_term in "[a-zA-Z]{3,10}",
        word_count in 100.0f32..500.0f32
    ) {
        // Create two documents where one has the extra term
        let doc_short = base_doc.clone();
        let doc_long = format!("{} {}", base_doc, extra_term);

        // Query for the extra term
        let query = extra_term.clone();

        let score_short = score_document_simple(&doc_short, "", &query, word_count);
        let score_long = score_document_simple(&doc_long, "", &query, word_count);

        // Document with the term should score higher (or equal if term wasn't in short doc)
        prop_assert!(
            score_long >= score_short,
            "Document with term should score >= same doc without term: {} >= {}",
            score_long,
            score_short
        );
    }

    /// Property 7: Whitespace-only documents
    ///
    /// Documents that are all whitespace should return zero for any query.
    #[test]
    fn prop_whitespace_documents(
        query in "[a-zA-Z0-9 ]{1,50}",
        word_count in 0.0f32..1000.0f32
    ) {
        let score1 = score_document_simple("   ", "\t\t\n", &query, word_count);
        let score2 = score_document_simple("", "", &query, word_count);

        prop_assert_eq!(score1, 0.0, "Whitespace-only document should score 0");
        prop_assert_eq!(score2, 0.0, "Empty document should score 0");
    }

    /// Property 8: Non-matching query returns zero
    ///
    /// If the query contains no terms present in the document, score should be zero.
    #[test]
    fn prop_non_matching_query_zero(
        title in "[a-z]{10,50}",
        summary in "[a-z]{10,50}",
        word_count in 10.0f32..1000.0f32
    ) {
        // Use a query with completely different letters
        let query = "xyzqwerty123";

        let score = score_document_simple(&title, &summary, query, word_count);

        // Since the title/summary only contains a-z and query has xyz/qwerty/123,
        // there should be no match
        prop_assert_eq!(score, 0.0, "Non-matching query should score 0");
    }

    /// Property 9: Order independence
    ///
    /// The order of terms in the query should not affect the score.
    #[test]
    fn prop_query_order_independence(
        title in "[a-z]{10,50}",
        summary in "[a-z]{10,50}",
        word_count in 10.0f32..1000.0f32
    ) {
        // Pick two terms from the document
        let combined = format!("{} {}", title, summary);
        let words: Vec<&str> = combined.split_whitespace().collect();

        if words.len() >= 2 {
            let term1 = words[0];
            let term2 = words[1];

            // Try both orders
            let query1 = format!("{} {}", term1, term2);
            let query2 = format!("{} {}", term2, term1);

            let score1 = score_document_simple(&title, &summary, &query1, word_count);
            let score2 = score_document_simple(&title, &summary, &query2, word_count);

            prop_assert_eq!(score1, score2, "Query term order should not affect score");
        }
        // If document has fewer than 2 words, test trivially passes
    }

    /// Property 10: Term frequency increases score
    ///
    /// All else being equal, a document with more occurrences of a term
    /// should score higher than one with fewer occurrences.
    #[test]
    fn prop_term_frequency_monotonicity(
        term in "[a-z]{3,8}",
        _filler in "[bcdfghjklmnpqrstvwxyz]{3,8}",
        word_count in 50.0f32..500.0f32
    ) {
        // Create documents with different term frequencies
        let doc_once = format!("{}", term);
        let doc_twice = format!("{} {}", term, term);
        let doc_many = format!("{} {} {} {} {}", term, term, term, term, term);

        let query = term.clone();

        let score_once = score_document_simple(&doc_once, "", &query, word_count);
        let score_twice = score_document_simple(&doc_twice, "", &query, word_count);
        let score_many = score_document_simple(&doc_many, "", &query, word_count);

        // More occurrences should yield higher (or equal) scores
        prop_assert!(
            score_twice >= score_once,
            "Two occurrences should score >= one: {} >= {}",
            score_twice,
            score_once
        );
        prop_assert!(
            score_many >= score_twice,
            "Many occurrences should score >= two: {} >= {}",
            score_many,
            score_twice
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// Basic unit test to ensure the function works correctly for a simple case.
    #[test]
    fn test_basic_functionality() {
        let score = score_document_simple("rust programming", "learn rust", "rust", 100.0);
        assert!(score > 0.0, "Matching term should produce positive score");
    }

    /// Test that completely unrelated terms return zero.
    #[test]
    fn test_unrelated_terms() {
        let score = score_document_simple(
            "rust programming",
            "systems language",
            "python django",
            100.0,
        );
        assert_eq!(score, 0.0, "No matching terms should produce zero score");
    }

    /// Test special characters are handled gracefully.
    #[test]
    fn test_special_characters() {
        let score = score_document_simple("test", "content", "test!@#$%", 100.0);
        // Function should not panic on special characters
        assert!(
            score.is_finite(),
            "Special characters should not cause non-finite score"
        );
    }
}
