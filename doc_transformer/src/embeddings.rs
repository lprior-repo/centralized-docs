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

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use thiserror::Error;

pub use crate::similarity::{
    build_index, build_index_with_params, query_neighbors, HnswIndex, SimilarityError,
};

/// Errors that can occur during embedding operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
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
    ///
    /// # Arguments
    ///
    /// * `texts` - The texts to embed
    ///
    /// # Returns
    ///
    /// A vector of embeddings, one for each input text.
    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError>;

    /// Estimate the number of tokens in a text.
    ///
    /// Different providers use different tokenization schemes.
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

/// `OpenAI` text embedding provider.
///
/// Supports text-embedding-3-small, text-embedding-3-large, and text-embedding-ada-002.
#[derive(Debug, Clone)]
pub struct OpenAIProvider {
    config: EmbeddingConfig,
    client: Client,
    model: String,
}

impl OpenAIProvider {
    /// Create a new `OpenAI` provider.
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingProviderError::MissingApiKey` if the API key is empty.
    /// Returns `EmbeddingProviderError::ApiError` if the HTTP client fails to build.
    pub fn new(api_key: impl Into<String>) -> Result<Self, EmbeddingProviderError> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(EmbeddingProviderError::MissingApiKey);
        }

        let config = EmbeddingConfig {
            api_key,
            api_url: Some("https://api.openai.com/v1/embeddings".to_string()),
            ..Default::default()
        };

        let client = Client::builder()
            .timeout(std::time::Duration::from_mins(1))
            .build()
            .map_err(|e| EmbeddingProviderError::ApiError {
                message: e.to_string(),
                status_code: None,
            })?;

        Ok(Self {
            config,
            client,
            model: "text-embedding-3-small".to_string(),
        })
    }

    /// Create a provider with a specific model.
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingProviderError` if the underlying provider fails to initialize
    /// (e.g., missing API key, invalid client configuration).
    pub fn with_model(
        api_key: impl Into<String>,
        model: &str,
    ) -> Result<Self, EmbeddingProviderError> {
        let mut provider = Self::new(api_key)?;
        provider.model = model.to_string();
        Ok(provider)
    }

    fn embedding_dim(&self) -> usize {
        match self.model.as_str() {
            "text-embedding-3-large" => 3072,
            _ => 1536,
        }
    }
}

