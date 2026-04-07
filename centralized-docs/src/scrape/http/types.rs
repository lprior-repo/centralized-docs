//! Domain types for the HTTP scrape module.

use thiserror::Error;
use url::Url;

/// Domain errors for the HTTP scrape module.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HttpError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Configuration overflow: {0}")]
    ConfigOverflow(&'static str),
    #[error("Execution failed: {0}")]
    #[allow(dead_code)]
    ExecutionFailed(String),
    #[error("Scrape failed: {0}")]
    ScrapeFailed(String),
    #[error("Connect timeout after {timeout_secs}s to host '{host}'")]
    ConnectTimeout { host: String, timeout_secs: u64 },
}

/// A validated URL ensuring semantic correctness before passing to the scraper.
#[derive(Debug, Clone)]
pub struct ValidatedUrl(Url);

impl ValidatedUrl {
    /// Attempt to parse and validate a URL string.
    pub fn try_new(url_str: &str) -> Result<Self, HttpError> {
        Url::parse(url_str)
            .map(Self)
            .map_err(|_| HttpError::InvalidUrl(url_str.to_string()))
    }

    /// Return the string representation of the URL.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Return the parsed URL for direct access (avoids re-parsing).
    pub(crate) fn inner(&self) -> &Url {
        &self.0
    }
}

/// A limit guaranteed to be representable precisely as an f64.
#[derive(Debug, Clone, Copy)]
pub struct SafeByteLimit(f64);

impl SafeByteLimit {
    /// Creates a new `SafeByteLimit`, returning an error if the value exceeds 2^53 - 1.
    pub fn try_new(limit: u64) -> Result<Self, HttpError> {
        // max exact representable integer in f64 is 2^53 - 1
        if limit <= 9_007_199_254_740_991 {
            #[allow(clippy::cast_precision_loss)]
            Ok(Self(limit as f64))
        } else {
            Err(HttpError::ConfigOverflow(
                "spider_max_page_bytes exceeds f64 precise range",
            ))
        }
    }

    /// Return the underlying f64 value.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

/// Scrape strategy enumeration for type-safe documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrapeStrategy {
    /// Standard page scraping
    Standard,
    /// Sitemap-based scraping
    Sitemap,
}

/// Halt reason for extraction state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HaltReason {
    PageLimitReached,
    TotalSizeExceeded,
    #[allow(dead_code)]
    IntegerOverflow,
}

/// Extraction status for state machine.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtractionStatus {
    #[default]
    Active,
    Halted(HaltReason),
}

/// A newtype wrapper for URL sets providing domain semantics.
///
/// Uses `im::HashSet` (structural sharing, Arc-based) so `insert` is O(log n)
/// instead of O(n) clone — critical in rayon fold→reduce where UrlSet is
/// cloned per-page.
#[derive(Debug, Clone, Default)]
pub struct UrlSet(im::HashSet<String>);

impl UrlSet {
    /// Create a new empty UrlSet.
    pub fn new() -> Self {
        Self(im::HashSet::new())
    }

    /// Insert a URL, returning a new UrlSet (persistent, no mutation).
    ///
    /// O(log n) via structural sharing — no full clone of the backing HAMT.
    #[must_use]
    pub fn insert(&self, url: String) -> Self {
        Self(self.0.update(url))
    }

    /// Check if a URL is present in the set.
    #[allow(dead_code)]
    pub fn contains(&self, url: &str) -> bool {
        self.0.contains(url)
    }

    /// Compute the union of two URL sets.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.clone().union(other.0.clone()))
    }
}

/// Domain errors for scrape failures.
///
/// Variants capture the specific failure mode to enable precise error handling
/// and informative user messages. The `ConnectionSilentlyDropped` variant is
/// critical for detecting TCP blackholes where spider-rs cannot distinguish
/// between a slow connection and a silently dropping connection.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScrapeError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Request timeout for {url} after {timeout_secs}s")]
    RequestTimeout { url: String, timeout_secs: u64 },

    #[error("DNS error for {url}: {message}")]
    DnsError { url: String, message: String },

    #[error("Connection refused for {url}")]
    ConnectionRefused { url: String },

    #[error("Connection silently dropped for {url} — peer unreachable")]
    ConnectionSilentlyDropped { url: String },

    #[error("Too many redirects for {url}")]
    TooManyRedirects { url: String },

    #[error("HTTP {status} error for {url}")]
    HttpError { url: String, status: u16 },

    #[error("SSL error for {url}: {message}")]
    SslError { url: String, message: String },

    #[error("I/O error: {0}")]
    IoError(String),

    /// Generic application-level error with URL and message.
    /// Used for errors that don't fit network-specific categories (e.g., empty page, size limits).
    #[error("{message}")]
    Generic { url: String, message: String },
}

impl ScrapeError {
    /// Create a connection silently dropped error — the critical variant for TCP blackholes.
    #[must_use]
    pub fn connection_silently_dropped(url: String) -> Self {
        Self::ConnectionSilentlyDropped { url }
    }

    /// Create an I/O error.
    #[must_use]
    pub fn io_error(msg: impl Into<String>) -> Self {
        Self::IoError(msg.into())
    }

    /// Create a DNS error.
    #[must_use]
    pub fn dns_error(url: String, message: String) -> Self {
        Self::DnsError { url, message }
    }

    /// Create a connection refused error.
    #[must_use]
    pub fn connection_refused(url: String) -> Self {
        Self::ConnectionRefused { url }
    }

    /// Create a request timeout error.
    #[must_use]
    pub fn request_timeout(url: String, timeout_secs: u64) -> Self {
        Self::RequestTimeout { url, timeout_secs }
    }

    /// Create an HTTP error with status code.
    #[must_use]
    pub fn http_error(url: String, status: u16) -> Self {
        Self::HttpError { url, status }
    }

    /// Create an SSL error.
    #[must_use]
    pub fn ssl_error(url: String, message: String) -> Self {
        Self::SslError { url, message }
    }

    /// Create a too many redirects error.
    #[must_use]
    pub fn too_many_redirects(url: String) -> Self {
        Self::TooManyRedirects { url }
    }

    /// Create a generic scrape error with URL and message (for application-level errors).
    #[must_use]
    pub fn generic(url: String, message: String) -> Self {
        Self::Generic { url, message }
    }

    /// Extract the URL from this error, if present.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::InvalidUrl(url)
            | Self::RequestTimeout { url, .. }
            | Self::DnsError { url, .. }
            | Self::ConnectionRefused { url }
            | Self::ConnectionSilentlyDropped { url }
            | Self::TooManyRedirects { url }
            | Self::HttpError { url, .. }
            | Self::SslError { url, .. }
            | Self::Generic { url, .. } => Some(url),
            Self::IoError(_) => None,
        }
    }
}
