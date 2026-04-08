#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

//! Semantic embedding providers for document vectors.
//!
//! This module provides infrastructure for generating semantic embeddings from text
//! using various providers (`OpenAI`, Cohere, etc.) and using them with the HNSW index.
//!
//! # Architecture
//!
//! - [`EmbeddingProvider`]: Trait for embedding providers
//! - [`Embedding`]: Wrapper around embedding vectors with metadata
//! - [`OpenAIProvider`]: `OpenAI` text embedding API
//! - [`CohereProvider`]: Cohere embedding API
//!
//! # Usage
//!
//! ```no_run
//! use doc_transformer::embeddings::{EmbeddingProvider, OpenAIProvider, Embedding};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let provider = OpenAIProvider::new("your-api-key")?;
//!     let embeddings = provider.embed_texts(&["Hello world", "Rust is fast"]).await?;
//!     Ok(())
//! }
//! ```

mod cohere;
mod openai;

pub use cohere::CohereProvider;
pub use openai::OpenAIProvider;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingVector(pub Vec<f32>);

/// Errors that can occur during embedding operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EmbeddingProviderError {
    #[error("API request failed: {message}")]
    ApiError {
        message: String,
        status_code: Option<u16>,
    },

    #[error("Rate limited: retry after {retry_after}s")]
    RateLimited { retry_after: u64 },

    #[error("Invalid API response: {message}")]
    InvalidResponse { message: String },

    #[error("JSON serialization error: {message}")]
    JsonError { message: String },

    #[error("API key not configured")]
    MissingApiKey,

    #[error("API key has invalid format")]
    InvalidApiKeyFormat,

    #[error("Content-Type header has invalid format")]
    InvalidContentType,

    #[error("API URL not configured")]
    MissingApiUrl,

    #[error("Text too long: {length} tokens (max {max})")]
    TextTooLong { length: usize, max: usize },

    #[error("No texts provided")]
    EmptyInput,
}

/// A single embedding vector with metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding {
    /// The embedding vector.
    pub vector: Vec<f32>,

    /// Optional text that generated this embedding.
    pub text: Option<String>,

    /// The model used to generate this embedding.
    pub model: String,
}

impl Embedding {
    /// Create a new embedding.
    #[must_use]
    pub fn new(vector: Vec<f32>, text: Option<String>, model: String) -> Self {
        Self {
            vector,
            text,
            model,
        }
    }

    /// Get the dimension of the embedding.
    #[inline]
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.vector.len()
    }

    /// Check if the embedding is valid (non-empty, no NaN/Infinity).
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.vector.is_empty() && self.vector.iter().all(|v| !v.is_nan() && !v.is_infinite())
    }
}

/// Trait for text embedding providers.
///
/// Implement this trait to add support for new embedding APIs.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Get the provider name.
    fn name(&self) -> &str;

    /// Get the embedding model name.
    fn model(&self) -> &str;

    /// Get the embedding dimension for this provider/model.
    fn dimension(&self) -> usize;

    /// Generate embeddings for a batch of texts.
    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError>;

    /// Estimate the number of tokens in a text.
    fn estimate_tokens(&self, text: &str) -> usize;
}

/// Common configuration for embedding providers.
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    /// API key for the provider.
    pub api_key: String,

    /// API endpoint URL (optional, defaults to provider default).
    pub api_url: Option<String>,

    /// Request timeout in seconds.
    pub timeout: u64,

    /// Maximum tokens per request.
    pub max_tokens_per_batch: usize,

    /// Maximum batch size (number of texts).
    pub batch_size: usize,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_url: None,
            timeout: 60,
            max_tokens_per_batch: 8000,
            batch_size: 100,
        }
    }
}

/// Convert embeddings to the format expected by HNSW index.
#[must_use]
pub fn embeddings_to_vectors(embeddings: &[Embedding]) -> Vec<Vec<f32>> {
    embeddings.iter().map(|e| e.vector.clone()).collect()
}

/// Local text embedding provider using FastEmbed.
#[cfg(feature = "localembed")]
pub mod local;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
