use crate::analyze::Analysis;
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

pub fn assign_ids(analyses: Vec<Analysis>) -> (Vec<Analysis>, HashMap<String, IdMapping>) {
    let mut link_map = HashMap::new();
    let mut id_counts: HashMap<String, usize> = HashMap::new();

    for analysis in &analyses {
        let parts: Vec<&str> = analysis.source_path.split('/').collect();
        let subcategory = if parts.len() > 1 {
            parts[parts.len() - 2].to_lowercase()
        } else {
            "general".to_string()
        };

        let filename_stem = Path::new(&analysis.source_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let mut slug = slugify(&filename_stem);

        let unique_key = format!("{}/{}/{}", analysis.category, subcategory, slug);
        let count = id_counts.entry(unique_key.clone()).or_insert(0);
        *count += 1;

        if *count > 1 {
            slug = format!("{}-{}", slug, count);
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

fn slugify(text: &str) -> String {
    let slug = text.to_lowercase();
    let slug = slug
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>();

    slug.split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .take(40)
        .collect()
}
