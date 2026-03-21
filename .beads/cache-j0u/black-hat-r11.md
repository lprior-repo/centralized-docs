# BLACK-HAT AUDIT R11 — cache/ + errors/

**Auditor**: black-hat-reviewer  
**Scope**: `centralized-docs/src/cache/` (5 files) + `centralized-docs/src/errors/` (6 files) + `cache/tests/` (7 files)  
**Date**: 2026-03-21  
**Prior context**: NONE (fresh audit, zero prior rounds loaded)

---

## STATUS: REJECTED — 2 DEFECTS FOUND

---

## 5-PHASE AUDIT

### Phase 1: PANIC / UNWRAP / EXPECT (Constraint 15 — ZERO_PANICS_LAW)

**SCAN**: Every line of every production `.rs` file. Test files are exempt per AGENTS.md.

| File | unwrap | expect | panic! |
|---|---|---|---|
| cache/config.rs | 0 | 0 | 0 |
| cache/hash.rs | 0 | 0 | 0 |
| cache/store/mod.rs | 0 | 0 | 0 |
| cache/store/dedup.rs | 0 | 0 | 0 |
| errors/cache.rs | 0 | 0 | 0 |
| errors/config.rs | 0 | 0 | 0 |
| errors/embedding.rs | 0 | 0 | 0 |
| errors/mod.rs | 0 | 0 | 0 |
| errors/transformer.rs | 0 | 0 | 0 |
| errors/validation.rs | 0 | 0 | 0 |

Module-level enforcement active at `errors/mod.rs:1-3`:
```rust
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
```

`catch_unwind` at `store/mod.rs:207` CATCHES panics from user-supplied closures — does not cause them. Correct usage.

**PHASE 1: PASS**

---

### Phase 2: MUTABILITY & LOOP SCAN (Constraint 16 — FUNCTIONAL_PRIMITIVES)

Rule: `No mut. No for/while loops.`

| Location | Violation | Severity |
|---|---|---|
| `cache/hash.rs:100` | `let mut hasher = Sha256::new();` | **DEFECT** |
| `cache/hash.rs:103` | `let mut array = [0u8; 32];` | **DEFECT** |
| `cache/store/dedup.rs:50` | `loop { ... }` spin-wait | **Low** |

**DEFECT-011: `content_hash` uses 2x `mut` — eliminable**

Both `mut` uses are avoidable. The entire function body can be replaced with:

```rust
pub fn content_hash(content: &[u8]) -> [u8; 32] {
    use sha2::Digest;
    sha2::Sha256::digest(content).into()
}
```

`GenericArray<u8, U32>` implements `Into<[u8; 32]>` via `generic-array`. Zero `mut`, zero intermediate variables.

**Low: `wait_once_lock` spin-loop**

The `loop` at `dedup.rs:50` is a bounded spin-wait with 30s deadline. Replacing with `Condvar` adds complexity for marginal benefit. Flagged but not blocking.

**PHASE 2: FAIL — DEFECT-011**

---

### Phase 3: UNSAFE SCAN

| File | unsafe blocks |
|---|---|
| All production files | 0 |

`#![forbid(unsafe_code)]` at `errors/mod.rs:5`.

**PHASE 3: PASS**

---

### Phase 4: DDD & ILLEGAL STATES (Constraint 5 — DDD_ARCHITECTURE)

| Type | Pattern | Assessment |
|---|---|---|
| `CacheBackend` (config.rs:27) | Enum: `Memory \| File(PathBuf)` | Illegal states unrepresentable |
| `CacheType` (config.rs:83) | Enum: `Document \| Scrape \| Transform` | Clean sealed selector |
| `InflightDecision<V>` (dedup.rs:184) | Enum: `Cached \| Owner \| WaiterResult` | State machine, correct |
| `CacheError` (errors/cache.rs:8) | `#[non_exhaustive]` enum | Forward-compatible |
| `DocTransformerError` (errors/mod.rs:39) | `#[non_exhaustive]` enum with `#[from]` | Clean delegation |
| All error enums | `Clone + PartialEq + Eq` | Comparable, testable |

