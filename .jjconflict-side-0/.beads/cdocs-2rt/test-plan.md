# Test Plan: cdocs-2rt — Deterministic Config Hashing & `compute_file_diff`

## Summary

| Metric | Count |
|--------|-------|
| Behaviors identified | 31 |
| BDD scenarios | 34 |
| Trophy allocation: Unit (Calc layer) | 10 |
| Trophy allocation: Integration (`/tests/`) | 22 |
| Trophy allocation: E2E (CLI) | 1 |
| Trophy allocation: Static | 1 |
| Proptest invariants | 6 |
| Fuzz targets | 2 |
| Kani harnesses | 3 |
| Mutation kill threshold | ≥90% |

**Target ratios**: ~60% integration / ~27% unit / ~3% e2e / ~3% static.

**Justification**: The primary function (`compute_file_diff`) reads files from disk via
rayon — its core behaviors are integration-layer by nature. Pure `compute_config_hash`
and the partition invariant can be unit-tested without I/O. Boundary scenarios for
duplicate source_paths, symlink traversal, config-path interaction, and key-format
mismatch are all integration-layer (require real filesystem state).

---

## 1. Behavior Inventory

> Format: `[Subject] [action] [outcome] when [condition]`

### `compute_config_hash` (8 behaviors)

1. `compute_config_hash` returns `content_hash(b"")` when `category_config_path` is `None`
2. `compute_config_hash` returns SHA-256 of file bytes when `category_config_path` points to a readable file
3. `compute_config_hash` returns `content_hash(b"")` when `category_config_path` points to a non-existent file
4. `compute_config_hash` returns `content_hash(b"")` when `category_config_path` points to an unreadable file
5. `compute_config_hash` returns identical `ContentHash` across calls when given identical input (determinism)
6. `compute_config_hash` returns distinct concrete hashes when given files with different contents
7. `compute_config_hash` returns `content_hash(b"")` when `category_config_path` points to an empty (0-byte) file
8. `compute_config_hash` returns exact SHA-256 digest when `category_config_path` points to a large (≥1MB) file

### `compute_file_diff` (23 behaviors)

9. `compute_file_diff` returns `Err(DiffError::SourceDirNotFound)` when `source_dir` does not exist
10. `compute_file_diff` returns `Err(DiffError::FileRead { path, source })` when a discovered file cannot be read from disk
11. `compute_file_diff` returns `Err(DiffError::PathTraversal { path })` when a `source_path` resolves outside `source_dir`
12. `compute_file_diff` classifies every file as `New` when `stored_hashes` is empty
13. `compute_file_diff` classifies every stored key as `Deleted` when `discovered_files` is empty
14. `compute_file_diff` classifies file as `Unchanged` when both content hash and config hash match stored values
15. `compute_file_diff` classifies file as `Changed` when on-disk content hash differs from stored content hash
16. `compute_file_diff` classifies file as `Changed` when config hash differs from stored config hash (content unchanged)
17. `compute_file_diff` classifies file as `Changed` when both content hash and config hash differ
18. `compute_file_diff` classifies file as `New` when it is absent from `stored_hashes`
19. `compute_file_diff` classifies stored-only paths as `Deleted`
20. `compute_file_diff` produces mutually exclusive buckets (partition invariant: intersection of any two is empty)
21. `compute_file_diff` produces collectively exhaustive buckets (union covers all discovered + stored paths)
22. `compute_file_diff` handles mixed scenarios: some unchanged, some changed, some new, some deleted in a single call
23. `compute_file_diff` returns all-empty buckets when both `discovered_files` and `stored_hashes` are empty
24. `compute_file_diff` never mutates its inputs or writes to disk (no-side-effect invariant)
25. `compute_file_diff` does not panic and produces a deterministic single-bucket result when `discovered_files` contains duplicate `source_path` entries
26. `compute_file_diff` classifies all files as `Changed` when `category_config_path` points to a nonexistent file but `stored_hashes` contains real config hashes
27. `compute_file_diff` returns `Err(DiffError::PathTraversal)` when a file inside `source_dir` is a symlink that resolves outside `source_dir`
28. `compute_file_diff` returns `Err(DiffError::FileRead)` or `Err(DiffError::PathTraversal)` when `source_path` is empty string `""`
29. `compute_file_diff` does not panic when `source_path` exceeds PATH_MAX length
30. `compute_file_diff` never panics when `stored_hashes` keys use a different format than `DiscoveryFile::source_path` values
31. `compute_file_diff` classifies by content hash alone regardless of `DiscoveryFile::size_bytes` value

---

## 2. Trophy Allocation

