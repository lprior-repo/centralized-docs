fn main() {
    let score1 = score_document_simple("Rust Rust Rust Rust Rust", "Rust Rust", "rust", 1.0);
    let score2 = score_document_simple("Rust", "Rust", "rust", 10000.0);
    println!("Score1: {}", score1);
    println!("Score2: {}", score2);
}

pub fn score_document_simple(title: &str, summary: &str, query: &str, word_count: f32) -> f32 {
    let k1 = 1.2;
    let b = 0.75;

    let document = format!("{title} {summary}");
    
    // Strip basic punctuation before splitting whitespace
    let clean_doc = document.replace(&[',', '.', '?', '!', ';', '(', ')', '[', ']', '{', '}', '"', '\''][..], "");
    let doc_words: Vec<&str> = clean_doc.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    let avg_doc_length = word_count.max(1.0);

    let clean_query = query.replace(&[',', '.', '?', '!', ';', '(', ')', '[', ']', '{', '}', '"', '\''][..], "");
    clean_query
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
            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_length / avg_doc_length));
            idf * (numerator / denominator.max(0.0001))
        })
        .sum()
}
