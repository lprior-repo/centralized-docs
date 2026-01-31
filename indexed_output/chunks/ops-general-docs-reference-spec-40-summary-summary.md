---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#40-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: A disjunction is normalized if there is no element. a for which there is an element b such that a ⊑ b
---


A disjunction is normalized if there is no element
a for which there is an element b such that a ⊑ b.


DEFAULT VALUES

Any value v may be associated with a default value d,
where d must be in instance of v (d ⊑ v).

Default values are introduced by means of disjunctions.
Any element of a disjunction can be marked as a default
by prefixing it with an asterisk * (a unary expression [/docs/reference/spec/#operators]).
Syntactically consecutive disjunctions are considered to be
part of a single disjunction,
