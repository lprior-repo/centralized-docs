# Contract: doc_transformer CLI constraint validation

## bead_id: doc-2apk
## bead_title: doc_transformer: Fix CLI constraint validation for --max-related-chunks
## phase: p0
## updated_at: 2026-03-01T13:56:35Z

## Contract

### Preconditions
- CLI is invoked with --max-related-chunks argument

### Postconditions
- When --max-related-chunks value is outside 1-100 range, CLI exits with code 1
- Error message indicates invalid value for --max-related-chunks
- Valid values (1-100) are accepted

### Invariants
- Constraint validation occurs at CLI parsing time, not runtime

### Acceptance Tests
1. `doc_transformer index dir --output /tmp/test --max-related-chunks 101` exits with code 1
2. `doc_transformer index dir --output /tmp/test --max-related-chunks 0` exits with code 1
3. `doc_transformer index dir --output /tmp/test --max-related-chunks 50` exits with code 0
4. Help text shows valid range as 1-100

## Related Files
- doc_transformer/src/main.rs (CLI argument parsing)
