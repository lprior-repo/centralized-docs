---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#28-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 529
summary: /    quotient               integers, floats. For any operator that accepts operands of type float, any operand may be
---

/    quotient               integers, floats

For any operator that accepts operands of type float, any operand may be
of type int or float, in which case the result will be float
if it cannot be represented as an int or if any of the operands are float,
or int otherwise.
So the result of 1 / 2 is 0.5 and is of type float.

The result of division by zero is bottom (an error).

Integer division is implemented through the builtin functions
quo, rem, div, and mod.

The unary operators + and - are defined for numeric values as follows:


Copy code
Copied!

+x                          is 0 + x
-x    negation              is 0 - x


STRING OPERATORS

Strings can be concatenated using the + operator:


Copy code
Copied!

s: "hi " + name + " and good bye"

String addition creates a new string by concatenating the operands.

A string can be repeated by multiplying it:


Copy code
Copied!

s: "etc. "*3  // "etc. etc. etc. "


COMPARISON OPERATORS

Comparison operators compare two concrete operands and yield a boolean value.


Copy code
Copied!

==    equal
!=    not equal
<     less
<=    less or equal
>     greater
>=    greater or equal
=~    matches regular expression
!~    does not match regular expression

In any comparison, both operands must be concrete; otherwise the result is
bottom (_|_).

The equality operators == and != can be applied to any two concrete
operands.
The ordering operators <, <=, >, and >= apply only to operands of the
same ordered type (numeric, string, or bytes).
The matching operators =~ and !~ apply to a string and a regular expression
operand.

For equality comparisons (== and !=):

 * Two values of different basic types are always unequal, except for integers
   and floating-point numbers (see below).
 * Null values are equal only to other null values.
 * Boolean values are equal if they are both true or both false.
 * Numeric values are equal if they represent the same number.
   When comparing an integer with a floating-point number, the integer is first
   converted to floating-point.
 * String values are equal if they contain the same sequence of bytes.
