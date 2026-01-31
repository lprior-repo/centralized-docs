---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: An important aspect of a lattice is that for every two elements,. there is a unique instance of both elements that subsumes all other
---

are upheld.

An important aspect of a lattice is that for every two elements,
there is a unique instance of both elements that subsumes all other
elements that are an instance of both elements.
This is called the greatest lower bound, or meet.
Now let’s imagine we could define a lattice for, say,
all configurations, schemas and data.
In that case, we could always unambiguously merge two such configurations
independently of order.
This is exactly what CUE does!

CUE’S HIERARCHY

In this section we will introduce CUE’s value hierarchy.
