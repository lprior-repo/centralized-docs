//! Git backend implementation using git2 for read operations
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! This module provides:
//! - `GitBackend` - VCS backend implementation using libgit2 (git2 crate)
//! - `GitBackendConfig` - Configuration for `GitBackend` creation
//!
//! # Design
//! - Uses git2 for read operations (status, branches, commits)
//! - Uses Git CLI (2.38+) for rebase operations (--update-refs support)
//! - Caches the `git2::Repository` handle for performance
//! - Thread-safe for read operations via Mutex

use std::{path::Path, process::Command, sync::Mutex};

use crate::vcs::{
    BackendType, BranchName, CommitId, RepoStatus, RepositoryPath, VcsBackend, VcsError,
};

/// Minimum required Git CLI version for rebase operations
const MIN_GIT_VERSION: (u32, u32) = (2, 38);

// ============================================================================
// GitBackend
// ============================================================================

/// Git backend implementation using git2 for read operations
///
/// # Invariants
/// - Repository is always a non-bare Git repository
/// - Repository path is absolute and canonical
/// - `git2::Repository` is opened once and cached
/// - Thread-safe for read operations via Mutex
pub struct GitBackend {
    /// Absolute path to the repository root
    path: RepositoryPath,
    /// Cached git2 repository handle (wrapped in Mutex for thread safety)
    repo: Mutex<git2::Repository>,
}

/// Configuration for `GitBackend` creation
#[derive(Debug, Clone)]
pub struct GitBackendConfig {
    /// Verify Git CLI version on open (default: true)
    pub verify_cli_version: bool,
}

impl Default for GitBackendConfig {
    fn default() -> Self {
        Self {
            verify_cli_version: true,
        }
    }
}

impl GitBackend {
    /// Open a Git repository at the given path
    ///
    /// # Preconditions
    /// - P1: Path exists on filesystem
    /// - P2: Path is a directory
    /// - P3: Path is inside a Git repository
    /// - P4: Repository is NOT bare
    /// - P5: git2 can open the repository
    ///
    /// # Postconditions
    /// - Q1: Returns `Ok(GitBackend)` with valid repo handle
    /// - Q12: `backend_type()` returns `BackendType::Git`
    /// - I1: Repository is non-bare
    /// - I6: Path is absolute and canonical
    ///
    /// # Errors
    /// - `VcsError::PathNotFound` if path doesn't exist
    /// - `VcsError::PathNotDirectory` if path is a file
    /// - `VcsError::NoVcsFound` if not a git repository
    /// - `VcsError::BareRepositoryNotSupported` if bare repo
    /// - `VcsError::GitOpenFailed` if git2 fails to open
    pub fn open(path: impl AsRef<Path>) -> Result<Self, VcsError> {
        Self::open_with_config(path, &GitBackendConfig::default())
    }

    /// Open with explicit configuration
    ///
    /// # Errors
    /// Same as [`open`](Self::open), plus:
    /// - `VcsError::GitCliVersionTooOld` if `verify_cli_version` is true and Git < 2.38
    pub fn open_with_config(
        path: impl AsRef<Path>,
        config: &GitBackendConfig,
    ) -> Result<Self, VcsError> {
        let path = path.as_ref();

        // Validate path exists and is a directory
        let repo_path = RepositoryPath::new(path)?;

        // Open git repository
        let repo = git2::Repository::discover(repo_path.as_path()).map_err(|e| {
            let message = e.message().to_string();
            VcsError::GitOpenFailed {
                path: repo_path.as_path().to_path_buf(),
                message,
                source: Some(e),
            }
        })?;

        // Check if bare repository
        if repo.is_bare() {
            return Err(VcsError::BareRepositoryNotSupported(
                repo_path.as_path().to_path_buf(),
            ));
        }

        // Get the working directory (repository root)
        let workdir = repo.workdir().ok_or_else(|| {
            VcsError::BareRepositoryNotSupported(repo_path.as_path().to_path_buf())
        })?;

        // Create canonical path from workdir
        let canonical_path = RepositoryPath::new(workdir)?;

        let backend = Self {
            path: canonical_path,
            repo: Mutex::new(repo),
        };

        // Verify CLI version if requested
        if config.verify_cli_version {
            backend.verify_cli_version()?;
        }

        Ok(backend)
    }

    /// Verify Git CLI version is 2.38+
    ///
    /// # Errors
    /// - `VcsError::CommandFailed` if git not found
    /// - `VcsError::GitCliVersionTooOld` if version < 2.38
    /// - `VcsError::GitParseError` if version parse fails
    pub fn verify_cli_version(&self) -> Result<String, VcsError> {
        let output =
            Command::new("git")
                .arg("--version")
                .output()
                .map_err(|e| VcsError::CommandFailed {
                    message: "Failed to execute git --version".to_string(),
                    source: Some(e),
                })?;

        if !output.status.success() {
            return Err(VcsError::CommandFailed {
                message: "git --version failed".to_string(),
                source: None,
            });
        }

        let version_output = String::from_utf8_lossy(&output.stdout);
        let version = parse_git_version(&version_output)?;

        if version < MIN_GIT_VERSION {
            return Err(VcsError::GitCliVersionTooOld {
                found: format!("{}.{}.0", version.0, version.1),
            });
        }

        Ok(format!("{}.{}.0", version.0, version.1))
    }
}

impl VcsBackend for GitBackend {
    /// Get the backend type
    ///
    /// # Postconditions
    /// - Q12: Always returns `BackendType::Git`
    fn backend_type(&self) -> BackendType {
        BackendType::Git
    }

