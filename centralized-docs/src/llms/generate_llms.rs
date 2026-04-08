use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasher;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::analyze::Analysis;
use crate::assign::IdMapping;

use super::config::LlmsConfig;
use super::text_utils::truncate_summary;

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
