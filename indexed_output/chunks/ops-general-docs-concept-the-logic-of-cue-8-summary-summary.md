---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: the answer is true. Again maybe no surprise, except that in CUE this is actually
---

the answer is true.
Again maybe no surprise, except that in CUE this is actually
an operation, denoted bool & true.

This also explains the odd fourth element in the graph labeled bottom.
Bottom, in this example, is the result of computing true & false.
A value cannot be both true and false, so this an error.
Bottom is analogous to an error in many other languages.
Bottom is an instance of every value and type, in fact.
More on errors later.

One more detail:
besides the meet operator (&), CUE also has a join operator (|),
