---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#27-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 521
summary:  * the value of the regular and non-optional field named x of struct a,.    if this field exists
---

 * the value of the regular and non-optional field named x of struct a,
   if this field exists
 * bottom (an error), otherwise


Copy code
Copied!

a: [ 1, 2 ][1]     // 2
b: [ 1, 2 ][2]     // _|_
c: [ 1, 2, ...][2] // _|_

// Defaults are selected for both operand and index:
x: [1, 2] | *[3, 4]
y: int | *1
z: x[y]  // 4

OPERATORS

Operators combine operands into expressions.


Copy code
Copied!

Expression = UnaryExpr | Expression binary_op Expression .
UnaryExpr  = PrimaryExpr | unary_op UnaryExpr .

binary_op  = "|" | "&" | "||" | "&&" | "==" | rel_op | add_op | mul_op  .
rel_op     = "!=" | "<" | "<=" | ">" | ">=" | "=~" | "!~" .
add_op     = "+" | "-" .
mul_op     = "*" | "/" .
unary_op   = "+" | "-" | "!" | "*" | rel_op .

Comparisons are discussed elsewhere [/docs/reference/spec/#comparison-operators].
For any binary operators, the operand types must unify.


OPERATOR PRECEDENCE

Unary operators have the highest precedence.

There are eight precedence levels for binary operators.
Multiplication operators binds strongest, followed by
addition operators, comparison operators,
&& (logical AND), || (logical OR), & (unification),
and finally | (disjunction):


Copy code
Copied!

Precedence    Operator
    7             *  /
    6             +  -
    5             ==  !=  <  <=  >  >= =~ !~
    4             &&
    3             ||
    2             &
    1             |

Binary operators of the same precedence associate from left to right.
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
*    product                integers, floats, strings, bytes
