#![allow(dead_code)]

//! Type-driven mathematics for ranking algorithms.
//!
//! Enforces invariants using newtypes so illegal states (like `NaN` scores or negative frequencies)
//! are unrepresentable.

use ordered_float::NotNan;
use std::fmt;

/// Error for mathematical invariants
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    NotFinite(&'static str),
    Negative(&'static str),
    Zero(&'static str),
}

impl std::error::Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite(name) => write!(f, "{name} must be finite (no NaN or Infinity)"),
            Self::Negative(name) => write!(f, "{name} cannot be negative"),
            Self::Zero(name) => write!(f, "{name} must be greater than zero"),
        }
    }
}

/// A finite, non-negative ranking score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(NotNan<f32>);

impl Score {
    #[must_use]
    pub fn zero() -> Self {
        Self(NotNan::default())
    }

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("Score"));
        }
        if value < 0.0 {
            return Err(MathError::Negative("Score"));
        }
        let not_nan = NotNan::new(value).map_err(|_| MathError::NotFinite("Score"))?;
        Ok(Self(not_nan))
    }

    #[must_use]
    pub fn value(self) -> f32 {
        self.0.into_inner()
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0.into_inner() + rhs.0.into_inner();
        let not_nan = NotNan::new(sum).unwrap_or_default();
        Self(not_nan)
    }
}

impl std::iter::Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Score::zero(), std::ops::Add::add)
    }
}

/// A non-negative term frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermFrequency(u32);

impl TermFrequency {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn try_new(value: u32) -> Result<Self, MathError> {
        Ok(Self(value))
    }

    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
}

/// A non-negative document length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentLength(u32);

impl DocumentLength {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn try_new(value: u32) -> Result<Self, MathError> {
        Ok(Self(value))
    }

    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
}

/// A finite, positive average document length.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AverageDocumentLength(f32);

impl AverageDocumentLength {
    pub const DEFAULT: Self = Self(1.0);

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("AverageDocumentLength"));
        }
        if value <= 0.0 {
            return Err(MathError::Zero("AverageDocumentLength"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn safe_new(value: f32) -> Self {
        if !value.is_finite() || value <= 0.0 {
            Self(1.0)
        } else {
            Self(value)
        }
    }

    #[must_use]
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Total number of documents in the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TotalDocuments(u32);

impl TotalDocuments {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Number of documents containing the term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentFrequency(u32);

impl DocumentFrequency {
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
}
