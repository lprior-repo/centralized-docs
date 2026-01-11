//! doc_transformer v5.0 - AI-Optimized Documentation Indexer
//!
//! Transform raw documentation into AI-friendly knowledge structures with:
//! - Web scraping via spider-rs
//! - Semantic chunking with context prefixes
//! - Knowledge DAG with relationship detection
//! - llms.txt generation for AI entry points

// Strict functional programming constraints
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]

mod analyze;
mod assign;
mod chunk;
mod config;
mod discover;
mod filter;
mod graph;
mod highlight;
mod index;
mod llms;
mod scrape;
mod transform;
mod validate;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "doc_transformer")]
#[command(version = "5.0")]
#[command(about = "Transform documentation into AI-optimized knowledge structures")]
#[command(long_about = "
doc_transformer v5.0 - The AI-Optimized Documentation Indexer

USAGE:
  doc_transformer scrape <URL> --output <DIR>    # Scrape a documentation site
  doc_transformer index <SOURCE> --output <DIR>  # Index local markdown files
  doc_transformer ingest <URL> --output <DIR>    # Scrape + index in one step
  doc_transformer <SOURCE> <OUTPUT>              # Legacy mode (same as index)

OUTPUT:
  llms.txt      - AI entry point (read this first)
  INDEX.json    - Machine-readable index with chunks and DAG
  COMPASS.md    - Human-readable navigation
  docs/         - Transformed documents with frontmatter
  chunks/       - Semantic chunks with context prefix
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Legacy: Source directory (use 'index' subcommand instead)
    #[arg(value_name = "SOURCE", required = false)]
    source_dir: Option<PathBuf>,

    /// Legacy: Output directory (use 'index' subcommand instead)
    #[arg(value_name = "OUTPUT", required = false)]
    output_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search indexed documentation using BM25
    Search {
        /// Query string to search for
        #[arg(value_name = "QUERY")]
        query: String,

        /// Directory containing INDEX.json
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        index_dir: PathBuf,

        /// Maximum number of results to return
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,

        /// Disable colored output
        #[arg(long)]
        no_color: bool,
    },

    /// Scrape a documentation website to local markdown files
    Scrape {
        /// URL of the documentation site to scrape
        #[arg(value_name = "URL")]
        url: String,

        /// Output directory for scraped content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Use sitemap.xml to discover pages (default: true)
        #[arg(long, default_value = "true")]
        sitemap: bool,

        /// Regex pattern to filter URLs by path
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,

        /// Delay between requests in milliseconds
        #[arg(short, long, default_value = "250")]
        delay: u64,

        /// Filter pages by BM25 relevance to query
        #[arg(short, long, value_name = "QUERY")]
        query: Option<String>,

        /// Minimum BM25 score to keep a page (default: 0.1)
        #[arg(long, default_value = "0.1")]
        threshold: f32,
    },

    /// Index local markdown files into AI-optimized structure
    Index {
        /// Source directory containing markdown files
        #[arg(value_name = "SOURCE")]
        source: PathBuf,

        /// Output directory for indexed content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Generate llms.txt entry point files
        #[arg(long, default_value = "true")]
        llms_txt: bool,

        /// Project name for llms.txt header
        #[arg(long, default_value = "Documentation")]
        project_name: String,

        /// Project description for llms.txt
        #[arg(long, default_value = "AI-optimized documentation index")]
        project_desc: String,

        /// Path to category rules config file
        #[arg(long, value_name = "FILE")]
        category_config: Option<PathBuf>,
    },

    /// Scrape and index in one step
    Ingest {
        /// URL of the documentation site
        #[arg(value_name = "URL")]
        url: String,

        /// Output directory for final indexed content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Regex pattern to filter URLs by path
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,

        /// Delay between requests in milliseconds
        #[arg(short, long, default_value = "250")]
        delay: u64,

        /// Filter pages by BM25 relevance to query
        #[arg(short, long, value_name = "QUERY")]
        query: Option<String>,

        /// Minimum BM25 score to keep a page (default: 0.1)
        #[arg(long, default_value = "0.1")]
        threshold: f32,

        /// Project name for llms.txt header
        #[arg(long)]
        project_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Search {
            query,
            index_dir,
            limit,
            no_color,
        }) => {
            run_search(&query, &index_dir, limit, !no_color)
        }

        Some(Commands::Scrape {
            url,
            output,
            sitemap,
            filter,
            delay,
            query,
            threshold,
        }) => {
            run_scrape(&url, &output, sitemap, filter, delay, query.as_deref(), threshold).await
        }

        Some(Commands::Index {
            source,
            output,
            llms_txt,
            project_name,
            project_desc,
            category_config,
        }) => {
            run_index(&source, &output, category_config.as_deref(), llms_txt, &project_name, &project_desc)
        }

        Some(Commands::Ingest {
            url,
            output,
            filter,
            delay,
            query,
            threshold,
            project_name,
        }) => {
            run_ingest(&url, &output, filter, delay, query, threshold, project_name).await
        }

        None => {
            // Legacy mode: two positional arguments
            if let (Some(source), Some(output)) = (cli.source_dir, cli.output_dir) {
                run_index(&source, &output, None, true, "Documentation", "AI-optimized documentation index")
            } else {
                eprintln!("Usage: doc_transformer <SOURCE> <OUTPUT>");
                eprintln!("   or: doc_transformer scrape <URL> --output <DIR>");
                eprintln!("   or: doc_transformer index <SOURCE> --output <DIR>");
                eprintln!("   or: doc_transformer ingest <URL> --output <DIR>");
                eprintln!("\nRun 'doc_transformer --help' for more information.");
                std::process::exit(1);
            }
        }
    }
}

