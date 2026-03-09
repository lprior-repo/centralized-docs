bead_id: seshat-rqc
bead_title: vcs: Create MetadataBackend trait for dual Git/JJ support
phase: test-plan
updated_at: 2026-03-09T16:00:00Z

# Martin Fowler Test Plan: MetadataBackend

## Test Structure (Given-When-Then)

### Git Backend Tests

#### Scenario: Write and Read Branch Metadata
- **Given**: A Git repository with initialized metadata backend
- **When**: Writing JSON metadata for a branch, then reading it back
- **Then**: Returns the exact same JSON string

#### Scenario: Delete Branch Metadata
- **Given**: A branch with metadata stored
- **When**: Deleting the branch metadata
- **Then**: Reading returns None

#### Scenario: List Tracked Branches
- **Given**: Multiple branches with metadata
- **When**: Listing all tracked branches
- **Then**: Returns sorted list of branch names

#### Scenario: Set and Get Trunk
- **Given**: An initialized Git repository
- **When**: Setting trunk to "main", then getting trunk
- **Then**: Returns Some(BranchName("main"))

#### Scenario: Set and Get Previous Branch
- **Given**: An initialized Git repository  
- **When**: Setting prev-branch to "feature/a", then getting it
- **Then**: Returns Some(BranchName("feature/a"))

#### Scenario: Is Initialized Check
- **Given**: A fresh Git repository (no trunk set)
- **When**: Checking if initialized
- **Then**: Returns false
- **When**: Setting trunk to "main"
- **Then**: Returns true

#### Scenario: Read Non-existent Branch
- **Given**: A Git repository with no metadata
- **When**: Reading metadata for non-existent branch
- **Then**: Returns None (not error)

### Error Scenarios

#### Scenario: Open Non-existent Path
- **Given**: A path that doesn't exist
- **When**: Opening GitMetadataBackend
- **Then**: Returns MetadataError::PathNotFound

#### Scenario: Open File Instead of Directory
- **Given**: A path to a file
- **When**: Opening GitMetadataBackend
- **Then**: Returns MetadataError::PathNotDirectory

### Patch-ID Tests

#### Scenario: Compute Patch-ID for Commit
- **Given**: A commit in the repository
- **When**: Computing patch-id
- **Then**: Returns stable string identifier

#### Scenario: Compute Diff Hash Between Commits
- **Given**: Two commits where first is ancestor of second
- **When**: Computing diff hash
- **Then**: Returns SHA256 hash of the diff

## Implementation Evidence
- 79 tests passing in stak-core for metadata functionality
- Tests cover: write/read, delete, list, trunk, prev-branch, initialization
- All tests use real Git operations (not mocks)
