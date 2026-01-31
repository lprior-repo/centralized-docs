---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#28-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: a: int | *1. b: int | *1
---



Copy code
Copied!

a: int | *1
b: int | *1
a: b
b: a

This is fine.
The obvious answer is a: 1, b: 1.

But now suppose we change one of the default values:


Copy code
Copied!

a: int | *1
b: int | *2
a: b
b: a

What should the answer be?
Picking either 1 or 2 as the default would result in a resolution of the
constraints, but would also be highly undesirable, as the result would depend
on the mood of the implementation.
This also starts to smell like an NP-complete constraint solving problem.
(Basic graph unification itself is pseudo linear.)
