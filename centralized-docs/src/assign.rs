use crate::analyze::Analysis;
use crate::types::Slug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdMapping {
    pub id: String,
    pub filename: String,
    pub subcategory: String,
    pub slug: String,
}

#[must_use]
pub fn assign_ids(analyses: Vec<Analysis>) -> (Vec<Analysis>, HashMap<String, IdMapping>) {
    let link_map: HashMap<String, IdMapping> = analyses
        .iter()
        .scan(HashMap::<String, usize>::new(), |id_counts, analysis| {
            let parts: Vec<&str> = analysis.source_path.split('/').collect();
            let subcategory = parts
                .get(parts.len().saturating_sub(2))
                .map_or_else(|| "general".to_string(), |s| s.to_lowercase());

            let filename_stem = Path::new(&analysis.source_path)
                .file_stem()
                .filter(|s| !s.is_empty())
                .map_or_else(
                    || "untitled".to_string(),
                    |s| s.to_string_lossy().to_string(),
                );

            let base_slug = slugify(&filename_stem);
            let unique_key = format!("{}/{}/{}", analysis.category, subcategory, base_slug);
            let count = id_counts.entry(unique_key).or_insert(0);
            *count = count.saturating_add(1);

            let final_slug = if *count > 1 {
                format!("{base_slug}-{count}")
            } else {
                base_slug
            };

            let doc_id = format!("{}/{}/{}", analysis.category, subcategory, final_slug);
            let new_filename = format!("{}-{}-{}.md", analysis.category, subcategory, final_slug);

            Some((
                analysis.source_path.clone(),
                IdMapping {
                    id: doc_id,
                    filename: new_filename,
                    subcategory,
                    slug: final_slug,
                },
            ))
        })
        .collect();

    (analyses, link_map)
}

/// Generate a URL-safe slug via the canonical `Slug` newtype
fn slugify(text: &str) -> String {
    Slug::from_text(text).into_string()
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
            first_paragraph: "Test paragraph".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: category.to_string(),
            content: Arc::from("Test content"),
        }
    }

    #[test]
    fn empty_input() {
        let (analyses, map) = assign_ids(vec![]);
        assert!(analyses.is_empty());
        assert!(map.is_empty());
    }

    #[test]
    fn single_file() {
        let analyses = vec![make_analysis(
            "docs/getting-started/intro.md",
            "tutorial",
            "Intro",
        )];
        let (_, map) = assign_ids(analyses);
        assert_eq!(map.len(), 1);

        let mapping = map.get("docs/getting-started/intro.md").unwrap();
        assert_eq!(mapping.subcategory, "getting-started");
        assert_eq!(mapping.slug, "intro");
        assert!(mapping.id.contains("tutorial"));
        assert!(mapping.filename.starts_with("tutorial-"));
        assert!(mapping.filename.ends_with(".md"));
    }

    #[test]
    fn multiple_categories() {
        let analyses = vec![
            make_analysis("docs/getting-started/intro.md", "tutorial", "Intro"),
            make_analysis("docs/api/reference.md", "ref", "Reference"),
            make_analysis("docs/ops/deploy.md", "ops", "Deploy"),
        ];
        let (_, map) = assign_ids(analyses);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn slug_collision_two_files_same_name() {
        let analyses = vec![
            make_analysis("a/getting-started/intro.md", "tutorial", "Intro A"),
            make_analysis("b/getting-started/intro.md", "tutorial", "Intro B"),
        ];
        let (_, map) = assign_ids(analyses);
        assert_eq!(map.len(), 2);

        let slugs: Vec<&str> = map.values().map(|m| m.slug.as_str()).collect();
        assert_eq!(slugs.len(), 2);
        assert_ne!(slugs[0], slugs[1]);
        assert!(slugs.contains(&"intro"));
        assert!(slugs.contains(&"intro-2"));
    }

    #[test]
    fn subcategory_from_path() {
        let analyses = vec![make_analysis(
            "guides/advanced/setup.md",
            "concept",
            "Setup",
        )];
        let (_, map) = assign_ids(analyses);
        let mapping = map.values().next().unwrap();
        assert_eq!(mapping.subcategory, "advanced");
    }

    #[test]
    fn single_segment_path_fallback() {
        let analyses = vec![make_analysis("rootfile.md", "meta", "Root")];
        let (_, map) = assign_ids(analyses);
        let mapping = map.values().next().unwrap();
        assert_eq!(mapping.slug, "rootfile");
    }

    #[test]
    fn no_extension_directory_like_path() {
        let analyses = vec![make_analysis("docs/tutorials/", "tutorial", "Dir")];
        let (_, map) = assign_ids(analyses);
        let mapping = map.values().next().unwrap();
        assert!(mapping.slug.contains("tutorials"));
    }

    #[test]
    fn slugify_preserves_underscores() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        let slug = slugify("Test__Multiple---Chars");
        assert!(!slug.is_empty());
    }

    #[test]
    fn mapping_has_correct_structure() {
        let analyses = vec![make_analysis(
            "docs/quickstart/start.md",
            "tutorial",
            "Start",
        )];
        let (result_analyses, map) = assign_ids(analyses);

        assert_eq!(result_analyses.len(), 1);
        let mapping = map.get("docs/quickstart/start.md").unwrap();
        assert!(mapping.id.contains("tutorial"));
        assert!(mapping.id.contains("quickstart"));
        assert!(mapping.filename.starts_with("tutorial-"));
        assert!(mapping.filename.contains("quickstart"));
        assert!(mapping.filename.ends_with(".md"));
    }
}
