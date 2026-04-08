//! OpenAI text embedding provider.

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use tracing::instrument;

use super::{Embedding, EmbeddingConfig, EmbeddingProvider, EmbeddingProviderError};

/// `OpenAI` text embedding provider.
///
/// Supports text-embedding-3-small, text-embedding-3-large, and text-embedding-ada-002.
#[derive(Debug, Clone)]
pub struct OpenAIProvider {
    pub(super) config: EmbeddingConfig,
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
    /// Returns `EmbeddingProviderError` if the underlying provider fails to initialize.
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

    pub(super) fn embedding_dim(&self) -> usize {
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

    #[instrument(skip_all, fields(text_count = texts.len(), model = %self.model))]
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

        response
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
            .collect()
    }

    fn estimate_tokens(&self, text: &str) -> usize {
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
