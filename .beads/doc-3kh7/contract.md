# Contract: doc-3kh7

**bead_id:** doc-3kh7  
**bead_title:** CLI: broken symlinks in source directory return exit code 0  
**phase:** p0  
**updated_at:** 2026-03-01T13:28:00Z

---

## Problem Statement

When a source directory contains broken symlinks, `doc_transformer index` silently ignores them and returns exit code 0, even though some content was skipped.

## Expected Behavior

Exit code should be non-zero (1 for user error) OR a warning should be printed to stderr indicating broken symlinks were encountered.

## Actual Behavior

Exit code 0, broken symlink silently skipped

## Severity

P0 - Data integrity issue

## Reproduction

1. Create a directory with a broken symlink
2. Run `doc_transformer index <dir> --output <output>`
3. Observe: Exit code 0, no indication of broken symlinks

---

## Contract Specification

### Preconditions

- Source directory exists
- User has read access to the directory
- Output directory is writable

### Postconditions

- If broken symlinks are encountered, either:
  - Return exit code 1 (user error), OR
  - Print warning to stderr about broken symlinks
- Valid files are still processed correctly

### Acceptance Criteria

1. **Broken symlink detection**: When a broken symlink is encountered in the source directory, the tool must either:
   - Exit with code 1 and display an error message, OR
   - Print a warning to stderr and continue

2. **Error message clarity**: Error/warning message must:
   - Clearly identify that broken symlinks were found
   - List the paths of broken symlinks (if feasible)
   - Not leak file contents in error messages

3. **Valid files processing**: Valid files in the directory should still be processed correctly

4. **Exit code consistency**: Exit code 1 for user input errors (broken symlinks), not exit code 0

---

## Test Cases

### Happy Path
- Directory with valid markdown files → Exit code 0, files processed

### Error Paths
- Directory with broken symlink only → Exit code 1 or warning printed
- Directory with mixed valid files and broken symlinks → Exit code 1 or warning printed, valid files processed

---

## Implementation Notes

- Look at the discover phase in the codebase
- Broken symlinks should be detected using `std::fs::read_link` or similar
- Consider using `symlink_metadata` instead of `metadata` to detect symlinks without following them
