# Contract: doc-17rw

## bead_id: doc-17rw
## bead_title: CLI: unreadable source files return exit code 0
## phase: p1
## updated_at: 2026-03-01T20:58:00Z

---

## Problem Statement

When a source file is unreadable (permission denied), the CLI reports the error but returns exit code 0 (success).

## Preconditions

- Source directory contains files with chmod 000

## Postconditions

- Exit code 1 or 2 when source files are unreadable

## Acceptance Tests

- Unreadable file: exit code non-zero

## Verification

```
mkdir /tmp/unreadable_test
echo "# Test" > /tmp/unreadable_test/secret.md
chmod 000 /tmp/unreadable_test/secret.md
ctd index /tmp/unreadable_test --output /tmp/out
echo $?
```
Expected: non-zero exit code
