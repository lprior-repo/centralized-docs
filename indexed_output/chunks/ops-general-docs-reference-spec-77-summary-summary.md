---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#77-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: Fields, structs and packages can be associated with a set of attributes. Attributes accumulate during unification, but implementations may remove
---


Fields, structs and packages can be associated with a set of attributes.
Attributes accumulate during unification, but implementations may remove
duplicates that have the same source string representation.
The interpretation of an attribute, including the handling of multiple
attributes for a given identifier, is up to the consumer of the attribute.

Field attributes define additional information about a field,
such as a mapping to a protocol buffer tag or alternative
name of the field when mapping to a different language.
