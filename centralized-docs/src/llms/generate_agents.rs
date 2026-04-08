use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::analyze::Analysis;
use crate::assign::IdMapping;

use super::config::LlmsConfig;

/// Generate AGENTS.md - coding instructions for AI assistants
///
/// This file provides project-specific instructions that AI coding assistants
/// should follow when working with this codebase. Adopted by `OpenAI` Codex,
/// Google Jules, and Cursor.
///
/// # Errors
///
/// Returns an error if:
/// - Writing output file fails
#[allow(clippy::too_many_lines)]
pub fn generate_agents_md<S: BuildHasher>(
    analyses: &[Analysis],
    _link_map: &HashMap<String, IdMapping, S>,
    config: &LlmsConfig,
    output_dir: &Path,
) -> Result<()> {
    // Count categories using functional pattern
    let categories: Vec<_> = analyses
        .iter()
        .map(|a| (a.category.clone(), ()))
        .into_group_map()
        .into_iter()
        .map(|(cat, items)| (cat, items.len()))
        .collect();

    let category_content: String = categories
        .iter()
        .map(|(cat, count)| format!("- **{cat}**: {count} documents"))
        .collect::<Vec<_>>()
        .join("\n");

    let content = format!(
        "# {} - Agent Instructions\n\n\
         > {}\n\n\
         ## Project Overview\n\n\
         This documentation index contains {} documents organized by category.\n\n\
         ### Document Categories\n\n\
         {category_content}\n\n\
         ## Navigation Guide\n\n\
         When working with this documentation:\n\n\
         1. **Start with llms.txt** - Read this first to understand the structure\n\
         2. **Use INDEX.json** - For programmatic lookup of documents and chunks\n\
         3. **Follow the DAG** - Use knowledge graph edges to find related content\n\
         4. **Chunk navigation** - Each chunk has `previous_chunk_id` and `next_chunk_id`\n\n\
         ## File Structure\n\n\
         ```\n\
         ./\n\
         ├── llms.txt           # AI entry point (read first)\n\
         ├── AGENTS.md          # This file - coding instructions\n\
         ├── INDEX.json         # Machine-readable index + knowledge graph\n\
         ├── NAVIGATION.md      # Human-readable navigation\n\
         ├── docs/              # Transformed documents with frontmatter\n\
         └── chunks/            # Semantic chunks with context prefix\n\
         ```\n\n\
         ## Chunk Format\n\n\
         Each chunk file contains:\n\
         - YAML frontmatter with `chunk_id`, `doc_id`, `token_count`, navigation pointers\n\
         - Context prefix from previous chunk (~50-100 tokens)\n\
         - Main content (~170 tokens average)\n\n\
         ## INDEX.json Structure\n\n\
         ```json\n\
         {{\n\
           \"documents\": [...],    // Document metadata\n\
           \"chunks\": [...],       // Chunk metadata with navigation\n\
           \"keywords\": {{...}},     // Term → doc_id lookup\n\
           \"graph\": {{             // Knowledge DAG\n\
             \"nodes\": [...],      // Documents and chunks\n\
             \"edges\": [...]       // Relationships (Parent, Sequential, Related)\n\
           }}\n\
         }}\n\
         ```\n\n\
         ## Best Practices\n\n\
         - **Don't guess**: Use INDEX.json to find exact document/chunk IDs\n\
         - **Read context**: When reading a chunk, consider reading previous/next chunks\n\
         - **Follow relationships**: Use graph edges to find related content\n\
         - **Check frontmatter**: Every document has `category`, `tags`, and `summary`\n",
        config.project_name,
        config.project_description,
        analyses.len()
    );

    fs::write(output_dir.join("AGENTS.md"), content)?;

    Ok(())
}
