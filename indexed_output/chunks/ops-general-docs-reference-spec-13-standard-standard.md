---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#13-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 516
summary: The predeclared boolean type is bool; it is a defined type and a separate. element in the lattice
---

The predeclared boolean type is bool; it is a defined type and a separate
element in the lattice.


Copy code
Copied!

bool_lit = "true" | "false" .


Copy code
Copied!

bool & true          true
true & true          true
true & false         _|_
bool & (false|true)  false | true
bool & (true|false)  true | false

NUMERIC VALUES

The integer type represents the set of all integral numbers.
The decimal floating-point type represents the set of all decimal floating-point
numbers.
They are two distinct types.
Both are instances instances of a generic number type.

The predeclared number, integer, and decimal floating-point types are
number, int and float; they are defined types.

A decimal floating-point literal always has type float;
it is not an instance of int even if it is an integral number.

Integer literals are always of type int and don’t match type float.

Numeric literals are exact values of arbitrary precision.
If the operation permits it, numbers should be kept in arbitrary precision.

Implementation restriction: although numeric values have arbitrary precision
in the language, implementations may implement them using an internal
representation with limited precision.
That said, every implementation must:

 * Represent integer values with at least 256 bits.
 * Represent floating-point values with a mantissa of at least 256 bits and
   a signed binary exponent of at least 16 bits.
 * Give an error if unable to represent an integer value precisely.
 * Give an error if unable to represent a floating-point value due to overflow.
 * Round to the nearest representable value if unable to represent
   a floating-point value due to limits on precision.
   These requirements apply to the result of any expression except for builtin
   functions, for which an unusual loss of precision must be explicitly documented.

STRINGS

The string type represents the set of UTF-8 strings,
not allowing surrogates.
The predeclared string type is string; it is a defined type.

The length of a string s (its size in bytes) can be discovered using
