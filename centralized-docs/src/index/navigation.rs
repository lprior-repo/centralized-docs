//! Navigation file generation and tag extraction.

use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::types::is_stopword;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Writes documentation navigation file with category-based navigation
#[allow(clippy::implicit_hasher)]
pub fn build_and_write_navigation<S: std::hash::BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    output_dir: &Path,
) -> Result<()> {
    let by_category: HashMap<String, Vec<(String, String, Vec<String>)>> = analyses
        .iter()
        .filter_map(|analysis| {
            link_map.get(&analysis.source_path).map(|mapping| {
                let tags = extract_tags(analysis);
                (
                    analysis.category.clone(),
                    (analysis.title.clone(), mapping.filename.clone(), tags),
                )
            })
        })
        .into_group_map();

    let navigation_content = ["tutorial", "concept", "ref", "ops", "meta"]
        .into_iter()
        .filter_map(|category| {
            by_category.get(category).map(|docs| {
                let section_docs =
                    docs.iter()
                        .take(5)
                        .fold(String::new(), |mut acc, (title, filename, tags)| {
                            let tag_str = tags
                                .iter()
                                .take(2)
                                .map(|t| format!("`{t}`"))
                                .collect::<Vec<_>>()
                                .join(" ");
                            use std::fmt::Write;
                            let _ = writeln!(acc, "- [{title}](./docs/{filename}) {tag_str}");
                            acc
                        });
                format!("## {}\n\n{}\n", category.to_uppercase(), section_docs)
            })
        })
        .collect::<String>();

    let navigation_content = format!(
        "# Documentation Navigation\n\n> **{} documents**\n\n{}",
        analyses.len(),
        navigation_content,
    );

    let navigation_file = output_dir.join("NAVIGATION.md");
    fs::write(navigation_file, navigation_content)?;
    Ok(())
}

/// Extract tags using functional composition
pub fn extract_tags(analysis: &Analysis) -> Vec<String> {
    std::iter::once(analysis.category.clone())
        .chain(
            analysis
                .headings
                .iter()
                .take(3)
                .flat_map(|h| h.text.split_whitespace())
                .filter(|word| word.len() > 4 && !is_stopword(&word.to_lowercase()))
                .map(str::to_lowercase),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
}
