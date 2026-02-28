//! Domain types for the release gate

/// A bead issue from the beads system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bead {
    /// Unique identifier (e.g., "doc-105n")
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Issue status
    pub status: BeadStatus,
    /// Priority level (0 = P0, highest priority)
    pub priority: u8,
}

/// Status of a bead issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeadStatus {
    /// Issue is open and needs resolution
    #[default]
    Open,
    /// Issue has been resolved
    Closed,
    /// Issue was deleted/tombstoned
    Tombstone,
}

impl BeadStatus {
    /// Parse status from JSON value
    pub fn from_str(s: &str) -> Self {
        match s {
            "closed" => Self::Closed,
            "tombstone" => Self::Tombstone,
            _ => Self::Open,
        }
    }

    /// Returns true if this status is considered "open" for release blocking
    #[must_use]
    pub fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }
}

/// Result of the P0 check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum P0CheckResult {
    /// No P0 beads are open - release is clear
    Passed,
    /// P0 beads are open - release is blocked
    Failed(Vec<Bead>),
}

/// Result of the entire gate check
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateResult {
    /// Result of the P0 beads check
    pub p0_check: P0CheckResult,
    /// Whether CI passed
    pub ci_passed: bool,
}
