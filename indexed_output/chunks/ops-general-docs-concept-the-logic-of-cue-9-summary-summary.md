---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: which computes the least upper bound. The result of true | false is indeed bool in CUE
---

which computes the least upper bound.
The result of true | false is indeed bool in CUE.


NUMBERS

With numbers things get a bit more interesting.
CUE has gone through various iterations of the number type system
to find the mix of being practical and strict, while still being simple.
CUE recognizes number, and the instances int and float as classes
of numbers.
For now it suffices to only consider number and int, the latter being
an instance of the former.

Let’s consider a lattice with some example numeric values.
