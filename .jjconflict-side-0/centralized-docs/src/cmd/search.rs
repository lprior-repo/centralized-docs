use crate::validate;
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

pub const SEARCH_JSON_ALREADY_EMITTED_PREFIX: &str = "__SEARCH_JSON_ALREADY_EMITTED__";

/// Run the search command using Tantivy (with fallback to BM25)
///
/// Strategy:
/// 1. Try to use Tantivy index if available (faster, better features)
/// 2. Fall back to INDEX.json + manual BM25 scoring if index missing
/// 3. Display results with scores and metadata
///
/// Note: Returns non-zero exit code if advanced search fails, even if fallback succeeds
#[derive(Debug, Serialize)]
struct CliSearchResult {
    rank: usize,
    category: String,
    title: String,
    path: String,
    summary: String,
    score: f32,
    backend: String,
}

fn emit_search_output(
    query: &str,
    backend: &str,
    results: &[CliSearchResult],
    limit: usize,
    json_output: bool,
    status: &str,
    advanced_search_failed: bool,
) -> Result<()> {
    if json_output {
        let output = serde_json::json!({
            "status": status,
            "query": query,
            "backend": backend,
            "advanced_search_failed": advanced_search_failed,
            "requested_limit": limit,
            "result_count": results.len(),
            "results": results,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("\n{}", "=".repeat(70));
        println!("DOC_TRANSFORMER SEARCH - Tantivy + BM25");
        println!("{}\n", "=".repeat(70));
        println!("Query: \"{query}\"");
        println!("Using {backend}\n");

        if results.is_empty() {
            println!("No results found for \"{query}\"");
        } else {
            println!("Results:\n");
            results.iter().for_each(|result| {
                println!(
                    "{}. [{}] {} (score: {:.2})",
                    result.rank, result.category, result.title, result.score
                );
                println!("   Path: {}", result.path);
                println!("   {}\n", result.summary);
            });

            println!("{}", "=".repeat(70));
            println!(
                "Showing {} of {} results",
                results.len().min(limit),
                results.len()
            );
            println!("{}\n", "=".repeat(70));
        }
    }

    Ok(())
}

pub fn run_search(
    query: &str,
    index_dir: &Path,
    limit: usize,
    _use_color: bool,
    json_output: bool,
) -> Result<()> {
    const MAX_QUERY_WORDS: usize = 100;

    // Validate query using centralized validation
    let query = validate::validate_query(query).map_err(|e| anyhow::anyhow!("{e}"))?;

    // Validate word count (additional constraint beyond basic validation)
    let word_count = query.split_whitespace().count();
    if word_count > MAX_QUERY_WORDS {
        anyhow::bail!("Query has too many terms ({word_count} words, max {MAX_QUERY_WORDS})");
    }

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    let index = match doc_transformer::search::open_existing_index(index_dir)? {
        Some(index) => index,
        None => {
            // Rebuild from INDEX.json will propagate the actual serde error if it's corrupted
            doc_transformer::search::rebuild_index_from_json(index_dir)?
        }
    };

    let results = doc_transformer::search::search_index(&index, query, limit)?;

    let cli_results: Vec<CliSearchResult> = results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let summary_short = if result.summary.chars().count() > 80 {
                let truncated: String = result.summary.chars().take(77).collect();
                format!("{truncated}...")
            } else {
                result.summary.clone()
            };

            CliSearchResult {
                rank: i.saturating_add(1),
                category: result.category.clone(),
                title: result.title.clone(),
                path: result.path.clone(),
                summary: summary_short,
                score: result.score.value(),
                backend: "tantivy".to_string(),
            }
        })
        .collect();

    let status = if cli_results.is_empty() {
        "no_results"
    } else {
        "ok"
    };

    emit_search_output(
        query,
        "tantivy",
        &cli_results,
        limit,
        json_output,
        status,
        false,
    )?;

    if cli_results.is_empty() && json_output {
        anyhow::bail!("{SEARCH_JSON_ALREADY_EMITTED_PREFIX}:no_results");
    }

    if cli_results.is_empty() {
        anyhow::bail!("No results found for '{query}'");
    }

    Ok(())
}
