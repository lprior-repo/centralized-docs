---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#32-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: \"lily:\nout of the water\nout of itself\n\n\" +. \"bass\npicking bugs\noff the moon\n\" +
---

Copied!

"lily:\nout of the water\nout of itself\n\n" +
"bass\npicking bugs\noff the moon\n" +
"    — Nick Virgilio, Selected Haiku, 1988"

VALUES

In addition to simple values like "hello" and 42.0, CUE has structs [/docs/reference/spec/#structs].
A struct is a map from labels to values, like {a: 42.0, b: "hello"}.
Structs are CUE’s only way of building up complex values;
lists, which we will see later,
are defined in terms of structs.

All possible values are ordered in a lattice,
a partial order where every two elements have a single greatest lower bound.
