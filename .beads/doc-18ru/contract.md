# Contract: doc-18ru

bead_id: doc-18ru
bead_title: graph: bidirectional edges create cycles in dag
phase: p0
updated_at: 2026-03-01T13:52:00Z

## EARS Requirements

### Ubiquitous
- THE SYSTEM SHALL maintain DAG structure without cycles

### Event-Driven
- WHEN related chunks are added via HNSW similarity, THE SYSTEM SHALL create directed edges that do not form cycles

### Unwanted
- IF chunk A relates to B AND chunk B relates to A, THE SYSTEM SHALL NOT create bidirectional edges that form cycles, because: Cycles break topological sorting

## Contracts

### Preconditions
- auth_required: false
- required_inputs: []
- system_state:
  - At least 2 chunks exist
  - HNSW similarity finds related chunks

### Postconditions
- state_changes:
  - Graph maintains DAG property
  - topological_order succeeds without fallback

### Invariants
- Edge count matches expected directed acyclic structure

## Acceptance Tests

### Happy Paths
- test_happy_path: Given Valid inputs, when User executes command, then Exit code is 0, Output is correct

### Error Paths
- test_error_path: Given Invalid inputs, when User executes command, then Exit code is non-zero, Error message is clear

## Verification Checkpoints

### Gate 0: Research
- must_pass_before: Writing code
- checks: All research questions answered
- evidence_required: Research notes documented

### Gate 1: Tests
- must_pass_before: Implementation
- checks: All tests written and failing
- evidence_required: Test files exist

### Gate 2: Implementation
- must_pass_before: Completion
- checks: All tests pass
- evidence_required: CI green

### Gate 3: Integration
- must_pass_before: Closing bead
- checks: E2E tests pass
- evidence_required: Manual verification complete

## Implementation Tasks

### Phase 0: Research
- Read relevant files and understand existing patterns

### Phase 1: Tests First
- Write failing tests (gate_0_research required)

### Phase 2: Implementation
- Implement to make tests pass (gate_1_tests required)

### Phase 4: Verification
- Run moon run :ci (gate_2_implementation required)

## Anti-Hallucination

### Read Before Write
- Must read: src/main.rs (Main entry point)

### Do
- Use functional patterns: map, and_then, ?
- Return Result<T, Error> from all fallible functions
- READ files before modifying them

### Do Not
- Do NOT use unwrap or expect
- Do NOT use panic!, todo!, or unimplemented!
- Do NOT modify clippy configuration

### Constitution
- Zero unwrap law: NEVER use .unwrap or .expect
- Test first: Tests MUST exist before implementation
