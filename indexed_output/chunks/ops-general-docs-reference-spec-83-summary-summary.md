---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#83-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: A list may be open or closed. An open list is indicated with a 
---

A list may be open or closed.
An open list is indicated with a ... at the end of an element list,
optionally followed by a value for the remaining elements.

The length of a closed list is the number of elements it contains.
The length of an open list is the number of elements as a lower bound
and an unlimited number of elements as its upper bound.


Copy code
Copied!

ListLit       = "[" [ ElementList [ "," ] ] "]" .
ElementList   = Ellipsis | Embedding { "," Embedding } [ "," Ellipsis ] .

Lists can be thought of as structs:
