use super::errors::FeatureError;

/// Validated regex pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexPattern(String);

impl RegexPattern {
    /// Creates a new validated regex pattern
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidRegex` if the pattern is not a valid regex.
    #[allow(dead_code)]
    pub fn new(pattern: String) -> Result<Self, FeatureError> {
        regex::Regex::new(&pattern)
            .map(|_| Self(pattern.clone()))
            .map_err(|_| FeatureError::InvalidRegex { pattern })
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated glob pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobPattern(String);

impl GlobPattern {
    /// Creates a new validated glob pattern
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidGlob` if the pattern is empty.
    #[allow(dead_code)]
    pub fn new(pattern: String) -> Result<Self, FeatureError> {
        // Basic validation - non-empty
        if pattern.is_empty() {
            return Err(FeatureError::InvalidGlob { pattern });
        }
        Ok(Self(pattern))
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// URL filtering configuration
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FilteringConfig {
    pub allow: Vec<GlobPattern>,
    pub deny: Vec<RegexPattern>,
}

impl FilteringConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_allow(mut self, patterns: Vec<GlobPattern>) -> Self {
        self.allow = patterns;
        self
    }

    #[must_use]
    pub fn with_deny(mut self, patterns: Vec<RegexPattern>) -> Self {
        self.deny = patterns;
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.allow.is_empty() && self.deny.is_empty()
    }
}
