---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#34-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: An astute reader may have noticed that there were cyclic references. between fields in some of the examples,
---


CYCLES

An astute reader may have noticed that there were cyclic references
between fields in some of the examples,
something that is not allowed in your typical programming or
configuration language.
CUE’s underlying model allows reasoning over cycles.
Consider a CUE program defining two fields;


Copy code
Copied!

a: b
b: a

This can only be interpreted to mean that a and b must be equal.
If there is no concrete value assigned to a or b,
they remain unspecified in the same way as if each had been declared as string.
