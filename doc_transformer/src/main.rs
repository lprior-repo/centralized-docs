mod discover;
mod analyze;
mod assign;
mod transform;
mod chunk;
mod graph;
mod index;
mod validate;
mod search;
mod incremental;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "doc-transformer")]
#[command(about = "Transform raw docs into AI-optimized knowledge structures v4.3")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Transform raw documentation into AI-optimized knowledge structures
    Transform {
        /// Source directory containing documentation
        #[arg(value_name = "DIR")]
        source_dir: PathBuf,

        /// Output directory for transformed docs
        #[arg(value_name = "DIR")]
        output_dir: PathBuf,

        /// Only process changed files (incremental mode)
        #[arg(long)]
        incremental: bool,

        /// Force full re-index (ignore incremental state)
        #[arg(long)]
        force: bool,

        /// Show detailed validation output
        #[arg(long)]
        verbose: bool,
    },

    /// Search documents or chunks
    Search {
        /// Query string to search for
        #[arg(value_name = "QUERY")]
        query: String,

        /// Index directory containing INDEX.json
        #[arg(short, long, value_name = "DIR")]
        index_dir: PathBuf,

        /// Maximum number of results to return
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,

        /// Search within chunks instead of documents
        #[arg(long)]
        chunks: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Transform { source_dir, output_dir, incremental, force, verbose } => {
            run_transform(&source_dir, &output_dir, incremental, force, verbose).await
        }
        Commands::Search { query, index_dir, limit, chunks } => {
            run_search(&query, &index_dir, limit, chunks)
        }
    }
}

async fn run_transform(source_dir: &PathBuf, output_dir: &PathBuf, incremental: bool, force: bool, verbose: bool) -> Result<()> {
    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v4.3 (Knowledge DAG)");
    println!("{}\n", "=".repeat(70));

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (files, _discover_manifest) = discover::discover_files(source_dir)?;
    println!("  DISCOVER: Found {} files\n", files.len());

    // INCREMENTAL MODE CHECK
    let (files_to_process, _changeset) = if incremental && !force {
        let prev_state = incremental::load_state(output_dir)?;

        if let Some(state) = prev_state {
            println!("[INCREMENTAL MODE]");
            let changeset = incremental::determine_changes(&files, Some(&state), source_dir)?;

            println!("  Added: {} files", changeset.added.len());
            println!("  Modified: {} files", changeset.modified.len());
            println!("  Unchanged: {} files (skipped)", changeset.unchanged.len());
            println!("  Deleted: {} files (removed from index)", changeset.deleted.len());
            println!("  Processing {} files...\n", changeset.total_to_process());

            let files_to_process: Vec<_> = files
                .iter()
                .filter(|f| {
                    changeset.added.contains(&f.source_path) || changeset.modified.contains(&f.source_path)
                })
                .cloned()
                .collect();

            (files_to_process, Some(changeset))
        } else {
            println!("[INCREMENTAL MODE] No previous state found - performing full index\n");
            (files.clone(), None)
        }
    } else if force {
        println!("[FORCE MODE] Ignoring incremental state - performing full re-index\n");
        (files.clone(), None)
    } else {
        (files.clone(), None)
    };

    // STEP 2: ANALYZE
    println!("[STEP 2] ANALYZE");
    let analyses = analyze::analyze_files(&files_to_process, source_dir)?;
    let categories = analyze::count_categories(&analyses);
    println!(
        "  ANALYZE: Processed {} files",
        analyses.len()
    );
    println!(
        "    Categories: ref={} concept={} tutorial={} ops={} meta={}\n",
        categories.get("ref").unwrap_or(&0),
        categories.get("concept").unwrap_or(&0),
        categories.get("tutorial").unwrap_or(&0),
        categories.get("ops").unwrap_or(&0),
        categories.get("meta").unwrap_or(&0)
    );

    // STEP 3: ASSIGN IDs
    println!("[STEP 3] ASSIGN IDs");
    let (analyses, link_map) = assign::assign_ids(analyses)?;
    println!("  ASSIGN: Generated {} IDs\n", analyses.len());

    // STEP 4: TRANSFORM
    println!("[STEP 4] TRANSFORM");
    let transform_result = transform::transform_all(&analyses, &link_map, output_dir)?;
    println!(
        "  TRANSFORM: {}/{} files ({} errors, {} skipped)\n",
        transform_result.success_count, transform_result.total_count,
        transform_result.error_count, transform_result.skipped_count
    );

    // STEP 5: CHUNK
    println!("[STEP 5] CHUNK");
    let chunks_result = chunk::chunk_all(&analyses, output_dir)?;
    println!(
        "  CHUNK: Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!(
        "    Semantic chunking: ~170 tokens/chunk with contextual prefixes\n"
    );

    // STEP 6: INDEX
    println!("[STEP 6] INDEX");
    index::build_and_write_index(&analyses, &link_map, &chunks_result, output_dir)?;
    index::build_and_write_compass(&analyses, &link_map, output_dir)?;
    println!("  INDEX: Created COMPASS.md and INDEX.json\n");

    // STEP 7: VALIDATE
    println!("[STEP 7] VALIDATE");
    let validation_result = validate::validate_all(output_dir)?;
    println!(
        "  {}/{} files passed ({} errors, {} warnings)",
        validation_result.files_passed,
        validation_result.files_checked,
        validation_result.total_errors,
        validation_result.total_warnings
    );

    if verbose && !validation_result.issues.is_empty() {
        println!();
        // Group issues by file
        use std::collections::HashMap;
        let mut issues_by_file: HashMap<String, Vec<&validate::ValidationIssue>> = HashMap::new();
        for issue in &validation_result.issues {
            issues_by_file.entry(issue.file.clone())
                .or_default()
                .push(issue);
        }

        // Display issues grouped by file
        for (file, issues) in issues_by_file.iter() {
            println!("  {}:", file);
            for issue in issues {
                let severity_marker = match issue.severity {
                    validate::Severity::Error => "[E]",
                    validate::Severity::Warning => "[W]",
                };
                if let Some(line) = issue.line {
                    println!("    {} {}: {} (line {})", severity_marker, issue.rule_id, issue.message, line);
                } else {
                    println!("    {} {}: {}", severity_marker, issue.rule_id, issue.message);
                }
            }
            println!();
        }
    }

    // STEP 8: VALIDATE LINKS
    println!("\n[STEP 8] VALIDATE LINKS");
    let link_validation = validate::validate_links(&analyses, source_dir)?;
    println!(
        "  LINKS: {}/{} internal links checked",
        link_validation.internal_links,
        link_validation.total_links
    );

    if link_validation.broken_links.is_empty() {
        println!("  All links are valid!\n");
    } else {
        println!("  {} broken links found:\n", link_validation.broken_links.len());
        for broken in &link_validation.broken_links {
            match broken.reason {
                validate::BrokenLinkReason::FileNotFound => {
                    println!("    {} → {} (file not found)", broken.source_file, broken.target);
                }
                validate::BrokenLinkReason::EmptyTarget => {
                    println!("    {} → (empty target)", broken.source_file);
                }
            }
        }
        println!();
    }

    // FINAL SUMMARY
    println!("{}", "=".repeat(70));
    println!("COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Source:     {}", source_dir.display());
    println!("Output:     {}", output_dir.display());
    println!("Documents:  {} analyzed, {} transformed, {} skipped",
             analyses.len(), transform_result.success_count, transform_result.skipped_count);
    println!("Chunks:     {} generated", chunks_result.total_chunks);
    println!(
        "Validation: {}/{} passed",
        validation_result.files_passed, validation_result.files_checked
    );
    println!("Errors:     {}", validation_result.total_errors);
    println!("Warnings:   {}", validation_result.total_warnings);
    println!(
        "Links:      {}/{} internal links valid ({} broken)",
        link_validation.internal_links - link_validation.broken_links.len(),
        link_validation.internal_links,
        link_validation.broken_links.len()
    );
    println!("{}\n", "=".repeat(70));

    // SAVE INCREMENTAL STATE
    if incremental && !force {
        use std::collections::HashMap;
        let mut doc_ids = HashMap::new();
        for analysis in &analyses {
            if let Some(mapping) = link_map.get(&analysis.source_path) {
                doc_ids.insert(analysis.source_path.clone(), mapping.id.clone());
            }
        }

        let new_state = incremental::create_state(&files, source_dir, &doc_ids)?;
        incremental::save_state(output_dir, &new_state)?;
        println!("  Incremental state saved\n");
    }

    Ok(())
}

