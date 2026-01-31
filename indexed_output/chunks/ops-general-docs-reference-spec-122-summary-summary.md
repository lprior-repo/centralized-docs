---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#122-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: interpolation, it will be extra resilient: if any of the arguments to the. interpolation fail, they will be printed as an expression
---

interpolation, it will be extra resilient: if any of the arguments to the
interpolation fail, they will be printed as an expression. This allows failing
expressions to be a part of the error message.


Copy code
Copied!

a: 1/0 | error("infinity and beyond!: \(1/0)")

LEN

The builtin function len takes arguments of various types and returns
a result of type int.


Copy code
Copied!

Argument type    Result

bytes            length of byte sequence
list             list length, smallest length for an open list
