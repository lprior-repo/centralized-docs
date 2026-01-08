mod discover;
mod analyze;
mod assign;
mod transform;
mod chunk;
mod graph;
mod index;
mod validate;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "doc-transformer")]
#[command(about = "Transform raw docs into AI-optimized knowledge structures v4.2")]
struct Args {
    /// Source directory containing documentation
    #[arg(value_name = "DIR")]
    source_dir: PathBuf,

    /// Output directory for transformed docs
    #[arg(value_name = "DIR")]
    output_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v4.3 (Knowledge DAG)");
    println!("{}\n", "=".repeat(70));

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (files, _discover_manifest) = discover::discover_files(&args.source_dir)?;
    println!("  DISCOVER: Found {} files\n", files.len());

    // STEP 2: ANALYZE
    println!("[STEP 2] ANALYZE");
    let analyses = analyze::analyze_files(&files, &args.source_dir)?;
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
    let (analyses, link_map) = assign::assign_ids(analyses);
    println!("  ASSIGN: Generated {} IDs\n", analyses.len());

    // STEP 4: TRANSFORM
    println!("[STEP 4] TRANSFORM");
    let transform_result = transform::transform_all(&analyses, &link_map, &args.output_dir)?;
    println!(
        "  TRANSFORM: {}/{} files ({} errors)\n",
        transform_result.success_count, transform_result.total_count, transform_result.error_count
    );

    // STEP 5: CHUNK
    println!("[STEP 5] CHUNK");
    let chunks_result = chunk::chunk_all(&analyses, &args.output_dir)?;
    println!(
        "  CHUNK: Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!(
        "    Semantic chunking: ~170 tokens/chunk with contextual prefixes\n"
    );

    // STEP 6: INDEX
    println!("[STEP 6] INDEX");
    index::build_and_write_index(&analyses, &link_map, &chunks_result, &args.output_dir)?;
    index::build_and_write_compass(&analyses, &link_map, &args.output_dir)?;
    println!("  INDEX: Created COMPASS.md and INDEX.json\n");

    // STEP 7: VALIDATE
    println!("[STEP 7] VALIDATE");
    let validation_result = validate::validate_all(&args.output_dir)?;
    println!(
        "  VALIDATE: {}/{} files passed. {} errors {} warnings\n",
        validation_result.files_passed,
        validation_result.files_checked,
        validation_result.total_errors,
        validation_result.total_warnings
    );

    // FINAL SUMMARY
    println!("{}", "=".repeat(70));
    println!("COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Source:     {}", args.source_dir.display());
    println!("Output:     {}", args.output_dir.display());
    println!("Documents:  {} transformed", analyses.len());
    println!("Chunks:     {} generated", chunks_result.total_chunks);
    println!(
        "Validation: {}/{} passed",
        validation_result.files_passed, validation_result.files_checked
    );
    println!("Errors:     {}", validation_result.total_errors);
    println!("Warnings:   {}", validation_result.total_warnings);
    println!("{}\n", "=".repeat(70));

    Ok(())
}
