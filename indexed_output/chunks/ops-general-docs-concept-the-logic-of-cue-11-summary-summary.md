---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: the traditional concepts of value and type. CUE defines the constraints we see here in terms of its binary operators
---

the traditional concepts of value and type.

CUE defines the constraints we see here in terms of its binary operators
>= and <.
It allows all binary operators that result in a boolean, except ==,
to be used as a constraint by leaving off the left value,
where op B defines the set of all values A for which A op B is true.
The constraint <10 means all numbers less than 10.
Note that we say all numbers, even though 10 is an integer.
This is because CUE allows implicit conversion between
number types in comparisons.