fn run_search(query: &str, index_dir: &PathBuf, limit: usize, chunks: bool) -> Result<()> {
    let index_path = index_dir.join("INDEX.json");

    if !index_path.exists() {
        anyhow::bail!(
            "INDEX.json not found at: {}\nPlease run the transform command first.",
            index_path.display()
        );
    }

    println!("\n{}", "=".repeat(70));
    if chunks {
        println!("CHUNK SEARCH: \"{}\"", query);
    } else {
        println!("DOCUMENT SEARCH: \"{}\"", query);
    }
    println!("{}\n", "=".repeat(70));

    if chunks {
        let results = search::search_chunks(&index_path, query, limit)?;

        if results.is_empty() {
            println!("No matching chunks found.\n");
        } else {
            println!("Found {} matching chunks:\n", results.len());

            for (i, result) in results.iter().enumerate() {
                println!("{}. {} [Score: {:.2}]", i + 1, result.chunk_id, result.score);
                println!("   Document: {}", result.doc_title);
                println!("   Level: {}", result.chunk_level);
                if let Some(heading) = &result.heading {
                    println!("   Heading: {}", heading);
                }
                println!("   Snippet: {}", result.snippet);
                println!();
            }
        }
    } else {
        let results = search::search_documents(&index_path, query, limit)?;

        if results.is_empty() {
            println!("No matching documents found.\n");
        } else {
            println!("Found {} matching documents:\n", results.len());

            for (i, result) in results.iter().enumerate() {
                println!("{}. {} [Score: {:.2}]", i + 1, result.title, result.score);
                println!("   ID: {}", result.id);
                println!("   Category: {}", result.category);
                println!("   Summary: {}", result.summary);
                println!();
            }
        }
    }

    println!("{}\n", "=".repeat(70));

    Ok(())
}
