---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#23-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: for a concrete value is not over. In other words, an instance may never violate the constraints of its parent
---

for a concrete value is not over.
In other words, an instance may never violate the constraints of its parent.
This property makes it very hard to inadvertently make false conclusions in CUE.
Default values do not change this property; they syntactically appear as
non-concrete values.
CUE also bails out and requires explicit values if two conflicting defaults
are specified for the same field, again limiting the search space.

With approaches that allow overrides, whether it be the complex inheritance
used in languages like GCL and Jsonnet
