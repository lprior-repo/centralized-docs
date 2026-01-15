---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#22
chunk_level: summary
chunk_type: prose
heading: 6. Output Structure
token_count: 132
summary:     │   └── 20000/.     ├── chunk_generation/
---


```
target/
    │
    │   └── 20000/
    │
    ├── chunk_generation/
    │   ├── 100/
    │   ├── 1000/
    │   ├── 5000/
    │   └── 10000/
    │
    ├── tag_generation/
    │   ├── 100/
    │   ├── 1000/
    │   ├── 5000/
    │   └── 10000/
    │
    └── report/
        ├── index.html (MAIN REPORT)
        ├── index-content.html
        └── assets/
            ├── plotting.js
            └── ...
