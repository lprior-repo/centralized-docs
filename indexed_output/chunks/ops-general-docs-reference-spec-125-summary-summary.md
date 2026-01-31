---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#125-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 129
summary: r = x - y*q  with 0 <= r < |y|. where |y| denotes the absolute value of y
---



Copy code
Copied!

r = x - y*q  with 0 <= r < |y|

where |y| denotes the absolute value of y.


Copy code
Copied!

 x     y   div(x, y)  mod(x, y)
 5     3        1          2
-5     3       -2          1
 5    -3       -1          2
-5    -3        2          1

For two integer values x and y,
the integer quotient q = quo(x, y) and remainder r = rem(x, y)
implement truncated division and
satisfy the following relationship:


Copy code
Copied!

x = q*y + r  and  |r| < |y|

with quo(x, y) truncated towards zero.
