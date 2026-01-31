---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#106-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 138
summary: For instance, x / y * z is the same as (x / y) * z. 23 + 3*x[i]
---

For instance, x / y * z is the same as (x / y) * z.


Copy code
Copied!

+x
23 + 3*x[i]
x <= f()
f() || g()
x == y+1 && y == z-1
2 | int
{ a: 1 } & { b: 2 }


ARITHMETIC OPERATORS

Arithmetic operators apply to numeric values and yield a result of the same type
as the first operand. The four standard arithmetic operators
(+, -, *, /) apply to integer and decimal floating-point types;
+ and * also apply to strings and bytes.


Copy code
Copied!

+    sum                    integers, floats, strings, bytes
-    difference             integers, floats
