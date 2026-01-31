---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#56-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: We say a label is defined for a struct if the struct has a field with the. corresponding label
---


We say a label is defined for a struct if the struct has a field with the
corresponding label.
The value for a label f of struct a is denoted a.f.
A struct a is an instance of b, or a ⊑ b, if for any label f
defined for b, label f is also defined for a and a.f ⊑ b.f.
Note that if a is an instance of b it may have fields with labels that
are not defined for b.

The (unique) struct with no fields, written {}, has every struct as an
instance. It can be considered the type of all structs.


Copy code
Copied!
