/// Configuration for llms.txt generation
#[derive(Debug, Clone)]
pub struct LlmsConfig {
    /// Project name (H1 in llms.txt)
    pub project_name: String,
    /// Brief project description (blockquote)
    pub project_description: String,
    /// Maximum documents per category in llms.txt (default: 5)
    pub max_per_category: usize,
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
            spec_version: "1.0".to_string(),
            project_version: "0.1.0".to_string(),
            include_frontmatter: true,
        }
    }
}
