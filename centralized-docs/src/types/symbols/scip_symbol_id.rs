use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScipSymbolIdError {
    #[error("SCIP scheme cannot be empty")]
    EmptyScheme,
    #[error("SCIP scheme contains invalid character: {0}")]
    InvalidScheme(String),
    #[error("SCIP module path cannot be empty")]
    EmptyModulePath,
    #[error("SCIP module path contains empty segment at position {0}")]
    EmptyModuleSegment(usize),
    #[error("SCIP module path must not start with '/'")]
    LeadingSlash,
    #[error("SCIP module path must not end with '/'")]
    TrailingSlash,
    #[error("SCIP module path must not contain '#'")]
    HashInModulePath,
    #[error("SCIP descriptor cannot be empty")]
    EmptyDescriptor,
    #[error("SCIP descriptor must not contain '/'")]
    SlashInDescriptor,
    #[error("Invalid SCIP symbol format: {0}")]
    InvalidFormat(String),
}

/// Deterministic SCIP-format symbol identifier.
///
/// Format: `<scheme>/<module_path>#<descriptor>`
/// Example: `rust/auth/AuthService#login()`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct ScipSymbolId(String);

impl ScipSymbolId {
    fn validate_scheme(scheme: &str) -> Result<(), ScipSymbolIdError> {
        if scheme.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyScheme);
        }
        if let Some(ch) = scheme.chars().find(|c| *c == '/' || *c == '#') {
            return Err(ScipSymbolIdError::InvalidScheme(ch.to_string()));
        }
        Ok(())
    }

    fn validate_module_path(path: &str) -> Result<(), ScipSymbolIdError> {
        if path.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyModulePath);
        }
        if path.starts_with('/') {
            return Err(ScipSymbolIdError::LeadingSlash);
        }
        if path.ends_with('/') {
            return Err(ScipSymbolIdError::TrailingSlash);
        }
        if path.find('#').is_some() {
            return Err(ScipSymbolIdError::HashInModulePath);
        }
        if let Some(pos) = path.find("//") {
            return Err(ScipSymbolIdError::EmptyModuleSegment(pos + 1));
        }
        Ok(())
    }

    fn validate_descriptor(descriptor: &str) -> Result<(), ScipSymbolIdError> {
        if descriptor.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyDescriptor);
        }
        if descriptor.contains('/') {
            return Err(ScipSymbolIdError::SlashInDescriptor);
        }
        if descriptor.contains('#') {
            return Err(ScipSymbolIdError::InvalidScheme("#".to_string()));
        }
        Ok(())
    }

    pub fn new(
        scheme: impl Into<String>,
        module_path: impl Into<String>,
        descriptor: impl Into<String>,
    ) -> Result<Self, ScipSymbolIdError> {
        let scheme = scheme.into();
        let module_path = module_path.into();
        let descriptor = descriptor.into();

        Self::validate_scheme(&scheme)?;
        Self::validate_module_path(&module_path)?;
        Self::validate_descriptor(&descriptor)?;

        let canonical = format!("{scheme}/{module_path}#{descriptor}");
        Ok(Self(canonical))
    }

    pub fn parse(s: &str) -> Result<Self, ScipSymbolIdError> {
        let hash_pos = match s.find('#') {
            Some(pos) => pos,
            None => return Err(ScipSymbolIdError::InvalidFormat(s.to_string())),
        };

        if s[hash_pos + 1..].contains('#') {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }

        let pre_hash = &s[..hash_pos];
        let slash_pos = match pre_hash.find('/') {
            Some(pos) => pos,
            None => return Err(ScipSymbolIdError::InvalidFormat(s.to_string())),
        };

        let scheme = &pre_hash[..slash_pos];
        let module_path = &pre_hash[slash_pos + 1..];
        let descriptor = &s[hash_pos + 1..];

        if scheme.is_empty() {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }
        if descriptor.is_empty() {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }

        Self::validate_scheme(scheme)?;
        Self::validate_module_path(module_path)?;
        Self::validate_descriptor(descriptor)?;

        Ok(Self(s.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn scheme(&self) -> &str {
        self.0
            .split('#')
            .next()
            .and_then(|pre_hash| pre_hash.split('/').next())
            .unwrap_or("")
    }

    #[must_use]
    pub fn module_path(&self) -> &str {
        self.0
            .split('#')
            .next()
            .and_then(|pre_hash| {
                let slash_pos = pre_hash.find('/')?;
                Some(&pre_hash[slash_pos + 1..])
            })
            .unwrap_or("")
    }

    #[must_use]
    pub fn descriptor(&self) -> &str {
        self.0.split('#').nth(1).unwrap_or("")
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ScipSymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ScipSymbolId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ScipSymbolId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ScipSymbolId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ScipSymbolId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}
