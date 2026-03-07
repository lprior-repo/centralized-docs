#![allow(dead_code)]

//! Type-driven mathematics for ranking algorithms.
//!
//! Enforces invariants using newtypes so illegal states (like NaN scores or negative frequencies)
//! are unrepresentable.

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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Score(f32);

impl Score {
    pub const ZERO: Self = Self(0.0);

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("Score"));
        }
        if value < 0.0 {
            return Err(MathError::Negative("Score"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

/// A finite, non-negative term frequency.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TermFrequency(f32);

impl TermFrequency {
    pub const ZERO: Self = Self(0.0);

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("TermFrequency"));
        }
        if value < 0.0 {
            return Err(MathError::Negative("TermFrequency"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

/// A finite, non-negative document length.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DocumentLength(f32);

impl DocumentLength {
    pub const ZERO: Self = Self(0.0);

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("DocumentLength"));
        }
        if value < 0.0 {
            return Err(MathError::Negative("DocumentLength"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f32 {
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

    pub fn safe_new(value: f32) -> Self {
        if !value.is_finite() || value <= 0.0 {
            Self(1.0)
        } else {
            Self(value)
        }
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

/// A finite inverse document frequency.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct InverseDocumentFrequency(f32);

impl InverseDocumentFrequency {
    pub const ONE: Self = Self(1.0);

    pub fn try_new(value: f32) -> Result<Self, MathError> {
        if !value.is_finite() {
            return Err(MathError::NotFinite("InverseDocumentFrequency"));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

/// A mathematically pure pseudo-BM25 function that takes strongly-typed arguments.
///
/// Computes `idf * ((tf * (k1 + 1)) / (tf + k1 * (1 - b + b * (doc_len / avg_doc_len))))`.
#[must_use]
pub fn pure_bm25(
    tf: TermFrequency,
    doc_len: DocumentLength,
    avg_doc_len: AverageDocumentLength,
    idf: InverseDocumentFrequency,
) -> Score {
    let k1 = 1.2_f32;
    let b = 0.75_f32;

    let tf_val = tf.value();
    let doc_len_val = doc_len.value();
    let avg_doc_len_val = avg_doc_len.value();
    let idf_val = idf.value();

    let numerator = tf_val * (k1 + 1.0);
    let denominator = tf_val + k1 * (1.0 - b + b * (doc_len_val / avg_doc_len_val));

    let raw_score = idf_val * (numerator / denominator.max(0.0001));

    Score::try_new(raw_score).unwrap_or(Score::ZERO)
}
