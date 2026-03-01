bead_id: doc-1b5q
bead_title: doc_transformer: Fix category-config file content leak
phase: p2
updated_at: 2026-03-01T13:55:00Z

# Implementation: Fix category-config file content leak

## Problem Summary
When `--category-config` points to a file with YAML parse errors (e.g., `/etc/passwd`), the error message echoes the sensitive file contents instead of showing a generic "invalid config" message.

## Root Cause
In `doc_transformer/src/config.rs`, the `CategoryConfig::load_from_file` function at line 141-176 calls `serde_yaml::from_str(&content)` which returns an error that includes the problematic content in its error message. This leaks sensitive file contents to the user.

## Files Modified

### 1. `doc_transformer/src/config.rs`

**Change:** Modify `CategoryConfig::load_from_file` to catch YAML parse errors and transform them into generic error messages that don't include file contents.

**Current code (lines 141-176):**
```rust
pub fn load_from_file(path: &Path) -> Result<Self> {
    let content = fs::read_to_string(path)?;
    let config: CategoryConfig = serde_yaml::from_str(&content)?;

    // Validate that default_category is set
    if config.default_category.is_empty() {
        anyhow::bail!("Config error: default_category is required and must not be empty");
    }

    // Validate that all categories are lowercase alphanumeric
    if !is_valid_category_name(&config.default_category) {
        anyhow::bail!(
            "Config error: default_category '{}' is not lowercase alphanumeric",
            config.default_category
        );
    }

    for rule in &config.rules {
        if !is_valid_category_name(&rule.category) {
            anyhow::bail!(
                "Config error: category '{}' is not lowercase alphanumeric",
                rule.category
            );
        }

        if !Self::has_valid_criteria(&rule.criteria) {
            anyhow::bail!(
                "Config error: rule for category '{}' has no criteria (all are None or empty)",
                rule.category
            );
        }
    }

    Ok(config)
}
```

**New code:**
```rust
pub fn load_from_file(path: &Path) -> Result<Self> {
    let content = fs::read_to_string(path)?;

    // Parse YAML with error sanitization to prevent content leak
    // serde_yaml's error message may include the problematic content,
    // so we catch and re-wrap with a generic message
    let config: CategoryConfig = serde_yaml::from_str(&content).map_err(|_e| {
        // Return a generic error without exposing file contents
        anyhow::anyhow!(
            "invalid config: failed to parse YAML at '{}'",
            path.display()
        )
    })?;

    // Validate that default_category is set
    if config.default_category.is_empty() {
        anyhow::bail!("invalid config: default_category is required and must not be empty");
    }

    // Validate that all categories are lowercase alphanumeric
    if !is_valid_category_name(&config.default_category) {
        anyhow::bail!(
            "invalid config: default_category '{}' is not lowercase alphanumeric",
            config.default_category
        );
    }

    for rule in &config.rules {
        if !is_valid_category_name(&rule.category) {
            anyhow::bail!(
                "invalid config: category '{}' is not lowercase alphanumeric",
                rule.category
            );
        }

        if !Self::has_valid_criteria(&rule.criteria) {
            anyhow::bail!(
                "invalid config: rule for category '{}' has no criteria (all are None or empty)",
                rule.category
            );
        }
    }

    Ok(config)
}
```

### 2. `doc_transformer/src/main.rs`

**Change:** Add "invalid config" to user_input_patterns to ensure exit code 1 for category config errors.

**Location:** Around line 1018 in `map_error_to_exit_code` function.

**Added pattern:**
```rust
"invalid config",
```

## Reasoning

1. **Security**: The fix ensures that sensitive file contents (like `/etc/passwd`) never appear in error messages. Instead, users see a generic "failed to parse YAML" message.

2. **User Experience**: The error message still indicates that the config file is invalid, but doesn't expose potentially sensitive data.

3. **Exit Code**: The error will cause exit code 1 (user error) because "invalid config" is now in the user_input_patterns list in main.rs.

4. **Validation Messages**: Also updated validation error messages to use "invalid config:" prefix consistently for all category config errors, making it clear these are user input errors.

## Testing the Fix

After implementing the fix, run:
```bash
doc_transformer index <source> --output /tmp/test --category-config /etc/passwd
```

Expected behavior:
- Shows generic "invalid config: failed to parse YAML at '/etc/passwd'" message
- Does NOT show file contents from `/etc/passwd`
- Exit code: 1 (user error)

Verification:
- `/etc/passwd`: Error message shows "invalid config: failed to parse YAML at '/etc/passwd'" - no content leaked ✅
- Exit code: 1 ✅
- Valid config files still work correctly ✅
