---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#38-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: it holds that d ⊑ e. This style of disjunctions is sometimes also referred to as sum types
---

it holds that d ⊑ e.)
This style of disjunctions is sometimes also referred to as sum types.
Since CUE values form a lattice, the disjunction of two CUE values is always unique.

These all follow from the definition of disjunction:

 * The disjunction of a with itself is always a.
 * The disjunction of a value a and b where a ⊑ b is always b.
 * The disjunction of a value a with bottom is always a.
 * The disjunction of two bottom values is bottom.

Disjunction in CUE is a binary expression [/docs/reference/spec/#operands], written a | b.
