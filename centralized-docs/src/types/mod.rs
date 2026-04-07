//! Strongly-typed identifiers and domain types for the document transformer.

pub mod symbols;

mod config_types;
mod content_types;
mod identifiers;
mod naming_types;
mod path_types;
#[cfg(test)]
mod types_tests;

// Re-export all public types so external consumers see the same API
pub use config_types::{ConfigError, HnswEfConstruction, HnswM, MaxRelatedChunks};
pub use content_types::{Keyword, KeywordError, Tag, TagError};
pub use identifiers::{ChunkId, ChunkIdError, DocumentId, DocumentIdError};
pub use naming_types::{Category, CategoryError, ProjectName, ProjectNameError, Title, TitleError};
pub use path_types::{FilePath, FilePathError, Slug, SlugError};

/// Stopwords to filter out from tags and keywords.
///
/// Used across multiple modules to ensure consistent filtering.
pub const STOPWORDS: [&str; 10] = [
    "this", "that", "these", "those", "about", "guide", "the", "and", "or", "for",
];

/// Check if a word is a stopword (case-insensitive).
///
/// Used in tag and keyword extraction to filter common words.
#[must_use]
pub fn is_stopword(word: &str) -> bool {
    STOPWORDS.contains(&word.to_lowercase().as_str())
}
