---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#13-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 139
summary: must be a letter. Identifiers starting with a # or _ are reserved for definitions and hidden
---

must be a letter.
Identifiers starting with a # or _ are reserved for definitions and hidden
fields.


Copy code
Copied!

identifier  = [ "#" | "_#" ] letter { letter | unicode_digit } .


Copy code
Copied!

a
_x9
fieldName
αβ

Some identifiers are predeclared [/docs/reference/spec/#predeclared-identifiers].

KEYWORDS

CUE has a limited set of keywords.
In addition, CUE reserves all identifiers starting with __ (double underscores)
as keywords.
These are typically targets of pre-declared identifiers.

All keywords may be used as labels (field names).
