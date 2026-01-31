---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#42-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: is only relevant when both an outer and nested disjunction are marked. Intuitively, when an expression needs to be resolved for an operation other
---

is only relevant when both an outer and nested disjunction are marked.

Intuitively, when an expression needs to be resolved for an operation other
than unification or disjunction,
non-starred elements are dropped in favor of starred ones if the starred ones
do not resolve to bottom.

To define the unification and disjunction operation we use the notation
⟨v⟩ to denote a CUE value v that is not associated with a default
and the notation ⟨v, d⟩ to denote a value v associated with a default
value d.