/// Run the scrape command
async fn run_scrape(
    url: &str,
    output: &PathBuf,
    use_sitemap: bool,
    filter: Option<String>,
    delay: u64,
    query: Option<&str>,
    threshold: f32,
) -> Result<()> {
    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 - SCRAPE");
    println!("{}\n", "=".repeat(70));

    println!("[SCRAPE] Target: {}", url);
    println!("  Options: sitemap={}, delay={}ms", use_sitemap, delay);
    if let Some(ref f) = filter {
        println!("  Filter: {}", f);
    }
    println!();

    let config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        use_sitemap,
        path_filter: filter,
        delay_ms: delay,
        ..Default::default()
    };

    println!("[SCRAPE] Starting crawl...");
    let mut result = scrape::scrape_site(&config).await?;

    println!("  Discovered: {} URLs", result.total_urls);
    println!("  Scraped: {} pages", result.success_count);
    println!("  Errors: {}", result.error_count);

    // Apply BM25 filtering if query is provided
    if let Some(q) = query {
        let (kept_pages, filtered_count) = scrape::filter_pages_by_relevance(result.pages, q, threshold);

        if kept_pages.is_empty() {
            println!("\n  WARNING: All pages filtered out by query.");
            println!("  Consider lowering the --threshold value.");
            return Ok(());
        }

        println!("  Filtered by relevance: {} pages removed", filtered_count);
        println!("  Kept: {} pages matching \"{}\"", kept_pages.len(), q);

        result.pages = kept_pages;
        result.success_count = result.pages.len();
    }

    if !result.errors.is_empty() {
        println!("\n  Error details:");
        for (url, err) in result.errors.iter().take(5) {
            println!("    - {}: {}", url, err);
        }
        if result.errors.len() > 5 {
            println!("    ... and {} more", result.errors.len().saturating_sub(5));
        }
    }

    println!("\n[WRITE] Saving to {}", output.display());
    std::fs::create_dir_all(output)?;
    scrape::write_scraped_pages(&result, output)?;

    println!("\n{}", "=".repeat(70));
    println!("SCRAPE COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Output:  {}", output.display());
    println!("Pages:   {} scraped", result.success_count);
    println!("Files:   .scrape/*.md + manifest.json");
    println!("{}\n", "=".repeat(70));

    Ok(())
}

