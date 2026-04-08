//! Writing scraped pages to output directory with TOC and related pages.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::super::validation::{PageFilterStatus, ScrapeResult};
use super::links::{build_link_indexes, find_related_pages_with_index, generate_toc};

/// Write scraped pages to output directory with TOC and related pages
pub fn write_scraped_pages(result: &ScrapeResult, output_dir: &Path) -> Result<()> {
    let scrape_dir = output_dir.join(".scrape");
    fs::create_dir_all(&scrape_dir)?;

    let all_pages = &result.pages;
    #[allow(unused_mut)]
    // HashMap entry API requires &mut self for and_modify — no functional alternative
    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    let filenames: Vec<String> = all_pages
        .iter()
        .map(|page| {
            let current_count = slug_counts
                .entry(page.slug.clone())
                .and_modify(|count| *count = count.saturating_add(1))
                .or_insert(1);

            if *current_count == 1 {
                format!("{}.md", page.slug)
            } else {
                format!("{}-{}.md", page.slug, *current_count)
            }
        })
        .collect();

    let url_to_filename: HashMap<String, String> = all_pages
        .iter()
        .zip(filenames.iter())
        .map(|(page, filename)| (page.url.clone(), filename.clone()))
        .collect();

    let collision_count = slug_counts.values().filter(|count| **count > 1).count();
    if collision_count > 0 {
        eprintln!(
            "[WARN] Detected {collision_count} slug collision groups; applied numeric filename suffixes"
        );
    }

    let (link_to_urls, url_to_page) = build_link_indexes(all_pages);

    all_pages
        .iter()
        .zip(filenames.iter())
        .map(|(page, filename)| {
            let filepath = scrape_dir.join(filename);

            let toc = generate_toc(&page.headers);

            let related = find_related_pages_with_index(page, &link_to_urls, &url_to_page);

            let related_section = if related.is_empty() {
                String::new()
            } else {
                related
                    .iter()
                    .fold(String::from("\n## Related Pages\n\n"), |section, related_page| {
                        let related_link = url_to_filename
                            .get(&related_page.url)
                            .map_or_else(|| format!("{}.md", related_page.slug), Clone::clone);
                        format!("{section}- [{}]({related_link})\n", related_page.title)
                    })
            };

            let filter_status_str = match page.filter_status {
                PageFilterStatus::Filtered => "true",
                PageFilterStatus::Unfiltered => "false",
            };
            let content = format!(
                "---\nurl: {}\ntitle: {}\nword_count: {}\nfiltered: {}\nelements_removed: {}\ndensity_score: {:.2}\n---\n\n{}{}{}",
                page.url, page.title, page.word_count, filter_status_str, page.elements_removed, page.density_score,
                toc, page.markdown, related_section
            );

            (filepath, content)
        })
        .try_for_each(|(filepath, content)| {
            fs::write(filepath, content).map_err(anyhow::Error::from)
        })?;

    let manifest_file = std::fs::File::create(scrape_dir.join("manifest.json"))?;
    serde_json::to_writer_pretty(manifest_file, result)?;

    Ok(())
}