#[async_trait]
impl EmbeddingProvider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn dimension(&self) -> usize {
        self.embedding_dim()
    }

    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError> {
        if texts.is_empty() {
            return Err(EmbeddingProviderError::EmptyInput);
        }

        let request_body = serde_json::json!({
            "model": self.model,
            "input": texts,
            "encoding_format": "float"
        });

        let mut headers = HeaderMap::new();
        let auth_header: HeaderValue = format!("Bearer {}", self.config.api_key)
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidApiKeyFormat)?;
        headers.insert(AUTHORIZATION, auth_header);
        let content_type: HeaderValue = "application/json"
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidContentType)?;
        headers.insert(CONTENT_TYPE, content_type);

        let api_url = self
            .config
            .api_url
            .as_ref()
            .ok_or(EmbeddingProviderError::MissingApiUrl)?;
        let response = self
            .client
            .post(api_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| EmbeddingProviderError::ApiError {
                message: e.to_string(),
                status_code: None,
            })?;

        if !response.status().is_success() {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return Err(EmbeddingProviderError::RateLimited {
                retry_after: u64::try_from(retry_after).unwrap_or(60),
            });
        }

        let response_text =
            response
                .text()
                .await
                .map_err(|e| EmbeddingProviderError::InvalidResponse {
                    message: e.to_string(),
                })?;

        let response: OpenAIResponse = serde_json::from_str(&response_text).map_err(|e| {
            EmbeddingProviderError::JsonError {
                message: e.to_string(),
            }
        })?;

        let dimension = self.embedding_dim();
        let mut embeddings = Vec::with_capacity(response.data.len());

        for item in response.data {
            if item.embedding.len() != dimension {
                return Err(EmbeddingProviderError::InvalidResponse {
                    message: format!(
                        "Embedding dimension mismatch: expected {}, got {}",
                        dimension,
                        item.embedding.len()
                    ),
                });
            }

            embeddings.push(Embedding {
                vector: item.embedding,
                text: None,
                model: self.model.clone(),
            });
        }

        Ok(embeddings)
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token
        text.len() / 4
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIResponse {
    data: Vec<OpenAIEmbeddingData>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIUsage {
    total_tokens: usize,
}

/// Cohere text embedding provider.
#[derive(Debug, Clone)]
pub struct CohereProvider {
    config: EmbeddingConfig,
    client: Client,
    model: String,
}

impl CohereProvider {
    /// Create a new Cohere provider.
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingProviderError::MissingApiKey` if the API key is empty.
    /// Returns `EmbeddingProviderError::ApiError` if the HTTP client fails to build.
    pub fn new(api_key: impl Into<String>) -> Result<Self, EmbeddingProviderError> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(EmbeddingProviderError::MissingApiKey);
        }

        let config = EmbeddingConfig {
            api_key,
            api_url: Some("https://api.cohere.com/v1/embed".to_string()),
            ..Default::default()
        };

        let client = Client::builder()
            .timeout(std::time::Duration::from_mins(1))
            .build()
            .map_err(|e| EmbeddingProviderError::ApiError {
                message: e.to_string(),
                status_code: None,
            })?;

        Ok(Self {
            config,
            client,
            model: "embed-english-v3.0".to_string(),
        })
    }

    fn embedding_dim(&self) -> usize {
        match self.model.as_str() {
            "embed-english-v2.0" | "embed-multilingual-v2.0" => 4096,
            _ => 1024,
        }
    }
}

#[async_trait]
impl EmbeddingProvider for CohereProvider {
    fn name(&self) -> &'static str {
        "cohere"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn dimension(&self) -> usize {
        self.embedding_dim()
    }

    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError> {
        if texts.is_empty() {
            return Err(EmbeddingProviderError::EmptyInput);
        }

        let request_body = serde_json::json!({
            "model": self.model,
            "texts": texts,
            "input_type": "search_document"
        });

        let mut headers = HeaderMap::new();
        let auth_header: HeaderValue = format!("Bearer {}", self.config.api_key)
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidApiKeyFormat)?;
        headers.insert(AUTHORIZATION, auth_header);
        let content_type: HeaderValue = "application/json"
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidContentType)?;
        headers.insert(CONTENT_TYPE, content_type);

        let api_url = self
            .config
            .api_url
            .as_ref()
            .ok_or(EmbeddingProviderError::MissingApiUrl)?;
        let response = self
            .client
            .post(api_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| EmbeddingProviderError::ApiError {
                message: e.to_string(),
                status_code: None,
            })?;

        if !response.status().is_success() {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return Err(EmbeddingProviderError::RateLimited {
                retry_after: u64::try_from(retry_after).unwrap_or(60),
            });
        }

        let response_text =
            response
                .text()
                .await
                .map_err(|e| EmbeddingProviderError::InvalidResponse {
                    message: e.to_string(),
                })?;

        let response: CohereResponse = serde_json::from_str(&response_text).map_err(|e| {
            EmbeddingProviderError::JsonError {
                message: e.to_string(),
            }
        })?;

        let dimension = self.embedding_dim();
        let mut embeddings = Vec::with_capacity(response.embeddings.len());

        for (idx, embedding) in response.embeddings.into_iter().enumerate() {
            if embedding.len() != dimension {
                return Err(EmbeddingProviderError::InvalidResponse {
                    message: format!(
                        "Embedding dimension mismatch: expected {}, got {}",
                        dimension,
                        embedding.len()
                    ),
                });
            }

            let text = texts.get(idx).map(|s| (*s).to_string());

            embeddings.push(Embedding {
                vector: embedding,
                text,
                model: self.model.clone(),
            });
        }

        Ok(embeddings)
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Cohere uses similar tokenization to OpenAI
        text.len() / 4
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CohereResponse {
    embeddings: Vec<Vec<f32>>,
    pub usage: CohereUsage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CohereUsage {
    pub tokens: usize,
}

/// Convert embeddings to the format expected by HNSW index.
#[must_use]
pub fn embeddings_to_vectors(embeddings: &[Embedding]) -> Vec<Vec<f32>> {
    embeddings.iter().map(|e| e.vector.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Slug;

    #[test]
    fn test_embedding_validity() {
        let valid = Embedding::new(vec![1.0, 0.5, -0.3], None, "test".to_string());
        assert!(valid.is_valid());
        assert_eq!(valid.dimension(), 3);

        let nan_embedding = Embedding::new(vec![1.0, f32::NAN], None, "test".to_string());
        assert!(!nan_embedding.is_valid());
    }

    #[test]
    fn test_embedding_from_text() {
        let provider = OpenAIProvider::new("test-key");
        assert!(provider.is_ok());

        let slug = Slug::from_text("Hello World!");
        assert_eq!(slug.as_str(), "hello-world");
    }
}
