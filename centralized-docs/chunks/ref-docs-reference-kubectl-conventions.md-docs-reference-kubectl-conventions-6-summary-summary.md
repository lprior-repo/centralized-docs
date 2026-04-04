---
doc_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
chunk_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions#6-summary
chunk_level: summary
chunk_type: prose
heading: Using `kubectl` in Reusable Scripts
token_count: 96
summary: * Tag the image with a version-specific tag and don't move that tag to a new version. For example, use `:v1234`, `v1.2.3`, `r03062016-1-4`, rather than `:latest` (For more information, see...
---

* Tag the image with a version-specific tag and don't move that tag to a new version. For example, use `:v1234`, `v1.2.3`, `r03062016-1-4`, rather than `:latest` (For more information, see [Kubernetes Configuration Good Practices](/blog/2025/11/25/configuration-good-practices/)).
* Check in the script for an image that is heavily parameterized.