| # | Behavior | Layer | Rationale |
|---|----------|-------|-----------|
| 1 | config hash returns empty-hash for None | Unit | Pure function, no I/O |
| 2 | config hash returns SHA-256 of file bytes | Integration | Reads from filesystem |
| 3 | config hash returns empty-hash for non-existent file | Integration | Filesystem I/O |
| 4 | config hash returns empty-hash for unreadable file | Integration | Filesystem I/O + permissions |
| 5 | config hash is deterministic (same input → same output) | Unit | Pure property test, no I/O variant |
| 6 | config hash returns distinct concrete hashes for different contents | Integration | Requires file creation |
| 7 | config hash returns empty-hash for empty (0-byte) file | Integration | Filesystem I/O (distinct code path from None) |
| 8 | config hash returns exact SHA-256 for large (≥1MB) file | Integration | Filesystem I/O, no OOM/truncation |
| 9 | file_diff returns SourceDirNotFound | Integration | Filesystem pre-condition check |
| 10 | file_diff returns FileRead on unreadable file | Integration | Filesystem I/O + permission simulation |
| 11 | file_diff returns PathTraversal on escape | Integration | Filesystem canonicalization |
| 12 | file_diff classifies all as New when no stored hashes | Integration | Reads real files from tempdir |
| 13 | file_diff classifies all as Deleted when no discovered files | Integration | Empty file list + stored state |
| 14 | file_diff classifies Unchanged correctly | Integration | Reads real files, compares hashes |
| 15 | file_diff classifies Changed (content differs) | Integration | Reads real files |
| 16 | file_diff classifies Changed (config differs) | Integration | Config file change simulation |
| 17 | file_diff classifies Changed (both differ) | Integration | Both change simultaneously |
| 18 | file_diff classifies New correctly | Integration | File absent from stored state |
| 19 | file_diff classifies Deleted correctly | Integration | Path in stored but not discovered |
| 20 | partition invariant (mutually exclusive) | Unit (proptest) | Pure set property |
| 21 | collective exhaustive invariant | Unit (proptest) | Pure set property |
| 22 | mixed scenario (all four buckets populated) | Integration | Full end-to-end classification |
| 23 | both-empty produces empty buckets | Integration | Boundary condition |
| 24 | no mutation of inputs or disk | Integration | Pre/post state comparison |
| 25 | duplicate source_path determinism | Integration | Real files + rayon parallelism |
| 26 | config path nonexistent → Changed classification | Integration | Config fallback interaction |
| 27 | symlink-based path traversal | Integration | Symlink creation + canonicalization |
| 28 | empty source_path boundary | Integration | Edge-case filesystem resolution |
| 29 | very long source_path (PATH_MAX) | Integration | Boundary-length path handling |
| 30 | stored_hashes key format mismatch | Integration | Incorrect but non-panicking classification |
| 31 | size_bytes = 0 does not affect classification | Integration | Metadata field isolation |
| — | clippy + cargo-deny + compile-time checks | Static | Type system enforces invariants |
| — | CLI-level diff flag exercises full pipeline | E2E | Black-box CLI invocation |

**Layer counts**: 10 unit / 22 integration / 1 e2e / 1 static = 34 total BDD scenarios

---

## 3. BDD Scenarios

### 3.1 `compute_config_hash` Returns Empty Hash For None

```
Given: category_config_path is None
When: compute_config_hash(None)
Then: returned ContentHash equals content_hash(b"")
```

Test function: `fn compute_config_hash_returns_empty_hash_when_none()`

### 3.2 `compute_config_hash` Returns SHA-256 Of File Bytes

```
Given: a temp file containing b"hello world"
When: compute_config_hash(Some(&file_path))
Then: returned ContentHash equals content_hash(b"hello world")
```

Test function: `fn compute_config_hash_returns_sha256_when_file_readable()`

### 3.3 `compute_config_hash` Returns Empty Hash For Non-existent File

```
Given: a Path to a file that does not exist on disk
When: compute_config_hash(Some(&nonexistent_path))
Then: returned ContentHash equals content_hash(b"")
```

Test function: `fn compute_config_hash_returns_empty_hash_when_file_missing()`

### 3.4 `compute_config_hash` Returns Empty Hash For Unreadable File

```
Given: a file with permissions 0o000 (no read access) on a POSIX system
When: compute_config_hash(Some(&unreadable_path))
Then: returned ContentHash equals content_hash(b"")
```

Test function: `fn compute_config_hash_returns_empty_hash_when_file_unreadable()`

### 3.5 `compute_config_hash` Is Deterministic

```
Given: a temp file containing b"deterministic test content"
When: calling compute_config_hash(Some(&path)) twice
Then: both calls return identical ContentHash values
And: the hash equals content_hash(b"deterministic test content")
```

Test function: `fn compute_config_hash_returns_identical_hash_across_calls()`

### 3.6 `compute_config_hash` Returns Distinct Concrete Hashes For Different Contents

```
Given: two temp files: file_a containing b"aaa", file_b containing b"bbb"
When: calling compute_config_hash(Some(&file_a)) and compute_config_hash(Some(&file_b))
Then: hash_a equals content_hash(b"aaa")  (concrete value for file_a)
And:  hash_b equals content_hash(b"bbb")  (concrete value for file_b)
And:  hash_a does NOT equal hash_b         (relational — redundant but explicit)
```

Test function: `fn compute_config_hash_returns_distinct_concrete_hashes_for_different_contents()`

### 3.7 `compute_config_hash` Returns Empty Hash For Empty File

```
Given: a temp file that exists but contains zero bytes (0-byte file)
When: compute_config_hash(Some(&empty_file_path))
Then: returned ContentHash equals content_hash(b"")
And:  this is the same value as compute_config_hash(None) but exercises
      a different code path (file exists and is read successfully, yields 0 bytes)
```

Test function: `fn compute_config_hash_returns_empty_hash_when_file_is_zero_bytes()`

### 3.8 `compute_config_hash` Handles Large File Without OOM Or Truncation

```
Given: a temp file containing exactly 1_048_576 bytes of b"X" repeated
When: compute_config_hash(Some(&large_file_path))
Then: returned ContentHash equals content_hash(&vec![b'X'; 1_048_576])
And:  computation completes without panic, OOM, or timeout
```

Test function: `fn compute_config_hash_returns_exact_sha256_when_file_is_large()`

