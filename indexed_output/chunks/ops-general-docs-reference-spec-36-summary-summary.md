---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#36-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: float ⋢ 5. 5     ⋢ 6
---

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
