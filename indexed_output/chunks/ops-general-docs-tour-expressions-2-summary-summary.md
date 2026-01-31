---
doc_id: ops/general/docs-tour-expressions
chunk_id: ops/general/docs-tour-expressions#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: built-in functions [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]. that produce an int by calculating integer division and remainder
---

built-in functions [/docs/howto/use-the-built-in-functions-div-mod-quo-rem/]
that produce an int by calculating integer division and remainder.
They support Euclidean division (div / mod) and truncated division (quo / rem).

Copied!
operators.cue

Copy code
Copied!

a: 1 + 1             // type int
b: "xxx" + "OOO"     // type string
c: 2 * 2             // type int
d: 2 * 2.0           // type float
e: 3.14159 - 0.14159 // type float

f: 6 / 2     // type: float
g: 6.0 / 2.0 // type: float

h: 1 <= 2.0 // type bool
