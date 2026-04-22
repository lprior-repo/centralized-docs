//! File path and URL slug newtypes.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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

// ---------------------------------------------------------------------------
// Bounded Filename Derivation
// ---------------------------------------------------------------------------

/// Derive a bounded filename from a stem, ensuring the result fits within
/// the 187-byte document filename budget.
///
/// Format: `{truncated_stem[:172]}-{hash8}.md` when truncation is needed.
/// When the stem is ≤ 172 bytes, returns `stem.to_string() + ".md"` directly.
///
/// The hash is computed from the FULL original stem (before truncation)
/// to guarantee collision resistance: two distinct stems produce distinct
/// hashes with overwhelming probability.
#[must_use]
pub fn bounded_name(stem: &str) -> String {
    const MAX_STEM_LEN: usize = 172;
    const HASH_SUFFIX_LEN: usize = 8;

    if stem.len() > MAX_STEM_LEN {
        // Compute SHA-256 of the full original stem for deterministic collision resistance
        let mut hasher = Sha256::new();
        hasher.update(stem.as_bytes());
        let hash_full = format!("{:x}", hasher.finalize());
        let hash_suffix = &hash_full[..HASH_SUFFIX_LEN];

        // Truncate stem and append hash suffix + extension
        // Result: truncated_stem (172) + '-' (1) + hash (8) + '.md' (3) = 184 bytes
        format!("{}-{}{}", &stem[..MAX_STEM_LEN], hash_suffix, ".md")
    } else {
        // No truncation needed; use natural name
        format!("{stem}.md")
    }
}

/// Derive a bounded chunk filename from a chunk stem and level suffix,
/// ensuring the result fits within the 200-byte chunk filename budget.
///
/// Format: `{truncated_stem[:172]}-{hash8}-{level}.md` when truncation is needed.
/// When the stem is ≤ 172 bytes, returns `stem.to_string() + "-{level}.md"` directly.
///
/// The hash is computed from the FULL original stem (before truncation)
/// to guarantee collision resistance.
#[must_use]
pub fn bounded_chunk_name(stem: &str, level: &str) -> String {
    const MAX_CHUNK_STEM_LEN: usize = 172;
    const HASH_SUFFIX_LEN: usize = 8;

    if stem.len() > MAX_CHUNK_STEM_LEN {
        let mut hasher = Sha256::new();
        hasher.update(stem.as_bytes());
        let hash_full = format!("{:x}", hasher.finalize());
        let hash_suffix = &hash_full[..HASH_SUFFIX_LEN];

        // Result: truncated_stem (172) + '-' (1) + hash (8) + '-' (1) + level + '.md' (3)
        // Max: 172 + 1 + 8 + 1 + 8 + 3 = 193 bytes (for level "standard")
        format!(
            "{}-{}-{}.md",
            &stem[..MAX_CHUNK_STEM_LEN],
            hash_suffix,
            level
        )
    } else {
        format!("{stem}-{level}.md")
    }
}
