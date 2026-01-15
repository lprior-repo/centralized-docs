---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#28
chunk_level: summary
chunk_type: prose
heading: Verification Steps
token_count: 86
summary:  **Expose configuration:**.    - Add `min_confidence` to `ScrapeConfig`
---

   ```


4. **Expose configuration:**
   - Add `min_confidence` to `ScrapeConfig`
   - Allow per-site confidence tuning

## Verification Steps

Run tests:
```bash
cd doc_transformer
cargo test --lib filter
# Result: ok. 39 passed; 0 failed
```

Build project:
```bash
cargo build
# Result: Finished `dev` profile [unoptimized + debuginfo]
```

