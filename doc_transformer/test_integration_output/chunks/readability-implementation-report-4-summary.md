---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#4
chunk_level: summary
chunk_type: prose
heading: Implementation Completed
token_count: 139
summary: #### Extracted Content Metadata. #[derive(Debug, Clone, PartialEq)]
---

#### Extracted Content Metadata
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedContent {
    pub title: Option<String>,
    pub content: String,
    pub confidence: f32,        // 0.0-1.0
    pub density_score: f32,     // 0.0-1.0
}
```

### 2. Enhanced FilterConfig

Added confidence threshold:
```rust
pub struct FilterConfig {
    // ... existing fields ...
    pub min_confidence: f32,  // NEW: 0.3 default
}
```

### 3. Improved `prune_html()` Function

Updated legacy API to use new extraction system with Railway-Oriented Programming:
