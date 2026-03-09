# Contract: Edge Operations Extraction

## Scope
- **What**: Extract edge connectivity and tolerance checking from projection.rs into edge_ops module
- **Where**: `diagram_tool/src/models/edge_ops.rs` (new file)

## Contract Clauses

### Preconditions
- P1a: Edge ID must not exist before connect
- P1b: Source node must exist before connect
- P1c: Target node must exist before connect
- P2: Edge ID must exist before disconnect

### Postconditions
- Q1: New edge present in returned projection after connect
- Q2: Edge absent in returned projection after disconnect
- Q3: Tolerance check returns Ok for valid edges

### Invariants
- I1: Edge IDs unique within projection
- I2: All edges reference valid nodes
- I3: Edge geometry values finite

### Error Taxonomy
| Error | Condition |
|-------|-----------|
| ReplayError::DuplicateEdge | Edge ID already exists |
| ReplayError::EdgeNotFound | Edge ID doesn't exist |
| ReplayError::PolicyViolation | Edge references missing node |
| ReplayError::InvariantViolation | Invalid edge geometry |

## Traceability

| Contract Clause | Test Coverage |
|-----------------|---------------|
| P1a | test_apply_edge_connect_returns_error_for_duplicate_edge_id |
| P1b | test_apply_edge_connect_returns_error_for_nonexistent_source |
| P1c | test_apply_edge_connect_returns_error_for_nonexistent_target |
| P2 | test_apply_edge_disconnect_returns_error_for_nonexistent_edge |
| Q1 | test_apply_edge_connect_creates_edge_successfully |
| Q2 | test_apply_edge_disconnect_removes_edge_successfully |
| Q3 | test_verify_edge_tolerance_returns_ok_for_valid_edges |
| I1 | test_verify_edge_tolerance_returns_error_for_duplicate_edge_ids |
| I2 | test_verify_edge_tolerance_returns_error_for_nonexistent_source_node |
| I3 | test_verify_edge_tolerance_returns_error_for_invalid_thickness |

## Evaluation Protocol
1. Compile: `cargo build --lib`
2. Test: `cargo test --lib edge_ops`
3. Clippy: `cargo clippy --lib`
4. All tests must pass
