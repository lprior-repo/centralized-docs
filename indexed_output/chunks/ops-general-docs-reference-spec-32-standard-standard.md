---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#32-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 519
summary: a result of type int. Argument type    Result
---

a result of type int.


Copy code
Copied!

Argument type    Result

bytes            length of byte sequence
list             list length, smallest length for an open list
struct           number of distinct data fields, excluding field constraints


Copy code
Copied!

Expression           Result
len("Hellø")         6
len([1, 2, 3])       3
len([1, 2, ...])     2

CLOSE

The builtin function close converts a partially defined, or open, struct
to a fully defined, or closed, struct.

AND

The builtin function and takes a list and returns the result of applying
the & operator to all elements in the list.
It returns top for the empty list.


Copy code
Copied!

Expression:          Result
and([a, b])          a & b
and([a])             a
and([])              _

OR

The builtin function or takes a list and returns the result of applying
the | operator to all elements in the list.
It returns bottom for the empty list.


Copy code
Copied!

Expression:          Result
or([a, b])           a | b
or([a])              a
or([])               _|_

DIV, MOD, QUO AND REM

For two integer values x and y,
the integer quotient q = div(x, y) and remainder r = mod(x, y)
implement Euclidean division and
satisfy the following relationship:


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