---

### 3.9 `compute_file_diff` Returns SourceDirNotFound

```
Given: source_dir is a path that does not exist (e.g., "/nonexistent/dir/xyz")
When: compute_file_diff(&[], &source_dir, None, &HashMap::new())
Then: returns Err(DiffError::SourceDirNotFound(path_string))
And: error message contains the source_dir path
```

Test function: `fn compute_file_diff_returns_source_dir_not_found_when_dir_missing()`

### 3.10 `compute_file_diff` Returns FileRead On Missing File

```
Given: source_dir exists, discovered_files contains one DiscoveryFile, but the file
       at source_dir.join(source_path) does not exist
When: compute_file_diff is called
Then: returns Err(DiffError::FileRead { path, source })
And: path matches the missing file's source_path
And: source is std::io::Error with kind NotFound
```

Test function: `fn compute_file_diff_returns_file_read_when_file_missing()`

### 3.10b `compute_file_diff` Returns FileRead On Permission Denied

```
Given: source_dir exists, discovered_files references a file with mode 0o000
When: compute_file_diff is called
Then: returns Err(DiffError::FileRead { path, source })
And: source is std::io::Error with kind PermissionDenied
```

Test function: `fn compute_file_diff_returns_file_read_when_permission_denied()`

### 3.11 `compute_file_diff` Returns PathTraversal On Dotdot Escape

```
Given: source_dir is a temp directory
And: discovered_files contains a DiscoveryFile with source_path "../../etc/passwd"
When: compute_file_diff is called
Then: returns Err(DiffError::PathTraversal { path })
And: path is "../../etc/passwd"
```

Test function: `fn compute_file_diff_returns_path_traversal_when_path_escapes_source_dir()`

### 3.12 `compute_file_diff` Rejects Absolute Path Outside Source Dir

```
Given: source_dir is a temp directory
And: discovered_files contains DiscoveryFile with source_path "/etc/passwd"
When: compute_file_diff is called
Then: returns Err(DiffError::PathTraversal { path })
And: path is "/etc/passwd"
```

Test function: `fn compute_file_diff_rejects_absolute_path_outside_source_dir()`

### 3.13 `compute_file_diff` Rejects Dotdot Prefix Path

```
Given: source_dir is a temp directory
And: discovered_files contains DiscoveryFile with source_path "../outside.md"
When: compute_file_diff is called
Then: returns Err(DiffError::PathTraversal { path })
And: path is "../outside.md"
```

Test function: `fn compute_file_diff_rejects_dotdot_path_traversal()`

### 3.14 `compute_file_diff` Classifies All New When No Stored Hashes

```
Given: source_dir with two files: "a.md" (content b"alpha") and "b.md" (content b"beta")
And: discovered_files = [DiscoveryFile for "a.md", DiscoveryFile for "b.md"]
And: stored_hashes is empty HashMap
When: compute_file_diff is called
Then: result.new contains exactly {"a.md", "b.md"}
And: result.unchanged is empty
And: result.changed is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_all_new_when_stored_hashes_empty()`

### 3.15 `compute_file_diff` Classifies All Deleted When No Discovered Files

```
Given: discovered_files is an empty slice
And: stored_hashes = {"old.md": StoredHashes { content_hash: X, config_hash: Y }}
And: source_dir exists (valid tempdir)
When: compute_file_diff is called
Then: result.deleted contains exactly {"old.md"}
And: result.unchanged is empty
And: result.changed is empty
And: result.new is empty
```

Test function: `fn compute_file_diff_classifies_all_deleted_when_no_discovered_files()`

### 3.16 `compute_file_diff` Classifies Unchanged Correctly

```
Given: source_dir with file "same.md" containing b"constant content"
And: discovered_files = [DiscoveryFile for "same.md"]
And: stored_hashes = {
    "same.md": StoredHashes {
        content_hash: content_hash(b"constant content"),
        config_hash: content_hash(b""),  // no config file provided
    }
}
And: category_config_path is None
When: compute_file_diff is called
Then: result.unchanged contains exactly {"same.md"}
And: result.changed is empty
And: result.new is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_unchanged_when_hashes_match()`

### 3.17 `compute_file_diff` Classifies Changed When Content Differs

```
Given: source_dir with file "edit.md" containing b"new content"
And: discovered_files = [DiscoveryFile for "edit.md"]
And: stored_hashes = {
    "edit.md": StoredHashes {
        content_hash: content_hash(b"old content"),
        config_hash: content_hash(b""),
    }
}
When: compute_file_diff is called
Then: result.changed contains exactly {"edit.md"}
And: result.unchanged is empty
And: result.new is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_changed_when_content_hash_differs()`

### 3.18 `compute_file_diff` Classifies Changed When Config Differs

```
Given: source_dir with file "stable.md" containing b"same bytes"
And: discovered_files = [DiscoveryFile for "stable.md"]
And: stored_hashes = {
    "stable.md": StoredHashes {
        content_hash: content_hash(b"same bytes"),
        config_hash: content_hash(b"old config"),
    }
}
And: category_config_path points to a file containing b"new config"
When: compute_file_diff is called
Then: result.changed contains exactly {"stable.md"}
And: result.unchanged is empty
And: result.new is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_changed_when_config_hash_differs()`

### 3.19 `compute_file_diff` Classifies Changed When Both Differ

