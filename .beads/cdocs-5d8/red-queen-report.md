# THE RED QUEEN'S VERDICT

```
═══════════════════════════════════════════════════════════════

  Champion:    resolve_manifest_dir (centralized-docs/src/watch/diff.rs)
  Generations: 4
  Lineage:     12 checks (6 contract + 6 adversarial)
  Beads Filed: 6 (cdocs-zim, cdocs-b4s, cdocs-fbl, cdocs-l9o, cdocs-tg7, cdocs-lna)
  Final:       CROWN FORFEIT

═══════════════════════════════════════════════════════════════
```

## FITNESS LANDSCAPE (computed from test results)

```
Dimension                Tests  Survivors  Fitness  Status
──────────────────────────────────────────────────────────────
type-confusion             8        6       0.750   HEMORRHAGING
symlink-attacks            1        0       0.000   EXHAUSTED
path-edge-cases            1        0       0.000   EXHAUSTED
fs-structure               1        0       0.000   EXHAUSTED
error-fields               1        0       0.000   EXHAUSTED
precedence                 1        0       0.000   EXHAUSTED
nonexistent-path           1        0       0.000   EXHAUSTED
permission                 1        0       0.000   EXHAUSTED
race-condition             1        0       0.000   EXHAUSTED
return-identity            1        0       0.000   EXHAUSTED
diff-integration           2        0       0.000   EXHAUSTED
idempotent                 1        0       0.000   EXHAUSTED
caller-chain               1        0       0.000   EXHAUSTED
path-special-chars         1        0       0.000   EXHAUSTED
error-traits               1        0       0.000   EXHAUSTED
precedence-invariant       1        0       0.000   EXHAUSTED
path-whitespace            1        0       0.000   COOLING
deep-nesting               1        0       0.000   COOLING
symlink-chain              1        0       0.000   COOLING
error-message-content      1        0       0.000   COOLING
return-json-validity       1        0       0.000   COOLING
case-sensitivity           1        0       0.000   COOLING
```

## ROOT CAUSE ANALYSIS

**Single root cause: `exists()` vs `is_file()` type confusion**

All 6 survivors trace to one defect in `diff.rs:45` and `diff.rs:47`:

```rust
// CURRENT (vulnerable):
if direct.exists() {          // Line 45 — returns true for ANY fs entry type
    Ok(path.to_path_buf())
} else if nested.exists() {   // Line 47 — same problem
    Ok(scrape_subdir)
}

// CORRECT:
if direct.is_file() {         // Returns true ONLY for regular files
    Ok(path.to_path_buf())
} else if nested.is_file() {
    Ok(scrape_subdir)
}
```

`Path::exists()` returns `true` for directories, FIFOs, sockets, symlinks-to-dirs,
and any other filesystem entry. The contract implies `manifest.json` is a FILE, but
the implementation accepts any filesystem node named `manifest.json`.

## SURVIVOR FINDINGS

### [GEN-1-1] MAJOR: manifest.json is a directory (direct path)

```
═══════════════════════════════════════════════════════════════
Generation:     1
Dimension:      type-confusion
Test:           rq_manifest_json_is_a_directory_not_a_file
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path) — function claims resolution succeeded
Impact:         Caller opens "manifest.json" → EISDIR error downstream
Bead:           cdocs-zim
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

### [GEN-1-2] MAJOR: .scrape/manifest.json is a directory (nested path)

```
═══════════════════════════════════════════════════════════════
Generation:     1
Dimension:      type-confusion
Test:           rq_scrape_manifest_json_is_a_directory_not_a_file
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path/.scrape) — function claims resolution succeeded
Impact:         Caller opens .scrape/manifest.json → EISDIR downstream
Bead:           cdocs-b4s
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

### [GEN-1-3] MAJOR: manifest.json is a symlink to a directory

```
═══════════════════════════════════════════════════════════════
Generation:     1
Dimension:      type-confusion
Test:           rq_manifest_json_is_symlink_to_directory
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path) — symlink resolves to directory, exists() = true
Impact:         Same as GEN-1-1 via symlink indirection
Bead:           cdocs-fbl
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

### [GEN-2-4] MAJOR: manifest.json is a FIFO (named pipe)

```
═══════════════════════════════════════════════════════════════
Generation:     2
Dimension:      type-confusion
Test:           rq_manifest_json_is_a_fifo
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path) — FIFO exists() returns true
Impact:         Caller tries to read FIFO → blocks indefinitely or fails
Bead:           cdocs-l9o
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

