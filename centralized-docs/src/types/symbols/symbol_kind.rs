use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SymbolKindError {
    #[error("Unknown SymbolKind: {0}")]
    UnknownKind(String),
}

/// Classification of a code symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Struct,
    Function,
    Trait,
    Enum,
    Module,
    Constant,
    Method,
    Field,
    Interface,
    TypeAlias,
    Variable,
    Package,
}

impl SymbolKind {
    const NAME_MAP: &'static [(&'static str, SymbolKind)] = &[
        ("struct", SymbolKind::Struct),
        ("function", SymbolKind::Function),
        ("trait", SymbolKind::Trait),
        ("enum", SymbolKind::Enum),
        ("module", SymbolKind::Module),
        ("constant", SymbolKind::Constant),
        ("method", SymbolKind::Method),
        ("field", SymbolKind::Field),
        ("interface", SymbolKind::Interface),
        ("type_alias", SymbolKind::TypeAlias),
        ("variable", SymbolKind::Variable),
        ("package", SymbolKind::Package),
    ];

    #[must_use]
    fn as_str(self) -> &'static str {
        match self {
            Self::Struct => "struct",
            Self::Function => "function",
            Self::Trait => "trait",
            Self::Enum => "enum",
            Self::Module => "module",
            Self::Constant => "constant",
            Self::Method => "method",
            Self::Field => "field",
            Self::Interface => "interface",
            Self::TypeAlias => "type_alias",
            Self::Variable => "variable",
            Self::Package => "package",
        }
    }

    fn from_str_ci(s: &str) -> Result<Self, SymbolKindError> {
        let lower = s.to_lowercase();
        Self::NAME_MAP
            .iter()
            .find(|(name, _)| *name == lower)
            .map(|(_, kind)| *kind)
            .ok_or_else(|| SymbolKindError::UnknownKind(s.to_string()))
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for SymbolKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SymbolKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str_ci(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[path = "symbol_kind_tests.rs"]
mod tests;