```
Given: source_dir with file "both.md" containing b"new file bytes"
And: discovered_files = [DiscoveryFile for "both.md"]
And: stored_hashes = {
    "both.md": StoredHashes {
        content_hash: content_hash(b"old file bytes"),
        config_hash: content_hash(b"old config bytes"),
    }
}
And: category_config_path points to a file containing b"new config bytes"
When: compute_file_diff is called
Then: result.changed contains exactly {"both.md"}
And: result.unchanged is empty
And: result.new is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_changed_when_both_hashes_differ()`

### 3.20 `compute_file_diff` Classifies New When Absent From Stored

```
Given: source_dir with file "fresh.md" containing b"brand new"
And: discovered_files = [DiscoveryFile for "fresh.md"]
And: stored_hashes does NOT contain "fresh.md"
When: compute_file_diff is called
Then: result.new contains exactly {"fresh.md"}
And: result.unchanged is empty
And: result.changed is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_classifies_new_when_not_in_stored_hashes()`

### 3.21 `compute_file_diff` Classifies Deleted When Not Discovered

```
Given: source_dir exists (valid tempdir, possibly empty)
And: discovered_files = []
And: stored_hashes = {
    "gone.md": StoredHashes { content_hash: X, config_hash: Y },
    "removed.md": StoredHashes { content_hash: Z, config_hash: W },
}
When: compute_file_diff is called
Then: result.deleted contains exactly {"gone.md", "removed.md"}
And: result.unchanged is empty
And: result.changed is empty
And: result.new is empty
```

Test function: `fn compute_file_diff_classifies_deleted_when_not_in_discovered_files()`

### 3.22 `compute_file_diff` Partition Invariant (Mutually Exclusive)

```
Given: any valid inputs (discovered_files, stored_hashes)
When: compute_file_diff succeeds
Then: intersection of unchanged ∩ changed == ∅
And: intersection of unchanged ∩ new == ∅
And: intersection of unchanged ∩ deleted == ∅
And: intersection of changed ∩ new == ∅
And: intersection of changed ∩ deleted == ∅
And: intersection of new ∩ deleted == ∅
```

Test function: `fn compute_file_diff_buckets_are_mutually_exclusive()`

### 3.23 `compute_file_diff` Collective Exhaustive Invariant

```
Given: any valid inputs (discovered_files, stored_hashes)
When: compute_file_diff succeeds
Then: union of unchanged ∪ changed ∪ new == set of all discovered source_paths
And: deleted == set of stored_hashes keys minus discovered source_paths
```

Test function: `fn compute_file_diff_buckets_are_collectively_exhaustive()`

### 3.24 `compute_file_diff` Mixed Scenario (All Four Buckets)

```
Given: source_dir with 4 files:
  - "kept.md" (content b"same") — in stored_hashes with matching hashes
  - "edited.md" (content b"changed") — in stored_hashes with different content hash
  - "added.md" (content b"new") — NOT in stored_hashes
And: stored_hashes also contains "removed.md" — NOT in discovered_files
And: discovered_files = [DiscoveryFile for each of the 3 on-disk files]
When: compute_file_diff is called
Then: result.unchanged == {"kept.md"}
And: result.changed == {"edited.md"}
And: result.new == {"added.md"}
And: result.deleted == {"removed.md"}
```

Test function: `fn compute_file_diff_populates_all_four_buckets_in_mixed_scenario()`

### 3.25 `compute_file_diff` Both Empty Produces Empty Buckets

```
Given: source_dir exists (valid tempdir)
And: discovered_files is empty slice
And: stored_hashes is empty HashMap
When: compute_file_diff is called
Then: result.unchanged is empty
And: result.changed is empty
And: result.new is empty
And: result.deleted is empty
```

Test function: `fn compute_file_diff_returns_empty_diff_when_both_inputs_empty()`

### 3.26 `compute_file_diff` Does Not Mutate Inputs Or Disk

```
Given: source_dir with file "safe.md" containing b"immutable"
And: discovered_files = [DiscoveryFile for "safe.md"]
And: stored_hashes = {"safe.md": StoredHashes { ... }}
And: a deep-clone of stored_hashes BEFORE the call
And: a snapshot of file modification times in source_dir BEFORE the call
When: compute_file_diff is called
Then: stored_hashes deep-equals its pre-call clone (no mutation)
And: no files in source_dir have been modified (mtimes unchanged)
And: no new files created in source_dir
```

Test function: `fn compute_file_diff_does_not_mutate_inputs_or_filesystem()`

### 3.27 `compute_file_diff` Large File Set Produces Correct Partition

```
Given: source_dir with 50 files, each containing unique content
And: discovered_files with 50 DiscoveryFile entries
And: stored_hashes: 20 with matching hashes, 15 with changed hashes, 15 not present
And: 10 additional keys in stored_hashes not in discovered_files
When: compute_file_diff is called
Then: unchanged.len() == 20
And: changed.len() == 15
And: new.len() == 15
And: deleted.len() == 10
And: partition invariant holds
```

Test function: `fn compute_file_diff_handles_large_file_set_correctly()`

### 3.28 `compute_file_diff` Duplicate Source Path Produces Deterministic Single-Bucket Result

```
Given: source_dir with file "dup.md" containing b"content-hash-value"
And: discovered_files = [
       DiscoveryFile { source_path: "dup.md", size_bytes: 100 },
       DiscoveryFile { source_path: "dup.md", size_bytes: 200 },
     ]
And: stored_hashes = {
    "dup.md": StoredHashes {
        content_hash: content_hash(b"content-hash-value"),
        config_hash: content_hash(b""),
    }
}
And: category_config_path is None
When: compute_file_diff is called
Then: result is Ok(FileDiff)
And: "dup.md" appears in exactly one bucket (unchanged)
And: calling compute_file_diff 10 times on the same inputs produces 10 identical results
     (verifies determinism despite rayon non-deterministic scheduling)
```

