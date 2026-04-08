//! Cohere text embedding provider.

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use tracing::instrument;

use super::{Embedding, EmbeddingConfig, EmbeddingProvider, EmbeddingProviderError};

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

    #[instrument(skip_all, fields(text_count = texts.len(), model = %self.model))]
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

        response
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
            .collect()
    }

    fn estimate_tokens(&self, text: &str) -> usize {
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