/// Run the index command (main pipeline)
fn run_index(
    source: &Path,
    output: &Path,
    category_config: Option<&Path>,
    generate_llms: bool,
    project_name: &str,
    project_desc: &str,
) -> Result<()> {
    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 (Knowledge DAG + llms.txt)");
    println!("{}\n", "=".repeat(70));

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (files, _discover_manifest) = discover::discover_files(source)?;
    println!("  Found {} files\n", files.len());

    // STEP 2: ANALYZE
    println!("[STEP 2] ANALYZE");
    let analyses = analyze::analyze_files(&files, source, category_config)?;
    let categories = analyze::count_categories(&analyses);
    println!("  Processed {} files", analyses.len());
    println!(
        "  Categories: ref={} concept={} tutorial={} ops={} meta={}\n",
        categories.get("ref").unwrap_or(&0),
        categories.get("concept").unwrap_or(&0),
        categories.get("tutorial").unwrap_or(&0),
        categories.get("ops").unwrap_or(&0),
        categories.get("meta").unwrap_or(&0)
    );

    // STEP 3: ASSIGN IDs
    println!("[STEP 3] ASSIGN IDs");
    let (analyses, link_map) = assign::assign_ids(analyses);
    println!("  Generated {} IDs\n", analyses.len());

    // STEP 4: TRANSFORM
    println!("[STEP 4] TRANSFORM");
    let transform_result = transform::transform_all(&analyses, &link_map, output)?;
    println!(
        "  {}/{} files ({} errors)\n",
        transform_result.success_count, transform_result.total_count, transform_result.error_count
    );

    // STEP 5: CHUNK (Hierarchical)
    println!("[STEP 5] CHUNK");
    let chunks_result = chunk::chunk_all(&analyses, output)?;
    println!(
        "  Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!(
        "  Hierarchical: {} summary, {} standard, {} detailed",
        chunks_result.summary_chunks, chunks_result.standard_chunks, chunks_result.detailed_chunks
    );
    println!("  ~512 tokens/chunk with contextual prefixes\n");

    // STEP 6: INDEX + GRAPH
    println!("[STEP 6] INDEX + GRAPH");
    index::build_and_write_index(&analyses, &link_map, &chunks_result, output)?;
    index::build_and_write_compass(&analyses, &link_map, output)?;
    println!("  Created INDEX.json and COMPASS.md\n");

    // STEP 7: LLMS.TXT + AGENTS.MD
    if generate_llms {
        println!("[STEP 7] LLMS.TXT + AGENTS.MD");
        let llms_config = llms::LlmsConfig {
            project_name: project_name.to_string(),
            project_description: project_desc.to_string(),
            generate_full: true,
            ..Default::default()
        };
        llms::generate_llms_txt(&analyses, &link_map, &llms_config, output)?;
        llms::generate_agents_md(&analyses, &link_map, &llms_config, output)?;
        if llms_config.generate_full {
            llms::generate_llms_full_txt(&analyses, &link_map, output)?;
            println!("  Created llms.txt, llms-full.txt, and AGENTS.md\n");
        } else {
            println!("  Created llms.txt and AGENTS.md\n");
        }
    }

    // STEP 8: VALIDATE
    println!("[STEP 8] VALIDATE");
    let validation_result = validate::validate_all(output)?;
    println!(
        "  {}/{} files passed ({} errors, {} warnings)\n",
        validation_result.files_passed,
        validation_result.files_checked,
        validation_result.total_errors,
        validation_result.total_warnings
    );

    // FINAL SUMMARY
    println!("{}", "=".repeat(70));
    println!("COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Source:     {}", source.display());
    println!("Output:     {}", output.display());
    println!("Documents:  {}", analyses.len());
    println!("Chunks:     {}", chunks_result.total_chunks);
    println!(
        "Validation: {}/{} passed",
        validation_result.files_passed, validation_result.files_checked
    );
    if generate_llms {
        println!("Entry:      llms.txt (AI should read this first)");
    }
    println!("{}\n", "=".repeat(70));

    Ok(())
}

