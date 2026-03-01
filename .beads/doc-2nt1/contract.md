# Contract: doc-2nt1

## bead_id: doc-2nt1
## bead_title: cli: Limit too large validation error returns exit code 0
## phase: p1
## updated_at: 2026-03-01T20:50:00Z

---

## Problem Statement

When limit value exceeds maximum (1000), the CLI returns exit code 0 (success) instead of exit code 1 (user error).

## Preconditions

- User runs `doc_transformer search` with --limit value > 1000

## Postconditions

- Exit code 1 when limit validation fails
- Clear error message about limit constraints

---

## Acceptance Tests

### Error Path
- `--limit 1000000000`: exit code 1, error message about limit

---

## Verification

Test: `doc_transformer search test --index-dir /tmp/index -n 1000000000; echo $?`
Expected: exit code 1
