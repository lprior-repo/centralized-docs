---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#21-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: undefined and void in addition to null. It is a necessary evil to give null some meaning
---

undefined and void in addition to null.
It is a necessary evil to give null some meaning
that is compatible with common practices,
within the context of its type system.

CUE got lucky.
CUE’s interpretation of null, optionality, and related concepts
is actually inspired by TypeScript.
But because types are values in CUE, TypeScript’s concepts of
undefined, void and null and optional fields, roughly collapse onto CUE’s
null, bottom (_|_), and optional fields,
resulting in a somewhat simpler model.

DEFAULT VALUES