/// Run the ingest command (scrape + index)
async fn run_ingest(
    url: &str,
    output: &Path,
    filter: Option<String>,
    delay: u64,
    query: Option<String>,
    threshold: f32,
    project_name: Option<String>,
) -> Result<()> {
    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 - INGEST (Scrape + Index)");
    println!("{}\n", "=".repeat(70));

    // Phase 1: Scrape
    println!("[PHASE 1] SCRAPE\n");

    let config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        use_sitemap: true,
        path_filter: filter,
        delay_ms: delay,
        ..Default::default()
    };

    let mut scrape_result = scrape::scrape_site(&config).await?;
    println!("  Scraped {} pages from {}", scrape_result.success_count, url);

    // Apply BM25 filtering if query is provided
    if let Some(ref q) = query {
        let (kept_pages, filtered_count) = scrape::filter_pages_by_relevance(scrape_result.pages, q, threshold);

        if kept_pages.is_empty() {
            println!("\n  WARNING: All pages filtered out by query.");
            println!("  Consider lowering the --threshold value.");
            return Ok(());
        }

        println!("  Filtered by relevance: {} pages removed", filtered_count);
        println!("  Kept: {} pages matching \"{}\"", kept_pages.len(), q);

        scrape_result.pages = kept_pages;
        scrape_result.success_count = scrape_result.pages.len();
    }

    println!();

    // Write scraped content to temp location within output
    let scrape_dir = output.join(".scrape");
    std::fs::create_dir_all(&scrape_dir)?;
    scrape::write_scraped_pages(&scrape_result, output)?;

    // Phase 2: Index
    println!("[PHASE 2] INDEX\n");

    // Derive project name from URL if not provided
    let name = project_name.unwrap_or_else(|| {
        url::Url::parse(url)
            .map(|u| u.host_str().unwrap_or("Documentation").to_string())
            .unwrap_or_else(|_| "Documentation".to_string())
    });

    // Use the scrape directory as source for indexing
    run_index(
        &scrape_dir,
        output,
        None,
        true,
        &name,
        &format!("Documentation scraped from {}", url),
    )?;

    Ok(())
}

/// Run the search command using BM25 ranking
fn run_search(query: &str, index_dir: &Path, limit: usize, use_color: bool) -> Result<()> {
    const MAX_QUERY_LENGTH: usize = 1000;
    const MAX_QUERY_WORDS: usize = 100;

    // Validate query is not empty
    let query = query.trim();
    if query.is_empty() {
        anyhow::bail!("Query cannot be empty");
    }

    // Validate query length
    if query.len() > MAX_QUERY_LENGTH {
        anyhow::bail!(
            "Query too long ({} chars, max {})",
            query.len(),
            MAX_QUERY_LENGTH
        );
    }

    // Validate word count
    let word_count = query.split_whitespace().count();
    if word_count > MAX_QUERY_WORDS {
        anyhow::bail!(
            "Query has too many terms ({} words, max {})",
            word_count,
            MAX_QUERY_WORDS
        );
    }

    use serde_json::Value;

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    let index_content = std::fs::read_to_string(&index_path)?;
    let index: Value = serde_json::from_str(&index_content)?;

    // Extract documents and chunks for searching
    let documents = index["documents"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid INDEX.json: missing documents array"))?;

    let chunks = index["chunks"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid INDEX.json: missing chunks array"))?;

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER SEARCH - BM25");
    println!("{}\n", "=".repeat(70));
    println!("Query: \"{}\"", query);
    println!("Searching {} documents, {} chunks...\n", documents.len(), chunks.len());

    // Calculate average document length for BM25
    let total_words: usize = documents.iter()
        .filter_map(|d| d["word_count"].as_u64())
        .map(|c| c as usize)
        .sum();
    let avg_doc_length = if !documents.is_empty() {
        total_words as f32 / documents.len() as f32
    } else {
        100.0
    };

    // Score each document
    let mut results: Vec<(f32, &Value)> = documents.iter()
        .map(|doc| {
            let title = doc["title"].as_str().unwrap_or("");
            let summary = doc["summary"].as_str().unwrap_or("");
            let searchable = format!("{} {}", title, summary);
            let score = filter::bm25_score(&searchable, query, avg_doc_length);
            (score, doc)
        })
        .filter(|(score, _)| *score > 0.0)
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Display results
    if results.is_empty() {
        println!("No results found for \"{}\"", query);
    } else {
        println!("Results:\n");
        for (i, (score, doc)) in results.iter().take(limit).enumerate() {
            let title = doc["title"].as_str().unwrap_or("Untitled");
            let path = doc["path"].as_str().unwrap_or("");
            let category = doc["category"].as_str().unwrap_or("");
            let summary = doc["summary"].as_str().unwrap_or("");

            // Truncate summary
            let summary_short = if summary.chars().count() > 80 {
                let truncated: String = summary
                    .chars()
                    .take(77)
                    .collect();
                format!("{}...", truncated)
            } else {
                summary.to_string()
            };

            println!("{}. [{}] {} (score: {:.2})", i.saturating_add(1), category, title, score);
            println!("   Path: {}", path);
            println!("   {}\n", summary_short);
        }

        println!("{}", "=".repeat(70));
        println!("Showing {} of {} results", results.len().min(limit), results.len());
        println!("{}\n", "=".repeat(70));
    }

    Ok(())
}
