---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#6-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1030
summary: (*1|2|3) | (1|*2|3)&2    ⟨1|2|3, 1|2⟩            M1, D1, U1, D2. (*1|2) & (1|*2)          ⟨1|2, _|_⟩              M1, D1, U2
---

(*1|2|3) | (1|*2|3)&2    ⟨1|2|3, 1|2⟩            M1, D1, U1, D2

(*1|2) & (1|*2)          ⟨1|2, _|_⟩              M1, D1, U2

The rules of subsumption for defaults can be derived from the above definitions
and are as follows.


Copy code
Copied!

⟨v2, d2⟩ ⊑ ⟨v1, d1⟩  if v2 ⊑ v1 and d2 ⊑ d1
⟨v1, d1⟩ ⊑ ⟨v⟩       if v1 ⊑ v
⟨v⟩      ⊑ ⟨v1, d1⟩  if v ⊑ d1


Copy code
Copied!

Expression                       Resolves to
"tcp" | "udp"                    "tcp" | "udp"
*"tcp" | "udp"                   "tcp"
float | *1                       1
*string | 1.0                    string
(*1|2) + (2|*3)                  4

(*1|2|3) | (1|*2|3)              1|2
(*1|2|3) & (1|*2|3)              1|2|3 // default is _|_

(* >=5 | int) & (* <=5 | int)    5

(*"tcp"|"udp") & ("udp"|*"tcp")  "tcp"
(*"tcp"|"udp") & ("udp"|"tcp")   "tcp"
(*"tcp"|"udp") & "tcp"           "tcp"
(*"tcp"|"udp") & (*"udp"|"tcp")  "tcp" | "udp" // default is _|_

(*true | false) & bool           true
(*true | false) & (true | false) true

{a: 1} | {b: 1}                  {a: 1} | {b: 1}
{a: 1} | *{b: 1}                 {b:1}
*{a: 1} | *{b: 1}                {a: 1} | {b: 1}
({a: 1} | {b: 1}) & {a:1}        {a:1}  | {a: 1, b: 1}
({a:1}|*{b:1}) & ({a:1}|*{b:1})  {b:1}

BOTTOM AND ERRORS

Any evaluation error in CUE results in a bottom value, represented by
the token _|_.
Bottom is an instance of every other value.
Any evaluation error is represented as bottom.

Implementations may associate error strings with different instances of bottom;
logically they all remain the same value.


Copy code
Copied!

bottom_lit = "_|_" .

TOP

Top is represented by the underscore character _, lexically an identifier.
Unifying any value v with top results in v itself.


Copy code
Copied!

Expr        Result
_ &  5        5
_ &  _        _
_ & _|_      _|_
_ | _|_       _

NULL

The null value is represented with the keyword null.
It has only one parent, top, and one child, bottom.
It is unordered with respect to any other value.


Copy code
Copied!

null_lit   = "null" .


Copy code
Copied!

null & 8     _|_
null & _     null
null & _|_   _|_

BOOLEAN VALUES

A boolean type represents the set of Boolean truth values denoted by
the keywords true and false.
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
