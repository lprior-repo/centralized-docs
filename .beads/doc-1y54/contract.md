# Contract: index-locking: implement stale lock recovery metadata

## Overview
Implement stale lock recovery metadata for index locking to automatically detect and recover from stale locks when prior index processes terminate unexpectedly.

## Requirements

### Ubiquitous Requirements
- **THE SYSTEM SHALL** include sufficient lock metadata to distinguish active vs stale lock ownership.

### Event-Driven Requirements
- **WHEN** index startup finds existing lock file, **THE SYSTEM SHALL** validate lock liveness and automatically recover stale locks.

### Unwanted Behavior Prevention
- **IF** prior index process terminated unexpectedly, **THE SYSTEM SHALL NOT** require manual lock deletion as the only recovery path, because: manual intervention blocks unattended pipelines and can cause deadlocks.

## Technical Specification

### Preconditions
- **Auth Required**: false
- **Required Inputs**: None
- **System State**: Output directory contains existing lock file from previous or concurrent run

### Postconditions
- **State Changes**:
  - Active locks prevent concurrent writers
  - Stale locks are safely reclaimed or produce machine-actionable remediation with metadata evidence

### Invariants
- At most one active index writer per output directory at any time

## Implementation Approach

### Lock Metadata Structure
The lock file should contain:
1. **PID** - Process ID that acquired the lock
2. **Timestamp** - When the lock was acquired
3. **Hostname** - Machine identifier to detect cross-machine lock files
4. **UUID** - Unique identifier for this lock instance

### Liveness Detection Strategy
1. Check if the PID is still running on the same hostname
2. Check if lock timestamp exceeds a configurable stale threshold
3. If PID is dead or threshold exceeded, lock is considered stale

### Recovery Strategy
- Validate lock liveness on startup
- If stale, safely reclaim by removing old lock file
- Log recovery action with metadata evidence for debugging

## Acceptance Tests

### Happy Paths
1. **Active lock detection**: Validates that an active lock (PID still running) prevents concurrent writers
2. **Stale lock recovery**: Validates that stale locks are automatically reclaimed on startup

### Error Paths
1. **Active lock rejection**: Validates that trying to acquire a lock when one is active returns clear error
2. **Corrupted lock file**: Validates that corrupted lock files produce clear error messages

## Verification Checkpoints

1. **Research Gate**: Understand existing patterns in doc_transformer/src/main.rs
2. **Test Gate**: Write failing tests before implementation
3. **Implementation Gate**: Make tests pass
4. **Integration Gate**: Full pipeline test passes