Test function: `fn compute_file_diff_produces_deterministic_result_when_duplicate_source_paths()`

### 3.29 `compute_file_diff` Classifies Changed When Config Path Points To Missing File

```
Given: source_dir with file "doc.md" containing b"unchanged content"
And: discovered_files = [DiscoveryFile for "doc.md"]
And: stored_hashes = {
    "doc.md": StoredHashes {
        content_hash: content_hash(b"unchanged content"),
        config_hash: content_hash(b"real config bytes"),  // was stored with real config
    }
}
And: category_config_path = Some(&nonexistent_path) where nonexistent_path does not exist on disk
When: compute_file_diff is called
Then: result.changed contains exactly {"doc.md"}
And: result.unchanged is empty
And: result.new is empty
And: result.deleted is empty

Rationale: compute_config_hash(nonexistent) returns content_hash(b""), which differs from
           the stored config_hash(b"real config bytes"), so the file is classified as Changed.
```

Test function: `fn compute_file_diff_classifies_changed_when_config_path_points_to_missing_file()`

### 3.30 `compute_file_diff` Rejects Symlink Traversal

```
Given: source_dir is a temp directory on a POSIX system
And: source_dir contains "link.md" which is a symlink to "/etc/passwd"
And: discovered_files = [DiscoveryFile { source_path: "link.md", size_bytes: 0 }]
When: compute_file_diff is called
Then: returns Err(DiffError::PathTraversal { path })
And: path is "link.md"

Note: This test is #[cfg(target_family = "unix")] only — symlink creation is
      not portable to Windows. Uses std::os::unix::fs::symlink.
```

Test function: `fn compute_file_diff_rejects_symlink_traversal()`

### 3.31 `compute_file_diff` Handles Empty Source Path

```
Given: source_dir is a temp directory (a directory, not a file)
And: discovered_files = [DiscoveryFile { source_path: "", size_bytes: 0 }]
When: compute_file_diff is called
Then: returns Err(DiffError::FileRead { path, source })
  OR  returns Err(DiffError::PathTraversal { path })
And: the path field in the error is ""

Rationale: source_dir.join("") resolves to source_dir itself, which is a directory.
           Reading it as a file fails. The exact error variant depends on whether the
           implementation checks canonicalization before reading. Both are acceptable;
           the test asserts the exact variant the implementation returns.
```

Test function: `fn compute_file_diff_returns_error_when_source_path_is_empty()`

### 3.32 `compute_file_diff` Does Not Panic On Very Long Source Path

```
Given: source_dir is a temp directory
And: discovered_files = [DiscoveryFile {
       source_path: "a".repeat(4096),  // PATH_MAX on Linux
       size_bytes: 0
     }]
When: compute_file_diff is called
Then: returns Err(...) — the exact variant does not matter (FileRead, PathTraversal,
      or OS error), but the call MUST NOT panic.
And: result is Err (not Ok with incorrect classification)
```

Test function: `fn compute_file_diff_does_not_panic_when_source_path_exceeds_path_max()`

### 3.33 `compute_file_diff` Never Panics On Mismatched Stored Hash Keys

```
Given: source_dir with file "real_file.md" containing b"hello"
And: discovered_files = [DiscoveryFile { source_path: "real_file.md", size_bytes: 5 }]
And: stored_hashes = {
    "REAL_FILE.MD": StoredHashes { content_hash: X, config_hash: Y },  // different case
    "./real_file.md": StoredHashes { content_hash: X, config_hash: Y },  // prefixed
    "subdir/../real_file.md": StoredHashes { content_hash: X, config_hash: Y },  // relative
}
When: compute_file_diff is called
Then: result is Ok(FileDiff) — no panic
And: "real_file.md" is classified as New (key format mismatch means stored_hashes
     entries are treated as unknown keys, all go to deleted)
And: result.deleted contains {"REAL_FILE.MD", "./real_file.md", "subdir/../real_file.md"}
And: result.new contains {"real_file.md"}
```

Test function: `fn compute_file_diff_never_panics_on_mismatched_stored_hash_keys()`

### 3.34 `compute_file_diff` Ignores Size Bytes And Classifies By Content Hash

```
Given: source_dir with file "sized.md" containing b"actual content here" (17 bytes)
And: discovered_files = [DiscoveryFile { source_path: "sized.md", size_bytes: 0 }]
And: stored_hashes = {
    "sized.md": StoredHashes {
        content_hash: content_hash(b"actual content here"),
        config_hash: content_hash(b""),
    }
}
And: category_config_path is None
When: compute_file_diff is called
Then: result.unchanged contains exactly {"sized.md"}
And: result.changed is empty
And: result.new is empty
And: result.deleted is empty

Rationale: size_bytes = 0 is incorrect metadata but the function classifies by
           actual content hash, not size_bytes. The file is Unchanged because
           the on-disk bytes match the stored content hash.
```

Test function: `fn compute_file_diff_ignores_size_bytes_and_classifies_by_content_hash()`

---

## 4. Proptest Invariants

### Proptest 1: `compute_config_hash` Determinism

