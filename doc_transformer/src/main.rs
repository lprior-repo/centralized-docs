//! doc_transformer v5.0 - AI-Optimized Documentation Indexer
//!
//! Transform raw documentation into AI-friendly knowledge structures with:
//! - Web scraping via spider-rs
//! - Semantic chunking with context prefixes
//! - Knowledge DAG with relationship detection
//! - llms.txt generation for AI entry points

mod analyze;
mod assign;
mod chunk;
mod discover;
mod filter;
mod graph;
mod index;
mod llms;
mod scrape;
mod transform;
mod validate;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

        /// Project name for llms.txt header
        #[arg(long)]
        project_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scrape {
            url,
            output,
            sitemap,
            filter,
            delay,
        }) => {
            run_scrape(&url, &output, sitemap, filter, delay).await
        }

        Some(Commands::Index {
            source,
            output,
            llms_txt,
            project_name,
            project_desc,
        }) => {
            run_index(&source, &output, llms_txt, &project_name, &project_desc)
        }

        Some(Commands::Ingest {
            url,
            output,
            filter,
            delay,
            project_name,
        }) => {
            run_ingest(&url, &output, filter, delay, project_name).await
        }

        None => {
            // Legacy mode: two positional arguments
            if let (Some(source), Some(output)) = (cli.source_dir, cli.output_dir) {
                run_index(&source, &output, true, "Documentation", "AI-optimized documentation index")
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
    let result = scrape::scrape_site(&config).await?;

    println!("  Discovered: {} URLs", result.total_urls);
    println!("  Scraped: {} pages", result.success_count);
    println!("  Errors: {}", result.error_count);

    if !result.errors.is_empty() {
        println!("\n  Error details:");
        for (url, err) in result.errors.iter().take(5) {
            println!("    - {}: {}", url, err);
        }
        if result.errors.len() > 5 {
            println!("    ... and {} more", result.errors.len() - 5);
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
    source: &PathBuf,
    output: &PathBuf,
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
    let analyses = analyze::analyze_files(&files, source)?;
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

    // STEP 5: CHUNK
    println!("[STEP 5] CHUNK");
    let chunks_result = chunk::chunk_all(&analyses, output)?;
    println!(
        "  Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!("  ~170 tokens/chunk with contextual prefixes\n");

    // STEP 6: INDEX + GRAPH
    println!("[STEP 6] INDEX + GRAPH");
    index::build_and_write_index(&analyses, &link_map, &chunks_result, output)?;
    index::build_and_write_compass(&analyses, &link_map, output)?;
    println!("  Created INDEX.json and COMPASS.md\n");

    // STEP 7: LLMS.TXT
    if generate_llms {
        println!("[STEP 7] LLMS.TXT");
        let llms_config = llms::LlmsConfig {
            project_name: project_name.to_string(),
            project_description: project_desc.to_string(),
            ..Default::default()
        };
        llms::generate_llms_txt(&analyses, &link_map, &llms_config, output)?;
        llms::generate_llms_full_txt(&analyses, &link_map, output)?;
        println!("  Created llms.txt and llms-full.txt\n");
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
    output: &PathBuf,
    filter: Option<String>,
    delay: u64,
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

    let scrape_result = scrape::scrape_site(&config).await?;
    println!("  Scraped {} pages from {}\n", scrape_result.success_count, url);

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
        true,
        &name,
        &format!("Documentation scraped from {}", url),
    )?;

    Ok(())
}
