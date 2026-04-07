//! Tests for all types module newtypes.

use super::*;

#[test]
fn test_documentid_valid() {
    let result = DocumentId::new("doc1");
    assert!(matches!(result, Ok(ref id) if id.as_str() == "doc1" && id.to_string() == "doc1"));
}

#[test]
fn test_documentid_empty() {
    let result = DocumentId::new("");
    assert!(result.is_err());
    assert!(matches!(result, Err(DocumentIdError::Empty)));
}

#[test]
fn test_documentid_whitespace_only() {
    let result = DocumentId::new("   ");
    assert!(result.is_err());
}

#[test]
fn test_chunkid_valid() {
    let result = ChunkId::new("chunk_1");
    assert!(matches!(result, Ok(ref id) if id.as_str() == "chunk_1"));
}

#[test]
fn test_tag_valid() {
    let result = Tag::new("Rust");
    assert!(matches!(result, Ok(ref tag) if tag.as_str() == "rust"));
}

#[test]
fn test_tag_case_insensitive() {
    let result = Tag::new("RUST");
    assert!(matches!(result, Ok(ref tag) if tag.as_str() == "rust"));
}

#[test]
fn test_tag_too_long() {
    let long_tag = "a".repeat(101);
    let result = Tag::new(long_tag);
    assert!(result.is_err());
    assert!(matches!(result, Err(TagError::TooLong(_))));
}

#[test]
fn test_keyword_valid() {
    let result = Keyword::new("function");
    assert!(matches!(result, Ok(ref kw) if kw.as_str() == "function"));
}

#[test]
fn test_keyword_too_short() {
    let result = Keyword::new("a");
    assert!(result.is_err());
    assert!(matches!(result, Err(KeywordError::TooShort(1))));
}

#[test]
fn test_keyword_too_long() {
    let long_kw = "a".repeat(51);
    let result = Keyword::new(long_kw);
    assert!(result.is_err());
    assert!(matches!(result, Err(KeywordError::TooLong(_))));
}

#[test]
fn test_keyword_case_insensitive() {
    let result = Keyword::new("FUNCTION");
    assert!(matches!(result, Ok(ref kw) if kw.as_str() == "function"));
}

#[test]
fn test_project_name_valid() {
    let result = ProjectName::new("My Project");
    assert!(matches!(result, Ok(ref name) if name.as_str() == "My Project"));
}

#[test]
fn test_project_name_empty() {
    let result = ProjectName::new("");
    assert!(result.is_err());
    assert!(matches!(result, Err(ProjectNameError::Empty)));
}

#[test]
fn test_project_name_invalid_chars() {
    let result = ProjectName::new("Test@Project");
    assert!(result.is_err());
    assert!(matches!(result, Err(ProjectNameError::InvalidCharacters)));
}

#[test]
fn test_category_valid() {
    let result = Category::new("tutorial");
    assert!(matches!(result, Ok(ref cat) if cat.as_str() == "tutorial"));
}

#[test]
fn test_category_case_insensitive() {
    let result = Category::new("TUTORIAL");
    assert!(matches!(result, Ok(ref cat) if cat.as_str() == "tutorial"));
}

#[test]
fn test_category_empty() {
    let result = Category::new("");
    assert!(result.is_err());
}

#[test]
fn test_title_valid() {
    let result = Title::new("Getting Started with Rust");
    assert!(matches!(result, Ok(ref title) if title.as_str() == "Getting Started with Rust"));
}

#[test]
fn test_title_empty() {
    let result = Title::new("");
    assert!(result.is_err());
}

#[test]
fn test_slug_from_text() {
    let slug = Slug::from_text("Hello World!");
    assert_eq!(slug.as_str(), "hello-world");
}

#[test]
fn test_slug_from_empty_text() {
    let slug = Slug::from_text("   ");
    assert_eq!(slug.as_str(), "untitled");
}

#[test]
fn test_slug_valid() {
    let result = Slug::new("my-document-slug");
    assert!(matches!(result, Ok(ref slug) if slug.as_str() == "my-document-slug"));
}

#[test]
fn test_slug_invalid_chars() {
    let result = Slug::new("invalid@slug!");
    assert!(result.is_err());
}

#[test]
fn test_filepath_valid() {
    let result = FilePath::new("docs/tutorial.md");
    assert!(matches!(result, Ok(ref path) if path.as_str() == "docs/tutorial.md"));
}

#[test]
fn test_filepath_contains_parent() {
    let result = FilePath::new("../etc/passwd");
    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(FilePathError::ContainsParentDirectory)
    ));
}
