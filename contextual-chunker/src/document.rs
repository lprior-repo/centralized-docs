//! Input document representation for chunking.
//!
//! Design by Contract:
//! - Invariants: id and title are non-empty; content can be empty but is valid UTF-8
//! - Precondition: id must be a valid unique identifier (URL-safe)
//! - Postcondition: all fields immutable after construction

use serde::{Deserialize, Serialize};

/// Validation error for document construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentValidationError {
    /// Document id must be non-empty.
    EmptyId,
    /// Document title must be non-empty.
    EmptyTitle,
}

impl std::fmt::Display for DocumentValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentValidationError::EmptyId => f.write_str("document id must be non-empty"),
            DocumentValidationError::EmptyTitle => f.write_str("document title must be non-empty"),
        }
    }
}

impl std::error::Error for DocumentValidationError {}

/// A document to be chunked into semantic segments.
///
/// This is the primary input type for the chunking engine.
/// It encapsulates the minimal information needed for semantic chunking:
/// unique identification, presentation, and content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier for the document (e.g., file path, URL slug).
    pub id: String,
    /// Human-readable document title.
    pub title: String,
    /// The actual document content (markdown format recommended).
    pub content: String,
}

impl Document {
    /// Create a new document without validation.
    ///
    /// Prefer [`Document::try_new`] for validated construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use contextual_chunker::Document;
    ///
    /// let doc = Document::new(
    ///     "guides/intro".to_string(),
    ///     "Introduction Guide".to_string(),
    ///     "## Getting Started\n\nThis is an introduction...".to_string(),
    /// );
    /// assert_eq!(doc.id, "guides/intro");
    /// ```
    pub fn new(id: String, title: String, content: String) -> Self {
        Document { id, title, content }
    }

    /// Create a new document with validation (C6: parse at the boundary).
    ///
    /// Returns `Err(DocumentValidationError)` if id or title is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use contextual_chunker::Document;
    ///
    /// let doc = Document::try_new(
    ///     "guides/intro".to_string(),
    ///     "Introduction Guide".to_string(),
    ///     "## Getting Started\n\nContent.".to_string(),
    /// );
    /// assert!(doc.is_ok());
    ///
    /// let bad = Document::try_new("".to_string(), "Title".to_string(), "content".to_string());
    /// assert!(bad.is_err());
    /// ```
    pub fn try_new(
        id: String,
        title: String,
        content: String,
    ) -> Result<Self, DocumentValidationError> {
        if id.is_empty() {
            return Err(DocumentValidationError::EmptyId);
        }
        if title.is_empty() {
            return Err(DocumentValidationError::EmptyTitle);
        }
        Ok(Document { id, title, content })
    }

    /// Validate document has required fields.
    ///
    /// Returns true if id and title are non-empty.
    /// Content can be empty for test documents.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.title.is_empty()
    }

    /// Calculate rough content size in tokens (~4 characters per token).
    #[must_use]
    pub fn estimated_tokens(&self) -> usize {
        (self.content.len() / 4).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "test-doc".to_string(),
            "Test Document".to_string(),
            "This is test content.".to_string(),
        );
        assert_eq!(doc.id, "test-doc");
        assert_eq!(doc.title, "Test Document");
        assert!(doc.is_valid());
    }

    #[test]
    fn test_document_try_new_valid() {
        let doc = Document::try_new("id".to_string(), "title".to_string(), "content".to_string());
        assert!(doc.is_ok());
        assert_eq!(doc.as_ref().unwrap().id, "id");
    }

    #[test]
    fn test_document_try_new_empty_id() {
        let err = Document::try_new("".to_string(), "title".to_string(), "content".to_string());
        assert_eq!(err.unwrap_err(), DocumentValidationError::EmptyId);
    }

    #[test]
    fn test_document_try_new_empty_title() {
        let err = Document::try_new("id".to_string(), "".to_string(), "content".to_string());
        assert_eq!(err.unwrap_err(), DocumentValidationError::EmptyTitle);
    }

    #[test]
    fn test_document_try_new_both_empty() {
        let err = Document::try_new("".to_string(), "".to_string(), "content".to_string());
        assert_eq!(err.unwrap_err(), DocumentValidationError::EmptyId);
    }

    #[test]
    fn test_document_validation() {
        let valid_doc = Document::new("id".to_string(), "title".to_string(), "".to_string());
        assert!(valid_doc.is_valid());

        let invalid_id = Document::new("".to_string(), "title".to_string(), "content".to_string());
        assert!(!invalid_id.is_valid());

        let invalid_title = Document::new("id".to_string(), "".to_string(), "content".to_string());
        assert!(!invalid_title.is_valid());
    }

    #[test]
    fn test_token_estimation() {
        let doc = Document::new(
            "test".to_string(),
            "Test".to_string(),
            "This is a test with about sixteen characters".to_string(),
        );
        let tokens = doc.estimated_tokens();
        assert!(tokens > 0);
        assert!((10..=12).contains(&tokens)); // ~44 chars / 4
    }

    #[test]
    fn test_unicode_content() {
        let doc = Document::new(
            "unicode".to_string(),
            "Unicode Test".to_string(),
            "This contains emoji 🎉 and CJK 中文 characters".to_string(),
        );
        assert!(doc.is_valid());
        assert!(doc.estimated_tokens() > 0);
    }
}
