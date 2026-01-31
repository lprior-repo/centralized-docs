---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#7-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: causing an ever-growing pile of assertions. CUE defaults, which are values marked with a * in disjunctions,
---

causing an ever-growing pile of assertions.


SEMANTICS

CUE defaults, which are values marked with a * in disjunctions,
preserve the beneficial properties of the lattice.
In order to do so,
CUE must ensure that the order of picking defaults does not influence the outcome.
Suppose we define two fields, each with the same default value.
We also define that these fields are equal to each other.


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
CUE wants no part of these shenanigans.
So the answer in this case is that there are no concrete values
as the defaults cannot be used.

The model for this is actually quite simple.
Conceptually, CUE keeps two parallel values, one for all possible values
and one for the default, which must be an instance of the former.
Roughly speaking, for the example with the conflict,
it simultaneously evaluates:


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
b: 2
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
that do not depend on each other.