    /// Get the repository path
    ///
    /// # Postconditions
    /// - I6: Returns absolute, canonical path
    fn path(&self) -> &RepositoryPath {
        &self.path
    }

    /// Get the current branch name
    ///
    /// # Preconditions
    /// - P5: Repository is open and valid
    ///
    /// # Postconditions
    /// - Q2: Branch name has no `refs/heads/` prefix
    /// - Q3: Returns `None` for detached HEAD
    /// - Q3b: Returns `None` for unborn branch (empty repo)
    ///
    /// # Errors
    /// - `VcsError::GitReferenceError` if HEAD is unreadable (corrupt)
    fn current_branch(&self) -> Result<Option<BranchName>, VcsError> {
        let repo = self.repo.lock().map_err(|_| {
            VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
        })?;

        let head = repo.head();

        match head {
            Ok(head) => {
                // Check if we're on a branch (not detached HEAD)
                let branch_name = head
                    .shorthand()
                    .filter(|name| !name.is_empty() && head.is_branch());

                branch_name
                    .map(|name| {
                        BranchName::new(name).map_err(|_| {
                            VcsError::GitReferenceError(format!("Invalid branch name: {name}"))
                        })
                    })
                    .transpose()
            }
            Err(e) => {
                // Handle unborn branch (branch exists but has no commits)
                // Try to get the branch name from HEAD reference
                if e.code() == git2::ErrorCode::UnbornBranch {
                    if let Ok(reference) = repo.head() {
                        if let Some(name) = reference.shorthand().filter(|n| !n.is_empty()) {
                            return BranchName::new(name).map(Some).map_err(|_| {
                                VcsError::GitReferenceError(format!("Invalid branch name: {name}"))
                            });
                        }
                    }
                    return Ok(None);
                }
                // Handle missing refs (partially initialized repo)
                if e.code() == git2::ErrorCode::NotFound {
                    return Ok(None);
                }
                // Other errors are real problems
                Err(VcsError::GitReferenceError(format!(
                    "Failed to read HEAD: {}",
                    e.message()
                )))
            }
        }
    }

    /// List all local branches
    ///
    /// # Preconditions
    /// - P5: Repository is open and valid
    ///
    /// # Postconditions
    /// - Q4: Returns only local branches (refs/heads/*)
    /// - Q5: Branch names have no `refs/heads/` prefix
    ///
    /// # Errors
    /// - `VcsError::GitReferenceError` if references unreadable
    fn list_branches(&self) -> Result<Vec<BranchName>, VcsError> {
        let repo = self.repo.lock().map_err(|_| {
            VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
        })?;

        let branches = repo.branches(Some(git2::BranchType::Local)).map_err(|e| {
            VcsError::GitReferenceError(format!("Failed to list branches: {}", e.message()))
        })?;

        let mut result = branches
            .map(|branch_result| {
                let (branch, _branch_type) = branch_result.map_err(|e| {
                    VcsError::GitReferenceError(format!("Failed to read branch: {}", e.message()))
                })?;

                let name = branch.name().map_err(|e| {
                    VcsError::GitReferenceError(format!(
                        "Failed to get branch name: {}",
                        e.message()
                    ))
                })?;

                Ok(name.and_then(|value| BranchName::new(value).ok()))
            })
            .collect::<Result<Vec<Option<BranchName>>, VcsError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        // Sort branches alphabetically
        result.sort_by(|a, b| a.as_str().cmp(b.as_str()));

        Ok(result)
    }

    /// Get repository status
    ///
    /// # Preconditions
    /// - P5: Repository is open and valid
    ///
    /// # Postconditions
    /// - Q6: Accurately reflects working directory state
    /// - Q7: `has_changes` is false when clean
    ///
    /// # Errors
    /// - `VcsError::GitOpenFailed` if status check fails
    fn status(&self) -> Result<RepoStatus, VcsError> {
        // Collect status data while holding the lock
        let (added, modified, deleted) = {
            let repo = self.repo.lock().map_err(|_| {
                VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
            })?;

            let mut opts = git2::StatusOptions::new();
            opts.include_untracked(false)
                .include_ignored(false)
                .include_unmodified(false)
                .recurse_untracked_dirs(false);

            let statuses = repo
                .statuses(Some(&mut opts))
                .map_err(|e| VcsError::GitOpenFailed {
                    path: self.path.as_path().to_path_buf(),
                    message: format!("Failed to get status: {}", e.message()),
                    source: Some(e),
                })?;

            statuses
                .iter()
                .fold((0u32, 0u32, 0u32), |(added, modified, deleted), entry| {
                    let status = entry.status();

                    let next_added =
                        added.saturating_add(u32::from(status.contains(git2::Status::INDEX_NEW)));
                    let next_modified = modified
                        .saturating_add(u32::from(status.contains(git2::Status::INDEX_MODIFIED)))
                        .saturating_add(u32::from(status.contains(git2::Status::WT_MODIFIED)));
                    let next_deleted = deleted
                        .saturating_add(u32::from(status.contains(git2::Status::INDEX_DELETED)))
                        .saturating_add(u32::from(status.contains(git2::Status::WT_DELETED)));

                    (next_added, next_modified, next_deleted)
                })
        };

        let has_changes = added > 0 || modified > 0 || deleted > 0;

        // Call current_branch (which needs its own lock)
        let current_branch = self.current_branch()?;

        Ok(RepoStatus {
            has_changes,
            added,
            modified,
            deleted,
            current_branch,
        })
    }

