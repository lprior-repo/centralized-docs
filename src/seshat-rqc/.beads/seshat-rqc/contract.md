bead_id: seshat-rqc
bead_title: vcs: Create MetadataBackend trait for dual Git/JJ support
phase: contract
updated_at: 2026-03-09T16:00:00Z

# Contract Specification: MetadataBackend Trait

## Overview
Create MetadataBackend trait in stak-core/vcs that abstracts branch metadata storage for both Git and JJ backends.

## Requirements

### Core Methods
| Method | Description | Git Implementation | JJ Implementation |
|--------|-------------|---------------------|-------------------|
| `read_branch_metadata` | Read JSON metadata for a branch | `refs/branch-metadata/<branch>` | `.jj/branch-metadata/<branch>.json` |
| `write_branch_metadata` | Write JSON metadata for a branch | Same as above | Same as above |
| `delete_branch_metadata` | Delete metadata for a branch | Delete ref | Delete file |
| `list_tracked_branches` | List all branches with metadata | Glob `refs/branch-metadata/*` | List `.jj/branch-metadata/*.json` |
| `get_trunk` | Get configured trunk branch | `refs/stax/trunk` | `.jj/branch-metadata/trunk` |
| `set_trunk` | Set trunk branch | Update `refs/stax/trunk` | Write `.jj/branch-metadata/trunk` |
| `get_prev_branch` | Get previous branch | `refs/stax/prev-branch` | `.jj/branch-metadata/prev-branch` |
| `set_prev_branch` | Set previous branch | Update ref | Write file |

### Storage Locations
- **Git**: Uses Git refs namespace `refs/branch-metadata/` for branch data, `refs/stax/` for config
- **JJ**: Uses `.jj/branch-metadata/` directory with JSON files

### Value Objects
- `BranchMetadata`: Contains parent_branch, timestamps, pr_number, pr_url, additional_info

## Patch-ID Requirements

### Methods
| Method | Description |
|--------|-------------|
| `compute_patch_id` | Compute stable ID for commit's diff (git patch-id --stable) |
| `compute_diff_hash` | Compute hash of diff between two commits (SHA256) |

## Implementation Status
✅ COMPLETE - Both MetadataBackend trait and patch-id computation are fully implemented in:
- `crates/stak-core/src/vcs/metadata.rs` - Full MetadataBackend trait with Git and JJ implementations
- `crates/stak-core/src/vcs/git.rs` - Git patch-id and diff-hash
- `crates/stak-core/src/vcs/jj.rs` - JJ patch-id and diff-hash

## Test Coverage
- 79 metadata tests passing (Git backend fully tested)
- JJ tests marked as ignored (JJ not installed in test environment)
