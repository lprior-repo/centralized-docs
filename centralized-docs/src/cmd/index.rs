use crate::cli::config::IndexConfig;
use crate::sys::dir::validate_output_path;
use crate::sys::lock::acquire_output_lock;
use crate::{analyze, assign, chunking_adapter, discover, index, llms, transform, validate};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Run the index command (main pipeline)
pub fn run_index(source: &Path, output: &Path, config: &IndexConfig) -> Result<()> {
    validate_output_path(output)?;
    let _output_lock = acquire_output_lock(output)?;

    // Log graph configuration parameters
    println!("[CONFIG] Graph Parameters:");
    println!(
        "  max_related_chunks: {} (default: 20)",
        config.max_related_chunks
    );
    println!(
        "  max_chunk_keywords: {} (default: 12)",
        config.max_chunk_keywords
    );
    println!("  hnsw_m: {} (default: 16)", config.hnsw_m);
    println!(
        "  hnsw_ef_construction: {} (default: 200)",
        config.hnsw_ef_construction
    );
    println!();

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (files, discover_manifest) =
        discover::discover_files(source, config.path_filter.as_deref())?;
    println!("  Found {} files\n", files.len());

    // Exit with error if no markdown files found (user error - exit code 1)
    if files.is_empty() {
        anyhow::bail!(
            "No markdown files found in source directory. Cannot index empty source.\n\
             Hint: Ensure the source directory contains files with .md, .mdx, .markdown, .txt, or .rst extensions."
        );
    }

    // STEP 2: ANALYZE
    // Use manifest.source_dir for analysis (handles both directory and single file cases)
    let analysis_base_path = PathBuf::from(&discover_manifest.source_dir);
    println!("[STEP 2] ANALYZE");
    let analyze_result = analyze::analyze_files(
        &files,
        &analysis_base_path,
        config.category_config.as_deref(),
    )?;

    // Report failed files as warnings instead of failing the entire build
    if !analyze_result.failed_files.is_empty() {
        analyze_result.failed_files.iter().for_each(|failed_file| {
            eprintln!(
                "Warning: Failed to analyze {}: {}",
                failed_file.source_path, failed_file.error
            );
        });
    }

    let analyses = analyze_result.analyses;
    let categories = analyze::count_categories(&analyses);
    println!("  Processed {} files\n", analyses.len());
    println!(
        "  Categories: ref={} concept={} tutorial={} ops={} meta={}\n",
        categories.get("ref").map_or(&0, |v| v),
        categories.get("concept").map_or(&0, |v| v),
        categories.get("tutorial").map_or(&0, |v| v),
        categories.get("ops").map_or(&0, |v| v),
        categories.get("meta").map_or(&0, |v| v)
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
    let chunks_result =
        chunking_adapter::chunk_all(&analyses, &link_map, output, config.max_document_bytes)?;
    println!(
        "  Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!(
        "  Hierarchical: {} summary, {} standard, {} detailed",
        chunks_result.summary_chunks, chunks_result.standard_chunks, chunks_result.detailed_chunks
    );
    println!("  ~512 tokens/chunk with contextual prefixes\n");

    // STEP 6: VALIDATE (before artifact writing - ensures atomic failure)
    println!("[STEP 6] VALIDATE");
    let validation_result = validate::validate_all(output)?;
    println!(
        "  {}/{} files passed ({} errors, {} warnings)\n",
        validation_result.files_passed,
        validation_result.files_checked,
        validation_result.total_errors,
        validation_result.total_warnings
    );

    // Bail early if validation fails - no artifacts written yet
    if validation_result.total_errors > 0 {
        let error_details = validation_result
            .failed_files
            .iter()
            .map(|f| format!("{}: {:?}", f.file_path, f.errors))
            .collect::<Vec<_>>()
            .join("\n  ");

        println!(
            "Validation failed: {} errors found across {} files.\nDetails:\n  {}",
            validation_result.total_errors, validation_result.files_checked, error_details
        );
    }

    // STEP 7: INDEX + GRAPH
    println!("[STEP 7] INDEX + GRAPH");
    index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output,
        &config.project_name,
        Some(config.max_related_chunks),
        Some(config.hnsw_m),
        Some(config.hnsw_ef_construction),
        Some(config.max_chunk_keywords),
    )?;
    index::build_and_write_compass(&analyses, &link_map, output)?;
    println!("  Created INDEX.json and COMPASS.md\n");

    // STEP 8: LLMS.TXT + AGENTS.MD
    if config.generate_llms {
        println!("[STEP 8] LLMS.TXT + AGENTS.MD");
        let llms_config = llms::LlmsConfig {
            project_name: config.project_name.clone(),
            project_description: config.project_desc.clone(),
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
    if config.generate_llms {
        println!("Entry:      llms.txt (AI should read this first)");
    }
    println!("{}\n", "=".repeat(70));

    Ok(())
}
