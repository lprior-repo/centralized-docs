---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#16-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1024
summary: error takes a single string argument.  If this argument is a literal
---


error takes a single string argument. If this argument is a literal
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
A successful validation yields the original value;
a failed validation yields an error.

Bounds (<10) are a type of validator.

Functions that return a boolean value can be used as validators by omitting
their first argument.

The remainder of this section defines builtin validators. These can only be
used as validators, so we will not refer to their function equivalents.

These builtins refer to finalized values, which means that the value being
validated is fully resolved, and defaults taken, before it is unified with the
schema.

MATCHN

The matchN builtin is a validator that checks if a specified number of schemas
from a given list unify successfully with the finalized value being validated.

matchN takes two arguments:

 * a numeric constraint specifying how many schemas must match,
 * a list of schemas to test against the value.

The validator evaluates each schema in the list by unifying it with the value.
It counts how many schemas unify successfully (without producing an error).
The validator succeeds if the count satisfies the numeric constraint provided
as the first argument.


Copy code
Copied!

// Exactly 2 schemas must match
value: "foo" & matchN(2, [string, !="bar", <4])  // true: string and !="bar" match

// At least 1 schema must match
value: 5 & matchN(>=1, [int, >10])  // true: int matches

// Exactly 0 schemas must match (none should match)
value: "test" & matchN(0, [int, >100])  // true: neither matches

If the numeric constraint cannot be satisfied even with incomplete information,
the error is marked as incomplete and will be reevaluated as more information
