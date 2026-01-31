---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#74-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: to data and are never required to be concrete. Referencing a definition will recursively close [/docs/reference/spec/#closed-structs] it
---

to data and are never required to be concrete.

Referencing a definition will recursively close [/docs/reference/spec/#closed-structs] it.
That is, a referenced definition will not unify with a struct
that would add a field anywhere within the definition that it does not
already define or explicitly allow with a pattern constraint or ....
Embedding [/docs/reference/spec/#embedding] allows bypassing this check.

If referencing a definition would always result in an error, implementations
may report this inconsistency at the point of its declaration.
