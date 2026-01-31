---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#58-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 132
summary: The result of a unification is bottom (_|_) if any of its defined. fields evaluates to bottom, recursively
---

The result of a unification is bottom (_|_) if any of its defined
fields evaluates to bottom, recursively.

A struct literal may contain multiple fields with the same label,
the result of which is the unification of all those fields.


Copy code
Copied!

StructLit       = "{" { Declaration "," } "}" .
Declaration     = Field | Ellipsis | Embedding | LetClause | attribute .
Ellipsis        = "..." [ Expression ] .
Embedding       = Comprehension | AliasExpr .
Field           = Label ":" { Label ":" } AliasExpr { attribute } .
