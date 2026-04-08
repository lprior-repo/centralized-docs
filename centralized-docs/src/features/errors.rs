use thiserror::Error;

/// All feature-related errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FeatureError {
    #[allow(dead_code)]
    #[error("invalid cache TTL: must be positive, got {0}s")]
    InvalidCacheTtl(u64),

    #[allow(dead_code)]
    #[error("invalid regex pattern: {pattern}")]
    InvalidRegex { pattern: String },

    #[allow(dead_code)]
    #[error("invalid glob pattern: {pattern}")]
    InvalidGlob { pattern: String },

    #[allow(dead_code)]
    #[error("JavaScript timeout must be at least 1ms, got {0}ms")]
    InvalidJsTimeout(u64),

    #[cfg(feature = "javascript")]
    #[allow(dead_code)]
    #[error("Chrome initialization failed: {0}")]
    ChromeInit(String),

    #[cfg(feature = "anti-detection")]
    #[allow(dead_code)]
    #[error("user agent generation failed")]
    UserAgentGeneration,
}
