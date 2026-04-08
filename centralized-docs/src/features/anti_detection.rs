/// Anti-detection strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strategy {
    /// No anti-detection
    #[default]
    None,
    /// Rotate User-Agent header
    RotatingUserAgent,
    /// Full stealth mode (spoof headers, random `UA`, etc.)
    FullStealth,
}

/// Anti-detection configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AntiDetectionConfig {
    pub strategy: Strategy,
}

impl AntiDetectionConfig {
    #[must_use]
    #[allow(dead_code)]
    pub fn new(strategy: Strategy) -> Self {
        Self { strategy }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn none() -> Self {
        Self {
            strategy: Strategy::None,
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn rotating_ua() -> Self {
        Self {
            strategy: Strategy::RotatingUserAgent,
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn full_stealth() -> Self {
        Self {
            strategy: Strategy::FullStealth,
        }
    }
}

impl Default for AntiDetectionConfig {
    fn default() -> Self {
        Self::none()
    }
}