    /// Check if a commit exists
    ///
    /// # Preconditions
    /// - P5: Repository is open and valid
    /// - P8: Commit ID is not empty (validated by `CommitId`)
    ///
    /// # Postconditions
    /// - Q8: Returns `true` for valid commit
    /// - Q9: Returns `false` for non-existent commit
    /// - Q9b: Returns `false` for malformed/invalid revision specifiers
    ///
    /// # Errors
    /// - `VcsError::GitOpenFailed` if lookup fails due to repository corruption
    fn commit_exists(&self, id: &CommitId) -> Result<bool, VcsError> {
        let repo = self.repo.lock().map_err(|_| {
            VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
        })?;

        // Try to resolve the commit ID
        let result = repo.revparse_single(id.as_str());

        match result {
            Ok(obj) => {
                // Check if it's actually a commit
                let is_commit = obj.kind() == Some(git2::ObjectType::Commit);
                Ok(is_commit)
            }
            Err(e) => {
                // For most git2 errors during revparse, the commit simply doesn't exist
                // or the revision specifier is invalid - return false in both cases
                // This includes: NotFound, Ambiguous, and parsing errors for invalid specs
                match e.code() {
                    git2::ErrorCode::NotFound
                    | git2::ErrorCode::Ambiguous
                    | git2::ErrorCode::InvalidSpec => Ok(false),
                    // Other errors (e.g., repository corruption) are real problems
                    _ => Err(VcsError::GitOpenFailed {
                        path: self.path.as_path().to_path_buf(),
                        message: format!("Failed to lookup commit: {}", e.message()),
                        source: Some(e),
                    }),
                }
            }
        }
    }

    /// Rebase the given branch onto its parent branch
    ///
    /// # Preconditions
    /// - Branch must exist in the repository
    /// - Working directory must be clean
    ///
    /// # Errors
    /// Returns `VcsError` if the rebase fails.
    fn sync(&self, branch: &BranchName, parent: &BranchName) -> Result<(), VcsError> {
        use std::process::Command;

        // Validate preconditions using railway-oriented composition
        self.is_clean().and_then(|clean| {
            if clean {
                Ok(clean)
            } else {
                Err(VcsError::DirtyWorkingDirectory)
            }
        })?;

        let branches = self.list_branches()?;
        let current = self.current_branch()?;

        // Check branch exists
        let is_current_branch = current
            .as_ref()
            .map(|b| b.as_str() == branch.as_str())
            .unwrap_or(false);
        let branch_exists =
            is_current_branch || branches.iter().any(|b| b.as_str() == branch.as_str());

        branch_exists
            .then_some(())
            .ok_or_else(|| VcsError::NotFound {
                entity: "Branch",
                id: branch.as_str().to_string(),
            })?;

        // Check parent exists
        let parent_exists =
            parent.as_str() == "trunk" || branches.iter().any(|b| b.as_str() == parent.as_str());

        parent_exists
            .then_some(())
            .ok_or_else(|| VcsError::NotFound {
                entity: "Parent branch",
                id: parent.as_str().to_string(),
            })?;

        // Store original branch to restore later
        let original_branch = current;

        // Checkout the branch to rebase
        let checkout_result = Command::new("git")
            .args(["checkout", branch.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to checkout branch '{}'", branch.as_str()),
                source: Some(e),
            })
            .and_then(|output| {
                output
                    .status
                    .success()
                    .then_some(())
                    .ok_or_else(|| VcsError::GitCliFailed {
                        command: format!("git checkout {}", branch.as_str()),
                        source: None,
                    })
            })?;

        let _ = checkout_result;

        // Perform rebase onto parent
        let _rebase_result = Command::new("git")
            .args(["rebase", "--update-refs", parent.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to rebase onto '{}'", parent.as_str()),
                source: Some(e),
            })
            .and_then(|output| {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let is_up_to_date =
                    stderr.contains("Current branch") && stderr.contains("is up to date");
                (output.status.success() || is_up_to_date)
                    .then_some(())
                    .ok_or_else(|| VcsError::GitCliFailed {
                        command: format!("git rebase --update-refs {}", parent.as_str()),
                        source: None,
                    })
            })?;

        // Restore original branch if different from target
        let _ = original_branch
            .filter(|orig| orig.as_str() != branch.as_str())
            .and_then(|orig| {
                Command::new("git")
                    .args(["checkout", orig.as_str()])
                    .current_dir(self.path.as_path())
                    .output()
                    .ok()
            });

        Ok(())
    }

    /// Get the tip commit of a branch
    fn get_branch_tip(&self, branch: &BranchName) -> Result<CommitId, VcsError> {
        let repo = self.repo.lock().map_err(|_| {
            VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
        })?;

        // Try to find the branch reference
        let ref_name = format!("refs/heads/{}", branch.as_str());
        let reference = repo
            .find_branch(&ref_name, git2::BranchType::Local)
            .or_else(|_| {
                // Try just the branch name directly
                repo.find_branch(branch.as_str(), git2::BranchType::Local)
            })
            .map_err(|_e| VcsError::NotFound {
                entity: "Branch",
                id: branch.as_str().to_string(),
            })?;

        // Peel to commit
        let commit = reference
            .get()
            .peel_to_commit()
            .map_err(|e| VcsError::GitReferenceError(e.message().to_string()))?;

        CommitId::new(commit.id().to_string()).map_err(|e| VcsError::InvalidCommitId(e.to_string()))
    }