`#[non_exhaustive]` on all public types prevents downstream match exhaustiveness breakage.

**PHASE 4: PASS**

---

### Phase 5: DRY SCAN (Constraint 17 — EXTREME_DRY)

Rule: `Never repeat logic.`

**DEFECT-012: Duplicate `From<std::io::Error>` implementation**

`errors/mod.rs` contains two IDENTICAL `std::io::Error` conversions:

1. **Method** at line 98-102:
```rust
pub fn from_io_error(error: std::io::Error) -> Self {
    DocTransformerError::Io(error.into())
}
```

2. **Trait impl** at line 105-109:
```rust
impl From<std::io::Error> for DocTransformerError {
    fn from(error: std::io::Error) -> Self {
        DocTransformerError::Io(error.into())
    }
}
```

The method `from_io_error` is entirely redundant — the `From` trait impl provides the same conversion automatically via `.into()`. The method is dead code masquerading as API.

**Fix**: Delete `from_io_error` (lines 98-103). Verify no call sites remain.

**Structural repetition (borderline, not a defect)**

`store/mod.rs` has 3 near-identical `put_*` methods (lines 90-146) and 3 near-identical `get_*` methods (lines 82-132). Each differs only in table constant + config flag. The private `get()`/`put_raw()` methods partially DRY this for `get_or_compute`, but the public API retains repetition. This is a deliberate trade-off: named type-safe methods vs DRY. Acceptable.

**PHASE 5: FAIL — DEFECT-012**

---

## FULL 17-CONSTRAINT VERDICT

| # | Constraint | Verdict | Notes |
|---|---|---|---|
| 1 | SCIENTIFIC_RIGOR | PASS | |
| 2 | CODEBASE_LOCATION | PASS | |
| 3 | WORKSPACE_ISOLATION | N/A | No VCS ops in scope |
| 4 | DESTRUCTIVE_OPS_BANNED | N/A | No destructive ops in scope |
| 5 | DDD_ARCHITECTURE | PASS | Enums seal states correctly |
| 6 | RUST_CONTRACTS | N/A | Contracts are separate artifacts |
| 7 | FUNCTIONAL_RUST | PASS | Data->Calc->Actions |
| 8 | COMBATIVE_TESTING | PASS | 15+ adversarial tests |
| 9 | TOOLING | N/A | No build commands in scope |
| 10 | NO_MIGRATIONS | PASS | `initialize_tables` is idempotent |
| 11 | ISSUE_TRACKING | N/A | No tracking in source |
| 12 | GO_SKILL_WORKFLOW | N/A | |
| 13 | LANDING_SKILL | N/A | |
| 14 | FUNCTIONAL_CORE_IMPERATIVE_SHELL | PASS | Pure hash fns; IO at redb boundary |
| 15 | ZERO_PANICS_LAW | **PASS** | 0 violations in production |
| 16 | FUNCTIONAL_PRIMITIVES | **FAIL** | DEFECT-011: 2x `mut` in `content_hash` |
| 17 | EXTREME_DRY | **FAIL** | DEFECT-012: Duplicate `From` impl |

**Score: 12 PASS / 0 FAIL / 5 N/A / 2 FAIL = 14 applicable, 12 pass**

---

## DEFECT SUMMARY

| ID | Severity | File:Line | Description | Fix |
|---|---|---|---|---|
| DEFECT-011 | Medium | `cache/hash.rs:99-106` | `content_hash` uses 2x `mut` — replace body with `sha2::Sha256::digest(content).into()` | 1-line change |
| DEFECT-012 | Low | `errors/mod.rs:89-103` | `from_io_error` method duplicates `impl From<std::io::Error>` — delete the method | Delete 5 lines |

**Low (informational)**: `dedup.rs:50` spin-loop — acceptable bounded wait; `eprintln!` at `dedup.rs:163` — should be `log::warn!` per own comment.

---

## REAPPROVAL CRITERIA

Fix DEFECT-011 and DEFECT-012, then re-run `moon run :ci`. No additional rounds needed.
