---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#49-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 132
summary: _ &  5        5. _ &  _        _
---

_ &  5        5
_ &  _        _
_ & _|_      _|_
_ | _|_       _

NULL

The null value is represented with the keyword null.
It has only one parent, top, and one child, bottom.
It is unordered with respect to any other value.


Copy code
Copied!

null_lit   = "null" .


Copy code
Copied!

null & 8     _|_
null & _     null
null & _|_   _|_

BOOLEAN VALUES

A boolean type represents the set of Boolean truth values denoted by
the keywords true and false.
The predeclared boolean type is bool; it is a defined type and a separate