    /// Check if one commit is an ancestor of another
    fn is_ancestor(&self, ancestor: &CommitId, descendant: &CommitId) -> Result<bool, VcsError> {
        let _repo = self.repo.lock().map_err(|_| {
            VcsError::GitReferenceError("Failed to acquire repository lock".to_string())
        })?;

        // Resolve both commits using a different approach - use git CLI
        use std::process::Command;

        // Check if ancestor is an ancestor of descendant using git merge-base --is-ancestor
        let output = Command::new("git")
            .args([
                "merge-base",
                "--is-ancestor",
                ancestor.as_str(),
                descendant.as_str(),
            ])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: "Failed to check ancestry".to_string(),
                source: Some(e),
            })?;

        // If the command succeeds, ancestor IS an ancestor of descendant
        // If it fails, it is NOT an ancestor
        Ok(output.status.success())
    }

    /// Fetch from remote
    fn fetch(&self) -> Result<(), VcsError> {
        use std::process::Command;

        // Try to fetch from origin
        let output = Command::new("git")
            .args(["fetch", "--all", "--prune"])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: "Failed to execute git fetch".to_string(),
                source: Some(e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            // Check if it's just "no remote configured" - that's OK
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("No remote") || stderr.contains("fatal: 'origin'") {
                // No remote configured - that's fine, just skip fetch
                Ok(())
            } else {
                Err(VcsError::GitCliFailed {
                    command: "git fetch --all --prune".to_string(),
                    source: None,
                })
            }
        }
    }

    fn create_branch(&self, branch: &BranchName, parent: &BranchName) -> Result<(), VcsError> {
        use std::process::Command;

        // Check for reserved branch name BEFORE calling VCS
        if crate::vcs::error_classification::is_reserved_branch_name(branch.as_str()) {
            return Err(VcsError::ReservedBranchName {
                name: branch.as_str().to_string(),
            });
        }

        let command_str = format!("git branch {} {}", branch.as_str(), parent.as_str());

        let output = Command::new("git")
            .args(["branch", branch.as_str(), parent.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to create branch: {}", e),
                source: Some(e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            // Classify the error from Git stderr
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(crate::vcs::error_classification::classify_git_branch_error(
                &stderr,
                &command_str,
                branch.as_str(),
                parent.as_str(),
            ))
        }
    }

    fn create_backup_ref(&self, name: String, commit: &CommitId) -> Result<(), VcsError> {
        use std::process::Command;
        let ref_name = format!("refs/backups/{}", name);
        let output = Command::new("git")
            .args(["update-ref", &ref_name, commit.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to create backup ref: {}", e),
                source: Some(e),
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(VcsError::GitCliFailed {
                command: format!("git update-ref {} {}", ref_name, commit.as_str()),
                source: None,
            })
        }
    }

    fn update_branch_ref(&self, branch: &BranchName, commit: &CommitId) -> Result<(), VcsError> {
        use std::process::Command;
        let ref_name = format!("refs/heads/{}", branch.as_str());
        let output = Command::new("git")
            .args(["update-ref", &ref_name, commit.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to update branch ref: {}", e),
                source: Some(e),
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(VcsError::GitCliFailed {
                command: format!("git update-ref {} {}", ref_name, commit.as_str()),
                source: None,
            })
        }
    }

    fn delete_backup_ref(&self, name: &str) -> Result<(), VcsError> {
        use std::process::Command;
        let ref_name = format!("refs/backups/{}", name);
        let _output = Command::new("git")
            .args(["update-ref", "-d", &ref_name])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to delete backup ref: {}", e),
                source: Some(e),
            })?;
        Ok(())
    }

    fn compute_patch_id(&self, commit: &CommitId) -> Result<String, VcsError> {
        use std::{
            io::Write,
            process::{Command, Stdio},
        };

        let commit_str = commit.as_str();

        // P8: Validate commit exists before computing patch-id
        if !self.commit_exists(commit)? {
            return Err(VcsError::NotFound {
                entity: "Commit",
                id: commit_str.to_string(),
            });
        }

        let show_output = Command::new("git")
            .args(["show", commit_str])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to get commit {}: {}", commit_str, e),
                source: Some(e),
            })?;

        if !show_output.status.success() {
            return Err(VcsError::NotFound {
                entity: "Commit",
                id: commit_str.to_string(),
            });
        }

        let mut child = Command::new("git")
            .args(["patch-id", "--stable"])
            .current_dir(self.path.as_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to spawn git patch-id: {}", e),
                source: Some(e),
            })?;

        if let Some(ref mut stdin) = child.stdin {
            stdin
                .write_all(&show_output.stdout)
                .map_err(|e| VcsError::CommandFailed {
                    message: format!("Failed to write to git patch-id stdin: {}", e),
                    source: Some(e),
                })?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to wait for git patch-id: {}", e),
                source: Some(e),
            })?;

        if !output.status.success() {
            return Err(VcsError::CommandFailed {
                message: "git patch-id command failed".to_string(),
                source: None,
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let patch_id = output_str.split_whitespace().next().ok_or_else(|| {
            VcsError::GitParseError("Failed to parse patch-id output".to_string())
        })?;

        Ok(patch_id.to_string())
    }

    fn compute_diff_hash(&self, from: &CommitId, to: &CommitId) -> Result<String, VcsError> {
        use sha2::{Digest, Sha256};

        let from_str = from.as_str();
        let to_str = to.as_str();

        // P9: Validate both commits exist
        if !self.commit_exists(from)? {
            return Err(VcsError::NotFound {
                entity: "Commit",
                id: from_str.to_string(),
            });
        }
        if !self.commit_exists(to)? {
            return Err(VcsError::NotFound {
                entity: "Commit",
                id: to_str.to_string(),
            });
        }

        // P10: Validate from is ancestor of to
        if !self.is_ancestor(from, to)? {
            return Err(VcsError::InvalidState(format!(
                "Commit {} is not an ancestor of {}",
                from_str, to_str
            )));
        }

        let output = std::process::Command::new("git")
            .args(["diff", from.as_str(), to.as_str()])
            .current_dir(self.path.as_path())
            .output()
            .map_err(|e| VcsError::CommandFailed {
                message: format!("Failed to compute diff: {}", e),
                source: Some(e),
            })?;

        if !output.status.success() {
            return Err(VcsError::InvalidState(format!(
                "git diff failed for {}..{}",
                from.as_str(),
                to.as_str()
            )));
        }

        let mut hasher = Sha256::new();
        hasher.update(&output.stdout);
        let hash = hasher.finalize();

        Ok(hex::encode(hash))
    }
}

/// Parse Git version from output like "git version 2.43.0"
fn parse_git_version(output: &str) -> Result<(u32, u32), VcsError> {
    let output = output.trim();

    // Expected format: "git version X.Y.Z" or "git version X.Y.Z.windows.1"
    let parts: Vec<&str> = output.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(VcsError::GitParseError(format!(
            "Unexpected git version format: {output}"
        )));
    }

    let version_str = parts[2];

    // Parse major.minor
    let version_parts: Vec<&str> = version_str.split('.').collect();

    if version_parts.len() < 2 {
        return Err(VcsError::GitParseError(format!(
            "Invalid version number: {version_str}"
        )));
    }

    let major = version_parts[0].parse::<u32>().map_err(|_| {
        VcsError::GitParseError(format!("Invalid major version: {}", version_parts[0]))
    })?;

    let minor = version_parts[1].parse::<u32>().map_err(|_| {
        VcsError::GitParseError(format!("Invalid minor version: {}", version_parts[1]))
    })?;

    Ok((major, minor))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::{fs, process::Command};

    use tempfile::TempDir;

    use super::*;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a temporary Git repository for testing
    fn create_test_repo() -> (TempDir, std::path::PathBuf) {
        let temp = TempDir::new().expect("Failed to create temp dir");
        let path = temp.path().to_path_buf();

        // Initialize repo
        let output = Command::new("git")
            .args(["init"])
            .current_dir(&path)
            .output()
            .expect("Failed to run git init");

        assert!(
            output.status.success(),
            "git init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Configure for tests
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&path)
            .output()
            .expect("Failed to configure git");

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&path)
            .output()
            .expect("Failed to configure git");

        (temp, path)
    }

    /// Create a bare repository
    fn create_bare_repo() -> (TempDir, std::path::PathBuf) {
        let temp = TempDir::new().expect("Failed to create temp dir");
        let path = temp.path().join("repo.git");

        let output = Command::new("git")
            .args(["init", "--bare"])
            .arg(&path)
            .output()
            .expect("Failed to run git init --bare");

        assert!(
            output.status.success(),
            "git init --bare failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        (temp, path)
    }

    /// Create initial commit and return the commit SHA
    fn create_initial_commit(repo_path: &std::path::Path) -> String {
        let file = repo_path.join("README.md");
        fs::write(&file, "# Test Repository\n").expect("Failed to write file");

        Command::new("git")
            .args(["add", "."])
            .current_dir(repo_path)
            .output()
            .expect("Failed to git add");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to git commit");

        // Get commit SHA
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to get HEAD");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    // =========================================================================
    // GitBackend::open Happy Path Tests
    // =========================================================================

    #[test]
    fn test_open_git_repo_returns_ok() {
        // Given: A valid Git repository with working tree
        let (_temp, path) = create_test_repo();

        // When: GitBackend::open() is called
        let result = GitBackend::open(&path);

        // Then: Returns Ok(GitBackend)
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_returns_gitbackend_with_correct_path() {
        // Given: A valid Git repository
        let (_temp, path) = create_test_repo();

        // When: GitBackend::open() is called
        let backend = GitBackend::open(&path).expect("Should open");

        // Then: Path is correct
        let backend_path = backend.path().as_path();
        assert!(backend_path.is_absolute());
    }

    #[test]
    fn test_backend_type_returns_git() {
        // Given: A GitBackend
        let (_temp, path) = create_test_repo();
        let backend = GitBackend::open(&path).expect("Should open");

        // When: backend_type() is called
        let backend_type = backend.backend_type();

        // Then: Returns Git
        assert_eq!(backend_type, BackendType::Git);
    }

    #[test]
    fn test_path_returns_absolute_canonical_path() {
        // Given: A GitBackend
        let (_temp, path) = create_test_repo();
        let backend = GitBackend::open(&path).expect("Should open");

        // When: path() is called
        let repo_path = backend.path();

        // Then: Path is absolute and canonical
        assert!(repo_path.as_path().is_absolute());
        let path_str = repo_path.as_path().to_string_lossy();
        assert!(!path_str.contains("/./"));
        assert!(!path_str.contains("/../"));
    }

    #[test]
    fn test_open_from_subdirectory_finds_repo_root() {
        // Given: A subdirectory inside a Git repo
        let (_temp, path) = create_test_repo();
        let subdir = path.join("src").join("lib");
        fs::create_dir_all(&subdir).expect("Failed to create subdir");

        // When: GitBackend::open() is called with subdirectory
        let result = GitBackend::open(&subdir);

        // Then: Finds repo root successfully
        assert!(result.is_ok());
    }

    // =========================================================================
    // current_branch Happy Path Tests
    // =========================================================================

    #[test]
    fn test_current_branch_on_main_returns_main() {
        // Given: Repository on main branch (default after init)
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: current_branch() is called
        let result = backend.current_branch();

        // Then: Returns Ok(Some(BranchName))
        assert!(result.is_ok());
        let branch = result.expect("Should have branch");
        assert!(branch.is_some());
    }

    #[test]
    fn test_current_branch_name_has_no_refs_prefix() {
        // Given: Repository on a branch
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: current_branch() is called
        let branch = backend.current_branch().expect("Should work");

        // Then: Branch name has no refs/heads/ prefix
        if let Some(name) = branch {
            assert!(!name.as_str().starts_with("refs/heads/"));
        }
    }

    #[test]
    fn test_current_branch_on_branch_with_slash_works() {
        // Given: Repository with a feature branch
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        // Create and checkout feature branch
        Command::new("git")
            .args(["checkout", "-b", "feature/test-branch"])
            .current_dir(&path)
            .output()
            .expect("Failed to create branch");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: current_branch() is called
        let branch = backend.current_branch().expect("Should work");

        // Then: Returns feature/test-branch
        assert!(branch.is_some());
        let name = branch.expect("Should have branch");
        assert_eq!(name.as_str(), "feature/test-branch");
    }

    // =========================================================================
    // current_branch Detached HEAD Tests
    // =========================================================================

    #[test]
    fn test_current_branch_detached_head_returns_none() {
        // Given: Repository in detached HEAD state
        let (_temp, path) = create_test_repo();
        let sha = create_initial_commit(&path);

        // Checkout specific commit (detached HEAD)
        Command::new("git")
            .args(["checkout", &sha])
            .current_dir(&path)
            .output()
            .expect("Failed to checkout commit");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: current_branch() is called
        let result = backend.current_branch();

        // Then: Returns Ok(None)
        assert!(result.is_ok());
        assert!(result.expect("Should have result").is_none());
    }

    // =========================================================================
    // list_branches Happy Path Tests
    // =========================================================================

    #[test]
    fn test_list_branches_returns_all_local_branches() {
        // Given: Repository with multiple branches
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        // Create additional branches
        Command::new("git")
            .args(["branch", "develop"])
            .current_dir(&path)
            .output()
            .expect("Failed to create branch");

        Command::new("git")
            .args(["branch", "feature/a"])
            .current_dir(&path)
            .output()
            .expect("Failed to create branch");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: list_branches() is called
        let branches = backend.list_branches().expect("Should work");

        // Then: Returns all local branches
        assert!(branches.len() >= 3);
    }

    #[test]
    fn test_list_branches_names_have_no_refs_prefix() {
        // Given: Repository with branches
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: list_branches() is called
        let branches = backend.list_branches().expect("Should work");

        // Then: No branch has refs/heads/ prefix
        for branch in &branches {
            assert!(!branch.as_str().starts_with("refs/heads/"));
        }
    }

    #[test]
    fn test_list_branches_single_branch_returns_one() {
        // Given: Repository with one branch (after initial commit)
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: list_branches() is called
        let branches = backend.list_branches().expect("Should work");

        // Then: Returns at least one branch
        assert!(!branches.is_empty());
    }

    // =========================================================================
    // status Happy Path Tests
    // =========================================================================

    #[test]
    fn test_status_clean_repo_returns_has_changes_false() {
        // Given: Clean repository
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: status() is called
        let status = backend.status().expect("Should work");

        // Then: has_changes is false
        assert!(!status.has_changes);
    }

    #[test]
    fn test_status_clean_repo_zero_counts() {
        // Given: Clean repository
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: status() is called
        let status = backend.status().expect("Should work");

        // Then: All counts are zero
        assert_eq!(status.added, 0);
        assert_eq!(status.modified, 0);
        assert_eq!(status.deleted, 0);
    }

    #[test]
    fn test_status_modified_file_has_changes_true() {
        // Given: Repository with modified file
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        // Modify a file
        let file = path.join("README.md");
        fs::write(&file, "# Modified content\n").expect("Failed to modify file");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: status() is called
        let status = backend.status().expect("Should work");

        // Then: has_changes is true
        assert!(status.has_changes);
        assert!(status.modified > 0);
    }

    #[test]
    fn test_status_deleted_file_increments_deleted() {
        // Given: Repository with deleted file
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        // Delete the file
        let file = path.join("README.md");
        fs::remove_file(&file).expect("Failed to delete file");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: status() is called
        let status = backend.status().expect("Should work");

        // Then: deleted count > 0
        assert!(status.deleted > 0);
    }

    // =========================================================================
    // commit_exists Happy Path Tests
    // =========================================================================

    #[test]
    fn test_commit_exists_valid_sha_returns_true() {
        // Given: A valid commit SHA
        let (_temp, path) = create_test_repo();
        let sha = create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new(&sha).expect("Valid commit ID");

        // When: commit_exists() is called
        let result = backend.commit_exists(&commit_id);

        // Then: Returns Ok(true)
        assert!(result.is_ok());
        assert!(result.expect("Should have result"));
    }

    #[test]
    fn test_commit_exists_short_sha_returns_true() {
        // Given: A short SHA
        let (_temp, path) = create_test_repo();
        let sha = create_initial_commit(&path);
        let short_sha = &sha[..7];

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new(short_sha).expect("Valid commit ID");

        // When: commit_exists() is called
        let result = backend.commit_exists(&commit_id);

        // Then: Returns Ok(true)
        assert!(result.is_ok());
        assert!(result.expect("Should have result"));
    }

    #[test]
    fn test_commit_exists_head_returns_true() {
        // Given: Repository with commits
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new("HEAD").expect("Valid commit ID");

        // When: commit_exists() is called
        let result = backend.commit_exists(&commit_id);

        // Then: Returns Ok(true)
        assert!(result.is_ok());
        assert!(result.expect("Should have result"));
    }

    // =========================================================================
    // commit_exists Error Path Tests
    // =========================================================================

    #[test]
    fn test_commit_exists_nonexistent_sha_returns_false() {
        // Given: A non-existent SHA
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new("deadbeef12345678901234567890123456789012")
            .expect("Valid commit ID format");

        // When: commit_exists() is called
        let result = backend.commit_exists(&commit_id);

        // Then: Returns Ok(false) - not an error
        assert!(result.is_ok());
        assert!(!result.expect("Should have result"));
    }

    #[test]
    fn test_commit_exists_invalid_sha_returns_false() {
        // Given: An invalid SHA
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new("not-a-valid-ref").expect("Valid string");

        // When: commit_exists() is called
        let result = backend.commit_exists(&commit_id);

        // Then: Returns Ok(false)
        assert!(result.is_ok());
        assert!(!result.expect("Should have result"));
    }

    // =========================================================================
    // is_clean Tests
    // =========================================================================

    #[test]
    fn test_is_clean_clean_repo_returns_true() {
        // Given: Clean repository
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        // When: is_clean() is called
        let result = backend.is_clean();

        // Then: Returns Ok(true)
        assert!(result.is_ok());
        assert!(result.expect("Should be clean"));
    }

    #[test]
    fn test_is_clean_with_modified_file_returns_false() {
        // Given: Repository with modified file
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        // Modify a file
        let file = path.join("README.md");
        fs::write(&file, "# Modified\n").expect("Failed to modify");

        let backend = GitBackend::open(&path).expect("Should open");

        // When: is_clean() is called
        let result = backend.is_clean();

        // Then: Returns Ok(false)
        assert!(result.is_ok());
        assert!(!result.expect("Should have result"));
    }

    // =========================================================================
    // verify_cli_version Tests
    // =========================================================================

    #[test]
    fn test_verify_cli_version_returns_version_string() {
        // Given: A GitBackend
        let (_temp, path) = create_test_repo();

        let config = GitBackendConfig {
            verify_cli_version: false,
        };
        let backend = GitBackend::open_with_config(&path, &config).expect("Should open");

        // When: verify_cli_version() is called
        let result = backend.verify_cli_version();

        // Then: Returns Ok(version_string)
        assert!(result.is_ok());
        let version = result.expect("Should have version");
        assert!(!version.is_empty());
    }

    // =========================================================================
    // GitBackend::open Error Path Tests
    // =========================================================================

    #[test]
    fn test_open_nonexistent_path_returns_path_not_found() {
        // Given: A non-existent path
        let nonexistent = "/nonexistent/path/xyz/test";

        // When: GitBackend::open() is called
        let result = GitBackend::open(nonexistent);

        // Then: Returns Err(VcsError::PathNotFound)
        assert!(matches!(result, Err(VcsError::PathNotFound(_))));
    }

    #[test]
    fn test_open_file_path_returns_path_not_directory() {
        // Given: A path to a file
        let temp = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp.path().join("test.txt");
        fs::write(&file_path, "content").expect("Failed to write file");

        // When: GitBackend::open() is called
        let result = GitBackend::open(&file_path);

        // Then: Returns Err(VcsError::PathNotDirectory)
        assert!(matches!(result, Err(VcsError::PathNotDirectory(_))));
    }

    #[test]
    fn test_open_non_git_directory_returns_git_open_failed() {
        // Given: A directory without .git
        let temp = TempDir::new().expect("Failed to create temp dir");

        // When: GitBackend::open() is called
        let result = GitBackend::open(temp.path());

        // Then: Returns Err(VcsError::GitOpenFailed)
        assert!(matches!(result, Err(VcsError::GitOpenFailed { .. })));
    }

    #[test]
    fn test_open_bare_repo_returns_bare_repository_not_supported() {
        // Given: A bare Git repository
        let (_temp, path) = create_bare_repo();

        // When: GitBackend::open() is called
        let result = GitBackend::open(&path);

        // Then: Returns Err(VcsError::BareRepositoryNotSupported)
        match result {
            Err(VcsError::BareRepositoryNotSupported(p)) => {
                assert_eq!(p, path);
            }
            Err(e) => panic!("Wrong error type: {e:?}"),
            Ok(_) => panic!("Should have returned error"),
        }
    }

    // =========================================================================
    // Contract Verification Tests
    // =========================================================================

    #[test]
    fn test_postcondition_q1_open_returns_valid_backend() {
        // Q1: open() returns Ok(GitBackend) with valid repo handle
        let (_temp, path) = create_test_repo();

        let backend = GitBackend::open(&path);

        assert!(backend.is_ok());
        let b = backend.expect("Should have backend");
        assert_eq!(b.backend_type(), BackendType::Git);
    }

    #[test]
    fn test_postcondition_q3_detached_returns_none() {
        // Q3: current_branch() returns None for detached HEAD
        let (_temp, path) = create_test_repo();
        let sha = create_initial_commit(&path);

        Command::new("git")
            .args(["checkout", &sha])
            .current_dir(&path)
            .output()
            .expect("Failed to checkout");

        let backend = GitBackend::open(&path).expect("Should open");

        let branch = backend.current_branch().expect("Should work");
        assert!(branch.is_none());
    }

    #[test]
    fn test_postcondition_q7_clean_has_changes_false() {
        // Q7: status().has_changes is false when clean
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        let status = backend.status().expect("Should work");
        assert!(!status.has_changes);
    }

    #[test]
    fn test_postcondition_q10_is_clean_true_when_clean() {
        // Q10: is_clean() returns true when no uncommitted changes
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        let clean = backend.is_clean().expect("Should work");
        assert!(clean);
    }

    #[test]
    fn test_postcondition_q12_backend_type_is_git() {
        // Q12: backend_type() returns BackendType::Git
        let (_temp, path) = create_test_repo();

        let backend = GitBackend::open(&path).expect("Should open");

        assert_eq!(backend.backend_type(), BackendType::Git);
    }

    #[test]
    fn test_invariant_i1_never_bare_repo() {
        // I1: GitBackend always wraps a non-bare repository
        let (_temp, path) = create_test_repo();

        let backend = GitBackend::open(&path).expect("Should open");

        // Backend was created successfully, so it's not bare
        assert_eq!(backend.backend_type(), BackendType::Git);
    }

    #[test]
    fn test_invariant_i6_path_is_absolute_canonical() {
        // I6: Path is always absolute and canonical
        let (_temp, path) = create_test_repo();

        let backend = GitBackend::open(&path).expect("Should open");

        let repo_path = backend.path();
        assert!(repo_path.as_path().is_absolute());
    }

    // =========================================================================
    // Contract Violation Tests
    // =========================================================================

    #[test]
    fn test_p4_violation_bare_repo_returns_bare_repository_not_supported() {
        // VIOLATES P4: GitBackend::open("/path/to/bare/repo.git")
        let (_temp, path) = create_bare_repo();

        let result = GitBackend::open(&path);

        // Then: Returns Err(VcsError::BareRepositoryNotSupported)
        // And: NOT a panic
        match result {
            Err(VcsError::BareRepositoryNotSupported(p)) => {
                assert_eq!(p, path);
            }
            Err(e) => panic!("Wrong error type: {e:?}"),
            Ok(_) => panic!("Should have returned error"),
        }
    }

    #[test]
    fn test_q2_violation_branch_prefix_not_returned() {
        // Q2: Branch names should not have refs/heads/ prefix
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        let branch = backend.current_branch().expect("Should work");
        if let Some(name) = branch {
            assert!(!name.as_str().starts_with("refs/heads/"));
        }
    }

    #[test]
    fn test_q9_violation_nonexistent_commit_returns_false() {
        // Q9: commit_exists() returns false for non-existent commit
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");
        let commit_id = CommitId::new("zzz999xxx888").expect("Valid string format");

        let result = backend.commit_exists(&commit_id);

        // Returns Ok(false), NOT Ok(true), NOT an error
        assert!(result.is_ok());
        assert!(!result.expect("Should have result"));
    }

    #[test]
    fn test_i2_violation_list_branches_no_prefix() {
        // I2: All branch names are normalized (no refs/heads/ prefix)
        let (_temp, path) = create_test_repo();
        create_initial_commit(&path);

        let backend = GitBackend::open(&path).expect("Should open");

        let branches = backend.list_branches().expect("Should work");

        for branch in &branches {
            assert!(
                !branch.as_str().starts_with("refs/heads/"),
                "Branch '{}' should not have refs/heads/ prefix",
                branch.as_str()
            );
        }
    }

    // =========================================================================
    // Version Parsing Tests
    // =========================================================================

    #[test]
    fn test_parse_git_version_standard() {
        let output = "git version 2.43.0";
        let result = parse_git_version(output);
        assert!(result.is_ok());
        assert_eq!(result.expect("Should parse"), (2, 43));
    }

    #[test]
    fn test_parse_git_version_with_windows_suffix() {
        let output = "git version 2.43.0.windows.1";
        let result = parse_git_version(output);
        assert!(result.is_ok());
        assert_eq!(result.expect("Should parse"), (2, 43));
    }

    #[test]
    fn test_parse_git_version_invalid_format() {
        let output = "invalid output";
        let result = parse_git_version(output);
        assert!(matches!(result, Err(VcsError::GitParseError(_))));
    }

    #[test]
    fn test_parse_git_version_invalid_number() {
        let output = "git version abc.def.ghi";
        let result = parse_git_version(output);
        assert!(matches!(result, Err(VcsError::GitParseError(_))));
    }

    // =========================================================================
    // Thread Safety Tests
    // =========================================================================

    #[test]
    fn test_gitbackend_is_send_sync() {
        // I4: Backend must be Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<GitBackend>();
    }

    // =========================================================================
    // Config Tests
    // =========================================================================

    #[test]
    fn test_git_backend_config_default() {
        let config = GitBackendConfig::default();
        assert!(config.verify_cli_version);
    }

    #[test]
    fn test_open_with_config_skip_version_check() {
        let (_temp, path) = create_test_repo();

        let config = GitBackendConfig {
            verify_cli_version: false,
        };

        // Should not fail even if git version check would fail
        let result = GitBackend::open_with_config(&path, &config);
        assert!(result.is_ok());
    }
}
