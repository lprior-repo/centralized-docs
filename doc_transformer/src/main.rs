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

        /// Filter files by BM25 relevance to query
        #[arg(short, long, value_name = "QUERY")]
        query: Option<String>,

        /// Minimum BM25 score to keep a file (default: 0.1)
        #[arg(long, default_value = "0.1")]
        threshold: f32,
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

    /// Explore the knowledge graph relationships
    Graph {
        /// Node ID to explore (document ID or chunk ID)
        #[arg(value_name = "NODE_ID")]
        node_id: String,

        /// Directory containing INDEX.json
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        index_dir: PathBuf,

        /// Show reachable nodes (transitive closure)
        #[arg(long)]
        reachable: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Transform { source_dir, output_dir, incremental, force, verbose, query, threshold } => {
            run_transform(&source_dir, &output_dir, incremental, force, verbose, query.as_deref(), threshold).await
        }
        Commands::Search { query, index_dir, limit, chunks } => {
            run_search(&query, &index_dir, limit, chunks)
        }
        Commands::Graph { node_id, index_dir, reachable } => {
            run_graph(&node_id, &index_dir, reachable)
        }
    }
}

async fn run_transform(source_dir: &PathBuf, output_dir: &PathBuf, incremental: bool, force: bool, verbose: bool, query: Option<&str>, threshold: f32) -> Result<()> {
    // VALIDATE OUTPUT FIRST (before any processing or printing)
    validate::validate_output_dir(output_dir)?;

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v4.3 (Knowledge DAG)");
    println!("{}\n", "=".repeat(70));

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (mut files, _discover_manifest) = discover::discover_files(source_dir)?;
    println!("  DISCOVER: Found {} files", files.len());

    // STEP 1.5: BM25 FILTERING (if query provided)
    if let Some(q) = query {
        println!("  Query filter: \"{}\" (threshold: {})", q, threshold);
        let (filtered_files, filtered_count) = discover::filter_files_by_relevance(files, q, threshold, source_dir)?;

        if filtered_files.is_empty() {
            println!("  WARNING: All files filtered out by query. No files to process.");
            println!("  Consider lowering the --threshold value.\n");
            return Ok(());
        }

        println!("  Filtered by relevance: {} files removed", filtered_count);
        println!("  Kept: {} files matching \"{}\"\n", filtered_files.len(), q);
        files = filtered_files;
    } else {
        println!();
    }

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

    if verbose && !validation_result.issues.is_empty() {
        println!(
            "  {}/{} files passed ({} errors, {} warnings)\n",
            validation_result.files_passed,
            validation_result.files_checked,
            validation_result.total_errors,
            validation_result.total_warnings
        );

        // Group issues by file
        use std::collections::HashMap;
        let mut issues_by_file: HashMap<String, Vec<&validate::ValidationIssue>> = HashMap::new();
        for issue in &validation_result.issues {
            issues_by_file.entry(issue.file.clone())
                .or_default()
                .push(issue);
        }

        // Display issues grouped by file (sorted for deterministic output)
        let mut sorted_files: Vec<_> = issues_by_file.keys().collect();
        sorted_files.sort();

        for file in sorted_files {
            let issues = &issues_by_file[file];
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
    } else if validation_result.issues.is_empty() {
        println!("  All {} files passed validation\n", validation_result.files_checked);
    } else {
        println!(
            "  {}/{} files passed ({} errors, {} warnings)\n",
            validation_result.files_passed,
            validation_result.files_checked,
            validation_result.total_errors,
            validation_result.total_warnings
        );
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

fn run_graph(node_id: &str, index_dir: &PathBuf, show_reachable: bool) -> Result<()> {
    use crate::graph::{GraphEdge, GraphNode};

    let index_path = index_dir.join("INDEX.json");

    if !index_path.exists() {
        anyhow::bail!(
            "INDEX.json not found at: {}\nPlease run the transform command first.",
            index_path.display()
        );
    }

    // Load INDEX.json
    let index_content = std::fs::read_to_string(&index_path)?;
    let index_value: serde_json::Value = serde_json::from_str(&index_content)?;

    // Extract graph data
    let graph_section = index_value
        .get("graph")
        .ok_or_else(|| anyhow::anyhow!("INDEX.json missing graph data"))?;

    let nodes: Vec<GraphNode> = serde_json::from_value(
        graph_section
            .get("nodes")
            .ok_or_else(|| anyhow::anyhow!("Graph section missing nodes"))?
            .clone(),
    )?;

    let edges: Vec<GraphEdge> = serde_json::from_value(
        graph_section
            .get("edges")
            .ok_or_else(|| anyhow::anyhow!("Graph section missing edges"))?
            .clone(),
    )?;

    // Find the requested node
    let node = nodes
        .iter()
        .find(|n| n.id == node_id)
        .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?;

    // Find incoming and outgoing edges
    let mut outgoing: Vec<(&GraphEdge, &GraphNode)> = Vec::new();
    let mut incoming: Vec<(&GraphEdge, &GraphNode)> = Vec::new();

    for edge in &edges {
        if edge.from == node_id {
            if let Some(to_node) = nodes.iter().find(|n| n.id == edge.to) {
                outgoing.push((edge, to_node));
            }
        }
        if edge.to == node_id {
            if let Some(from_node) = nodes.iter().find(|n| n.id == edge.from) {
                incoming.push((edge, from_node));
            }
        }
    }

    // Calculate reachable nodes if requested
    let reachable_count = if show_reachable {
        let mut dag = crate::graph::KnowledgeDAG::new();
        for n in &nodes {
            dag.add_node(n.clone());
        }
        for e in &edges {
            dag.add_edge(e.clone());
        }
        let reachable = dag.reachable_from(node_id);
        Some(reachable.len().saturating_sub(1)) // Subtract 1 to exclude the node itself
    } else {
        None
    };

    // Display output
    println!("\n{}", "=".repeat(70));
    println!("KNOWLEDGE GRAPH: {}", node_id);
    println!("{}\n", "=".repeat(70));

    let node_type = match node.node_type {
        crate::graph::NodeType::Document => "Document",
        crate::graph::NodeType::Chunk => "Chunk",
    };

    let truncated_title = if node.title.len() > 50 {
        format!("{}...", &node.title[..47])
    } else {
        node.title.clone()
    };

    println!("Node: {} ({})", node_id, node_type);
    println!("Title: {}\n", truncated_title);

    if outgoing.is_empty() {
        println!("Outgoing Edges: None\n");
    } else {
        println!("Outgoing Edges ({}):", outgoing.len());
        for (edge, target) in &outgoing {
            let edge_type_str = format!("{:?}", edge.edge_type);
            let target_title = if target.title.len() > 40 {
                format!("{}...", &target.title[..37])
            } else {
                target.title.clone()
            };
            println!(
                "  → {} [{}, weight: {:.2}]",
                target.id, edge_type_str, edge.weight
            );
            println!("     {}", target_title);
        }
        println!();
    }

    if incoming.is_empty() {
        println!("Incoming Edges: None\n");
    } else {
        println!("Incoming Edges ({}):", incoming.len());
        for (edge, source) in &incoming {
            let edge_type_str = format!("{:?}", edge.edge_type);
            let source_title = if source.title.len() > 40 {
                format!("{}...", &source.title[..37])
            } else {
                source.title.clone()
            };
            println!(
                "  ← {} [{}, weight: {:.2}]",
                source.id, edge_type_str, edge.weight
            );
            println!("     {}", source_title);
        }
        println!();
    }

    if let Some(count) = reachable_count {
        println!("Reachable: {} nodes\n", count);
    }

    if outgoing.is_empty() && incoming.is_empty() {
        println!("No relationships found\n");
    }

    println!("{}\n", "=".repeat(70));

    Ok(())
}
