# Implementation: Fix CLI constraint validation for --max-related-chunks

## bead_id: doc-2apk
## bead_title: ctd: Fix CLI constraint validation for --max-related-chunks

## Files Read

1. `.beads/doc-2apk/contract.md` - Contract specification
2. `ctd/src/main.rs` - CLI argument parsing implementation

## Contract Requirements

- **Preconditions**: CLI is invoked with --max-related-chunks argument
- **Postconditions**:
  - When --max-related-chunks value is outside 1-100 range, CLI exits with code 2
  - Error message indicates invalid value for --max-related-chunks
  - Valid values (1-100) are accepted
- **Invariants**: Constraint validation occurs at CLI parsing time, not runtime
- **Acceptance Tests**:
  1. `ctd index dir --output /tmp/test --max-related-chunks 101` exits with code 2
  2. `ctd index dir --output /tmp/test --max-related-chunks 0` exits with code 2
  3. `ctd index dir --output /tmp/test --max-related-chunks 50` exits with code 0
  4. Help text shows valid range as 1-100

## Changes Made

### 1. Fix validation range (lines 203-205)

Changed the upper bound from 1000 to 100 in `validate_max_related_chunks`:

```rust
// Before:
if value > 1000 {
    return Err(format!(
        "max_related_chunks must be at most 1000, got '{s}'"
    ));
}

// After:
if value > 100 {
    return Err(format!(
        "max_related_chunks must be at most 100, got '{s}'"
    ));
}
```

### 2. Fix exit code for validation errors (lines 735-743, 753-761)

Changed error handling to exit with code 2 for validation errors ( clap `ValueValidation` and `InvalidValue` error kinds):

```rust
// Before:
eprintln!("{}", e);
process::exit(1);

// After:
// Validation errors (ValueValidation, InvalidValue) exit with code 2 per contract
// Other errors exit with code 1
let exit_code = if e.kind() == clap::error::ErrorKind::ValueValidation
    || e.kind() == clap::error::ErrorKind::InvalidValue
{
    2
} else {
    1
};
eprintln!("{}", e);
process::exit(exit_code);
```

## Verification Results

All acceptance tests pass:

```bash
# Test 1: Invalid value 101 - exits with code 2
$ ctd index /tmp/testdir --output /tmp/test --max-related-chunks 101
error: invalid value '101' for '--max-related-chunks <N>': max_related_chunks must be at most 100, got '101'
Exit code: 2  # ✓ PASS

# Test 2: Invalid value 0 - exits with code 2
$ ctd index /tmp/testdir --output /tmp/test --max-related-chunks 0
error: invalid value '0' for '--max-related-chunks <N>': max_related_chunks must be at least 1, got '0'
Exit code: 2  # ✓ PASS

# Test 3: Valid value 50 - exits with code 0
$ ctd index /tmp/testdir --output /tmp/test --max-related-chunks 50
# ... successful output ...
Exit code: 0  # ✓ PASS

# Test 4: Help text shows valid range
$ ctd index --help | grep "max-related-chunks"
--max-related-chunks <N>
        Maximum number of related chunks per document (1-100, default: 20) [default: 20]  # ✓ PASS
```

## Notes

- The validation function is used by clap's `value_parser` attribute, which runs at CLI parsing time (not runtime), satisfying the contract's "invariant" requirement
- The help text already showed the correct range (1-100), so no help text change was needed
- Both `ValueValidation` and `InvalidValue` error kinds are handled to ensure all validation errors exit with code 2
