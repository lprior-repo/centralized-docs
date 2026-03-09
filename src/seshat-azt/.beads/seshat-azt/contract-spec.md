# Contract Specification

## Context
- **Feature**: Extract Edge Operations & Validations from projection.rs
- **Domain terms**:
  - `EdgeConnect` - Creates a directed edge from source node to target node
  - `EdgeDisconnect` - Removes an edge by its ID
  - `EdgeId` - Unique identifier for edges
  - `NodeId` - Unique identifier for nodes
  - `DiagramProjection` - The complete diagram state after replaying events
  - `verify_edge_tolerance` - Validates all edges satisfy defined tolerance boundaries
- **Assumptions**:
  - The edge_ops module will live in the same crate as projection.rs
  - Edge operations are pure functions that return a new DiagramProjection
  - All edge operations are deterministic and side-effect free
- **Open questions**:
  - What cycle detection logic should be included in edge_ops?
  - Should we extract tolerance constants (e.g., max thickness) into edge_ops?

## Preconditions

- **P1**: For `apply_edge_connect`:
  - Edge ID must not already exist in the projection
  - Source node must exist in the projection
  - Target node must exist in the projection

- **P2**: For `apply_edge_disconnect`:
  - Edge ID must exist in the projection

- **P3**: For `verify_edge_tolerance`:
  - Input projection must be non-null (checked by Rust borrow rules)

## Postconditions

- **Q1**: For `apply_edge_connect`:
  - The returned projection contains the new edge with the given ID
  - All existing edges remain unchanged
  - All existing nodes remain unchanged

- **Q2**: For `apply_edge_disconnect`:
  - The returned projection does not contain the disconnected edge
  - All other edges remain unchanged
  - All nodes remain unchanged

- **Q3**: For `verify_edge_tolerance`:
  - Returns `Ok(())` if all edges satisfy tolerance constraints
  - Returns `Err` with specific error variant if any edge violates constraints

## Invariants

- **I1**: Edge IDs are unique within a projection
- **I2**: All edges reference valid source and target nodes
- **I3**: Edge geometry values (label_offset_t, thickness) are finite floating-point numbers

## Error Taxonomy

- `ReplayError::DuplicateEdge` - When edge ID already exists for connect operations
- `ReplayError::EdgeNotFound` - When edge ID does not exist for disconnect operations
- `ReplayError::PolicyViolation` - When an edge references a non-existent source or target node
- `ReplayError::InvariantViolation` - When edge geometry is invalid (non-finite values)

## Contract Signatures

```rust
/// Apply EdgeConnect operation
pub fn apply_edge_connect(
    state: DiagramProjection,
    id: &str,
    source: &str,
    target: &str,
) -> Result<DiagramProjection, ReplayError>

/// Apply EdgeDisconnect operation
pub fn apply_edge_disconnect(
    state: DiagramProjection,
    id: &str,
) -> Result<DiagramProjection, ReplayError>

/// Apply an edge operation to the projection
pub fn apply_edge_op(
    state: DiagramProjection,
    op: &DomainOp,
) -> Result<DiagramProjection, ReplayError>

/// Verify edge tolerance constraints in the projection
pub fn verify_edge_tolerance(state: &DiagramProjection) -> Result<(), ReplayError>
```

## Type Encoding

| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| Edge ID uniqueness | Runtime-checked constructor | `Result<EdgeId, ReplayError>` |
| Source node exists | Runtime-checked | `state.has_node(&source_id)` |
| Target node exists | Runtime-checked | `state.has_node(&target_id)` |
| Edge exists for disconnect | Runtime-checked | `state.has_edge(&edge_id)` |
| Edge geometry finite | Runtime-checked | `f64::is_finite()` |

## Violation Examples

- **VIOLATES P1a**: `apply_edge_connect(state, "edge-1", "node-a", "node-b")` where edge-1 already exists -- should produce `Err(ReplayError::InvariantViolation("duplicate edge ID: edge-1"))`
- **VIOLATES P1b**: `apply_edge_connect(state, "edge-new", "nonexistent", "node-b")` -- should produce `Err(ReplayError::InvariantViolation("source node not found: nonexistent"))`
- **VIOLATES P1c**: `apply_edge_connect(state, "edge-new", "node-a", "nonexistent")` -- should produce `Err(ReplayError::InvariantViolation("target node not found: nonexistent"))`
- **VIOLATES P2**: `apply_edge_disconnect(state, "nonexistent-edge")` -- should produce `Err(ReplayError::InvariantViolation("edge not found: nonexistent-edge"))`
- **VIOLATES Q3**: `verify_edge_tolerance(state)` where an edge has `thickness = f64::NAN` -- should produce `Err(ReplayError::InvariantViolation("edge X has invalid thickness"))`

## Ownership Contracts

- **Ownership transfer**: `state: DiagramProjection` - Caller transfers ownership, function creates new projection (functional style)
- **Shared borrow**: `state: &DiagramProjection` - For `verify_edge_tolerance`, read-only access
- **Mutations**: No `&mut` parameters - all operations are pure functional transformations

## Non-goals

- [ ] Cycle detection in graphs (handled by separate `dag` module)
- [ ] Edge style/color validation (handled by document types)
- [ ] Persisting edge state to database
