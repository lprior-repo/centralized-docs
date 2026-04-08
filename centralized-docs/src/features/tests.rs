use super::cache::{CacheConfig, CacheTtl};
use super::config::{FeatureConfig, FeatureConfigBuilder};
use super::errors::FeatureError;
use super::filtering::{FilteringConfig, GlobPattern, RegexPattern};

#[cfg(feature = "javascript")]
use super::javascript::{JavascriptConfig, Milliseconds, RenderMode};

#[cfg(feature = "anti-detection")]
use super::anti_detection::{AntiDetectionConfig, Strategy};

#[test]
fn test_cache_ttl_rejects_zero() {
    assert!(matches!(
        CacheTtl::new(0),
        Err(FeatureError::InvalidCacheTtl(0))
    ));
}

#[test]
fn test_cache_ttl_accepts_positive() {
    assert!(CacheTtl::new(60).is_ok());
}

#[test]
fn test_regex_pattern_validation() {
    assert!(RegexPattern::new(r"\d+".to_string()).is_ok());
    assert!(RegexPattern::new(r"(".to_string()).is_err());
}

#[test]
fn test_glob_pattern_validation() {
    assert!(GlobPattern::new("/docs/*".to_string()).is_ok());
    assert!(GlobPattern::new(String::new()).is_err());
}

#[cfg(feature = "javascript")]
#[test]
fn test_milliseconds_rejects_zero() {
    assert!(matches!(
        Milliseconds::new(0),
        Err(FeatureError::InvalidJsTimeout(0))
    ));
}

#[test]
fn test_feature_config_is_empty() {
    let config = FeatureConfig::new();
    assert!(config.is_empty());
}

#[cfg(feature = "enhanced")]
#[test]
fn test_builder_cache() {
    FeatureConfigBuilder::new()
        .enable_cache(300)
        .map(|builder| {
            let config = builder.build();
            assert!(config.cache.is_some());
            assert!(config.cache.as_ref().is_some_and(|c| c.enabled));
        })
        .unwrap();
}

#[cfg(feature = "enhanced")]
#[test]
fn test_builder_filtering() {
    FeatureConfigBuilder::new()
        .allow_patterns(vec!["/docs/*".to_string()])
        .map(|builder| {
            let config = builder.build();
            assert!(config.filtering.is_some());
            assert!(!config.filtering.as_ref().is_none_or(|f| f.allow.is_empty()));
        })
        .unwrap();
}

#[test]
fn test_cache_config_default_is_disabled() {
    let config = CacheConfig::default();
    assert!(!config.enabled);
}

#[test]
fn test_cache_config_enabled_with_ttl() {
    CacheTtl::new(600)
        .map(|ttl| {
            let config = CacheConfig::enabled_with_ttl(ttl);
            assert!(config.enabled);
            assert_eq!(config.ttl.seconds(), 600);
        })
        .unwrap();
}

#[test]
fn test_filtering_config_default_is_empty() {
    let config = FilteringConfig::default();
    assert!(config.is_empty());
}

#[test]
fn test_filtering_config_with_allow() {
    GlobPattern::new("/docs/*".to_string())
        .map(|pattern| {
            let patterns = vec![pattern];
            let config = FilteringConfig::new().with_allow(patterns.clone());
            assert!(!config.allow.is_empty());
            assert_eq!(config.allow.len(), 1);
        })
        .unwrap();
}

#[test]
fn test_filtering_config_with_deny() {
    RegexPattern::new(r"\.pdf$".to_string())
        .map(|pattern| {
            let patterns = vec![pattern];
            let config = FilteringConfig::new().with_deny(patterns.clone());
            assert!(!config.deny.is_empty());
            assert_eq!(config.deny.len(), 1);
        })
        .unwrap();
}

#[cfg(feature = "javascript")]
#[test]
fn test_javascript_config_smart() {
    JavascriptConfig::smart()
        .map(|config| {
            assert_eq!(config.mode, RenderMode::Smart);
            assert_eq!(config.timeout.millis(), 30000);
        })
        .unwrap();
}

#[cfg(feature = "javascript")]
#[test]
fn test_javascript_config_never() {
    JavascriptConfig::never()
        .map(|config| {
            assert_eq!(config.mode, RenderMode::Never);
            assert_eq!(config.timeout.millis(), 1000);
        })
        .unwrap();
}

#[cfg(feature = "javascript")]
#[test]
fn test_javascript_config_with_timeout() {
    Milliseconds::new(5000)
        .map(|timeout| {
            let config = JavascriptConfig {
                mode: RenderMode::Always,
                timeout,
            };
            assert_eq!(config.timeout.millis(), 5000);
        })
        .unwrap();
}

#[cfg(feature = "anti-detection")]
#[test]
fn test_anti_detection_config_none() {
    let config = AntiDetectionConfig::none();
    assert_eq!(config.strategy, Strategy::None);
}

#[cfg(feature = "anti-detection")]
#[test]
fn test_anti_detection_config_rotating_ua() {
    let config = AntiDetectionConfig::rotating_ua();
    assert_eq!(config.strategy, Strategy::RotatingUserAgent);
}

#[cfg(feature = "anti-detection")]
#[test]
fn test_anti_detection_config_full_stealth() {
    let config = AntiDetectionConfig::full_stealth();
    assert_eq!(config.strategy, Strategy::FullStealth);
}

#[test]
fn test_feature_config_new_is_empty() {
    let config = FeatureConfig::new();
    assert!(config.is_empty());
}

#[cfg(feature = "enhanced")]
#[test]
fn test_feature_config_with_cache() {
    CacheTtl::new(300)
        .map(|ttl| {
            let cache_config = CacheConfig::enabled_with_ttl(ttl);
            let config = FeatureConfig::new().with_cache(cache_config);
            assert!(!config.is_empty());
            assert!(config.cache.is_some());
        })
        .unwrap();
}

#[cfg(feature = "enhanced")]
#[test]
fn test_feature_config_with_filtering() {
    let filtering_config = FilteringConfig::new();
    let config = FeatureConfig::new().with_filtering(filtering_config);
    assert!(!config.is_empty());
    assert!(config.filtering.is_some());
}

#[cfg(feature = "javascript")]
#[test]
fn test_feature_config_with_javascript() {
    JavascriptConfig::smart()
        .map(|js_config| {
            let config = FeatureConfig::new().with_javascript(js_config);
            assert!(!config.is_empty());
            assert!(config.javascript.is_some());
        })
        .unwrap();
}

#[cfg(feature = "javascript")]
#[test]
fn test_feature_config_with_javascript() {
    JavascriptConfig::smart()
        .map(|js_config| {
            let config = FeatureConfig::new().with_javascript(js_config);
            assert!(!config.is_empty());
            assert!(config.javascript.is_some());
        })
        .unwrap();
}

#[test]
fn test_cache_ttl_default() {
    let ttl = CacheTtl::default();
    assert_eq!(ttl.seconds(), 300);
}

#[test]
fn test_regex_pattern_as_str() {
    RegexPattern::new(r"\d+".to_string())
        .map(|pattern| {
            assert_eq!(pattern.as_str(), r"\d+");
        })
        .unwrap();
}

#[test]
fn test_regex_pattern_as_str() {
    RegexPattern::new(r"\d+".to_string())
        .map(|pattern| {
            assert_eq!(pattern.as_str(), r"\d+");
        })
        .unwrap();
}

#[test]
fn test_glob_pattern_as_str() {
    GlobPattern::new("/docs/*".to_string())
        .map(|pattern| {
            assert_eq!(pattern.as_str(), "/docs/*");
        })
        .unwrap();
}
