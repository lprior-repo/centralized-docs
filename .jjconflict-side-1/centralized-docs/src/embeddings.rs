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

#[cfg(feature = "localembed")]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use serde::Serialize;

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
    #[allow(unknown_lints, clippy::duration_suboptimal_units)]
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
            .timeout(std::time::Duration::from_secs(60))
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
        let base = Self::new(api_key)?;
        Ok(Self {
            config: base.config,
            client: base.client,
            model: model.to_string(),
        })
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

        let auth_header: HeaderValue = format!("Bearer {}", self.config.api_key)
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidApiKeyFormat)?;
        let content_type: HeaderValue = "application/json"
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidContentType)?;

        let headers = [(AUTHORIZATION, auth_header), (CONTENT_TYPE, content_type)]
            .into_iter()
            .collect::<HeaderMap>();

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
                .map_or(60, |v| v);

            return Err(EmbeddingProviderError::RateLimited {
                retry_after: u64::try_from(retry_after).map_or(60, |v| v),
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

        let embeddings: Result<Vec<Embedding>, EmbeddingProviderError> = response
            .data
            .into_iter()
            .map(|item| {
                if item.embedding.len() != dimension {
                    return Err(EmbeddingProviderError::InvalidResponse {
                        message: format!(
                            "Embedding dimension mismatch: expected {}, got {}",
                            dimension,
                            item.embedding.len()
                        ),
                    });
                }

                Ok(Embedding {
                    vector: item.embedding,
                    text: None,
                    model: self.model.clone(),
                })
            })
            .collect();

        embeddings
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token
        text.len() / 4
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated by serde deserialization from API responses
struct OpenAIResponse {
    data: Vec<OpenAIEmbeddingData>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated by serde deserialization from API responses
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated by serde deserialization from API responses
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
    #[allow(unknown_lints, clippy::duration_suboptimal_units)]
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
            .timeout(std::time::Duration::from_secs(60))
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

        let auth_header: HeaderValue = format!("Bearer {}", self.config.api_key)
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidApiKeyFormat)?;
        let content_type: HeaderValue = "application/json"
            .parse()
            .map_err(|_| EmbeddingProviderError::InvalidContentType)?;

        let headers = [(AUTHORIZATION, auth_header), (CONTENT_TYPE, content_type)]
            .into_iter()
            .collect::<HeaderMap>();

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
                .map_or(60, |v| v);

            return Err(EmbeddingProviderError::RateLimited {
                retry_after: u64::try_from(retry_after).map_or(60, |v| v),
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

        let embeddings: Result<Vec<Embedding>, EmbeddingProviderError> = response
            .embeddings
            .into_iter()
            .enumerate()
            .map(|(idx, embedding)| {
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

                Ok(Embedding {
                    vector: embedding,
                    text,
                    model: self.model.clone(),
                })
            })
            .collect();

        embeddings
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Cohere uses similar tokenization to OpenAI
        text.len() / 4
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated by serde deserialization from API responses
struct CohereResponse {
    embeddings: Vec<Vec<f32>>,
    pub usage: CohereUsage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields populated by serde deserialization from API responses
struct CohereUsage {
    pub tokens: usize,
}

/// Convert embeddings to the format expected by HNSW index.
#[must_use]
pub fn embeddings_to_vectors(embeddings: &[Embedding]) -> Vec<Vec<f32>> {
    embeddings.iter().map(|e| e.vector.clone()).collect()
}

/// Local text embedding provider using FastEmbed.
/// Runs embedding models (like BGE-small) completely locally.
#[cfg(feature = "localembed")]
pub struct LocalFastEmbedProvider {
    model_name: String,
    model: std::sync::Arc<tokio::sync::Mutex<TextEmbedding>>,
    dim: usize,
}

#[cfg(feature = "localembed")]
impl std::fmt::Debug for LocalFastEmbedProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalFastEmbedProvider")
            .field("model_name", &self.model_name)
            .field("dim", &self.dim)
            .finish()
    }
}

#[cfg(feature = "localembed")]
impl Clone for LocalFastEmbedProvider {
    fn clone(&self) -> Self {
        Self {
            model_name: self.model_name.clone(),
            model: std::sync::Arc::clone(&self.model),
            dim: self.dim,
        }
    }
}

#[cfg(feature = "localembed")]
impl LocalFastEmbedProvider {
    pub fn new() -> Result<Self, EmbeddingProviderError> {
        // FastEmbed handles downloading and caching the model
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(false),
        )
        .map_err(|e| EmbeddingProviderError::ApiError {
            message: format!("Failed to initialize FastEmbed: {e}"),
            status_code: None,
        })?;

        Ok(Self {
            model_name: "bge-small-en-v1.5".to_string(),
            model: std::sync::Arc::new(tokio::sync::Mutex::new(model)),
            dim: 384, // BGE-small has 384 dimensions
        })
    }
}

#[cfg(feature = "localembed")]
#[async_trait]
impl EmbeddingProvider for LocalFastEmbedProvider {
    fn name(&self) -> &'static str {
        "local_fastembed"
    }

    fn model(&self) -> &str {
        &self.model_name
    }

    fn dimension(&self) -> usize {
        self.dim
    }

    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError> {
        if texts.is_empty() {
            return Err(EmbeddingProviderError::EmptyInput);
        }

        let mut model_guard = self.model.lock().await;

        let texts_vec: Vec<String> = texts.iter().map(|&s| s.to_string()).collect();
        let fastembed_result =
            model_guard
                .embed(texts_vec, None)
                .map_err(|e| EmbeddingProviderError::ApiError {
                    message: format!("FastEmbed inference failed: {e}"),
                    status_code: None,
                })?;

        let mut embeddings = Vec::with_capacity(fastembed_result.len());
        for (i, vec) in fastembed_result.into_iter().enumerate() {
            embeddings.push(Embedding {
                vector: vec,
                text: Some(texts[i].to_string()),
                model: self.model_name.clone(),
            });
        }

        Ok(embeddings)
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimate for local model
        text.len() / 4
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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

    #[test]
    fn test_embedding_empty_vector_invalid() {
        let empty = Embedding::new(vec![], None, "test".to_string());
        assert!(!empty.is_valid());
        assert_eq!(empty.dimension(), 0);
    }

    #[test]
    fn test_embedding_infinity_invalid() {
        let inf_emb = Embedding::new(vec![1.0, f32::INFINITY], None, "test".to_string());
        assert!(!inf_emb.is_valid());
    }

    #[test]
    fn test_embedding_with_text() {
        let emb = Embedding::new(
            vec![0.1, 0.2],
            Some("hello".to_string()),
            "model-a".to_string(),
        );
        assert_eq!(emb.text.as_deref(), Some("hello"));
        assert_eq!(emb.model, "model-a");
    }

    #[test]
    fn test_embedding_clone() {
        let emb = Embedding::new(vec![1.0, 2.0], Some("text".to_string()), "m".to_string());
        let emb2 = emb.clone();
        assert_eq!(emb, emb2);
    }

    #[test]
    fn test_embedding_vector_serde_roundtrip() {
        let ev = EmbeddingVector(vec![1.0, 2.0, 3.0]);
        let json = serde_json::to_string(&ev).unwrap();
        let ev2: EmbeddingVector = serde_json::from_str(&json).unwrap();
        assert_eq!(ev.0, ev2.0);
    }

    #[test]
    fn test_openai_provider_new_empty_key() {
        let result = OpenAIProvider::new("");
        assert_eq!(result.unwrap_err(), EmbeddingProviderError::MissingApiKey);
    }

    #[test]
    fn test_openai_provider_with_model() {
        let provider = OpenAIProvider::with_model("test-key", "text-embedding-3-large").unwrap();
        assert_eq!(provider.model(), "text-embedding-3-large");
        assert_eq!(provider.dimension(), 3072);
    }

    #[test]
    fn test_openai_provider_default_model() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        assert_eq!(provider.model(), "text-embedding-3-small");
        assert_eq!(provider.dimension(), 1536);
    }

    #[test]
    fn test_openai_provider_name() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_openai_provider_estimate_tokens() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        let tokens = provider.estimate_tokens("Hello world this is a test");
        assert!(tokens > 0);
        assert_eq!(tokens, "Hello world this is a test".len() / 4);
    }

    #[test]
    fn test_openai_provider_estimate_tokens_empty() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        assert_eq!(provider.estimate_tokens(""), 0);
    }

    #[test]
    fn test_cohere_provider_new_empty_key() {
        let result = CohereProvider::new("");
        assert_eq!(result.unwrap_err(), EmbeddingProviderError::MissingApiKey);
    }

    #[test]
    fn test_cohere_provider_name() {
        let provider = CohereProvider::new("test-key").unwrap();
        assert_eq!(provider.name(), "cohere");
    }

    #[test]
    fn test_cohere_provider_default_model() {
        let provider = CohereProvider::new("test-key").unwrap();
        assert_eq!(provider.model(), "embed-english-v3.0");
        assert_eq!(provider.dimension(), 1024);
    }

    #[test]
    fn test_cohere_provider_estimate_tokens() {
        let provider = CohereProvider::new("test-key").unwrap();
        assert_eq!(provider.estimate_tokens("12345678"), 2);
    }

    #[test]
    fn test_error_variants_display() {
        let errors = vec![
            EmbeddingProviderError::ApiError {
                message: "fail".to_string(),
                status_code: Some(500),
            },
            EmbeddingProviderError::RateLimited { retry_after: 30 },
            EmbeddingProviderError::InvalidResponse {
                message: "bad".to_string(),
            },
            EmbeddingProviderError::JsonError {
                message: "parse err".to_string(),
            },
            EmbeddingProviderError::MissingApiKey,
            EmbeddingProviderError::InvalidApiKeyFormat,
            EmbeddingProviderError::InvalidContentType,
            EmbeddingProviderError::MissingApiUrl,
            EmbeddingProviderError::TextTooLong {
                length: 10000,
                max: 8000,
            },
            EmbeddingProviderError::EmptyInput,
        ];
        for err in errors {
            let msg = err.to_string();
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn test_error_variants_debug() {
        let err = EmbeddingProviderError::ApiError {
            message: "test".to_string(),
            status_code: None,
        };
        let dbg = format!("{err:?}");
        assert!(dbg.contains("ApiError"));
    }

    #[test]
    fn test_error_variants_eq() {
        assert_eq!(
            EmbeddingProviderError::MissingApiKey,
            EmbeddingProviderError::MissingApiKey
        );
        assert_eq!(
            EmbeddingProviderError::EmptyInput,
            EmbeddingProviderError::EmptyInput
        );
        assert_ne!(
            EmbeddingProviderError::MissingApiKey,
            EmbeddingProviderError::EmptyInput
        );
    }

    #[test]
    fn test_embeddings_to_vectors() {
        let embeddings = vec![
            Embedding::new(vec![1.0, 2.0], None, "m".to_string()),
            Embedding::new(vec![3.0, 4.0], None, "m".to_string()),
        ];
        let vectors = embeddings_to_vectors(&embeddings);
        assert_eq!(vectors.len(), 2);
        assert_eq!(vectors[0], vec![1.0, 2.0]);
        assert_eq!(vectors[1], vec![3.0, 4.0]);
    }

    #[test]
    fn test_embeddings_to_vectors_empty() {
        let vectors = embeddings_to_vectors(&[]);
        assert!(vectors.is_empty());
    }

    #[test]
    fn test_embedding_config_default() {
        let config = EmbeddingConfig::default();
        assert!(config.api_key.is_empty());
        assert!(config.api_url.is_none());
        assert_eq!(config.timeout, 60);
        assert_eq!(config.max_tokens_per_batch, 8000);
        assert_eq!(config.batch_size, 100);
    }

    #[test]
    fn test_embedding_provider_error_is_error() {
        let err: Box<dyn std::error::Error> = Box::new(EmbeddingProviderError::MissingApiKey);
        let _ = err.to_string();
    }
}
