# Verification: doc-3nx3

## bead_id: doc-3nx3
## bead_title: CLI: unreadable source files return exit code 0
## phase: p3
## updated_at: 2026-03-01T14:18:00Z

---

## Scope

Manual CLI verification of contract requirements against current implementation.

Binary tested:

```bash
/home/lewis/src/centralized-docs/target/debug/doc_transformer
```

Test workspace:

```bash
/tmp/doc-3nx3-qa-manual
```

---

## Executed Tests (with evidence)

### Test 1 - All source files unreadable (chmod 000)

Command:

```bash
/home/lewis/src/centralized-docs/target/debug/doc_transformer index /tmp/doc-3nx3-qa-manual/all_unreadable --output /tmp/doc-3nx3-qa-manual/out/all
```

Exit code:

```text
1
```

Stdout (actual):

```text
[STEP 1] DISCOVER
  Found 1 files

[STEP 2] ANALYZE
```

Stderr (actual):

```text
Error: Failed to analyze any of the 1 discovered file(s). Check file permissions, encoding (files must be valid UTF-8), and that files are not corrupted. Errors: hidden.md: Permission denied (os error 13)
```

Expected vs actual:
- Expected: non-zero exit and clear unreadable-file message
- Actual: exit 1 and explicit permission-denied message
- Status: PASS

---

### Test 2 - Mixed readable + unreadable file (partial I/O error)

Command:

```bash
/home/lewis/src/centralized-docs/target/debug/doc_transformer index /tmp/doc-3nx3-qa-manual/mixed --output /tmp/doc-3nx3-qa-manual/out/mixed
```

Exit code:

```text
1
```

Stdout (actual):

```text
[STEP 1] DISCOVER
  Found 2 files

[STEP 2] ANALYZE
```

Stderr (actual):

```text
Error: analysis failed: hidden.md: Permission denied (os error 13)
```

Expected vs actual:
- Expected: exit 0 with warning output for partial I/O errors
- Actual: hard failure with exit 1
- Status: FAIL

---

### Test 3 - Mixed readable file + unreadable directory (partial I/O error)

Command:

```bash
/home/lewis/src/centralized-docs/target/debug/doc_transformer index /tmp/doc-3nx3-qa-manual/mixed_dir --output /tmp/doc-3nx3-qa-manual/out/mixed_dir
```

Exit code:

```text
1
```

Stdout (actual):

```text
[STEP 1] DISCOVER
```

Stderr (actual):

```text
Error: Cannot read file '/tmp/doc-3nx3-qa-manual/mixed_dir/restricted': permission denied
Error: Error: Cannot read 1 file(s) due to permission denied: /tmp/doc-3nx3-qa-manual/mixed_dir/restricted. Please check file permissions with 'chmod +r' or remove unreadable files.
```

Expected vs actual:
- Expected: exit 0 with warning output for partial I/O errors when readable files exist
- Actual: hard failure with exit 1
- Status: FAIL

---

### Test 4 - Happy path readable source

Command:

```bash
/home/lewis/src/centralized-docs/target/debug/doc_transformer index /tmp/doc-3nx3-qa-manual/happy --output /tmp/doc-3nx3-qa-manual/out/happy
```

Exit code:

```text
0
```

Stdout (actual, key lines):

```text
[STEP 2] ANALYZE
  Processed 1 files
...
COMPLETE
Validation: 1/1 passed
```

Stderr (actual):

```text
(empty)
```

Expected vs actual:
- Expected: success exit 0 for readable files
- Actual: exit 0, full workflow completed
- Status: PASS

---

## Contract Requirement Verdict

1. Exit code 1 when unreadable source files exist
   - Result: PASS (verified in Test 1)

2. Clear error message about unreadable files
   - Result: PASS (permission denied context is explicit)

3. Warning output for partial I/O errors
   - Result: FAIL (Tests 2 and 3 hard-fail with exit 1 instead of warning-mode continuation)

---

## Findings

- MAJOR: Partial unreadable input currently aborts indexing (`exit 1`) instead of warning and continuing with readable files.
- Reproduction:
  1) Create one readable markdown file and one chmod `000` markdown file in same source directory.
  2) Run `doc_transformer index <dir> --output <out>`.
  3) Observe `exit 1` and `analysis failed: ... Permission denied`.

---

## QA Gate

QA Gate: FAILED

Reason: Contract requirement #3 is not met by runtime behavior.
