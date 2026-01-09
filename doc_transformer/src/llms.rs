//! llms.txt generation module
//!
//! Generates llms.txt and llms-full.txt files following the llms.txt specification.
//! These files provide AI-friendly entry points into the documentation.
//!
//! Specification: https://llmstxt.org/

use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for llms.txt generation
#[derive(Debug, Clone)]
pub struct LlmsConfig {
    /// Project name (H1 in llms.txt)
    pub project_name: String,
    /// Brief project description (blockquote)
    pub project_description: String,
    /// Maximum documents per category in llms.txt (default: 5)
    pub max_per_category: usize,
    /// Include full content in llms-full.txt (default: true)
    pub generate_full: bool,
}

impl Default for LlmsConfig {
    fn default() -> Self {
        Self {
            project_name: "Documentation".to_string(),
            project_description: "AI-optimized documentation index".to_string(),
            max_per_category: 5,
            generate_full: true,
        }
    }
}

/// Generate llms.txt - curated overview for AI consumption
///
/// Format follows the llms.txt specification:
/// - H1: Project name
/// - Blockquote: Brief description
/// - H2 sections: Categorized document links
/// - Optional section: Secondary content
pub fn generate_llms_txt(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    config: &LlmsConfig,
    output_dir: &Path,
) -> Result<()> {
    let mut content = String::new();

    // H1: Project name (required)
    content.push_str(&format!("# {}\n\n", config.project_name));

    // Blockquote: Description
    content.push_str(&format!("> {}\n\n", config.project_description));

    // Key context for AI
    content.push_str("Key context for AI:\n");
    content.push_str(&format!("- Total documents: {}\n", analyses.len()));
    content.push_str("- Format: Markdown with YAML frontmatter\n");
    content.push_str("- Chunking: Semantic chunks with context prefix (~170 tokens)\n");
    content.push_str("- Navigation: Knowledge DAG with Jaccard similarity\n\n");

    // Group by category
    let mut by_category: HashMap<&str, Vec<(&Analysis, &IdMapping)>> = HashMap::new();
    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            by_category
                .entry(&analysis.category)
                .or_default()
                .push((analysis, mapping));
        }
    }

    // Getting Started (tutorials)
    if let Some(tutorials) = by_category.get("tutorial") {
        content.push_str("## Getting Started\n\n");
        for (analysis, mapping) in tutorials.iter().take(config.max_per_category) {
            let desc = truncate_summary(&analysis.first_paragraph, 60);
            content.push_str(&format!(
                "- [{}](./docs/{}): {}\n",
                analysis.title, mapping.filename, desc
            ));
        }
        content.push('\n');
    }

    // Core Concepts
    if let Some(concepts) = by_category.get("concept") {
        content.push_str("## Core Concepts\n\n");
        for (analysis, mapping) in concepts.iter().take(config.max_per_category) {
            let desc = truncate_summary(&analysis.first_paragraph, 60);
            content.push_str(&format!(
                "- [{}](./docs/{}): {}\n",
                analysis.title, mapping.filename, desc
            ));
        }
        content.push('\n');
    }

    // API Reference
    if let Some(refs) = by_category.get("ref") {
        content.push_str("## API Reference\n\n");
        for (analysis, mapping) in refs.iter().take(config.max_per_category) {
            let desc = truncate_summary(&analysis.first_paragraph, 60);
            content.push_str(&format!(
                "- [{}](./docs/{}): {}\n",
                analysis.title, mapping.filename, desc
            ));
        }
        content.push('\n');
    }

    // Operations
    if let Some(ops) = by_category.get("ops") {
        content.push_str("## Operations\n\n");
        for (analysis, mapping) in ops.iter().take(config.max_per_category) {
            let desc = truncate_summary(&analysis.first_paragraph, 60);
            content.push_str(&format!(
                "- [{}](./docs/{}): {}\n",
                analysis.title, mapping.filename, desc
            ));
        }
        content.push('\n');
    }

    // Optional section (meta)
    if let Some(meta) = by_category.get("meta") {
        content.push_str("## Optional\n\n");
        for (analysis, mapping) in meta.iter().take(config.max_per_category) {
            content.push_str(&format!("- [{}](./docs/{})\n", analysis.title, mapping.filename));
        }
        content.push('\n');
    }

    // Machine-readable index reference
    content.push_str("## Machine-Readable Index\n\n");
    content.push_str("- [INDEX.json](./INDEX.json): Complete searchable index with keywords, chunks, and knowledge graph\n");
    content.push_str("- [COMPASS.md](./COMPASS.md): Human-readable navigation guide\n");

    fs::write(output_dir.join("llms.txt"), content)?;

    Ok(())
}

/// Generate llms-full.txt - all documentation content concatenated
///
/// This file contains all document content for models with large context windows.
/// Each document is separated by a header with metadata.
pub fn generate_llms_full_txt(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<()> {
    let docs_dir = output_dir.join("docs");
    let mut content = String::new();

    content.push_str("# Full Documentation\n\n");
    content.push_str(&format!(
        "> This file contains all {} documents concatenated for large context models.\n\n",
        analyses.len()
    ));
    content.push_str("---\n\n");

    // Sort by category then title for consistent ordering
    let mut sorted: Vec<_> = analyses
        .iter()
        .filter_map(|a| link_map.get(&a.source_path).map(|m| (a, m)))
        .collect();
    sorted.sort_by(|a, b| {
        a.0.category
            .cmp(&b.0.category)
            .then_with(|| a.0.title.cmp(&b.0.title))
    });

    for (analysis, mapping) in sorted {
        // Document header
        content.push_str(&format!("## {} [{}]\n\n", analysis.title, analysis.category));
        content.push_str(&format!("**Path**: docs/{}\n", mapping.filename));
        content.push_str(&format!("**ID**: {}\n\n", mapping.id));

        // Read and include document content
        let doc_path = docs_dir.join(&mapping.filename);
        if let Ok(doc_content) = fs::read_to_string(&doc_path) {
            // Skip frontmatter if present
            let body = skip_frontmatter(&doc_content);
            content.push_str(body);
            content.push_str("\n\n");
        } else {
            // Fall back to summary if file not found
            content.push_str(&analysis.first_paragraph);
            content.push_str("\n\n");
        }

        content.push_str("---\n\n");
    }

    fs::write(output_dir.join("llms-full.txt"), content)?;

    Ok(())
}

/// Truncate summary to fit in a description
fn truncate_summary(text: &str, max_len: usize) -> String {
    let cleaned = text.replace('\n', " ").trim().to_string();
    if cleaned.len() <= max_len {
        cleaned
    } else {
        format!("{}...", &cleaned[..max_len.saturating_sub(3)])
    }
}

/// Skip YAML frontmatter from document content
fn skip_frontmatter(content: &str) -> &str {
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            return content[end + 6..].trim_start();
        }
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_summary() {
        assert_eq!(truncate_summary("Short text", 20), "Short text");
        assert_eq!(
            truncate_summary("This is a much longer piece of text that needs truncation", 20),
            "This is a much lo..."
        );
    }

    #[test]
    fn test_skip_frontmatter() {
        let with_fm = "---\ntitle: Test\n---\n\nContent here";
        assert_eq!(skip_frontmatter(with_fm), "Content here");

        let without_fm = "Just content";
        assert_eq!(skip_frontmatter(without_fm), "Just content");
    }
}
