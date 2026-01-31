---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#9-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 513
summary: A value is concrete if it is either an atom, or a struct whose field values. of regular (non-hidden and non-definition fields) are all concrete, recursively
---


A value is concrete if it is either an atom, or a struct whose field values
of regular (non-hidden and non-definition fields) are all concrete, recursively.

CUE’s values also include what we normally think of as types, like string and
float.
It does not distinguish between types and values:
only the relationship of values in the lattice is important.
Each CUE “type” subsumes the concrete values that one would normally think
of as part of that type.
For example, "hello" is an instance of string, and 42.0 is an instance of
float.
In addition to string and float, CUE has null, int, bool, and bytes.
We informally call these CUE’s “basic types”.


Copy code
Copied!

false ⊑ bool
true  ⊑ bool
true  ⊑ true
5.0   ⊑ float
bool  ⊑ _
_|_   ⊑ _
_|_   ⊑ _|_

_     ⋢ _|_
_     ⋢ bool
int   ⋢ bool
bool  ⋢ int
false ⋢ true
true  ⋢ false
float ⋢ 5.0
5     ⋢ 6

UNIFICATION

The unification of values a and b
is defined as the greatest lower bound of a and b. (That is, the
value u such that u ⊑ a and u ⊑ b,
and for any other value v for which v ⊑ a and v ⊑ b
it holds that v ⊑ u.)
Since CUE values form a lattice, the unification of two CUE values is
always unique.

These all follow from the definition of unification:

 * The unification of a with itself is always a.
 * The unification of values a and b where a ⊑ b is always a.
 * The unification of a value with bottom is always bottom.

Unification in CUE is a binary expression [/docs/reference/spec/#operands], written a & b.
It is commutative, associative, and idempotent.
As a consequence, order of evaluation is irrelevant, a property that is key
to many of the constructs in the CUE language as well as the tooling layered
on top of it.

DISJUNCTION

The disjunction of values a and b
is defined as the least upper bound of a and b.
(That is, the value d such that a ⊑ d and b ⊑ d,
and for any other value e for which a ⊑ e and b ⊑ e,
it holds that d ⊑ e.)
This style of disjunctions is sometimes also referred to as sum types.
