//! llms.txt generation module
//!
//! Generates llms.txt and llms-full.txt files following the llms.txt specification.
//! These files provide AI-friendly entry points into the documentation.
//!
//! Specification: <https://llmstxt.org/>

use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
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
    /// llms.txt specification version (default: "1.0")
    pub spec_version: String,
    /// Project version (default: "0.1.0")
    pub project_version: String,
    /// Enable YAML frontmatter with metadata (default: true)
    pub include_frontmatter: bool,
}

impl Default for LlmsConfig {
    fn default() -> Self {
        Self {
            project_name: "Documentation".to_string(),
            project_description: "AI-optimized documentation index".to_string(),
            max_per_category: 5,
            generate_full: true,
            spec_version: "1.0".to_string(),
            project_version: "0.1.0".to_string(),
            include_frontmatter: true,
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
///
/// # Errors
///
/// Returns an error if:
/// - Writing output file fails
#[allow(clippy::too_many_lines)]
pub fn generate_llms_txt<S: BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    config: &LlmsConfig,
    output_dir: &Path,
) -> Result<()> {
    // Group by category using functional pattern
    let by_category: HashMap<&str, Vec<(&Analysis, &IdMapping)>> = analyses
        .iter()
        .filter_map(|analysis| {
            if let Some(mapping) = link_map.get(&analysis.source_path) {
                Some((analysis.category.as_str(), (analysis, mapping)))
            } else {
                eprintln!(
                    "Warning: Missing ID mapping for document: {}",
                    analysis.source_path
                );
                None
            }
        })
        .into_group_map();

    let frontmatter = if config.include_frontmatter {
        format!(
            "---\nversion: \"{}\"\nproject: \"{}\"\nproject_version: \"{}\"\nupdated: \"{}\"\ndocuments: {}\nindex: \"./INDEX.json\"\n---\n\n",
            config.spec_version,
            config.project_name,
            config.project_version,
            chrono::Utc::now().format("%Y-%m-%d"),
            analyses.len()
        )
    } else {
        String::new()
    };

    fn build_section(
        by_category: &HashMap<&str, Vec<(&Analysis, &IdMapping)>>,
        title: &str,
        category: &str,
        max_per_category: usize,
        include_desc: bool,
    ) -> String {
        by_category
            .get(category)
            .filter(|items| !items.is_empty())
            .map(|items| {
                let links: String = items
                    .iter()
                    .take(max_per_category)
                    .map(|(analysis, mapping)| {
                        if include_desc {
                            let desc = truncate_summary(&analysis.first_paragraph, 60);
                            format!(
                                "- [{}](./docs/{}): {desc}\n",
                                analysis.title, mapping.filename
                            )
                        } else {
                            format!("- [{}](./docs/{})\n", analysis.title, mapping.filename)
                        }
                    })
                    .collect();
                format!("## {title}\n\n{links}\n")
            })
            .map_or_else(String::new, std::convert::identity)
    }

    let sections: String = [
        build_section(
            &by_category,
            "Getting Started",
            "tutorial",
            config.max_per_category,
            true,
        ),
        build_section(
            &by_category,
            "Core Concepts",
            "concept",
            config.max_per_category,
            true,
        ),
        build_section(
            &by_category,
            "API Reference",
            "ref",
            config.max_per_category,
            true,
        ),
        build_section(
            &by_category,
            "Operations",
            "ops",
            config.max_per_category,
            true,
        ),
        build_section(
            &by_category,
            "Optional",
            "meta",
            config.max_per_category,
            false,
        ),
    ]
    .join("");

    let content = format!(
        "{frontmatter}# {}\n\n> {}\n\nKey context for AI:\n- Total documents: {}\n- Format: Markdown with YAML frontmatter\n- Chunking: Semantic chunks with context prefix (~170 tokens)\n- Navigation: Knowledge DAG with Jaccard similarity\n\n{sections}## Machine-Readable Index\n\n- [INDEX.json](./INDEX.json): Complete searchable index with keywords, chunks, and knowledge graph\n- [COMPASS.md](./COMPASS.md): Human-readable navigation guide\n",
        config.project_name,
        config.project_description,
        analyses.len()
    );

    fs::write(output_dir.join("llms.txt"), content)?;

    Ok(())
}

/// Generate llms-full.txt - all documentation content concatenated
///
/// This file contains all document content for models with large context windows.
/// Each document is separated by a header with metadata.
///
/// # Errors
///
/// Returns an error if:
/// - Reading document files fails
/// - Writing output file fails
pub fn generate_llms_full_txt<S: BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    output_dir: &Path,
) -> Result<()> {
    let header = format!(
        "# Full Documentation\n\n> This file contains all {} documents concatenated for large context models.\n\n---\n\n",
        analyses.len()
    );

    // Sort by category then title for consistent ordering using functional pattern
    let sorted: Vec<_> = analyses
        .iter()
        .filter_map(|a| {
            if let Some(m) = link_map.get(&a.source_path) {
                Some((a, m))
            } else {
                eprintln!(
                    "Warning: Missing ID mapping for document: {}",
                    a.source_path
                );
                None
            }
        })
        .sorted_by(|a, b| {
            a.0.category
                .cmp(&b.0.category)
                .then_with(|| a.0.title.cmp(&b.0.title))
        })
        .collect_vec();

    let full_content: String = sorted
        .iter()
        .fold(String::new(), |mut acc, (analysis, mapping)| {
            let body = skip_frontmatter(&analysis.content).to_string();
            use std::fmt::Write;
            let _ = write!(
                acc,
                "## {} [{}]\n\n**Path**: docs/{}\n**ID**: {}\n\n{}\n\n---\n\n",
                analysis.title, analysis.category, mapping.filename, mapping.id, body
            );
            acc
        });

    let content = format!("{header}{full_content}");

    fs::write(output_dir.join("llms-full.txt"), content)?;

    Ok(())
}

/// Truncate summary to fit in a description
#[must_use]
pub fn truncate_summary(text: &str, max_len: usize) -> String {
    let cleaned = text.replace('\n', " ").trim().to_string();
    let char_count = cleaned.chars().count();

    if char_count <= max_len {
        return cleaned;
    }

    // Handle edge cases
    if max_len == 0 {
        return String::new();
    }

    if max_len <= 3 {
        // Can't fit "...", just return truncated without ellipsis
        return safe_truncate_chars(&cleaned, max_len);
    }

    // Normal case: truncate and add "..."
    let truncated = safe_truncate_chars(&cleaned, max_len.saturating_sub(3));
    format!("{truncated}...")
}

/// Safely truncate a string to a maximum number of characters, ensuring UTF-8 character boundaries
fn safe_truncate_chars(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    text.char_indices()
        .take(max_chars)
        .last()
        .map(|(idx, c)| {
            let byte_end = idx.saturating_add(c.len_utf8());
            text[..byte_end].to_string()
        })
        .map_or_else(String::new, std::convert::identity)
}

/// Skip YAML frontmatter from document content using functional pattern
fn skip_frontmatter(content: &str) -> &str {
    content
        .strip_prefix("---")
        .and_then(|stripped| {
            stripped
                .find("---")
                .map(|end| stripped[end.saturating_add(3)..].trim_start())
        })
        .map_or(content, |s| s)
}

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
/// # Errors
///
/// Returns an error if:
/// - Writing output file fails
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
         ```
         ./\n\
         ├── llms.txt           # AI entry point (read first)\n\
         ├── llms-full.txt      # Full content for large context models\n\
         ├── AGENTS.md          # This file - coding instructions\n\
         ├── INDEX.json         # Machine-readable index + knowledge graph\n\
         ├── COMPASS.md         # Human-readable navigation\n\
         ├── docs/              # Transformed documents with frontmatter\n\
         └── chunks/            # Semantic chunks with context prefix\n\
         ```
\n\n\
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
         ```
\n\n\
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn make_analysis(source_path: &str, category: &str, title: &str) -> Analysis {
        Analysis {
            source_path: source_path.to_string(),
            title: title.to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "A short summary for testing".to_string(),
            word_count: 50,
            has_code: false,
            has_tables: false,
            category: category.to_string(),
            content: Arc::from("Some content here"),
        }
    }

    fn make_link_map(pairs: &[(&str, &str)]) -> HashMap<String, IdMapping> {
        pairs
            .iter()
            .map(|(path, filename)| {
                let mapping = IdMapping {
                    id: path.to_string(),
                    filename: filename.to_string(),
                    subcategory: "test".to_string(),
                    slug: "test".to_string(),
                };
                (path.to_string(), mapping)
            })
            .collect()
    }

    #[test]
    fn test_truncate_summary() {
        assert_eq!(truncate_summary("Short text", 20), "Short text");
        assert_eq!(
            truncate_summary(
                "This is a much longer piece of text that needs truncation",
                20
            ),
            "This is a much lo..."
        );
    }

    #[test]
    fn test_truncate_summary_empty_string() {
        assert_eq!(truncate_summary("", 10), "");
    }

    #[test]
    fn test_truncate_summary_zero_max_len() {
        assert_eq!(truncate_summary("Hello", 0), "");
    }

    #[test]
    fn test_truncate_summary_max_len_one() {
        let result = truncate_summary("Hello", 1);
        assert_eq!(result, "H");
    }

    #[test]
    fn test_truncate_summary_max_len_two() {
        let result = truncate_summary("Hello", 2);
        assert_eq!(result, "He");
    }

    #[test]
    fn test_truncate_summary_max_len_three() {
        let result = truncate_summary("Hello", 3);
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_truncate_summary_newlines_stripped() {
        assert_eq!(truncate_summary("Line1\nLine2", 20), "Line1 Line2");
    }

    #[test]
    fn test_skip_frontmatter() {
        let with_fm = "---\ntitle: Test\n---\n\nContent here";
        assert_eq!(skip_frontmatter(with_fm), "Content here");

        let without_fm = "Just content";
        assert_eq!(skip_frontmatter(without_fm), "Just content");
    }

    #[test]
    fn test_skip_frontmatter_empty() {
        assert_eq!(skip_frontmatter(""), "");
    }

    #[test]
    fn test_skip_frontmatter_incomplete() {
        let partial = "---\ntitle: Test\n";
        assert_eq!(skip_frontmatter(partial), "---\ntitle: Test\n");
    }

    #[test]
    fn test_llms_config_default() {
        let config = LlmsConfig::default();
        assert_eq!(config.project_name, "Documentation");
        assert_eq!(config.max_per_category, 5);
        assert!(config.generate_full);
        assert!(config.include_frontmatter);
        assert_eq!(config.spec_version, "1.0");
        assert_eq!(config.project_version, "0.1.0");
    }

    #[test]
    fn test_generate_llms_txt_basic() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses = vec![
            make_analysis(
                "docs/getting-started/intro.md",
                "tutorial",
                "Getting Started",
            ),
            make_analysis("docs/api/ref.md", "ref", "API Reference"),
        ];
        let link_map = make_link_map(&[
            (
                "docs/getting-started/intro.md",
                "tutorial-getting-started-intro.md",
            ),
            ("docs/api/ref.md", "ref-api-ref.md"),
        ]);
        let config = LlmsConfig::default();

        generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
        assert!(content.contains("# Documentation"));
        assert!(content.contains("Getting Started"));
        assert!(content.contains("API Reference"));
        assert!(content.contains("docs/tutorial-getting-started-intro.md"));
    }

    #[test]
    fn test_generate_llms_txt_no_frontmatter() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses = vec![make_analysis("a.md", "tutorial", "Test")];
        let link_map = make_link_map(&[("a.md", "a.md")]);
        let config = LlmsConfig {
            include_frontmatter: false,
            ..Default::default()
        };

        generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
        assert!(!content.contains("---"));
        assert!(content.contains("# Documentation"));
    }

    #[test]
    fn test_generate_llms_txt_empty_analyses() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses: Vec<Analysis> = vec![];
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let config = LlmsConfig::default();

        generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
        assert!(content.contains("# Documentation"));
        assert!(content.contains("Total documents: 0"));
    }

    #[test]
    fn test_generate_llms_full_txt_basic() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses = vec![make_analysis("a.md", "tutorial", "Test Doc")];
        let link_map = make_link_map(&[("a.md", "tutorial-test.md")]);

        generate_llms_full_txt(&analyses, &link_map, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms-full.txt")).unwrap();
        assert!(content.contains("# Full Documentation"));
        assert!(content.contains("Test Doc"));
        assert!(content.contains("Some content here"));
    }

    #[test]
    fn test_generate_llms_full_txt_with_frontmatter() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut analysis = make_analysis("a.md", "tutorial", "Doc");
        analysis.content = Arc::from("---\ntitle: Test\n---\n\nReal content");
        let link_map = make_link_map(&[("a.md", "a.md")]);

        generate_llms_full_txt(&[analysis], &link_map, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms-full.txt")).unwrap();
        assert!(content.contains("Real content"));
        assert!(!content.contains("title: Test"));
    }

    #[test]
    fn test_generate_llms_full_txt_empty() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses: Vec<Analysis> = vec![];
        let link_map: HashMap<String, IdMapping> = HashMap::new();

        generate_llms_full_txt(&analyses, &link_map, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("llms-full.txt")).unwrap();
        assert!(content.contains("0 documents"));
    }

    #[test]
    fn test_generate_agents_md_basic() {
        let dir = tempfile::TempDir::new().unwrap();
        let analyses = vec![
            make_analysis("a.md", "tutorial", "Tutorial 1"),
            make_analysis("b.md", "tutorial", "Tutorial 2"),
            make_analysis("c.md", "concept", "Concept 1"),
        ];
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let config = LlmsConfig::default();

        generate_agents_md(&analyses, &link_map, &config, dir.path()).unwrap();

        let content = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(content.contains("Agent Instructions"));
        assert!(content.contains("3 documents"));
        assert!(content.contains("tutorial"));
        assert!(content.contains("concept"));
    }

    #[test]
    fn test_safe_truncate_chars() {
        assert_eq!(safe_truncate_chars("hello", 3), "hel");
        assert_eq!(safe_truncate_chars("hello", 0), "");
        assert_eq!(safe_truncate_chars("hello", 10), "hello");
    }
}
