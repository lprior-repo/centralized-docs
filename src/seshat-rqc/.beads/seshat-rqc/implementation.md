bead_id: seshat-rqc
bead_title: vcs: Create MetadataBackend trait for dual Git/JJ support
phase: implementation
updated_at: 2026-03-09T16:05:00Z

# Implementation Summary

## Changes Made

### Fix: Add Precondition Validation to Patch-ID Methods

**File: `crates/stak-core/src/vcs/git.rs`**

1. `compute_patch_id` (lines 695-760):
   - Added P8 validation: `commit_exists(commit)?` before computing patch-id
   - Returns `VcsError::NotFound` if commit doesn't exist

2. `compute_diff_hash` (lines 762-800):
   - Added P9 validation: `commit_exists(from)?` and `commit_exists(to)?`
   - Added P10 validation: `is_ancestor(from, to)?`
   - Returns appropriate errors for non-existent commits or non-ancestor relationship

**File: `crates/stak-core/src/vcs/jj.rs`**

1. `compute_patch_id` (lines 641-675):
   - Added P8 validation: `commit_exists(commit)?` before computing patch-id

2. `compute_diff_hash` (lines 677-705):
   - Added P9 validation: `commit_exists(from)?` and `commit_exists(to)?`
   - Added P10 validation: `is_ancestor(from, to)?`

## Verification
- ✅ Code compiles successfully
- ✅ Moon quick check passes
- ✅ All 79 metadata tests pass

## Remaining Notes
- The I5 invariant issue (cross-backend patch-id incompatibility) is acknowledged but not fixable without breaking changes - the algorithms are fundamentally different between Git and JJ
- The metadata.rs implementation is solid and unchanged
