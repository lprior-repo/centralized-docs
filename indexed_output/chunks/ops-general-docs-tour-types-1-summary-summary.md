---
doc_id: ops/general/docs-tour-types
chunk_id: ops/general/docs-tour-types#1-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary:  Tour [https://cuelang. org/docs/tour/]
---


Menu

 1. Tour [https://cuelang.org/docs/tour/]
 2. Types and Values [https://cuelang.org/docs/tour/types/]


 3. TYPE HIERARCHY

CUE defines the following type hierarchy:

_
_|_
{...}
[...]
null
bool
string
bytes
number
int
float
CUE's predefined type hierarchy

CUE defines the value top (or any),
written “_”, such that all types are an instance of top,
and the value bottom (or error),
written “_|_”,
which is an instance of all types.

We can mix the terms types and values interchangeably because
