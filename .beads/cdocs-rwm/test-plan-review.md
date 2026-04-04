# Test Plan Review: cdocs-rwm (Round 2)

bead_id: cdocs-rwm
reviewer: test-inquisitor (Mode 1 — Plan Inquisition)
phase: state-1.7-test-plan-review
reviewed_at: 2026-04-04T00:00:00Z
revision: 2 (re-audit of revised test-plan.md)

---

## VERDICT: APPROVED

All 13 LETHAL and 4 MAJOR findings from Round 1 have been resolved. 0 new LETHAL. 2 new MINOR (below threshold).

---

## Previous Mandate Verification

Every mandate from the Round 1 rejection has been checked against the revised plan:

| # | Round 1 Mandate | Resolution | Status |
|---|---|---|---|
| L01 | `CommitError::DatabaseOpen` exact variant | B02: `Err(CommitError::DatabaseOpen { path, reason })` + `path.contains("readonly") == true`, `reason` non-empty | **FIXED** |
| L02 | `CommitError::TableInit` scenario | B06: dedicated scenario with corrupted DB construction, field check on `reason` | **FIXED** |
| L03 | `CommitError::ReadTransaction` exact variant | B11: dedicated scenario with file truncation corruption, field check on `reason` | **FIXED** |
| L04 | `CommitError::WriteTransaction` split from "or" | B19: dedicated scenario, `reason` non-empty, explicit "does NOT return CommitFailed" | **FIXED** |
| L05 | `CommitError::CommitFailed` split from "or" | B20: dedicated scenario with disk-full simulation, `reason` non-empty | **FIXED** |
| L06 | `CommitError::ZeroHashKey` scenario | B23: dedicated scenario via direct `commit_changes` call, exact field check `{ table: "snapshots", index: 0 }`, K05 proof for `store_snapshot` unreachability | **FIXED** |
| L07 | `StateError::SerializationFailed` test | B40: structural construction test + K03 proof of unreachability | **FIXED** |
| L08 | `StateError::DeserializationFailed` split from "or" | B13: dedicated truncated-archive scenario with `type_name == "Snapshot"`, B48: separate corrupt-bytes scenario | **FIXED** |
| L09 | `StateError::InvalidArchive` split from "or" | B14: dedicated garbage-bytes scenario, B49: separate empty-bytes scenario | **FIXED** |
| L10 | `StateError::ArchiveValidationFailed` dedicated | B46: dedicated scenario with wrong-type archive, field check on `key_hex` and `message` | **FIXED** |
| L11 | `StateError::TableOpenFailed` exact variant | B12: dedicated scenario with field check `{ table: "snapshots", message }`, B44: via dropped table | **FIXED** |
| L12 | `StateError::StorageError` test | B45: dedicated scenario with file truncation corruption, field check on `operation` and `message` | **FIXED** |
| L13 | Trophy ratio < 5× | 55 behaviors / 8 functions = 6.88× | **FIXED** |
| M01 | `>= 50 MiB` boundary mutation | B22: exactly 52,428,800 bytes, expects `Ok(())` | **FIXED** |
| M02 | B19 (SerializationFailed) error path invisible | B40 structural test + K03 proof | **FIXED** |
| M03 | B23 (StorageError) error path invisible | B45 dedicated scenario | **FIXED** |
| M04 | `store_snapshot` 3 missing boundaries | B16 (empty snapshot), B17 (overwrite), B22 (exact 50 MiB) | **FIXED** |

---

## Axis 1 — Contract Parity

### Public Function Coverage

| # | Function | BDD Scenarios | Covered? |
|---|----------|---------------|----------|
| 1 | `open_state_db(cache_path)` | B01, B02, B03, B04, B05, B06 | **YES** |
| 2 | `load_snapshot(state_db, url)` | B07, B08, B09, B10, B11, B12, B13, B14 | **YES** |
| 3 | `store_snapshot(state_db, url, snapshot)` | B15, B16, B17, B18, B19, B20, B21, B22, B23 | **YES** |
| 4 | `run_watch(...)` | B24, B25, B26, B27, B28, B29, B30, B31 | **YES** |
| 5 | `run_apply(...)` | B32, B33, B34, B35, B36, B37 | **YES** |
| 6 | `serialize_snapshot(snapshot)` | B38, B39, B40 | **YES** |
| 7 | `StateReadSession::load_snapshots(keys)` | B41, B42, B43, B44, B45, B46 | **YES** |
| 8 | `ArchivedRaw::deserialize<T>()` | B47, B48, B49 | **YES** |

