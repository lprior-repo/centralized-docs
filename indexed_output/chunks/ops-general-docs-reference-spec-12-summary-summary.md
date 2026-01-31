---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary:  * one of the characters ), ], }, or ?.  * an ellipsis 
---

 * one of the characters ), ], }, or ?
 * an ellipsis ...

Although commas are automatically inserted, the parser will require
explicit commas between two list elements.

To reflect idiomatic use, examples in this document elide commas using
these rules.

IDENTIFIERS

Identifiers name entities such as fields and aliases.
An identifier is a sequence of one or more letters (which includes _ and $)
and digits, optionally preceded by # or _#.
It may not be _ or $.
The first character in an identifier, or after an # if it contains one,
