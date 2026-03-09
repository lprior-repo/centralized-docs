# Martin Fowler Test Plan

## Happy Path Tests

### test_apply_edge_connect_creates_edge_successfully
**Given**: A projection with two existing nodes "node-a" and "node-b"
**When**: Calling `apply_edge_connect(state, "edge-1", "node-a", "node-b")`
**Then**:
- Returns `Ok(new_state)`
- `new_state.edges` contains edge with ID "edge-1"
- Edge has source "node-a" and target "node-b"
- Original nodes remain unchanged

### test_apply_edge_disconnect_removes_edge_successfully
**Given**: A projection with an existing edge "edge-1" connecting "node-a" to "node-b"
**When**: Calling `apply_edge_disconnect(state, "edge-1")`
**Then**:
- Returns `Ok(new_state)`
- `new_state.edges` does not contain "edge-1"
- All other edges remain unchanged

### test_apply_edge_op_dispatches_to_connect
**Given**: A projection with two nodes
**When**: Calling `apply_edge_op` with `DomainOp::EdgeConnect`
**Then**:
- Returns `Ok(new_state)` with the new edge

### test_apply_edge_op_dispatches_to_disconnect
**Given**: A projection with an edge
**When**: Calling `apply_edge_op` with `DomainOp::EdgeDisconnect`
**Then**:
- Returns `Ok(new_state)` without the edge

### test_verify_edge_tolerance_returns_ok_for_valid_edges
**Given**: A projection with valid edges (finite geometry, existing nodes)
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Ok(())`

## Error Path Tests

### test_apply_edge_connect_returns_error_for_duplicate_edge_id
**Given**: A projection with edge "edge-1" already existing
**When**: Calling `apply_edge_connect(state, "edge-1", "node-a", "node-b")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("duplicate edge ID: edge-1"))`

### test_apply_edge_connect_returns_error_for_nonexistent_source
**Given**: A projection with node "node-b" but no "nonexistent" node
**When**: Calling `apply_edge_connect(state, "edge-new", "nonexistent", "node-b")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("source node not found: nonexistent"))`

### test_apply_edge_connect_returns_error_for_nonexistent_target
**Given**: A projection with node "node-a" but no "nonexistent" node
**When**: Calling `apply_edge_connect(state, "edge-new", "node-a", "nonexistent")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("target node not found: nonexistent"))`

### test_apply_edge_disconnect_returns_error_for_nonexistent_edge
**Given**: A projection with no edges
**When**: Calling `apply_edge_disconnect(state, "nonexistent-edge")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("edge not found: nonexistent-edge"))`

### test_apply_edge_op_returns_error_for_non_edge_operation
**Given**: A projection
**When**: Calling `apply_edge_op` with `DomainOp::NodeAdd`
**Then**:
- Returns `Err(ReplayError::InvalidEvent("not an edge operation: ..."))`

### test_verify_edge_tolerance_returns_error_for_duplicate_edge_ids
**Given**: A projection with duplicate edge IDs (via manual state construction)
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Err(ReplayError::DuplicateEdge(...))`

### test_verify_edge_tolerance_returns_error_for_nonexistent_source_node
**Given**: A projection with an edge referencing a non-existent source node
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Err(ReplayError::PolicyViolation("edge X references non-existent source node: ..."))`

### test_verify_edge_tolerance_returns_error_for_nonexistent_target_node
**Given**: A projection with an edge referencing a non-existent target node
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Err(ReplayError::PolicyViolation("edge X references non-existent target node: ..."))`

### test_verify_edge_tolerance_returns_error_for_invalid_label_offset
**Given**: A projection with an edge having non-finite label_offset_t (NaN or Inf)
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("edge X has invalid label_offset_t"))`

### test_verify_edge_tolerance_returns_error_for_invalid_thickness
**Given**: A projection with an edge having non-finite thickness (NaN or Inf)
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("edge X has invalid thickness"))`

## Edge Case Tests

### test_apply_edge_connect_with_empty_node_ids
**Given**: A projection with nodes
**When**: Calling `apply_edge_connect(state, "edge", "", "")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("source node not found: "))`

### test_apply_edge_connect_with_self_loop
**Given**: A projection with node "node-a"
**When**: Calling `apply_edge_connect(state, "edge-1", "node-a", "node-a")`
**Then**:
- Returns `Ok(new_state)` (self-loops are allowed by default)

### test_apply_edge_disconnect_on_empty_projection
**Given**: An empty projection with no edges
**When**: Calling `apply_edge_disconnect(state, "any-edge")`
**Then**:
- Returns `Err(ReplayError::InvariantViolation("edge not found: any-edge"))`

### test_verify_edge_tolerance_on_empty_projection
**Given**: An empty projection
**When**: Calling `verify_edge_tolerance(&state)`
**Then**:
- Returns `Ok(())`

### test_apply_edge_connect_preserves_existing_edges
**Given**: A projection with edges ["edge-1", "edge-2"]
**When**: Adding a new edge "edge-3"
**Then**:
- New projection contains all three edges

## Contract Verification Tests

### test_precondition_edge_not_exists_before_connect
**Given**: A projection
**When**: Attempting to connect with existing edge ID
**Then**: Returns error (precondition enforced)

### test_postcondition_edge_exists_after_connect
**Given**: A valid connect operation
**Then**: New state contains the edge (postcondition satisfied)

### test_invariant_all_edges_reference_valid_nodes
**Given**: A projection passing tolerance check
**Then**: All edges reference existing nodes (invariant holds)

## Contract Violation Tests

### test_duplicate_edge_id_violation_returns_invariant_error
Given: `apply_edge_connect(state, "edge-1", "node-a", "node-b")` where edge-1 exists
When: Function is called with duplicate edge ID
Then: Returns `Err(ReplayError::InvariantViolation("duplicate edge ID: edge-1"))`

### test_nonexistent_source_violation_returns_invariant_error
Given: `apply_edge_connect(state, "edge-new", "nonexistent", "node-b")`
When: Function is called with non-existent source
Then: Returns `Err(ReplayError::InvariantViolation("source node not found: nonexistent"))`

### test_nonexistent_target_violation_returns_invariant_error
Given: `apply_edge_connect(state, "edge-new", "node-a", "nonexistent")`
When: Function is called with non-existent target
Then: Returns `Err(ReplayError::InvariantViolation("target node not found: nonexistent"))`

### test_nonexistent_edge_disconnect_violation_returns_invariant_error
Given: `apply_edge_disconnect(state, "nonexistent-edge")`
When: Function is called with non-existent edge
Then: Returns `Err(ReplayError::InvariantViolation("edge not found: nonexistent-edge"))`

### test_invalid_edge_geometry_violation_returns_invariant_error
Given: `verify_edge_tolerance(state)` where edge has thickness = f64::NAN
When: Function is called with invalid geometry
Then: Returns `Err(ReplayError::InvariantViolation("edge X has invalid thickness"))`
