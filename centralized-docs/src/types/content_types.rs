//! Tag and keyword newtypes for document classification.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// A newtype wrapper for tags.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tag(String);

impl Tag {
    /// Create a new Tag, validating that it's not empty and reasonably sized.
    #[allow(dead_code)]
    pub fn new(tag: impl Into<String>) -> Result<Self, TagError> {
        let s = tag.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(TagError::Empty);
        }

        if trimmed.len() > 100 {
            return Err(TagError::TooLong(trimmed.len()));
        }

        Ok(Tag(trimmed.to_lowercase()))
    }

    /// Get the underlying string value.
    #[allow(dead_code)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String.
    #[allow(dead_code)]
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Tag {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Tag {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur when creating Tag.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[allow(dead_code)]
pub enum TagError {
    #[error("Tag cannot be empty")]
    Empty,
    #[error("Tag too long: {0} characters (max 100)")]
    TooLong(usize),
}

/// A newtype wrapper for keywords.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Keyword(String);

impl Keyword {
    /// Create a new Keyword, validating that it's not empty and meets minimum length.
    #[allow(dead_code)]
    pub fn new(keyword: impl Into<String>) -> Result<Self, KeywordError> {
        let s = keyword.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(KeywordError::Empty);
        }

        if trimmed.len() < 2 {
            return Err(KeywordError::TooShort(trimmed.len()));
        }

        if trimmed.len() > 50 {
            return Err(KeywordError::TooLong(trimmed.len()));
        }

        Ok(Keyword(trimmed.to_lowercase()))
    }

    /// Get the underlying string value.
    #[allow(dead_code)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String.
    #[allow(dead_code)]
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Keyword {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Keyword {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur when creating Keyword.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[allow(dead_code)]
pub enum KeywordError {
    #[error("Keyword cannot be empty")]
    Empty,
    #[error("Keyword too short: {0} characters (min 2)")]
    TooShort(usize),
    #[error("Keyword too long: {0} characters (max 50)")]
    TooLong(usize),
}
