# BEAD-007: URL Validation Accepts URLs Without Valid Host

**Epic**: Input Validation
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/scrape.rs:348-363` (`validate_url`)
- **The Smell**: The URL validation only checks for `http`/`https` scheme but doesn't validate that the URL has a valid host. Malformed URLs like `https://?query=value` or `https://` (empty host) pass validation but will fail when passed to spider-rs, potentially causing confusing errors or undefined behavior.

```rust
fn validate_url(url: &str) -> Result<url::Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    let parsed = url::Url::parse(trimmed).context("Invalid URL format")?;

    match parsed.scheme() {
        "http" | "https" => Ok(parsed),  // No host validation!
        scheme => anyhow::bail!("Invalid URL scheme '{}'", scheme),
    }
}
```

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| User provides URL without host | `validate_url` | Return error "URL must have a valid host" |
| User provides URL with empty host | `validate_url` | Return error "URL host cannot be empty" |
| User provides URL with invalid characters in host | `validate_url` | Return error from URL parser |

### 2. DbC (Design by Contract)

**Preconditions**:
- Input is a string (possibly empty, possibly whitespace-only)

**Postconditions**:
- Returns `Ok(Url)` only if:
  - Scheme is `http` or `https`
  - Host is present and non-empty
  - URL is well-formed
- Returns `Err` with descriptive message for all invalid cases

**Invariants**:
- All returned URLs can be safely passed to spider-rs
- No panic possible in validation

### 3. Schema & Edge Cases

**Improved Validation**:
```rust
fn validate_url(url: &str) -> Result<url::Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    let parsed = url::Url::parse(trimmed).context("Invalid URL format")?;

    // Validate scheme
    match parsed.scheme() {
        "http" | "https" => {}
        scheme => anyhow::bail!(
            "Invalid URL scheme '{}': only http and https are supported",
            scheme
        ),
    }

    // Validate host exists and is non-empty
    match parsed.host_str() {
        Some(host) if !host.is_empty() => {}
        Some(_) => anyhow::bail!("URL host cannot be empty"),
        None => anyhow::bail!("URL must have a valid host"),
    }

    // Optional: Validate host is a valid domain or IP
    // Could add DNS resolution check for extra safety

    Ok(parsed)
}
```

**Edge Cases**:
| Input | Expected Result |
|-------|-----------------|
| `https://example.com` | Ok |
| `https://example.com/path` | Ok |
| `https://` | Err: "URL must have a valid host" |
| `https://?foo=bar` | Err: "URL must have a valid host" |
| `https://192.168.1.1` | Ok (valid IP) |
| `https://[::1]` | Ok (valid IPv6) |
| `https://example.com:8080` | Ok (with port) |
| `https://:8080/path` | Err: "URL host cannot be empty" |
| `HTTPS://EXAMPLE.COM` | Ok (case insensitive) |

---

## FIX LOCATIONS

1. `src/scrape.rs:348-363` - Add host validation after scheme check

---

## TEST CASES

```rust
#[test]
fn test_validate_url_missing_host() {
    assert!(validate_url("https://").is_err());
    assert!(validate_url("https://?foo=bar").is_err());
    assert!(validate_url("https://:8080/path").is_err());
}

#[test]
fn test_validate_url_valid_hosts() {
    assert!(validate_url("https://example.com").is_ok());
    assert!(validate_url("https://192.168.1.1").is_ok());
    assert!(validate_url("https://[::1]").is_ok());
    assert!(validate_url("https://localhost:3000").is_ok());
}
```
