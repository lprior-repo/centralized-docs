---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: The goal here is to get the big picture, and will only present the details. when it helps for this purpose
---

The goal here is to get the big picture, and will only present the details
when it helps for this purpose.


BOOLEANS

Let’s start simple, with booleans.

bool
true
false
⊥ (bottom)

This diagram shows that CUE interprets both true and false as
an instance of bool.
No surprises there.
What is less ordinary is that, to CUE, bool is just as much a value
as true and false.
For instance, when we say that a value is both a bool and true,
or in lattice terms,
if we find the greatest lower bound of these values,
