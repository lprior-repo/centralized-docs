use super::cache::{CacheConfig, CacheTtl};
use super::errors::FeatureError;
use super::filtering::{FilteringConfig, GlobPattern, RegexPattern};

#[cfg(feature = "javascript")]
use super::javascript::{JavascriptConfig, Milliseconds};

#[cfg(feature = "anti-detection")]
use super::anti_detection::AntiDetectionConfig;

// ===========================================================================
// COMPOSITE FEATURE CONFIG
// ===========================================================================

/// Master feature configuration
///
/// All fields are optional to enable zero-cost when features disabled.
/// Use the builder pattern for construction.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeatureConfig {
    #[cfg(feature = "enhanced")]
    pub cache: Option<CacheConfig>,

    #[cfg(feature = "enhanced")]
    pub filtering: Option<FilteringConfig>,

    #[cfg(feature = "javascript")]
    pub javascript: Option<JavascriptConfig>,

    #[cfg(feature = "anti-detection")]
    pub anti_detection: Option<AntiDetectionConfig>,
}

impl FeatureConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "enhanced")]
    #[must_use]
    pub fn with_cache(mut self, config: CacheConfig) -> Self {
        self.cache = Some(config);
        self
    }

    #[cfg(feature = "enhanced")]
    #[must_use]
    pub fn with_filtering(mut self, config: FilteringConfig) -> Self {
        self.filtering = Some(config);
        self
    }

    #[cfg(feature = "javascript")]
    #[must_use]
    pub fn with_javascript(mut self, config: JavascriptConfig) -> Self {
        self.javascript = Some(config);
        self
    }

    #[cfg(feature = "anti-detection")]
    #[must_use]
    pub fn with_anti_detection(mut self, config: AntiDetectionConfig) -> Self {
        self.anti_detection = Some(config);
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        #[cfg(feature = "enhanced")]
        let enhanced_empty = self.cache.is_none() && self.filtering.is_none();

        #[cfg(feature = "javascript")]
        let js_empty = self.javascript.is_none();

        #[cfg(feature = "anti-detection")]
        let anti_empty = self.anti_detection.is_none();

        #[cfg(not(any(
            feature = "enhanced",
            feature = "javascript",
            feature = "anti-detection"
        )))]
        let base_empty = true;

        #[cfg(all(
            feature = "enhanced",
            feature = "javascript",
            feature = "anti-detection"
        ))]
        {
            enhanced_empty && js_empty && anti_empty
        }
        #[cfg(all(
            feature = "enhanced",
            feature = "javascript",
            not(feature = "anti-detection")
        ))]
        {
            enhanced_empty && js_empty
        }
        #[cfg(all(
            feature = "enhanced",
            not(feature = "javascript"),
            feature = "anti-detection"
        ))]
        {
            enhanced_empty && anti_empty
        }
        #[cfg(all(
            feature = "enhanced",
            not(feature = "javascript"),
            not(feature = "anti-detection")
        ))]
        {
            enhanced_empty
        }
        #[cfg(all(
            not(feature = "enhanced"),
            feature = "javascript",
            feature = "anti-detection"
        ))]
        {
            js_empty && anti_empty
        }
        #[cfg(all(
            not(feature = "enhanced"),
            feature = "javascript",
            not(feature = "anti-detection")
        ))]
        {
            js_empty
        }
        #[cfg(all(
            not(feature = "enhanced"),
            not(feature = "javascript"),
            feature = "anti-detection"
        ))]
        {
            anti_empty
        }
        #[cfg(not(any(
            feature = "enhanced",
            feature = "javascript",
            feature = "anti-detection"
        )))]
        {
            base_empty
        }
    }
}

// ===========================================================================
// BUILDER FOR CONVENIENCE
// ===========================================================================

/// Builder for `FeatureConfig`
#[derive(Debug, Clone, Default)]
pub struct FeatureConfigBuilder {
    #[cfg(feature = "enhanced")]
    cache: Option<CacheConfig>,
    #[cfg(feature = "enhanced")]
    filtering: Option<FilteringConfig>,
    #[cfg(feature = "javascript")]
    javascript: Option<JavascriptConfig>,
    #[cfg(feature = "anti-detection")]
    anti_detection: Option<AntiDetectionConfig>,
}

impl FeatureConfigBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "enhanced")]
    /// Enables cache with specified TTL
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidCacheTtl` if `ttl_seconds` is zero.
    #[allow(dead_code)]
    pub fn enable_cache(mut self, ttl_seconds: u64) -> Result<Self, FeatureError> {
        self.cache = Some(CacheConfig::enabled_with_ttl(CacheTtl::new(ttl_seconds)?));
        Ok(self)
    }

    #[cfg(feature = "enhanced")]
    #[must_use]
    #[allow(dead_code)]
    pub fn disable_cache(mut self) -> Self {
        self.cache = Some(CacheConfig::disabled());
        self
    }

    #[cfg(feature = "enhanced")]
    /// Sets allowed URL patterns
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidGlob` if any pattern is invalid.
    #[allow(dead_code)]
    pub fn allow_patterns(mut self, patterns: Vec<String>) -> Result<Self, FeatureError> {
        let validated = patterns
            .into_iter()
            .map(GlobPattern::new)
            .collect::<Result<Vec<_>, _>>()?;
        self.filtering = Some(FilteringConfig::new().with_allow(validated));
        Ok(self)
    }

    #[cfg(feature = "enhanced")]
    /// Sets denied URL patterns
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidRegex` if any pattern is invalid.
    #[allow(dead_code)]
    pub fn deny_patterns(mut self, patterns: Vec<String>) -> Result<Self, FeatureError> {
        let validated = patterns
            .into_iter()
            .map(RegexPattern::new)
            .collect::<Result<Vec<_>, _>>()?;
        self.filtering = Some(FilteringConfig::new().with_deny(validated));
        Ok(self)
    }

    #[cfg(feature = "javascript")]
    /// Enables smart `JavaScript` rendering with timeout
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidJsTimeout` if `timeout_ms` is zero.
    #[allow(dead_code)]
    pub fn smart_js(mut self, timeout_ms: u64) -> Result<Self, FeatureError> {
        self.javascript =
            Some(JavascriptConfig::smart()?.with_timeout(Milliseconds::new(timeout_ms)?));
        Ok(self)
    }

    #[cfg(feature = "anti-detection")]
    #[must_use]
    #[allow(dead_code)]
    pub fn stealth(mut self) -> Self {
        self.anti_detection = Some(AntiDetectionConfig::full_stealth());
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn build(self) -> FeatureConfig {
        FeatureConfig {
            #[cfg(feature = "enhanced")]
            cache: self.cache,
            #[cfg(feature = "enhanced")]
            filtering: self.filtering,
            #[cfg(feature = "javascript")]
            javascript: self.javascript,
            #[cfg(feature = "anti-detection")]
            anti_detection: self.anti_detection,
        }
    }
}
