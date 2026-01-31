---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#26-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: know these constraints apply. Such information is not readily available for
---

know these constraints apply.
Such information is not readily available for
fully expanded configurations.1
But also with inheritance-based solutions
that allow arbitrary overrides, templates give little information.

The ability to enforce constraints top down is crucial for any
large-scale configuration setup.
GCL and Jsonnet address this with assertions.
Assertions, however, are typically decoupled from their fields,
making them both hard to discover and hard to reason about.
Where CUE simplifies constraints
