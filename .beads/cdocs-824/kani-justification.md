# Kani Justification: cdocs-824

**Bead ID:** cdocs-824
**Title:** data: add zero-copy state dependencies to centralized-docs crate
**Phase:** State 5.7 (Kani)
**Date:** 2026-04-05

## Why Kani Is Not Applicable

This bead is classified as a **chore** that only adds dependencies to `Cargo.toml`. No Rust code was written or modified.

### Kani Applicability Analysis

Kani is a formal verification tool that:
- Proves properties about Rust code
- Finds bugs like out-of-bounds array access, panics, etc.
- Requires harnesses that specify properties to verify

### This Bead Changed Nothing That Kani Can Verify

1. **No custom functions** — This bead wrote zero Rust functions
2. **No algorithmic code** — No algorithms to verify
3. **Only dependency declarations** — Just `Cargo.toml` entries:
   ```toml
   rkyv = { version = "0.8", features = ["std", "bytecheck"] }
   bytemuck = { version = "1", features = ["derive", "const_zeroed"] }
   ```

4. **External dependencies are already verified** — bytemuck and rkyv have their own test suites

### What Would Require Kani

If this bead had:
- Written new data structures with invariants
- Implemented serialization/deserialization logic
- Created state machines with safety requirements
- Modified unsafe code blocks

Then Kani would be mandatory. But none of these apply.

## Formal Justification

| Criterion | Assessment |
|-----------|------------|
| Critical invariants to verify | None — no custom code written |
| Safety properties required | None — dependency declarations are safe |
| Kani harnesses written | None — test-writer correctly skipped (no code to verify) |
| Evidence this is correct | External crates (bytemuck, rkyv) have their own verification |

## Conclusion

**KANI: NOT APPLICABLE**

The test-writer correctly determined that no Kani harnesses were needed for this chore bead. This is appropriate because:
1. No implementation code exists in this bead
2. Only external dependency declarations were added
3. The added crates are mature and well-tested

**Proceed to State 7 (Architectural Drift)**
