---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#65-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: with any field with name f in a for which p & f is not bottom. When unifying struct a and b,
---

with any field with name f in a for which p & f is not bottom.
When unifying struct a and b,
any pattern constraint declared in a and b
are also declared in the result of unification.

Additionally, a default constraint, denoted ...value, defines a value
to unify with any field for which there is no other declaration in a struct.
When unifying structs a and b,
a default constraint ...v declared in a
defines that the value v should unify with any field in the resulting struct c
whose label does not unify with any of the patterns of the pattern
