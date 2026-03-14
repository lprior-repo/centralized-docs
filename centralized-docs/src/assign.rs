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
    let mut link_map = HashMap::new();
    let mut id_counts: HashMap<String, usize> = HashMap::new();

    for analysis in &analyses {
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

        let mut slug = slugify(&filename_stem);

        let unique_key = format!("{}/{}/{}", analysis.category, subcategory, slug);
        let count = id_counts.entry(unique_key.clone()).or_insert(0);
        *count = count.saturating_add(1);

        if *count > 1 {
            slug = format!("{slug}-{count}");
        }

        let doc_id = format!("{}/{}/{}", analysis.category, subcategory, slug);
        let new_filename = format!("{}-{}-{}.md", analysis.category, subcategory, slug);

        link_map.insert(
            analysis.source_path.clone(),
            IdMapping {
                id: doc_id,
                filename: new_filename,
                subcategory,
                slug,
            },
        );
    }

    (analyses, link_map)
}

/// Generate a URL-safe slug via the canonical `Slug` newtype
fn slugify(text: &str) -> String {
    Slug::from_text(text).into_string()
}
