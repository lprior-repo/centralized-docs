# Contract: analyze-contract malformed markdown handling

## Bug Summary
When `analyze` command encounters malformed markdown files, they are silently dropped while the command exits with success (0). This violates the contract that automation should detect data ingestion loss.

## Requirements

### EARS Requirements
- **Ubiquitous**: "THE SYSTEM SHALL make partial analysis failures explicit in machine-checkable outcome signals."
- **Event-driven**: "WHEN any discovered file fails analyze due to encoding or parse errors, THE SYSTEM SHALL return a non-success status or explicit partial-failure contract output."
- **Unwanted**: "IF one or more files are dropped during analyze, THE SYSTEM SHALL NOT report unconditional success indistinguishable from full success, because automation cannot detect ingestion data loss."

### Preconditions
- Auth required: false
- Required inputs: none
- System state: Discovery returned at least one file

### Postconditions
- State changes: Exit status and summary fields expose whether all files were processed successfully

### Invariants
- Processed count + failed count equals discovered count

## Acceptance Tests

### Happy Path
- Given: Valid inputs, When: User executes command, Then: Exit code is 0, Output is correct

### Error Paths
- Given: Invalid/malformed inputs, When: User executes command, Then: Exit code is non-zero OR explicit partial-failure contract output with failed files listed

## Implementation Tasks
1. Read relevant files (analyze.rs, main.rs) to understand existing patterns
2. Write failing tests
3. Implement to make tests pass
4. Run moon run :ci

## Related Files
- `ctd/src/analyze.rs:110` - Related implementation
- `ctd/src/main.rs:1183` - Related implementation

## Constraints
- Use functional patterns: map, and_then, ?
- Return Result<T, Error> from all fallible functions
- Zero unwrap law: NEVER use .unwrap or .expect
- Test first: Tests MUST exist before implementation