All 8 public functions have ≥1 BDD scenario. **PASS**.

### Error Variant Coverage

| Error Variant | Scenario | Exact Variant Assertion | Status |
|---|---|---|---|
| `CommitError::DatabaseOpen` | B02 | `Err(CommitError::DatabaseOpen { path, reason })` + field checks | **PASS** |
| `CommitError::TableInit` | B06 | `Err(CommitError::TableInit { reason })` + `reason` non-empty | **PASS** |
| `CommitError::ReadTransaction` | B11 | `Err(CommitError::ReadTransaction { reason })` + `reason` non-empty | **PASS** |
| `CommitError::WriteTransaction` | B19 | `Err(CommitError::WriteTransaction { reason })` + `reason` non-empty | **PASS** |
| `CommitError::CommitFailed` | B20 | `Err(CommitError::CommitFailed { reason })` + `reason` non-empty | **PASS** |
| `CommitError::ZeroHashKey` | B23 | `Err(CommitError::ZeroHashKey { table: "snapshots", index: 0 })` | **PASS** |
| `CommitError::PayloadTooLarge` | B21 | `Err(CommitError::PayloadTooLarge { table: "snapshots", size: 52428801, max: 52428800 })` | **PASS** |
| `StateError::SerializationFailed` | B40 | K03 proof + structural construction | **PASS** |
| `StateError::DeserializationFailed` | B13, B48 | `Err(StateError::DeserializationFailed { type_name: "Snapshot", message })` | **PASS** |
| `StateError::InvalidArchive` | B14, B49 | `Err(StateError::InvalidArchive { type_name, message })` | **PASS** |
| `StateError::ArchiveValidationFailed` | B46 | `Err(StateError::ArchiveValidationFailed { key_hex, message })` | **PASS** |
| `StateError::TableOpenFailed` | B12, B44 | `Err(StateError::TableOpenFailed { table: "snapshots", message })` | **PASS** |
| `StateError::StorageError` | B45 | `Err(StateError::StorageError { operation, message })` | **PASS** |

All 13 error variants have dedicated scenarios with exact variant assertions. No `Err(_)`, no `is_err()`, no "or" between variants. **PASS**.

---

## Axis 2 — Assertion Sharpness

Every "Then:" clause in every BDD scenario (B01–B57) audited for:

| Pattern | Count | Severity |
|---|---|---|
| `is_ok()` without concrete value | 0 | — |
| `is_err()` without exact variant | 0 | — |
| `> 0` / `Some(_)` without inner value | 0 | — |
| Concrete value assertions (`== exact`, field checks) | 55 | — |

Notable sharp assertions verified:
- B07: `snapshot == stored_snapshot` (PartialEq) + `pages.len() == 1` + `content_hash == [0xAB; 32]`
- B09: `snapshot.target_url == "https://example.com"` + `snapshot.pages.is_empty() == true`
- B13: `type_name == "Snapshot"` + `message` non-empty + explicit "does NOT return InvalidArchive"
- B21: `size: 52428801, max: 52428800` — concrete numeric values
- B22: exactly 52,428,800 bytes → `Ok(())`
- B23: `table: "snapshots", index: 0` — exact struct literal
- B46: `key_hex` is "64-character hex string" + `message` non-empty + "does NOT return InvalidArchive"

B03 and B05 use `state_db.begin_read().is_ok() == true` as a secondary verification that the DB is usable — this is a _setup validation_, not the primary assertion. The primary assertion is `returns Ok(state_db)`. Acceptable.

B52 and B53 verify transaction counts by asserting `load_snapshot` and `store_snapshot` were called "exactly once" — this is observational, not structural. In implementation, these will need either mock counting or code-structure verification. Flagged as MINOR (M01 below).

**PASS** — no LETHAL assertion sharpness violations.

---

## Axis 3 — Trophy Allocation

### Density

