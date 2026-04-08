use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

const SYMBOL_ROLE_MASK: u32 = 0x1F;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SymbolRoleError {
    #[error("Unknown SymbolRole bit: {0}")]
    UnknownBit(u32),
}

/// Bitmask of semantic roles a symbol can play.
///
/// Uses hand-rolled u32 constants (no bitflags crate dependency).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct SymbolRole(u32);

impl SymbolRole {
    pub const DEFINITION: Self = SymbolRole(1);
    pub const READ: Self = SymbolRole(2);
    pub const WRITE: Self = SymbolRole(4);
    pub const GENERATED: Self = SymbolRole(8);
    pub const TEST: Self = SymbolRole(16);
    pub const ALL: Self = SymbolRole(31);

    pub fn from_bits(bits: u32) -> Result<Self, SymbolRoleError> {
        if bits & !SYMBOL_ROLE_MASK != 0 {
            return Err(SymbolRoleError::UnknownBit(bits));
        }
        Ok(Self(bits))
    }

    #[must_use]
    pub fn from_bits_truncate(bits: u32) -> Self {
        Self(bits & SYMBOL_ROLE_MASK)
    }

    #[must_use]
    pub const fn empty() -> Self {
        SymbolRole(0)
    }

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        SymbolRole(self.0 | other.0)
    }

    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        SymbolRole(self.0 & other.0)
    }
}

impl fmt::Display for SymbolRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "none");
        }

        let flags: [(&str, u32); 5] = [
            ("definition", 1),
            ("read", 2),
            ("write", 4),
            ("generated", 8),
            ("test", 16),
        ];

        let names: Vec<&str> = flags
            .iter()
            .filter(|(_, bit)| (self.0 & bit) != 0)
            .map(|(name, _)| *name)
            .collect();

        write!(f, "{}", names.join("+"))
    }
}

impl BitOr for SymbolRole {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SymbolRole(self.0 | rhs.0)
    }
}

impl BitAnd for SymbolRole {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        SymbolRole(self.0 & rhs.0)
    }
}

impl BitOrAssign for SymbolRole {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for SymbolRole {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl<'de> Deserialize<'de> for SymbolRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        Self::from_bits(bits).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[path = "symbol_role_tests_core.rs"]
mod tests_core;

#[cfg(test)]
#[path = "symbol_role_tests_extras.rs"]
mod tests_extras;