```
Invariant: For any byte string B written to a temp file,
           compute_config_hash(Some(&path)) == content_hash(B)
           AND calling it twice returns identical results.

Strategy: any::<Vec<u8>>() with 0..=1024 bytes
          Write to temp file, hash, compare.

Anti-invariant: N/A — function is infallible, no failure class.
```

### Proptest 2: `compute_config_hash(None)` Is Constant

```
Invariant: compute_config_hash(None) ALWAYS equals content_hash(b"")
           regardless of any other state.

Strategy: Call compute_config_hash(None) 1000 times.
          Every call must return the same ContentHash.

Anti-invariant: N/A — no failure class.
```

### Proptest 3: Partition Invariant (Single-Bucket Membership)

```
Invariant: For ANY valid (discovered_files, stored_hashes) pair where all files
           exist under source_dir:
           No path string appears in more than one HashSet within FileDiff.
           pairwise_intersection(all_buckets) == ∅

Strategy:
  - Generate a set of paths (1..=20 unique strings)
  - Randomly partition into: on_disk subset + stored_only subset
  - Generate random content bytes for each on_disk file
  - Generate StoredHashes with either matching or different hashes
  - Call compute_file_diff
  - Assert pairwise intersection is empty

Anti-invariant: If source_dir does not exist, the function returns Err —
                 proptest should only feed valid filesystem states.
```

### Proptest 4: Collective Exhaustive Invariant

```
Invariant: For ANY valid inputs:
           unchanged ∪ changed ∪ new == {all discovered source_paths}
           deleted == {stored_hashes keys} \ {discovered source_paths}

Strategy: Same as Proptest 3 setup.
          After calling compute_file_diff, verify:
          - union of unchanged + changed + new == discovered_paths set
          - deleted == stored_keys - discovered_paths

Anti-invariant: If a discovered file is unreadable, function returns Err.
                 Test only generates readable files.
```

### Proptest 5: DiffStatus Classification Rules

```
Invariant: For any file F with on_disk content hash C_disk and current config hash C_cfg:
           - F ∈ unchanged iff F ∈ stored_hashes AND stored.content_hash == C_disk AND stored.config_hash == C_cfg
           - F ∈ changed iff F ∈ stored_hashes AND (stored.content_hash != C_disk OR stored.config_hash != C_cfg)
           - F ∈ new iff F ∉ stored_hashes
           - F ∉ deleted (always, since F is discovered)

Strategy: For each generated file, randomly pick one of:
          1. Match both hashes → expect Unchanged
          2. Mismatch content only → expect Changed
          3. Mismatch config only → expect Changed
          4. Omit from stored_hashes → expect New

Anti-invariant: Files cannot be both new and unchanged simultaneously.
```

### Proptest 6: Rayon Determinism With Duplicate Source Paths

```
Invariant: For any valid inputs where discovered_files contains duplicate
           source_path entries, calling compute_file_diff N=5 times on the
           same inputs always produces byte-identical FileDiff results.
           The function must not exhibit nondeterministic classification
           due to rayon scheduling.

Strategy:
  - Generate 2..=5 unique file paths
  - Write random content (0..=256 bytes) to each file
  - Create discovered_files with each path duplicated 2..=3 times
    (different size_bytes values for each duplicate)
  - Generate stored_hashes with random match/mismatch for each unique path
  - Call compute_file_diff 5 times
  - Assert all 5 results are equal (PartialEq on FileDiff)
  - Assert partition invariant holds on each result

Anti-invariant: If file content changes between calls (not in test scope),
                 results may differ. Test ensures identical inputs → identical outputs.
```

---

## 5. Fuzz Targets

### Fuzz Target 1: `compute_file_diff` Source Path Input

```
Input type: &str (arbitrary source_path strings)
Risk: Path traversal bypass, panic on unexpected UTF-8,
      panic on null bytes in path, assertion failure in canonicalize
Corpus seeds:
  - "" (empty string)
  - "../../etc/passwd"
  - "/absolute/path"
  - "normal.md"
  - "deep/nested/path/file.md"
  - "..\..\\windows-style" (mixed separators)
  - "\0null\0bytes"
  - "\u{ffff}" (Unicode edge case)
  - "." (current dir)
  - "a/b/c/../../../../d" (collapsing traversal)
  - "a".repeat(4096) (PATH_MAX boundary)
  - "link.md" (would be symlink seed if available)

Harness pseudo-code:
  let dir = tempfile::tempdir().unwrap();
  // Create a dummy file at the fuzz path IF it's safe
  // Call compute_file_diff with fuzz input as source_path
  // Assert: no panic, result is either Ok or DiffError variant
```

### Fuzz Target 2: `compute_config_hash` Path Input

```
Input type: arbitrary &[u8] interpreted as file content,
            plus arbitrary PathBuf
Risk: panic in SHA-256 on large input, panic in fs::read,
      OOM on huge file
Corpus seeds:
  - Empty bytes
  - 1 byte
  - 1MB of zeros
  - Random binary data
  - Valid UTF-8 markdown
  - Invalid UTF-8 sequences

Harness pseudo-code:
  let dir = tempfile::tempdir().unwrap();
  let file_path = dir.path().join("config.yaml");
  fs::write(&file_path, fuzz_bytes).unwrap();
  let hash = compute_config_hash(Some(&file_path));
  assert_eq!(hash, content_hash(&fuzz_bytes));
  // Assert: no panic, deterministic output
```

---

## 6. Kani Harnesses

### Kani Harness 1: `FileDiff` Bucket Intersection Is Empty

