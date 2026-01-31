---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: The encoding/openapi [https://pkg. dev/cuelang
---

The encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
package provides options to make a definition self-contained, to filter
constraints, and so on. The expanding references option enables the
“Structural OpenAPI” form required by CRDs targeting Kubernetes version 1.15
and later.

FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints with data sources
of truth, no matter where they exist.

