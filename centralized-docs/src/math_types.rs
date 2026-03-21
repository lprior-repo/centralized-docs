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
        #[allow(clippy::unwrap_used)]
        let not_nan = match NotNan::new(sum) {
            Ok(v) => v,
            Err(_) => Default::default(),
        };
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn math_error_display() {
        assert_eq!(
            MathError::NotFinite("X").to_string(),
            "X must be finite (no NaN or Infinity)"
        );
        assert_eq!(MathError::Negative("X").to_string(), "X cannot be negative");
        assert_eq!(
            MathError::Zero("X").to_string(),
            "X must be greater than zero"
        );
    }

    #[test]
    fn math_error_debug() {
        assert!(format!("{:?}", MathError::NotFinite("X")).contains("NotFinite"));
        assert!(format!("{:?}", MathError::Negative("X")).contains("Negative"));
        assert!(format!("{:?}", MathError::Zero("X")).contains("Zero"));
    }

    #[test]
    fn math_error_is_error() {
        let err: Box<dyn std::error::Error> = Box::new(MathError::NotFinite("test"));
        let _ = err.to_string();
    }

    #[test]
    fn score_zero() {
        let s = Score::zero();
        assert_eq!(s.value(), 0.0);
    }

    #[test]
    fn score_try_new_valid() {
        let s = Score::try_new(1.5).unwrap();
        assert_eq!(s.value(), 1.5);
    }

    #[test]
    fn score_try_new_zero() {
        let s = Score::try_new(0.0).unwrap();
        assert_eq!(s.value(), 0.0);
    }

    #[test]
    fn score_try_new_negative() {
        assert!(matches!(
            Score::try_new(-1.0),
            Err(MathError::Negative("Score"))
        ));
    }

    #[test]
    fn score_try_new_nan() {
        assert!(matches!(
            Score::try_new(f32::NAN),
            Err(MathError::NotFinite("Score"))
        ));
    }

    #[test]
    fn score_try_new_infinity() {
        assert!(matches!(
            Score::try_new(f32::INFINITY),
            Err(MathError::NotFinite("Score"))
        ));
        assert!(matches!(
            Score::try_new(f32::NEG_INFINITY),
            Err(MathError::NotFinite("Score"))
        ));
    }

    #[test]
    fn score_add() {
        let a = Score::try_new(2.0).unwrap();
        let b = Score::try_new(3.0).unwrap();
        let c = a + b;
        assert_eq!(c.value(), 5.0);
    }

    #[test]
    fn score_add_zero() {
        let a = Score::try_new(1.5).unwrap();
        let b = Score::zero();
        let c = a + b;
        assert_eq!(c.value(), 1.5);
    }

    #[test]
    fn score_sum() {
        let scores: Vec<Score> = vec![
            Score::try_new(1.0).unwrap(),
            Score::try_new(2.0).unwrap(),
            Score::try_new(3.0).unwrap(),
        ];
        let total: Score = scores.into_iter().sum();
        assert_eq!(total.value(), 6.0);
    }

    #[test]
    fn score_sum_empty() {
        let scores: Vec<Score> = vec![];
        let total: Score = scores.into_iter().sum();
        assert_eq!(total.value(), 0.0);
    }

    #[test]
    fn score_ord() {
        let a = Score::try_new(1.0).unwrap();
        let b = Score::try_new(2.0).unwrap();
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn score_clone_copy() {
        let s = Score::try_new(42.0).unwrap();
        let s2 = s;
        assert_eq!(s.value(), s2.value());
    }

    #[test]
    fn score_debug() {
        let s = Score::try_new(1.0).unwrap();
        let dbg = format!("{:?}", s);
        assert!(dbg.contains("Score"));
    }

    #[test]
    fn term_frequency_zero_const() {
        assert_eq!(TermFrequency::ZERO.value(), 0);
    }

    #[test]
    fn term_frequency_new() {
        let tf = TermFrequency::new(42);
        assert_eq!(tf.value(), 42);
    }

    #[test]
    fn term_frequency_try_new() {
        let tf = TermFrequency::try_new(10).unwrap();
        assert_eq!(tf.value(), 10);
    }

    #[test]
    fn term_frequency_ord() {
        let a = TermFrequency::new(1);
        let b = TermFrequency::new(10);
        assert!(a < b);
    }

    #[test]
    fn document_length_zero_const() {
        assert_eq!(DocumentLength::ZERO.value(), 0);
    }

    #[test]
    fn document_length_new() {
        let dl = DocumentLength::new(100);
        assert_eq!(dl.value(), 100);
    }

    #[test]
    fn document_length_try_new() {
        let dl = DocumentLength::try_new(50).unwrap();
        assert_eq!(dl.value(), 50);
    }

    #[test]
    fn average_document_length_default() {
        assert_eq!(AverageDocumentLength::DEFAULT.value(), 1.0);
    }

    #[test]
    fn average_document_length_try_new_valid() {
        let adl = AverageDocumentLength::try_new(250.0).unwrap();
        assert_eq!(adl.value(), 250.0);
    }

    #[test]
    fn average_document_length_try_new_negative() {
        assert!(matches!(
            AverageDocumentLength::try_new(-1.0),
            Err(MathError::Zero("AverageDocumentLength"))
        ));
    }

    #[test]
    fn average_document_length_try_new_zero() {
        assert!(matches!(
            AverageDocumentLength::try_new(0.0),
            Err(MathError::Zero("AverageDocumentLength"))
        ));
    }

    #[test]
    fn average_document_length_try_new_nan() {
        assert!(matches!(
            AverageDocumentLength::try_new(f32::NAN),
            Err(MathError::NotFinite("AverageDocumentLength"))
        ));
    }

    #[test]
    fn average_document_length_safe_new_valid() {
        let adl = AverageDocumentLength::safe_new(100.0);
        assert_eq!(adl.value(), 100.0);
    }

    #[test]
    fn average_document_length_safe_new_zero() {
        let adl = AverageDocumentLength::safe_new(0.0);
        assert_eq!(adl.value(), 1.0);
    }

    #[test]
    fn average_document_length_safe_new_negative() {
        let adl = AverageDocumentLength::safe_new(-5.0);
        assert_eq!(adl.value(), 1.0);
    }

    #[test]
    fn average_document_length_safe_new_nan() {
        let adl = AverageDocumentLength::safe_new(f32::NAN);
        assert_eq!(adl.value(), 1.0);
    }

    #[test]
    fn average_document_length_safe_new_infinity() {
        let adl = AverageDocumentLength::safe_new(f32::INFINITY);
        assert_eq!(adl.value(), 1.0);
    }

    #[test]
    fn average_document_length_partial_ord() {
        let a = AverageDocumentLength::try_new(10.0).unwrap();
        let b = AverageDocumentLength::try_new(20.0).unwrap();
        assert!(a < b);
    }

    #[test]
    fn total_documents_new() {
        let td = TotalDocuments::new(500);
        assert_eq!(td.value(), 500);
    }

    #[test]
    fn total_documents_ord() {
        let a = TotalDocuments::new(10);
        let b = TotalDocuments::new(20);
        assert!(a < b);
    }

    #[test]
    fn document_frequency_new() {
        let df = DocumentFrequency::new(42);
        assert_eq!(df.value(), 42);
    }

    #[test]
    fn document_frequency_ord() {
        let a = DocumentFrequency::new(5);
        let b = DocumentFrequency::new(10);
        assert!(a < b);
    }
}
