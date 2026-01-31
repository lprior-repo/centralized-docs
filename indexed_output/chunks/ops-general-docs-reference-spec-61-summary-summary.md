---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#61-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: Syntactically, a field is marked as a constraint. by following its label with an optional marker ?
---


Syntactically, a field is marked as a constraint
by following its label with an optional marker ?
or required marker !.
These markers are not part of the field name.

A struct that has a required field constraint with a bottom value
evaluates to bottom.
An optional field constraint with a bottom value does not invalidate
the struct that contains it
as long as it is not unified with a defined field.

The subsumption relation for fields with the various markers is defined as


Copy code
Copied!

{a: x} ⊑ {a!: x} ⊑ {a?: x}
