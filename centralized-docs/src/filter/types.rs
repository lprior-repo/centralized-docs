/// Strategy for content filtering (PLAN.md requirement)
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(dead_code)] // Public API - exported for library users, not used internally
pub enum FilterStrategy {
    /// Use pruning heuristics (text/link density)
    #[default]
    Pruning,
    /// Use BM25 query-based filtering
    BM25,
    /// No filtering (keep all content)
    None,
}

/// Configuration for content filtering
#[derive(Debug, Clone)]
pub struct FilterConfig {
    /// Filtering strategy to use
    #[allow(dead_code)] // Public API - part of exported interface
    pub strategy: FilterStrategy,
    /// Minimum text density threshold (0.0 - 1.0)
    pub density_threshold: f32,
    /// Minimum word count to keep a section
    pub min_word_count: usize,
    /// Tags to always remove
    pub remove_tags: Vec<String>,
    /// CSS classes/IDs that indicate navigation (to remove)
    pub nav_patterns: Vec<String>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            strategy: FilterStrategy::default(),
            density_threshold: 0.45,
            min_word_count: 10,
            remove_tags: vec![
                "nav".to_string(),
                "footer".to_string(),
                "aside".to_string(),
                "script".to_string(),
                "style".to_string(),
                "noscript".to_string(),
                "iframe".to_string(),
            ],
            nav_patterns: vec![
                "nav".to_string(),
                "sidebar".to_string(),
                "footer".to_string(),
                "header".to_string(),
                "menu".to_string(),
                "breadcrumb".to_string(),
                "pagination".to_string(),
                "toc".to_string(),
                "table-of-contents".to_string(),
            ],
        }
    }
}

/// Result of content filtering
#[derive(Debug)]
#[allow(dead_code)] // Public API - exported for library users, not used internally
pub struct FilterResult {
    /// Cleaned HTML content (used in tests and for future filtering enhancements)
    #[allow(dead_code)] // Public API field
    pub html: String,
    /// Number of elements removed
    pub removed_count: usize,
    /// Density score of kept content
    pub density_score: crate::math_types::Score,
    /// Whether Readability was successfully used (vs fallback to custom pruning)
    #[allow(dead_code)] // Public API field
    pub used_readability: bool,
    /// Whether content extraction resulted in empty content
    #[allow(dead_code)] // Public API field
    pub is_empty: bool,
}

impl FilterResult {
    /// Create a new `FilterResult` with explicit `is_empty` field
    #[must_use]
    pub fn with_is_empty(self, is_empty: bool) -> Self {
        Self { is_empty, ..self }
    }
}
