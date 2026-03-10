# Contract: doc-2rhu

## bead_id: doc-2rhu
## bead_title: Fix index to not write artifacts on validation failure
## phase: p0
## updated_at: 2026-03-01T16:30:00Z

---

## Problem Statement

Fix index command to either fail atomically (no artifacts written) or clearly indicate partial success. Currently writes INDEX.json, COMPASS.md, llms.txt then exits with failure.

**Test:** index with broken links should either fail atomically or clearly warn about partial success.

---

## Preconditions

- Source directory contains files that will fail validation
- User runs `doc_transformer index <dir> --output <output>`

---

## Postconditions

### State Changes
- Option 1: FAIL atomically - no artifacts written if validation will fail
- Option 2: Clearly indicate partial success with warning

### Return Guarantees
- If validation fails, either:
  - No artifacts are written (atomic failure), OR
  - Clear warning that artifacts were written despite validation issues

---

## Acceptance Tests

### Happy Path
- Valid source files: artifacts written, exit code 0

### Error Path
- Source with validation issues: Either atomic failure (no artifacts) OR clear warning about partial success
- Artifacts should NOT be written silently when validation will fail

---

## Implementation Strategy

Move validation step BEFORE artifact writing (steps 6 and 7). This ensures:
1. If validation fails, no artifacts are written
2. User gets clear error before any partial state is created

---

## Verification

Test: Create source with validation issues, run index, check artifacts
Expected: Either no artifacts OR clear warning about partial success
