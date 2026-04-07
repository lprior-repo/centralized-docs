//! Project name, category, and title newtypes.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

// ---------------------------------------------------------------------------
// ProjectName
// ---------------------------------------------------------------------------

/// A newtype wrapper for project names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectName(String);

#[allow(dead_code)]
impl ProjectName {
    /// Create a new `ProjectName`, validating format.
    pub fn new(name: impl Into<String>) -> Result<Self, ProjectNameError> {
        let s = name.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(ProjectNameError::Empty);
        }

        if trimmed.len() > 100 {
            return Err(ProjectNameError::TooLong(trimmed.len()));
        }

        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ')
        {
            return Err(ProjectNameError::InvalidCharacters);
        }

        Ok(ProjectName(trimmed.to_string()))
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

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ProjectName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ProjectName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProjectNameError {
    #[error("Project name cannot be empty")]
    Empty,
    #[error("Project name too long: {0} characters (max 100)")]
    TooLong(usize),
    #[error("Project name contains invalid characters (only alphanumeric, hyphen, underscore, space allowed)")]
    InvalidCharacters,
}

// ---------------------------------------------------------------------------
// Category
// ---------------------------------------------------------------------------

/// A newtype wrapper for category names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Category(String);

#[allow(dead_code)]
impl Category {
    /// Create a new Category.
    pub fn new(category: impl Into<String>) -> Result<Self, CategoryError> {
        let s = category.into();
        let trimmed = s.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err(CategoryError::Empty);
        }

        if trimmed.len() > 50 {
            return Err(CategoryError::TooLong(trimmed.len()));
        }

        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(CategoryError::InvalidCharacters);
        }

        Ok(Category(trimmed))
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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Category {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Category {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CategoryError {
    #[error("Category cannot be empty")]
    Empty,
    #[error("Category too long: {0} characters (max 50)")]
    TooLong(usize),
    #[error(
        "Category contains invalid characters (only alphanumeric, hyphen, underscore allowed)"
    )]
    InvalidCharacters,
}

// ---------------------------------------------------------------------------
// Title
// ---------------------------------------------------------------------------

/// A newtype wrapper for document titles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Title(String);

#[allow(dead_code)]
impl Title {
    /// Create a new Title.
    pub fn new(title: impl Into<String>) -> Result<Self, TitleError> {
        let s = title.into();
        let trimmed = s.trim().to_string();

        if trimmed.is_empty() {
            return Err(TitleError::Empty);
        }

        if trimmed.len() > 500 {
            return Err(TitleError::TooLong(trimmed.len()));
        }

        Ok(Title(trimmed))
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

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Title {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TitleError {
    #[error("Title cannot be empty")]
    Empty,
    #[error("Title too long: {0} characters (max 500)")]
    TooLong(usize),
}
