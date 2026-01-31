---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#31-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: that do not depend on each other. REASONING AND INFERENCE
---

that do not depend on each other.

REASONING AND INFERENCE

The values lattice brings CUE another advantage: the ability to reason about
values, schemas, and constraints.

We already discussed how limiting inheritance,
whether language-based or file-based,
makes it easier for people to reason about values.
But it also makes it easier for machines.

BOILERPLATE REMOVAL

CUE’s severe restrictions on inheritance limit its
ability to define hierarchies of templates to remove boilerplate.
But CUE provides some new mechanisms for removing boilerplate.
