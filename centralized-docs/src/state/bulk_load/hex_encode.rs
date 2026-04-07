//! Pure calculation: hex encoding.

/// Encode a byte slice as lowercase hex string.
///
/// Pure function: no side effects, deterministic output.
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(
        String::with_capacity(bytes.len().saturating_mul(2)),
        |mut acc, b| {
            use std::fmt::Write;
            let _ = write!(acc, "{b:02x}");
            acc
        },
    )
}
