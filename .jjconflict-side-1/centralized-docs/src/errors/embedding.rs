//! Embedding and similarity search error types.

use thiserror::Error;

/// Embedding and similarity search errors.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EmbeddingError {
    #[error("failed to generate embedding: {message}")]
    GenerationFailed { message: String },

    #[error("embedding API error: {message}")]
    ApiError {
        message: String,
        status_code: Option<u16>,
    },

    #[error("embedding rate limited: retry after {retry_after}s")]
    RateLimited { retry_after: u64 },

    #[error("embedding dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("invalid embedding: contains NaN or Infinity")]
    InvalidEmbedding,

    #[error("no embedding provider configured")]
    NoProviderConfigured,

    #[error("embedding provider not supported: {provider}")]
    UnsupportedProvider { provider: String },
}
