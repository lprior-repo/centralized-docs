---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#126-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary:  x     y   quo(x, y)  rem(x, y).  5     3        1          2
---



Copy code
Copied!

 x     y   quo(x, y)  rem(x, y)
 5     3        1          2
-5     3       -1         -2
 5    -3       -1          2
-5    -3        1         -2

A zero divisor in either case results in bottom (an error).

BUILTIN VALIDATORS

A validator validates the value at the position where it is defined.
A successful validation yields the original value;
a failed validation yields an error.

Bounds (<10) are a type of validator.

Functions that return a boolean value can be used as validators by omitting
