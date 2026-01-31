---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#30-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: Equating a and b clearly results in a conflict (1 != 2) and each will. result in _|_, leaving the left values as the only viable answer
---

a: b
b: a

Equating a and b clearly results in a conflict (1 != 2) and each will
result in _|_, leaving the left values as the only viable answer.

Now consider the two values corresponding to the original example:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 1
a: b
b: a

Here the defaults are not in conflict and can safely be returned.
Note that it is not an all-or-nothing game.
The parallel values are determined on a field-by-field basis.
So defaults can be selected, or not, independently for fields
