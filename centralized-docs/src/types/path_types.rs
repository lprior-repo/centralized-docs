//! File path and URL slug newtypes.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

// ---------------------------------------------------------------------------
// FilePath
// ---------------------------------------------------------------------------

/// A newtype wrapper for validated file paths.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FilePath(String);

#[allow(dead_code)]
impl FilePath {
    /// Create a new `FilePath` from a `PathBuf` or string.
    pub fn new(path: impl Into<String>) -> Result<Self, FilePathError> {
        let s = path.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(FilePathError::Empty);
        }

        if trimmed.contains("..") {
            return Err(FilePathError::ContainsParentDirectory);
        }

        Ok(FilePath(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        std::path::Path::new(&self.0)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    #[must_use]
    pub fn filename(&self) -> Option<&str> {
        std::path::Path::new(&self.0)
            .file_name()
            .and_then(|name| name.to_str())
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for FilePath {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<std::path::PathBuf> for FilePath {
    fn from(path: std::path::PathBuf) -> Self {
        FilePath(path.to_string_lossy().into_owned())
    }
}

impl From<&std::path::Path> for FilePath {
    fn from(path: &std::path::Path) -> Self {
        FilePath(path.to_string_lossy().into_owned())
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum FilePathError {
    #[error("File path cannot be empty")]
    Empty,
    #[error("File path contains parent directory reference (..)")]
    ContainsParentDirectory,
}

// ---------------------------------------------------------------------------
// Slug
// ---------------------------------------------------------------------------

/// A newtype wrapper for URL-safe slugs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Slug(String);

#[allow(dead_code)]
impl Slug {
    /// Create a new Slug from a string.
    pub fn new(slug: impl Into<String>) -> Result<Self, SlugError> {
        let s = slug.into();
        let cleaned = s.trim().to_lowercase();

        if cleaned.is_empty() {
            return Err(SlugError::Empty);
        }

        if cleaned.len() > 200 {
            return Err(SlugError::TooLong(cleaned.len()));
        }

        if !cleaned
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(SlugError::InvalidCharacters);
        }

        Ok(Slug(cleaned))
    }

    /// Create a Slug from a title or name.
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        let slug = text
            .trim()
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .chars()
            .take(200)
            .collect::<String>();

        if slug.is_empty() {
            return Slug("untitled".to_string());
        }

        Slug(slug)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Slug {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Slug {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SlugError {
    #[error("Slug cannot be empty")]
    Empty,
    #[error("Slug too long: {0} characters (max 200)")]
    TooLong(usize),
    #[error("Slug contains invalid characters (only alphanumeric, hyphen, underscore allowed)")]
    InvalidCharacters,
}
