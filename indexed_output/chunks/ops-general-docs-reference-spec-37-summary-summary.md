---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#37-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: Unification in CUE is a binary expression [/docs/reference/spec/#operands], written a & b. It is commutative, associative, and idempotent
---


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
