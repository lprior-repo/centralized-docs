//! Local text embedding provider using FastEmbed.

use async_trait::async_trait;
use tracing::instrument;

use super::{Embedding, EmbeddingProvider, EmbeddingProviderError};

#[cfg(feature = "localembed")]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

/// Local text embedding provider using FastEmbed.
/// Runs embedding models (like BGE-small) completely locally.
pub struct LocalFastEmbedProvider {
    model_name: String,
    model: std::sync::Arc<std::sync::Mutex<TextEmbedding>>,
    dim: usize,
}

impl std::fmt::Debug for LocalFastEmbedProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalFastEmbedProvider")
            .field("model_name", &self.model_name)
            .field("dim", &self.dim)
            .finish()
    }
}

impl Clone for LocalFastEmbedProvider {
    fn clone(&self) -> Self {
        Self {
            model_name: self.model_name.clone(),
            model: std::sync::Arc::clone(&self.model),
            dim: self.dim,
        }
    }
}

impl LocalFastEmbedProvider {
    /// Create a new local FastEmbed provider.
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingProviderError::ApiError` if the model fails to initialize.
    pub fn new() -> Result<Self, EmbeddingProviderError> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(false),
        )
        .map_err(|e| EmbeddingProviderError::ApiError {
            message: format!("Failed to initialize FastEmbed: {e}"),
            status_code: None,
        })?;

        Ok(Self {
            model_name: "bge-small-en-v1.5".to_string(),
            model: std::sync::Arc::new(std::sync::Mutex::new(model)),
            dim: 384,
        })
    }
}

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

    #[instrument(skip_all, fields(text_count = texts.len(), model = %self.model_name))]
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
        text.len() / 4
    }
}
