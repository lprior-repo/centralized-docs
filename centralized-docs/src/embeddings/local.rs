//! Local text embedding provider using FastEmbed.

use async_trait::async_trait;
use tracing::instrument;

use super::{Embedding, EmbeddingProvider, EmbeddingProviderError};

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

/// Local text embedding provider using `FastEmbed`.
/// Runs embedding models (like BGE-small) completely locally.
pub struct LocalFastEmbedProvider {
    model_name: String,
    model: std::sync::Arc<tokio::sync::Mutex<TextEmbedding>>,
    dim: usize,
}

impl std::fmt::Debug for LocalFastEmbedProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalFastEmbedProvider")
            .field("model_name", &self.model_name)
            .field("dim", &self.dim)
            .finish_non_exhaustive()
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
    /// Create a new local `FastEmbed` provider.
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
            model: std::sync::Arc::new(tokio::sync::Mutex::new(model)),
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

        let model = self.model.clone();
        let model_name = self.model_name.clone();
        let texts_vec: Vec<String> = texts.iter().map(|text| (*text).to_string()).collect();
        let requested_texts = texts_vec.clone();

        let fastembed_result = tokio::task::spawn_blocking(move || {
            let mut model_guard = model.blocking_lock();
            model_guard.embed(texts_vec, None)
        })
        .await
        .map_err(|err| EmbeddingProviderError::ApiError {
            message: format!("FastEmbed task failed: {err}"),
            status_code: None,
        })?
        .map_err(|err| EmbeddingProviderError::ApiError {
            message: format!("FastEmbed inference failed: {err}"),
            status_code: None,
        })?;

        Ok(fastembed_result
            .into_iter()
            .zip(requested_texts)
            .map(|(vector, text)| Embedding {
                vector,
                text: Some(text),
                model: model_name.clone(),
            })
            .collect())
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        text.len() / 4
    }
}