```
Property: For any FileDiff with up to N=3 paths per bucket,
          the intersection of any two distinct buckets is the empty set.

Bound: 3 paths per bucket (12 total paths), all distinct strings
       from a small alphabet {a, b, c, ...}.

Rationale: The partition invariant is the most critical correctness property.
           A proptest can miss rare partition violations if the hash collision
           or dedup logic has a subtle bug. Kani exhaustively checks all
           combinations within the bound.

Verification:
  fn verify_partition(diff: &FileDiff) {
      kani::assert(
          diff.unchanged.intersection(&diff.changed).count() == 0,
          "unchanged ∩ changed == ∅"
      );
      // ... all 6 pairwise intersections
  }
```

### Kani Harness 2: `compute_config_hash` Returns Valid `ContentHash`

```
Property: compute_config_hash always returns a ContentHash whose
          internal [u8; 32] is exactly the SHA-256 digest of the input.
          For None input, it equals SHA-256 of b"".

Bound: Input sizes 0..=64 bytes (small bound for model checker).

Rationale: Cryptographic hash correctness — if the hash function is wrong,
           all downstream cache invalidation breaks silently. Kani proves
           the byte-level implementation matches the spec for bounded inputs.
```

### Kani Harness 3: `DiffStatus` Exhaustive Match

```
Property: Every code path in compute_file_diff assigns each path to exactly
          one DiffStatus variant. No path is unassigned.

Bound: 1..=3 discovered files, 0..=3 stored hashes.

Rationale: State machine completeness — ensures the match/if-let chains
           covering Unchanged/Changed/New are exhaustive. A missed arm
           would silently drop a file from all buckets.
```

---

## 7. Mutation Testing Checkpoints

**Threshold: ≥90% mutation kill rate**

### Critical Mutations to Catch

| Mutation | Caught by Test |
|----------|---------------|
| Remove `None => content_hash(b"")` branch in `compute_config_hash` | Test 3.1: returns empty hash for None |
| Replace `content_hash(b"")` with `content_hash(b"x")` in error fallback | Test 3.3: returns empty hash for missing file |
| Remove `SourceDirNotFound` check at entry of `compute_file_diff` | Test 3.9: returns SourceDirNotFound |
| Remove `PathTraversal` canonicalization check | Test 3.11 + 3.12 + 3.13 + 3.30: all traversal variants |
| Swap `Unchanged` ↔ `Changed` classification logic | Test 3.16 + 3.17: exact bucket membership |
| Remove `New` branch (files not in stored_hashes) | Test 3.14 + 3.20: New classification |
| Remove `Deleted` bucket population | Test 3.15 + 3.21: Deleted classification |
| Change `&&` to `||` in Unchanged condition | Test 3.18: Changed when only config differs |
| Skip rayon parallelism (use sequential iter) | Test 3.27: large file set correctness (behavior unchanged, but test validates output) |
| Remove content hash computation (return dummy hash) | Test 3.16: Unchanged classification requires exact hash match |
| Remove config hash computation in diff | Test 3.18: Changed when config differs |
| Negate partition invariant check (if any internal) | Test 3.22: explicit partition verification |
| Flip `stored_hashes.get()` to always return `None` | Test 3.16: Unchanged needs stored match |
| Flip `stored_hashes.get()` to always return `Some` | Test 3.20: New needs stored miss |
| Remove `FileRead` error propagation | Test 3.10: returns FileRead on missing file |
| Change `Err(_)` fallback in config hash to `Err(panic)` | Test 3.3 + 3.4: infallible graceful fallback |
| **Duplicate source_path: pick first hash instead of last** | **Test 3.28**: deterministic result with duplicates — repeated calls verify consistent bucket assignment + Proptest 6 verifies determinism across 5 repeated calls |
| **Duplicate source_path: nondeterministic result via rayon race** | **Test 3.28**: 10 repeated calls assert identical results; Proptest 6: 5 calls assert identical |
| **Return hash of file PATH instead of file BYTES** | Test 3.2 + 3.6: both assert `content_hash(file_bytes)` not `content_hash(path_str)` |
| **Config path nonexistent → use old config hash instead of empty hash** | Test 3.29: verifies files with stored real config_hash become Changed |
| **Symlink traversal not detected** | Test 3.30: symlink to /etc/passwd returns PathTraversal |
| **size_bytes influences classification** | Test 3.34: size_bytes=0 with real content → Unchanged (not Changed/New) |

### Mutations Expected to Survive (acceptable)

| Mutation | Why Acceptable |
|----------|---------------|
| Logging/eprintln message text changes | Not behavioral, cosmetic |
| Variable renames | No functional impact |
| Reordering of independent `if` branches | Same classification result |
| Skip rayon, use sequential iter | Behavior identical. Output-correctness tests pass. |

---

## 8. Combinatorial Coverage Matrix

### Matrix A: `compute_config_hash`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| None input | `None` | `content_hash(b"")` (exact value) | Unit |
| Valid file | `Some(path)` to readable file with known bytes | `content_hash(known_bytes)` (exact value) | Integration |
| Non-existent file | `Some(path)` to missing file | `content_hash(b"")` (exact value) | Integration |
| Unreadable file | `Some(path)` to 0o000 file | `content_hash(b"")` (exact value) | Integration |
| Determinism | Same path, called N times | All N results byte-identical | Unit |
| Different content | Two files with distinct bytes `b"aaa"` vs `b"bbb"` | `hash_a == content_hash(b"aaa")`, `hash_b == content_hash(b"bbb")`, `hash_a != hash_b` | Integration |
| Empty file (0 bytes) | `Some(path)` to 0-byte file | `content_hash(b"")` — distinct code path from None | Integration |
| Large file (1MB+) | `Some(path)` to 1MB file | `SHA-256(large_bytes)` — no OOM, no truncation | Integration |

