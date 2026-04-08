//! Tests for semantic embedding providers.

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
