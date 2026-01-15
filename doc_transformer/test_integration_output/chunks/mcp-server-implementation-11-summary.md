---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#11
chunk_level: summary
chunk_type: table
heading: Implementation Details
token_count: 84
summary:     let results = tantivy_results. unwrap_or_else(|| {
---

        .ok()

    let results = tantivy_results
        .unwrap_or_else(|| {
            // Fallback: simple text matching
            fallback_docs.iter()
                .filter(|doc| /* case-insensitive search */)
                .take(limit)
                .collect()
        });

    Ok(json!({ "results": results }))
}
```

---