### Matrix B: `compute_file_diff` — Happy Path

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| All new (empty stored) | files present, `stored_hashes={}` | `new={all paths}`, others empty | Integration |
| All deleted (empty discovered) | `discovered_files=[]`, stored has entries | `deleted={all stored keys}`, others empty | Integration |
| All unchanged | Matching hashes for all files | `unchanged={all paths}`, others empty | Integration |
| Mixed: all four buckets | Some match, some differ, some new, some deleted | Each path in exactly one expected bucket | Integration |
| Both empty | `[]` + `{}` | All four buckets empty | Integration |

### Matrix C: `compute_file_diff` — Error Variants

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Source dir missing | Non-existent `source_dir` | `Err(DiffError::SourceDirNotFound(path))` | Integration |
| File missing on disk | Discovered file not on disk | `Err(DiffError::FileRead { path, source })` with `source.kind() == NotFound` | Integration |
| File permission denied | Discovered file with mode 0o000 | `Err(DiffError::FileRead { path, source })` with `source.kind() == PermissionDenied` | Integration |
| Path traversal (dotdot) | `source_path = "../../etc/passwd"` | `Err(DiffError::PathTraversal { path })` | Integration |
| Path traversal (absolute) | `source_path = "/etc/passwd"` | `Err(DiffError::PathTraversal { path })` | Integration |
| Path traversal (symlink) | `source_path = "link.md"` → symlink to `/etc/passwd` | `Err(DiffError::PathTraversal { path })` | Integration |
| Empty source_path | `source_path = ""` | `Err(FileRead or PathTraversal)` — no panic | Integration |
| Very long source_path | `source_path.len() >= 4096` | `Err(...)` — no panic | Integration |

### Matrix D: `compute_file_diff` — Classification Logic

| Scenario | Content Match? | Config Match? | In Stored? | Expected Status | Layer |
|----------|---------------|---------------|------------|-----------------|-------|
| Both match | Yes | Yes | Yes | Unchanged | Integration |
| Content differs | No | Yes | Yes | Changed | Integration |
| Config differs | Yes | No | Yes | Changed | Integration |
| Both differ | No | No | Yes | Changed | Integration |
| Not in stored | N/A | N/A | No | New | Integration |
| In stored, not discovered | N/A | N/A | N/A | Deleted | Integration |

### Matrix E: `compute_file_diff` — Boundary & Edge Cases

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Duplicate source_path | Two DiscoveryFile entries, same path | Single-bucket result, deterministic across 10 calls | Integration |
| Config path nonexistent | `Some(missing_path)`, stored has real config_hash | All files in `changed` (config degraded to empty) | Integration |
| Symlink traversal | Symlink inside source_dir pointing outside | `Err(DiffError::PathTraversal)` | Integration |
| Empty source_path | `DiscoveryFile { source_path: "", size_bytes: 0 }` | `Err(FileRead or PathTraversal)` — no panic | Integration |
| Very long source_path | `source_path` at PATH_MAX (4096) | `Err(...)` — no panic | Integration |
| Stored hash key mismatch | Keys differ in case/prefix from source_paths | Ok(FileDiff) — no panic, incorrect but safe classification | Integration |
| size_bytes = 0 | `DiscoveryFile { source_path: "x.md", size_bytes: 0 }`, real file content | Classification by content hash (Unchanged if hash matches) | Integration |

### Matrix F: Invariants (Proptest)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Partition: mutually exclusive | Any valid filesystem state | Pairwise intersection empty | Proptest |
| Partition: collectively exhaustive | Any valid filesystem state | Union == discovered, deleted == stored - discovered | Proptest |
| Classification correctness | Any single file with random hash state | Correct DiffStatus per rules | Proptest |
| Config hash determinism | Any byte content | Same hash for same bytes | Proptest |
| Rayon determinism with duplicates | Duplicate source_paths in discovered_files | Identical FileDiff across N repeated calls | Proptest |

---

## Open Questions

1. **Kani availability**: Kani requires nightly Rust and may not be available in CI. Recommend
   gating Kani harnesses behind `#[cfg(kani)]` and running them in a separate CI stage or
   locally before merges.

2. **Empty source_path error variant**: Scenario 3.31 accepts either `FileRead` or `PathTraversal`
   for `source_path = ""`. The test should assert whichever variant the implementation actually
   returns. If the implementation canonicalizes before reading, it may detect the directory
   resolution as traversal. If it reads first, it will get an I/O error. The test must pin this
   down during implementation — update the Then clause to assert the exact variant.

---

## Appendix: Test File Organization

```
centralized-docs/tests/
├── diff_compute_tests.rs          ← Integration tests for compute_file_diff (scenarios 3.9–3.34)
└── proptests/
    └── diff_proptests.rs          ← Proptest invariants (scenarios 4.1–4.6)

centralized-docs/src/
└── analyze.rs                     ← Unit tests for compute_config_hash (scenarios 3.1–3.8)
                                    (#[cfg(test)] mod tests)

centralized-docs/fuzz/
├── fuzz_targets/
│   ├── diff_source_path.rs        ← Fuzz target 1
│   └── config_hash_content.rs     ← Fuzz target 2
```
