---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#16-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: a?: int & <10. a: int & <10
---


a?: int
a: int
a?: int & <10
a: int & <10
Required is more specific than optional
a?: 0
a?: 1
a?: ⊥
a: 0
a: 1
⊥
Conflicting values for optional fields result in disallowing that field, conflicting required fields result in a faulty struct

An important thing to note is that, unlike for required fields,
conflicting values for an optional field do not cause a struct to be faulty.
This definition was a result from fitting the notion of closed structs into
the value lattice.
But it can also be explained with some logic.