### [GEN-2-5] MAJOR: manifest.json is a Unix domain socket

```
═══════════════════════════════════════════════════════════════
Generation:     2
Dimension:      type-confusion
Test:           rq_manifest_json_is_a_socket
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path) — socket exists() returns true
Impact:         Caller tries to read socket → EOPNOTSUPP or similar
Bead:           cdocs-tg7
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

### [GEN-2-6] MAJOR: .scrape/manifest.json is symlink to directory

```
═══════════════════════════════════════════════════════════════
Generation:     2
Dimension:      type-confusion
Test:           rq_nested_manifest_json_is_symlink_to_directory
Expected:       resolve_manifest_dir returns Err (not a file)
Actual:         Ok(path/.scrape) — symlink-to-dir exists() returns true
Impact:         Same as GEN-1-2 via symlink indirection in nested path
Bead:           cdocs-lna
done_when:      LOCKED (permanent regression gate)
═══════════════════════════════════════════════════════════════
```

## DEFENDED DIMENSIONS (55 tests passed, 0 survivors)

The following attack vectors were PROBED and SURVIVED by the champion:

| Dimension | Tests | Result |
|-----------|-------|--------|
| Symlink attacks (broken, circular, valid) | 5 | All correct |
| Path edge cases (empty, dot, double-slash, deep, root, trailing slash) | 7 | All correct |
| FS structure (.scrape is file, hidden manifest, empty, invalid JSON) | 8 | All correct |
| Error field correctness (relative, deep, non-existent paths) | 6 | All correct |
| Precedence invariants (direct always wins, fallback after removal) | 5 | All correct |
| Non-existent paths | 2 | All correct |
| Permission denied | 1 | Correct (graceful degradation) |
| Race conditions (delete/move between calls) | 3 | Correct (non-atomic, documented) |
| Return path identity (manifest.json always readable) | 4 | All correct |
| Diff integration (invalid/missing manifests) | 3 | All correct |
| Idempotent (100 calls deterministic) | 2 | All correct |
| Error traits (Clone, Debug, std::error::Error) | 3 | All correct |
| Special chars (tabs, newlines, unicode, spaces) | 5 | All correct |
| Case sensitivity (Manifest.json vs manifest.json) | 1 | Correct on Linux |
| Deep nesting (.scrape/.scrape NOT searched) | 1 | Correct |
| Symlink chains (deep resolution) | 1 | Correct |
| Char device via symlink (/dev/null) | 1 | Correct (symlink metadata check) |

## RECOMMENDED FIX

Replace `exists()` with `is_file()` at two call sites in `diff.rs`:

```rust
pub fn resolve_manifest_dir(path: &Path) -> Result<PathBuf, ManifestResolveError> {
    let direct = path.join("manifest.json");
    let scrape_subdir = path.join(".scrape");
    let nested = scrape_subdir.join("manifest.json");

    if direct.is_file() {          // ← was: direct.exists()
        Ok(path.to_path_buf())
    } else if nested.is_file() {   // ← was: nested.exists()
        Ok(scrape_subdir)
    } else {
        Err(ManifestResolveError::NotFound {
            path: path.to_path_buf(),
            scrape_subdir,
            direct,
            nested,
        })
    }
}
```

This one-line change (x2) eliminates all 6 survivors. `is_file()` returns `true` only
for regular files, rejecting directories, FIFOs, sockets, and other special entries.

## VALIDATION

```
Full Validation Results: 6/12 passed
  Contract checks: 6/6 PASS
  Adversarial checks: 0/6 PASS (all type-confusion survivors persist)

Ratchet Status: BROKEN — 6 regression checks failing
Equilibrium: Reached at Generation 4 (3 consecutive zero-new-survivor gens)
```

## CAMPAIGN SUMMARY

| Metric | Value |
|--------|-------|
| Total generations | 4 |
| Total tests executed | 61 |
| Tests passed (champion defended) | 55 |
| Tests failed (survivors found) | 6 |
| Beads filed | 6 |
| Dimensions explored | 23 |
| Dimensions with survivors | 1 (type-confusion) |
| Root cause | Single defect: `exists()` vs `is_file()` |
| Fix complexity | 2 lines changed |
| Crown verdict | **FORFEIT** |

---

*Generated by the Red Queen — Deterministic Adversarial Evolution*
*"It takes all the running you can do, to keep in the same place."*
*Date: 2026-04-20*
