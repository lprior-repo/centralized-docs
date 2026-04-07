//! Document and chunk identifier newtypes.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// A newtype wrapper for document IDs that prevents accidental mixing with other string types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DocumentId(String);

impl DocumentId {
    /// Create a new `DocumentId`, validating that it's not empty.
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>) -> Result<Self, DocumentIdError> {
        let s = id.into();
        if s.trim().is_empty() {
            Err(DocumentIdError::Empty)
        } else {
            Ok(DocumentId(s))
        }
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

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for DocumentId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for DocumentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for DocumentId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur when creating `DocumentId`.
#[derive(Debug, Clone, thiserror::Error)]
#[allow(dead_code)]
pub enum DocumentIdError {
    #[error("Document ID cannot be empty")]
    Empty,
}

/// A newtype wrapper for chunk IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChunkId(String);

impl ChunkId {
    /// Create a new `ChunkId`, validating that it's not empty.
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>) -> Result<Self, ChunkIdError> {
        let s = id.into();
        if s.trim().is_empty() {
            Err(ChunkIdError::Empty)
        } else {
            Ok(ChunkId(s))
        }
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

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ChunkId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ChunkId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ChunkId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur when creating `ChunkId`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[allow(dead_code)]
pub enum ChunkIdError {
    #[error("Chunk ID cannot be empty")]
    Empty,
}
