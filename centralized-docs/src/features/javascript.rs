use std::time::Duration;

use super::errors::FeatureError;

/// Positive millisecond duration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milliseconds(u64);

impl Milliseconds {
    /// Creates a new positive millisecond duration
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidJsTimeout` if `ms` is zero.
    #[allow(dead_code)]
    pub fn new(ms: u64) -> Result<Self, FeatureError> {
        if ms > 0 {
            Ok(Self(ms))
        } else {
            Err(FeatureError::InvalidJsTimeout(ms))
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn as_duration(&self) -> Duration {
        Duration::from_millis(self.0)
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn millis(&self) -> u64 {
        self.0
    }
}

/// `JavaScript` rendering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderMode {
    /// Auto-detect if `JS` rendering is needed
    #[default]
    Smart,
    /// Always use `Chrome` rendering
    Always,
    /// Never use `Chrome` (`HTTP` only)
    Never,
}

/// `JavaScript` rendering configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JavascriptConfig {
    pub mode: RenderMode,
    pub timeout: Milliseconds,
}

impl JavascriptConfig {
    /// Creates a new `JavaScript` rendering configuration
    ///
    /// # Errors
    ///
    /// Never fails - returns Ok for all inputs.
    #[allow(dead_code)]
    pub fn new(mode: RenderMode, timeout: Milliseconds) -> Result<Self, FeatureError> {
        Ok(Self { mode, timeout })
    }

    /// Creates a configuration with smart rendering mode
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidJsTimeout` if the default timeout validation fails.
    #[allow(dead_code)]
    pub fn smart() -> Result<Self, FeatureError> {
        Ok(Self {
            mode: RenderMode::Smart,
            timeout: Milliseconds::new(30000)?, // 30s default
        })
    }

    /// Creates a configuration with never rendering mode
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidJsTimeout` if the default timeout validation fails.
    #[allow(dead_code)]
    pub fn never() -> Result<Self, FeatureError> {
        Ok(Self {
            mode: RenderMode::Never,
            timeout: Milliseconds::new(1000)?, // Minimal timeout
        })
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_timeout(mut self, timeout: Milliseconds) -> Self {
        self.timeout = timeout;
        self
    }
}

impl Default for JavascriptConfig {
    fn default() -> Self {
        Self {
            mode: RenderMode::Smart,
            timeout: Milliseconds(30000),
        }
    }
}