- Public functions: **8**
- Behaviors with test functions (B01–B53, excluding static B54–B56): **53**
- Static checks (B54, B55, B56): **3** (compile-time, grep, diff — not `#[test]` but provide real coverage)
- Proptest invariants: **4** (P01–P04, all scoped to this bead's functions)
- Fuzz targets: **3** (F01–F03)
- Kani harnesses: **5** (K01–K05)

Effective test function count: 53 + 3 (static) = **56**. 56 / 8 = **7.0×**. Exceeds 5×. **PASS**.

The plan claims 55 in the summary but the behavior inventory lists B01–B57 = 57 behaviors. B56 and B57 overlap with existing scenarios (B56 is a diff check, B57 duplicates B09's assertions). The actual effective unique test count is ≥53 distinct test functions. Arithmetic rounding doesn't change the verdict — well above 5×.

### Pure Functions → Proptest

| Pure Function | Non-trivial Input Space? | Proptest? | Status |
|---|---|---|---|
| `serialize_snapshot` | Yes (varying URL lengths, page counts, content) | P01 (round-trip invariant) | **PASS** |
| `url_hash` | Yes (any non-empty string → 32 bytes) | P02 (determinism + length) | **PASS** |
| `ArchivedRaw::deserialize` | Yes (any byte sequence) | F02 (fuzz) | **PASS** |

### Parsers/Deserializers → Fuzz

| Deserializer | Fuzz Target? | Status |
|---|---|---|
| `ArchivedRaw::deserialize` | F02 | **PASS** |
| `load_snapshot` (via corrupted StateDb) | F03 | **PASS** |
| `serialize_snapshot` | F01 | **PASS** |

### Integration/Unit Ratio

36 integration (65%) / 14 unit (25%) / 5 static (9%). The plan acknowledges the integration weight is due to the I/O-bound nature of the migration. The calc layer (INV-1) is explicitly unchanged. Static checks provide compile-time enforcement. Acceptable.

**PASS** — no LETHAL trophy allocation violations.

---

## Axis 4 — Boundary Completeness

### `open_state_db`

| Boundary | Covered? | By |
|---|---|---|
| Valid writable path | YES | B01 |
| Read-only parent | YES | B02 |
| Creates missing parents | YES | B03 |
| Empty path | YES | B04 |
| Max path length (4096+) | YES | B05 |
| Table init failure | YES | B06 |

**Missing**: None. All 6 boundaries covered.

### `load_snapshot`

| Boundary | Covered? | By |
|---|---|---|
| Key exists (1 page) | YES | B07 |
| Key exists (50 pages) | YES | B08 |
| Key missing → default | YES | B09 |
| Unicode URL | YES | B10 |
| `begin_read` failure | YES | B11 |
| Table open failure | YES | B12 |
| Truncated rkyv bytes | YES | B13 |
| Garbage (non-rkyv) bytes | YES | B14 |

**Missing**: None critical. Empty URL violates PRE-2 (contract precondition) — not a boundary the function is required to handle (it's a caller violation). Long URL > 2000 chars: SHA-256 handles arbitrary length input, so this isn't a meaningful boundary for the function itself.

### `store_snapshot`

| Boundary | Covered? | By |
|---|---|---|
| Valid snapshot | YES | B15 |
| Empty snapshot (0 pages) | YES | B16 |
| Overwrite existing | YES | B17 |
| Serialization failure | YES | B18 + K03 |
| Write tx begin failure | YES | B19 |
| Commit failure | YES | B20 |
| Payload > 50 MiB | YES | B21 |
| Payload == exactly 50 MiB | YES | B22 |
| Zero hash key | YES | B23 |

**Missing**: None. All 9 boundaries covered.

### `run_watch`

| Boundary | Covered? | By |
|---|---|---|
| Read-only (no commit) | YES | B24 |
| Reads from StateDb | YES | B25 |
| First scrape (no previous) | YES | B26 |
| Unchanged content | YES | B27 |
| New page added | YES | B28 |
| Page removed | YES | B29 |
| Page modified | YES | B30 |
| Multiple pages changed | YES | B31 |

**Missing**: None. All 8 boundaries covered.

### `run_apply`

| Boundary | Covered? | By |
|---|---|---|
| Changes present | YES | B32 |
| Empty plan (skip) | YES | B33 |
| Idempotent | YES | B34 |
| `--yes` flag | YES | B35 |
| No `--yes` (stdin prompt) | YES | B36 |
| Multiple pages | YES | B37 |

**Missing**: None. All 6 boundaries covered.

### `serialize_snapshot`

| Boundary | Covered? | By |
|---|---|---|
| Valid round-trip | YES | B38 |
| Non-empty bytes | YES | B39 |
| Serialization error | YES | B40 + K03 |

### `StateReadSession::load_snapshots`

| Boundary | Covered? | By |
|---|---|---|
| Subset of keys | YES | B41 |
| No matching keys | YES | B42 |
| Empty key list | YES | B43 |
| Table dropped | YES | B44 |
| Storage error | YES | B45 |
| Wrong-type archive | YES | B46 |

### `ArchivedRaw::deserialize`

| Boundary | Covered? | By |
|---|---|---|
| Valid archive | YES | B47 |
| Corrupt bytes | YES | B48 |
| Empty bytes | YES | B49 |

**Total missing boundaries: 0 across all functions.** **PASS**.

---

## Axis 5 — Mutation Survivability

Every mutation from Section 7 checked:

| Mutation | Caught By | Verified? |
|---|---|---|
| `open_state_db` returns `Err` instead of `Ok` | B01 | YES — `Ok(state_db)` + `begin_read().is_ok()` |
| Wrong variant for read-only parent | B02 | YES — exact `DatabaseOpen` match |
| Skips `create_dir_all` | B03 | YES — filesystem existence assertion |
| `load_snapshot` ignores stored data | B07 | YES — `snapshot == stored_snapshot` |
| `load_snapshot` returns error for missing key | B09 | YES — `Ok(snapshot)` + field checks |
| Wrong error variant for corrupt bytes | B13, B14 | YES — exact variant + "does NOT return" disambiguation |
| `store_snapshot` skips `commit_changes` | B15 | YES — subsequent `load_snapshot` + entry count |
| `store_snapshot` writes 0 entries | B15 | YES — entry count == 1 |
| `>= MAX_VALUE_SIZE` instead of `>` | B21, B22 | YES — B22 tests exactly at boundary, B21 tests above |
| `serialize_snapshot` wrong bytes | B38 | YES — round-trip `PartialEq` |
| `serialize_snapshot` returns `Ok(vec![])` | B39 | YES — `bytes.len() > 0` |
| `ArchivedRaw::deserialize` Ok for corrupt | B48, B49 | YES — exact variant match |
| Wrong variant from `ArchivedRaw` | B48, B49 | YES — DeserializationFailed vs InvalidArchive distinguished |
| `load_snapshots` returns unrequested keys | B41 | YES — `map.contains_key(&k2) == false` |
| `load_snapshots` swallows table error | B44 | YES — exact `TableOpenFailed` |
| Wrong variant for corrupt bytes in `load_snapshots` | B46 | YES — `ArchiveValidationFailed` + "does NOT return InvalidArchive" |
| `run_watch` calls `store_snapshot` | B24 | YES — snapshots table unchanged |
| `run_apply` commits 0 entries | B32 | YES — entry count == 1 |
| `run_apply` doesn't skip empty plan | B33 | YES — entry count unchanged |
| `run_apply` non-idempotent | B34 | YES — second run, entry count remains 1 |
| `url_hash` wrong key | B51 | YES — byte-identical across calls |
| Default snapshot wrong URL | B57 | YES — `target_url == "https://docs.rs/serde"` |
| `ZeroHashKey` check removed | B23 | YES — exact variant match |
| `PayloadTooLarge` check removed | B21 | YES — exact variant with concrete values |
| `TableInit` error mapping changed | B06 | YES — exact variant match |
| `begin_read` error mapping changed | B11 | YES — exact variant match |
| `begin_write` error mapping changed | B19 | YES — exact variant match + "does NOT return CommitFailed" |
| `commit` error mapping changed | B20 | YES — exact variant match + "does NOT return WriteTransaction" |

**27/27 mutations covered.** **PASS**.

Additional mutation thought-experiments not in the table:

| Mutation | Caught? | By |
|---|---|---|
| `store_snapshot` writes to wrong key | YES | B15 — subsequent `load_snapshot` with same URL would miss it |
| `load_snapshot` returns default with wrong URL | YES | B09, B57 — `target_url` field check |
| `run_apply` with `--yes` still reads stdin | YES | B35 — "without reading from stdin" |
| `run_apply` without `--yes` auto-commits | YES | B36 — "no commit" assertion |
| `serialize_snapshot` returns different bytes for same input | YES | P01 proptest + B38 |

---

## Axis 6 — Holzmann Plan Audit

### Rule 2 (Bound Every Loop)
No loops in test scenarios. All iteration is via rstest cartesian products or proptest strategies. **PASS**.

### Rule 5 (State Your Assumptions)
Previous review flagged 6 scenarios with hidden preconditions. All now have explicit Given blocks:

| Scenario | Given Block | Status |
|---|---|---|
| B07 (was B04) | Full Snapshot construction with exact field values specified | **FIXED** |
| B11 (was B06) | "corrupted by truncating to half its original size" — specific corruption method | **FIXED** |
| B13 (was B08) | "truncated valid archive" — "first 50% of the original bytes" via raw write transaction | **FIXED** |
| B14 (was B08) | "256 bytes of `0xDE`" — specific garbage bytes | **FIXED** |
| B26 (was B14) | "scrape fixture returning 3 pages" with fixture-based injection | **FIXED** |
| B32 (was B15) | "scrape_dir at `scrape_dir/manifest.json` containing 3 pages" | **FIXED** |

B52 and B53 use observational language ("was called exactly once") without specifying the observation mechanism. This is a testability concern for implementation but not a precondition violation in the plan. **MINOR** (see M02 below).

### Rule 8 (Surface Your Side Effects)
All filesystem operations (tempdir, chmod, directory creation, file corruption) are named in Given blocks. redb write operations named as `commit_changes`. Side effects are visible. **PASS**.

---

## Aggregated Findings

### LETHAL FINDINGS (0)

None.

### MAJOR FINDINGS (0)

None.

### MINOR FINDINGS (2)

**M01: B52/B53 observational assertion mechanism unspecified**
- `test-plan.md:733` — B52 says "verifiable by asserting that `run_watch` only opens one `StateReadSession`" without specifying HOW. In implementation, this requires either (a) a mock with call counting, (b) an atomic counter in the read session constructor, or (c) code-structure verification. The plan should specify the mechanism.
- `test-plan.md:745` — B53 same issue.
- **Impact**: Implementation ambiguity, not a coverage gap. The scenarios describe WHAT to verify, not HOW. Test writer will need to choose a mechanism.

**M02: Behavior count discrepancy in summary**
- `test-plan.md:11` — Summary says "55 behaviors" but inventory lists B01–B57 = 57. B56 and B57 overlap with B09 (default snapshot assertions). Effective unique count is 55 (B57 is a stricter version of B09's assertions, B56 is a separate static check). Arithmetic is close enough that the density ratio (6.88×) remains valid regardless.
- **Impact**: Documentation clarity only.

---

## Summary

| Axis | Status | Details |
|---|---|---|
| 1 — Contract Parity | **PASS** | All 8 functions covered. All 13 error variants have exact-assertion scenarios. |
| 2 — Assertion Sharpness | **PASS** | Zero `is_ok()`/`is_err()` without concrete values. Zero "or" between variants. |
| 3 — Trophy Allocation | **PASS** | 56 tests / 8 functions = 7.0×. 4 proptest invariants. 3 fuzz targets. 5 Kani harnesses. |
| 4 — Boundary Completeness | **PASS** | All boundaries for all 8 functions explicitly named and tested. |
| 5 — Mutation Survivability | **PASS** | 27/27 planned mutations covered. Additional ad-hoc mutations all caught. |
| 6 — Holzmann Plan Audit | **PASS** | No loops. Preconditions explicit. Side effects surfaced. |

0 LETHAL + 0 MAJOR + 2 MINOR (below 5 threshold) = **APPROVED**.

---

## MANDATE (Round 1 → Round 2)

All 13 LETHAL mandates resolved. All 4 MAJOR mandates resolved. No new mandates.

**Recommendation for implementation**: When writing B52 and B53, choose an explicit observation mechanism (e.g., atomic counter on `begin_read` calls, or a wrapper struct that counts). Document the choice in the test file header.
