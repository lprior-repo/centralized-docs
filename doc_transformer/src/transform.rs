use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct TransformResult {
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
}

pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let docs_dir = output_dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let mut success_count = 0;
    let mut error_count = 0;

    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            match transform_file(analysis, mapping, &docs_dir) {
                Ok(_) => success_count += 1,
                Err(e) => {
                    eprintln!("TRANSFORM ERROR: {}: {}", analysis.source_path, e);
                    error_count += 1;
                }
            }
        }
    }

    Ok(TransformResult {
        success_count,
        total_count: analyses.len(),
        error_count,
    })
}

fn transform_file(analysis: &Analysis, mapping: &IdMapping, docs_dir: &Path) -> Result<()> {
    let doc_id = &mapping.id;
    let filename = &mapping.filename;

    // Generate frontmatter
    let tags = generate_tags(analysis);
    let tags_str = tags
        .iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");

    let frontmatter = format!(
        "---\nid: {}\ntitle: {}\ncategory: {}\ntags: [{}]\n---",
        doc_id, analysis.title, analysis.category, tags_str
    );

    // Build context block
    let context_text = if analysis.first_paragraph.is_empty() {
        analysis.title.clone()
    } else {
        analysis.first_paragraph[..std::cmp::min(150, analysis.first_paragraph.len())]
            .to_string()
    };

    let context = format!("> **Context**: {}", context_text);

    // Add See Also section
    let see_also = "## See Also\n\n- [Documentation Index](./COMPASS.md)\n";

    // Assemble final content
    let final_content = format!(
        "{}\n\n# {}\n\n{}\n\n{}\n\n{}\n",
        frontmatter, analysis.title, context, analysis.content, see_also
    );

    // Write file
    let output_file = docs_dir.join(filename);
    fs::write(output_file, final_content)?;

    Ok(())
}

fn generate_tags(analysis: &Analysis) -> Vec<String> {
    let mut tags = vec![analysis.category.clone()];

    // Add heading nouns
    for heading in analysis.headings.iter().take(3) {
        for word in heading.text.split_whitespace() {
            if word.len() > 4 && !is_stopword(word) {
                tags.push(word.to_lowercase());
            }
        }
    }

    // Unique and limit to 5
    tags.sort();
    tags.dedup();
    tags.truncate(5);
    tags
}

fn is_stopword(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "this" | "that" | "these" | "those" | "about" | "guide"
    )
}
